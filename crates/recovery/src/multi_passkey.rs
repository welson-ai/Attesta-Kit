use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Represents a single passkey entry in a multi-passkey setup
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct PasskeyEntry {
    /// The P-256 public key from the passkey (64 bytes uncompressed)
    pub public_key: [u8; 64],
    
    /// The credential ID from WebAuthn
    pub credential_id: Vec<u8>,
    
    /// A human-readable name/description for this passkey
    pub name: Vec<u8>, // UTF-8 encoded string
    
    /// Whether this passkey is enabled
    pub enabled: bool,
    
    /// Timestamp when this passkey was added
    pub added_at: i64,
}

impl PasskeyEntry {
    pub fn new(
        public_key: [u8; 64],
        credential_id: Vec<u8>,
        name: String,
        added_at: i64,
    ) -> Self {
        Self {
            public_key,
            credential_id,
            name: name.into_bytes(),
            enabled: true,
            added_at,
        }
    }

    pub fn name_str(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.name.clone())
    }
}

/// Manages multiple passkeys for an account
/// Enables social recovery and multi-device access
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct MultiPasskey {
    /// The primary passkey (main authentication method)
    pub primary: PasskeyEntry,
    
    /// Additional passkeys for recovery and multi-device access
    pub additional: Vec<PasskeyEntry>,
    
    /// Minimum number of passkeys required for recovery operations
    pub recovery_threshold: u8,
    
    /// Maximum number of passkeys allowed
    pub max_passkeys: u8,
}

impl MultiPasskey {
    /// Creates a new MultiPasskey setup with a single primary passkey
    pub fn new(
        primary_public_key: [u8; 64],
        primary_credential_id: Vec<u8>,
        primary_name: String,
        created_at: i64,
        recovery_threshold: u8,
        max_passkeys: u8,
    ) -> Self {
        let primary = PasskeyEntry::new(
            primary_public_key,
            primary_credential_id,
            primary_name,
            created_at,
        );

        Self {
            primary,
            additional: Vec::new(),
            recovery_threshold: recovery_threshold.max(1).min(max_passkeys),
            max_passkeys: max_passkeys.max(1),
        }
    }

    /// Adds an additional passkey
    pub fn add_passkey(
        &mut self,
        public_key: [u8; 64],
        credential_id: Vec<u8>,
        name: String,
        added_at: i64,
    ) -> Result<(), &'static str> {
        // Check if we've reached the maximum
        if (self.additional.len() as u8 + 1) >= self.max_passkeys {
            return Err("Maximum number of passkeys reached");
        }

        // Check if this credential ID already exists
        if self.additional.iter().any(|p| p.credential_id == credential_id) {
            return Err("Credential ID already exists");
        }

        let entry = PasskeyEntry::new(public_key, credential_id, name, added_at);
        self.additional.push(entry);

        Ok(())
    }

    /// Removes a passkey by credential ID
    pub fn remove_passkey(&mut self, credential_id: &[u8]) -> Result<(), &'static str> {
        // Can't remove the primary passkey
        if self.primary.credential_id == credential_id {
            return Err("Cannot remove primary passkey");
        }

        let initial_len = self.additional.len();
        self.additional.retain(|p| p.credential_id != credential_id);

        if self.additional.len() == initial_len {
            Err("Passkey not found")
        } else {
            Ok(())
        }
    }

    /// Finds a passkey by credential ID
    pub fn find_passkey(&self, credential_id: &[u8]) -> Option<&PasskeyEntry> {
        if self.primary.credential_id == credential_id {
            return Some(&self.primary);
        }
        self.additional.iter().find(|p| p.credential_id == credential_id)
    }

    /// Gets all enabled passkeys
    pub fn enabled_passkeys(&self) -> Vec<&PasskeyEntry> {
        let mut enabled = Vec::new();
        
        if self.primary.enabled {
            enabled.push(&self.primary);
        }
        
        enabled.extend(self.additional.iter().filter(|p| p.enabled));
        
        enabled
    }

    /// Checks if we have enough passkeys for recovery
    pub fn can_recover(&self) -> bool {
        self.enabled_passkeys().len() >= self.recovery_threshold as usize
    }

    /// Serializes to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        borsh::to_vec(self)
    }

    /// Deserializes from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        borsh::from_slice(data)
    }
}
