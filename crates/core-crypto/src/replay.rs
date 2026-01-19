use sha2::{Digest, Sha256};
use crate::errors::CryptoError;

/// Tools for preventing replay attacks
///
/// A replay attack is when someone tries to use the same transaction twice.
/// We prevent this by using "nonces" - numbers that can only be used once.
///
/// Think of it like a one-time password that expires after use.
pub struct ReplayProtection;

impl ReplayProtection {
    /// Creates a unique nonce from a message, timestamp, and user's public key
    ///
    /// We combine these inputs and hash them to create a unique identifier
    /// for this specific transaction at this specific time for this specific user.
    ///
    /// # Parameters
    /// - `message`: The transaction message or data being protected
    /// - `timestamp`: The current time (helps ensure uniqueness)
    /// - `user_pubkey`: The user's public key (ties the nonce to a specific user)
    ///
    /// # Returns
    /// A 32-byte nonce that's unique for this combination of inputs
    pub fn generate_nonce(message: &[u8], timestamp: i64, user_pubkey: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(message);
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(user_pubkey);
        hasher.finalize().into()
    }

    /// Checks if a nonce has the right format (must be exactly 32 bytes)
    ///
    /// This is a quick validation before we try to use the nonce.
    /// The actual check for whether it's been used happens elsewhere.
    pub fn validate_nonce_format(nonce: &[u8]) -> Result<(), CryptoError> {
        const NONCE_SIZE: usize = 32;
        if nonce.len() != NONCE_SIZE {
            return Err(CryptoError::InvalidNonce);
        }
        Ok(())
    }

    /// Checks if a nonce has already been used
    ///
    /// If a nonce has been used before, allowing it again would let someone
    /// replay an old transaction. This function helps prevent that.
    ///
    /// # Note
    /// In production, you'd check against on-chain storage or a database
    /// of used nonces. This is a simplified version that checks a local array.
    ///
    /// # Parameters
    /// - `nonce`: The nonce to check
    /// - `used_nonces`: A list of nonces that have already been used
    ///
    /// # Returns
    /// `true` if the nonce has been used before (don't allow the transaction),
    /// `false` if it's new (transaction is allowed)
    pub fn is_nonce_used(nonce: &[u8; 32], used_nonces: &[[u8; 32]]) -> bool {
        // Simple linear search - fine for small lists
        // For production, consider using a HashSet or checking on-chain
        used_nonces.contains(nonce)
    }

    /// Marks a nonce as used so it can't be reused
    ///
    /// After we've successfully processed a transaction, we need to record
    /// that its nonce has been used. This prevents anyone from submitting
    /// the same transaction again.
    ///
    /// # Parameters
    /// - `nonce`: The nonce that was just used
    /// - `used_nonces`: The list to add it to (will be updated in place)
    pub fn mark_nonce_used(nonce: &[u8; 32], used_nonces: &mut Vec<[u8; 32]>) {
        // Only add if not already present (idempotent operation)
        if !used_nonces.contains(nonce) {
            used_nonces.push(*nonce);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_nonce() {
        let message = b"test message";
        let timestamp = 1234567890i64;
        let user_pubkey = b"test public key";

        let nonce = ReplayProtection::generate_nonce(message, timestamp, user_pubkey);
        assert_eq!(nonce.len(), 32);
    }

    #[test]
    fn test_generate_nonce_different_inputs_different_outputs() {
        let message1 = b"message 1";
        let message2 = b"message 2";
        let timestamp = 1234567890i64;
        let user_pubkey = b"test public key";

        let nonce1 = ReplayProtection::generate_nonce(message1, timestamp, user_pubkey);
        let nonce2 = ReplayProtection::generate_nonce(message2, timestamp, user_pubkey);

        assert_ne!(nonce1, nonce2);
    }

    #[test]
    fn test_validate_nonce_format_valid() {
        let nonce = [0u8; 32];
        assert!(ReplayProtection::validate_nonce_format(&nonce).is_ok());
    }

    #[test]
    fn test_validate_nonce_format_invalid() {
        let nonce = [0u8; 31];
        assert!(ReplayProtection::validate_nonce_format(&nonce).is_err());
        
        let nonce = [0u8; 33];
        assert!(ReplayProtection::validate_nonce_format(&nonce).is_err());
    }

    #[test]
    fn test_is_nonce_used() {
        let nonce1 = [1u8; 32];
        let nonce2 = [2u8; 32];
        let mut used_nonces = vec![nonce1];

        assert!(ReplayProtection::is_nonce_used(&nonce1, &used_nonces));
        assert!(!ReplayProtection::is_nonce_used(&nonce2, &used_nonces));
    }

    #[test]
    fn test_mark_nonce_used() {
        let nonce = [1u8; 32];
        let mut used_nonces = Vec::new();

        ReplayProtection::mark_nonce_used(&nonce, &mut used_nonces);
        assert!(used_nonces.contains(&nonce));
        assert_eq!(used_nonces.len(), 1);

        // Adding same nonce again shouldn't duplicate
        ReplayProtection::mark_nonce_used(&nonce, &mut used_nonces);
        assert_eq!(used_nonces.len(), 1);
    }
}
