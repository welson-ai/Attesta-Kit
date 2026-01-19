use sha2::{Digest, Sha256};
use crate::errors::CryptoError;
use crate::p256_verify::verify_p256_signature;

/// All the parts of a WebAuthn signature that we need to verify it
///
/// When a user authenticates with their passkey (TouchID, FaceID, etc.),
/// the browser/device creates this structure. We store it and use it to
/// verify the signature on-chain without ever seeing the private key.
#[derive(Debug, Clone)]
pub struct WebAuthnSignature {
    /// The raw data from the authenticator (contains flags, counter, etc.)
    /// This tells us things like whether the user was present, verified, etc.
    pub authenticator_data: Vec<u8>,
    
    /// The client-side data as JSON (contains the challenge, origin, type of operation)
    /// This proves the signature was created in response to our specific challenge
    pub client_data_json: Vec<u8>,
    
    /// The actual signature over the combined authenticator_data + client_data_json hash
    /// This is what we verify using the public key
    pub signature: Vec<u8>,
    
    /// The credential ID that identifies which passkey was used
    /// This helps us find the right public key to verify with
    pub credential_id: Vec<u8>,
}

impl WebAuthnSignature {
    /// Creates a new WebAuthnSignature from all its parts
    ///
    /// This is the simplest way to create a WebAuthnSignature when you already
    /// have all the pieces from a WebAuthn authentication.
    pub fn new(
        authenticator_data: Vec<u8>,
        client_data_json: Vec<u8>,
        signature: Vec<u8>,
        credential_id: Vec<u8>,
    ) -> Self {
        Self {
            authenticator_data,
            client_data_json,
            signature,
            credential_id,
        }
    }

    /// Converts this signature into bytes so we can store it on-chain
    ///
    /// The format is: length1 + data1 + length2 + data2 + ...
    /// We store the length of each field before the field itself so we know
    /// how to read it back later.
    pub fn to_bytes(&self) -> Vec<u8> {
        // Calculate total size to avoid reallocations
        let total_size = 4 * 4  // Four length fields (u32 each = 4 bytes)
            + self.authenticator_data.len()
            + self.client_data_json.len()
            + self.signature.len()
            + self.credential_id.len();
        
        let mut bytes = Vec::with_capacity(total_size);
        
        // Write each field: length first, then the actual data
        bytes.extend_from_slice(&(self.authenticator_data.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&self.authenticator_data);
        
        bytes.extend_from_slice(&(self.client_data_json.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&self.client_data_json);
        
        bytes.extend_from_slice(&(self.signature.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&self.signature);
        
        bytes.extend_from_slice(&(self.credential_id.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&self.credential_id);
        
        bytes
    }

    /// Reads bytes back into a WebAuthnSignature
    ///
    /// This does the opposite of `to_bytes()`. It reads the length-prefixed
    /// format and reconstructs the signature structure.
    ///
    /// # Returns
    /// - `Ok(WebAuthnSignature)` if the data is valid
    /// - `Err(CryptoError::InvalidSignatureFormat)` if the data is corrupted or incomplete
    pub fn from_bytes(data: &[u8]) -> Result<Self, CryptoError> {
        // Helper function to read a u32 length and advance the offset
        fn read_length(data: &[u8], offset: &mut usize) -> Result<usize, CryptoError> {
            if *offset + 4 > data.len() {
                return Err(CryptoError::InvalidSignatureFormat);
            }
            let len = u32::from_le_bytes([
                data[*offset],
                data[*offset + 1],
                data[*offset + 2],
                data[*offset + 3],
            ]) as usize;
            *offset += 4;
            Ok(len)
        }

        // Helper function to read a slice of bytes
        fn read_bytes(data: &[u8], offset: &mut usize, len: usize) -> Result<Vec<u8>, CryptoError> {
            if *offset + len > data.len() {
                return Err(CryptoError::InvalidSignatureFormat);
            }
            let result = data[*offset..*offset + len].to_vec();
            *offset += len;
            Ok(result)
        }

        let mut offset = 0;

        // Read authenticator_data
        let auth_data_len = read_length(data, &mut offset)?;
        let authenticator_data = read_bytes(data, &mut offset, auth_data_len)?;

        // Read client_data_json
        let client_data_len = read_length(data, &mut offset)?;
        let client_data_json = read_bytes(data, &mut offset, client_data_len)?;

        // Read signature
        let sig_len = read_length(data, &mut offset)?;
        let signature = read_bytes(data, &mut offset, sig_len)?;

        // Read credential_id
        let cred_id_len = read_length(data, &mut offset)?;
        let credential_id = read_bytes(data, &mut offset, cred_id_len)?;

        Ok(Self {
            authenticator_data,
            client_data_json,
            signature,
            credential_id,
        })
    }
}

/// Verifies that a WebAuthn signature is valid
///
/// This checks that:
/// 1. The signature was created by the private key matching the public key
/// 2. The challenge in the signature matches what we expected
/// 3. The signature format is correct
///
/// # Parameters
/// - `webauthn_sig`: The complete WebAuthn signature structure
/// - `public_key`: The public key from the passkey (64 bytes, uncompressed)
/// - `expected_challenge`: The challenge we sent - must match what's in the signature
///
/// # Returns
/// - `Ok(())` if the signature is valid and the challenge matches
/// - `Err(CryptoError)` if anything is wrong
///
/// # How it works
/// WebAuthn signatures work by signing a combination of:
/// - The authenticator data (from the device)
/// - The hash of the client data JSON (from the browser)
///
/// We reconstruct this same combination and verify the signature matches.
pub fn verify_webauthn_signature(
    webauthn_sig: &WebAuthnSignature,
    public_key: &[u8],
    expected_challenge: &[u8],
) -> Result<(), CryptoError> {
    // Authenticator data must be at least 37 bytes (RP ID hash + flags + counter)
    // If it's shorter, the data is definitely invalid
    const MIN_AUTHENTICATOR_DATA_LEN: usize = 37;
    if webauthn_sig.authenticator_data.len() < MIN_AUTHENTICATOR_DATA_LEN {
        return Err(CryptoError::InvalidAuthenticatorData);
    }

    // Check that the client_data_json contains our expected challenge
    // This ensures the signature was created in response to our specific request
    let client_data_str = String::from_utf8_lossy(&webauthn_sig.client_data_json);
    
    // Convert expected_challenge to a string for searching (but handle errors gracefully)
    let expected_challenge_str = std::str::from_utf8(expected_challenge)
        .unwrap_or("");
    
    if expected_challenge_str.is_empty() || !client_data_str.contains(expected_challenge_str) {
        return Err(CryptoError::ChallengeMismatch);
    }

    // Hash the client data JSON using SHA-256
    // This is part of the WebAuthn specification
    let client_data_hash = Sha256::digest(&webauthn_sig.client_data_json);

    // Build the exact message that was signed
    // Format: authenticator_data (variable length) + client_data_hash (32 bytes)
    let message_len = webauthn_sig.authenticator_data.len() + client_data_hash.len();
    let mut message = Vec::with_capacity(message_len);
    message.extend_from_slice(&webauthn_sig.authenticator_data);
    message.extend_from_slice(&client_data_hash);

    // Now verify the signature over this combined message
    verify_p256_signature(&message, &webauthn_sig.signature, public_key)?;

    Ok(())
}
