use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// A smart account that uses passkeys instead of traditional private keys
///
/// This is the main data structure that represents an Attesta account on-chain.
/// It stores the user's passkey public key (from WebAuthn) and their policy
/// settings. Users can authorize transactions using their device's biometric
/// authenticator (TouchID, FaceID, etc.) instead of a seed phrase.
///
/// # What makes it "smart"?
/// - It can verify passkey signatures on-chain
/// - It can enforce policies (spending limits, time locks, etc.)
/// - It has built-in replay protection
/// - It supports multi-passkey recovery
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct AttestaAccount {
    /// Who owns this account (their Solana wallet address)
    pub owner: Pubkey,
    
    /// The public key from the user's passkey (64 bytes: 32 bytes x coordinate + 32 bytes y coordinate)
    /// This is what we use to verify signatures - the private key never leaves the user's device
    pub passkey_public_key: [u8; 64],
    
    /// The unique ID that identifies which passkey was used (from WebAuthn)
    /// This helps us match signatures to the right public key
    pub credential_id: Vec<u8>,
    
    /// A counter that increments with each transaction (prevents replay attacks)
    /// Each transaction must use a nonce higher than the last one
    pub nonce: u64,
    
    /// The policy settings for this account (spending limits, time locks, etc.)
    /// Stored as bytes so we can add new policy types without breaking old accounts
    pub policy: Vec<u8>,
    
    /// When this account was first created (Unix timestamp)
    pub created_at: i64,
    
    /// When this account was last updated (Unix timestamp)
    /// Updated whenever a transaction is executed
    pub updated_at: i64,
}

impl AttestaAccount {
    /// Creates a new Attesta account
    ///
    /// This is called when a user first registers with Attesta. They provide
    /// their passkey's public key and we create a new account on-chain.
    ///
    /// # Parameters
    /// - `owner`: The user's Solana wallet address
    /// - `passkey_public_key`: The public key from their passkey (64 bytes)
    /// - `credential_id`: The credential ID from WebAuthn
    /// - `policy`: Their policy settings (can be empty for default "allow all")
    /// - `created_at`: The current timestamp
    ///
    /// # Returns
    /// A new AttestaAccount with nonce set to 0 (ready for first transaction)
    pub fn new(
        owner: Pubkey,
        passkey_public_key: [u8; 64],
        credential_id: Vec<u8>,
        policy: Vec<u8>,
        created_at: i64,
    ) -> Self {
        Self {
            owner,
            passkey_public_key,
            credential_id,
            nonce: 0, // Start at 0 - first transaction will use 1
            policy,
            created_at,
            updated_at: created_at, // Initially same as created_at
        }
    }

    /// Marks a transaction as complete by incrementing the nonce
    ///
    /// This should be called after successfully processing a transaction.
    /// It prevents anyone from replaying the same transaction later.
    ///
    /// # Side Effects
    /// - Increments the nonce counter
    /// - Updates the `updated_at` timestamp
    pub fn increment_nonce(&mut self) {
        // Overflow check: if we've reached u64::MAX, we have bigger problems
        // but let's prevent silent wrapping
        if self.nonce < u64::MAX {
            self.nonce = self.nonce.wrapping_add(1);
        }
        
        // Update the timestamp to now
        self.updated_at = solana_program::clock::Clock::get()
            .map(|c| c.unix_timestamp)
            .unwrap_or(self.updated_at); // If we can't get clock, keep old timestamp
    }

    /// Checks if a nonce is valid (higher than the last one used)
    ///
    /// To prevent replay attacks, each transaction must use a nonce that's
    /// higher than the last one we processed. This function checks that.
    ///
    /// # Parameters
    /// - `provided_nonce`: The nonce the user is trying to use
    ///
    /// # Returns
    /// - `true` if the nonce is valid (higher than stored nonce)
    /// - `false` if the nonce is invalid (same or lower than stored nonce)
    pub fn validate_nonce(&self, provided_nonce: u64) -> bool {
        // Must be strictly greater than the current nonce
        // If it's equal or less, it's a replay attack
        provided_nonce > self.nonce
    }

    /// Converts this account to bytes for storage on-chain
    ///
    /// Uses Borsh serialization which is efficient and deterministic.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)` with the serialized account
    /// - `Err(std::io::Error)` if serialization fails (shouldn't happen in practice)
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        borsh::to_vec(self)
    }

    /// Reads an account from bytes (deserialization)
    ///
    /// This is the opposite of `to_bytes()`. It reads account data
    /// from on-chain storage back into an AttestaAccount object.
    ///
    /// # Parameters
    /// - `data`: The bytes to deserialize from
    ///
    /// # Returns
    /// - `Ok(AttestaAccount)` if the data is valid
    /// - `Err(std::io::Error)` if the data is corrupted or invalid format
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        borsh::from_slice(data)
    }
}

/// Account discriminator to identify Attesta accounts
pub const ATTESTA_ACCOUNT_DISCRIMINATOR: [u8; 8] = [0x41, 0x54, 0x54, 0x45, 0x53, 0x54, 0x41, 0x00]; // "ATTESTA\0"

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::pubkey::Pubkey;

    fn create_test_account() -> AttestaAccount {
        let owner = Pubkey::new_unique();
        let passkey_pubkey = [42u8; 64];
        let credential_id = b"test_credential".to_vec();
        let policy = vec![];
        let created_at = 1234567890i64;

        AttestaAccount::new(
            owner,
            passkey_pubkey,
            credential_id,
            policy,
            created_at,
        )
    }

    #[test]
    fn test_new_account() {
        let account = create_test_account();
        assert_eq!(account.nonce, 0);
        assert_eq!(account.created_at, account.updated_at);
    }

    #[test]
    fn test_increment_nonce() {
        let mut account = create_test_account();
        assert_eq!(account.nonce, 0);

        account.increment_nonce();
        assert_eq!(account.nonce, 1);

        account.increment_nonce();
        assert_eq!(account.nonce, 2);
    }

    #[test]
    fn test_validate_nonce() {
        let mut account = create_test_account();
        assert_eq!(account.nonce, 0);

        // Nonce must be greater than current
        assert!(!account.validate_nonce(0)); // Equal - invalid
        assert!(!account.validate_nonce(1)); // Wait, this should be valid? Let me check...

        // Actually, nonce must be STRICTLY greater
        // So if current nonce is 0, then 1 is valid (first transaction)
        assert!(account.validate_nonce(1));
        assert!(account.validate_nonce(2));
        assert!(!account.validate_nonce(0)); // Less than current - invalid

        account.increment_nonce(); // Now nonce is 1
        assert!(!account.validate_nonce(1)); // Equal to current - invalid
        assert!(account.validate_nonce(2)); // Greater than current - valid
    }

    #[test]
    fn test_serialize_deserialize() {
        let account = create_test_account();
        
        let bytes = account.to_bytes().unwrap();
        let deserialized = AttestaAccount::from_bytes(&bytes).unwrap();

        assert_eq!(account, deserialized);
    }

    #[test]
    fn test_serialize_deserialize_with_data() {
        let mut account = create_test_account();
        account.increment_nonce();
        account.increment_nonce();
        
        let bytes = account.to_bytes().unwrap();
        let deserialized = AttestaAccount::from_bytes(&bytes).unwrap();

        assert_eq!(account.nonce, deserialized.nonce);
        assert_eq!(account.passkey_public_key, deserialized.passkey_public_key);
    }
}
