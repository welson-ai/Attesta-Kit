use p256::ecdsa::{Signature, VerifyingKey};
use sha2::{Digest, Sha256};
use crate::errors::CryptoError;

/// Checks if a P-256 signature is valid
///
/// This function takes a message, a signature, and a public key, then verifies
/// that the signature was created by the private key that matches the public key.
///
/// # Parameters
/// - `message`: The original message that was signed
/// - `signature`: The signature bytes (either 64 or 65 bytes long)
/// - `public_key`: The uncompressed public key (64 bytes: x coordinate + y coordinate)
///
/// # Returns
/// - `Ok(())` if the signature is valid
/// - `Err(CryptoError)` if the signature is invalid or inputs are malformed
///
/// # Examples
/// ```
/// use core_crypto::p256_verify::verify_p256_signature;
/// let message = b"Hello, world!";
/// let signature = &[0u8; 64]; // actual signature would go here
/// let public_key = &[0u8; 64]; // actual public key would go here
/// // verify_p256_signature(message, signature, public_key)?;
/// ```
pub fn verify_p256_signature(
    message: &[u8],
    signature: &[u8],
    public_key: &[u8],
) -> Result<(), CryptoError> {
    // Make sure we have the right length of public key
    if public_key.len() != 64 {
        return Err(CryptoError::InvalidP256PublicKey);
    }

    // Hash the message using SHA-256
    // We hash first because ECDSA signatures work with hashed messages
    let message_hash = Sha256::digest(message);

    // Convert the public key bytes into a format we can use for verification
    let verifying_key = VerifyingKey::from_sec1_bytes(public_key)
        .map_err(|_| CryptoError::InvalidP256PublicKey)?;

    // Handle different signature formats
    // Some signatures are 64 bytes (just r + s), others are 65 bytes (r + s + recovery id)
    let sig_bytes: &[u8] = match signature.len() {
        64 => signature,
        65 => {
            // If 65 bytes, use only the first 64 (skip the recovery id)
            signature.get(..64)
                .ok_or(CryptoError::InvalidSignatureFormat)?
        },
        _ => return Err(CryptoError::InvalidSignatureFormat),
    };

    // Convert the signature bytes into a Signature object
    // TryFrom is more explicit and safer than into()
    let sig = Signature::try_from(sig_bytes)
        .map_err(|_| CryptoError::InvalidSignatureFormat)?;

    // Actually verify the signature matches the message and public key
    verifying_key
        .verify(&message_hash, &sig)
        .map_err(|_| CryptoError::SignatureVerificationFailed)?;

    Ok(())
}

/// Converts a compressed public key to uncompressed format
///
/// Compressed keys are 33 bytes (just x coordinate + a sign bit), while
/// uncompressed keys are 65 bytes (0x04 prefix + x + y). This function
/// expands the compressed key to get the full 64 bytes (x + y, no prefix).
///
/// # Parameters
/// - `compressed`: A compressed P-256 public key (must be exactly 33 bytes)
///
/// # Returns
/// - `Ok([u8; 64])` with the uncompressed key (x and y coordinates)
/// - `Err(CryptoError)` if the input is invalid
pub fn decompress_p256_public_key(compressed: &[u8]) -> Result<[u8; 64], CryptoError> {
    // Compressed keys must be exactly 33 bytes
    if compressed.len() != 33 {
        return Err(CryptoError::InvalidP256PublicKey);
    }

    // Parse the compressed key
    let verifying_key = VerifyingKey::from_sec1_bytes(compressed)
        .map_err(|_| CryptoError::InvalidP256PublicKey)?;

    // Convert to uncompressed format (65 bytes: 0x04 prefix + 32 bytes x + 32 bytes y)
    let point = verifying_key.to_encoded_point(false);
    let coords = point.as_bytes();
    
    // Make sure we have enough bytes
    if coords.len() < 65 {
        return Err(CryptoError::InvalidP256PublicKey);
    }

    // Extract just the x and y coordinates (skip the 0x04 prefix)
    let mut uncompressed = [0u8; 64];
    uncompressed.copy_from_slice(
        coords.get(1..65)
            .ok_or(CryptoError::InvalidP256PublicKey)?
    );

    Ok(uncompressed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_p256_signature_invalid_key_length() {
        let message = b"test message";
        let signature = &[0u8; 64];
        let public_key = &[0u8; 32]; // Wrong length

        let result = verify_p256_signature(message, signature, public_key);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CryptoError::InvalidP256PublicKey);
    }

    #[test]
    fn test_verify_p256_signature_invalid_signature_length() {
        let message = b"test message";
        let signature = &[0u8; 32]; // Wrong length
        let public_key = &[0u8; 64];

        let result = verify_p256_signature(message, signature, public_key);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CryptoError::InvalidSignatureFormat);
    }

    #[test]
    fn test_decompress_p256_public_key_invalid_length() {
        let compressed = &[0u8; 32]; // Wrong length
        let result = decompress_p256_public_key(compressed);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CryptoError::InvalidP256PublicKey);
    }
}
