use sha2::{Digest, Sha256};
use borsh::{BorshDeserialize, BorshSerialize};

/// Encrypted backup of account recovery information
/// This enables users to recover their account even if they lose all devices
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct EncryptedBackup {
    /// Hash of the encryption key (for verification)
    /// The actual key should be derived from a user's recovery phrase or secret
    pub key_hash: [u8; 32],
    
    /// Encrypted data containing:
    /// - Passkey public keys
    /// - Credential IDs
    /// - Policy configurations
    /// - Account metadata
    pub encrypted_data: Vec<u8>,
    
    /// Nonce/IV used for encryption (should be random for each backup)
    pub nonce: [u8; 12], // 96 bits for AES-GCM
    
    /// Timestamp when backup was created
    pub created_at: i64,
    
    /// Version of the backup format (for future compatibility)
    pub version: u8,
}

impl EncryptedBackup {
    /// Creates a new encrypted backup
    /// Note: In production, use proper AES-GCM encryption
    /// This is a simplified version for structure
    pub fn new(
        encryption_key: &[u8],
        account_data: &[u8],
        created_at: i64,
    ) -> Self {
        // Hash the encryption key for verification
        let key_hash = Sha256::digest(encryption_key);
        let mut key_hash_array = [0u8; 32];
        key_hash_array.copy_from_slice(&key_hash);

        // Generate a random nonce (in production, use secure random)
        // For now, derive from timestamp and key
        let mut nonce = [0u8; 12];
        let nonce_input = Sha256::digest(&[encryption_key, &created_at.to_le_bytes()].concat());
        nonce.copy_from_slice(&nonce_input[..12]);

        // In production: Encrypt account_data using AES-GCM with encryption_key and nonce
        // For now, we'll just store a placeholder
        let encrypted_data = account_data.to_vec(); // Should be encrypted in production

        Self {
            key_hash: key_hash_array,
            encrypted_data,
            nonce,
            created_at,
            version: 1,
        }
    }

    /// Verifies that the provided key matches the backup's key hash
    pub fn verify_key(&self, encryption_key: &[u8]) -> bool {
        let key_hash = Sha256::digest(encryption_key);
        key_hash.as_slice() == self.key_hash
    }

    /// Decrypts the backup data (simplified - in production use AES-GCM)
    /// Returns the decrypted account data if the key is correct
    pub fn decrypt(&self, encryption_key: &[u8]) -> Result<Vec<u8>, &'static str> {
        // Verify the key
        if !self.verify_key(encryption_key) {
            return Err("Invalid encryption key");
        }

        // In production: Decrypt encrypted_data using AES-GCM with encryption_key and nonce
        // For now, just return the data (since we didn't actually encrypt it)
        Ok(self.encrypted_data.clone())
    }

    /// Serializes the backup to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        borsh::to_vec(self)
    }

    /// Deserializes bytes into an EncryptedBackup
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        borsh::from_slice(data)
    }
}

/// Helper for deriving an encryption key from a recovery phrase
pub fn derive_backup_key(recovery_phrase: &str) -> [u8; 32] {
    let hash = Sha256::digest(recovery_phrase.as_bytes());
    let mut key = [0u8; 32];
    key.copy_from_slice(&hash);
    key
}
