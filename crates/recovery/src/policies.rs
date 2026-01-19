use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Different types of policies users can set for their account
///
/// Policies are rules that control when transactions are allowed.
/// They help protect users by limiting what their account can do,
/// even if someone gets hold of their passkey.
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq)]
pub enum PolicyType {
    /// No restrictions - all transactions are allowed (default setting)
    /// Use this if you trust your passkey completely
    Open,
    
    /// Maximum amount allowed per transaction
    /// Example: "Never spend more than 1 SOL at a time"
    SpendingLimit,
    
    /// Maximum amount allowed per day
    /// Example: "Never spend more than 10 SOL per day"
    DailyLimit,
    
    /// Requires multiple passkeys to sign the same transaction
    /// Example: "Both my phone and laptop must approve large transactions"
    MultiSig,
    
    /// Transactions can only happen after a specific time
    /// Example: "Lock my account until next month" (for savings)
    TimeLocked,
}

/// A policy that controls what transactions are allowed
///
/// Each account can have one policy that defines restrictions on transactions.
/// The policy type determines what kind of restriction, and the config
/// contains the specific values (like the spending limit amount).
///
/// # Example
/// ```ignore
/// // Allow spending up to 1 SOL per transaction
/// let policy = Policy::spending_limit(1_000_000_000); // 1 SOL in lamports
/// ```
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Policy {
    /// What type of policy this is
    pub policy_type: PolicyType,
    
    /// The specific settings for this policy (depends on the type)
    /// 
    /// Format depends on policy_type:
    /// - `Open`: Empty (no config needed)
    /// - `SpendingLimit`: 8 bytes (u64 in little-endian) - max amount in lamports
    /// - `DailyLimit`: 16 bytes (u64 amount + i64 reset_timestamp)
    /// - `MultiSig`: Variable length - list of required signer public keys (32 bytes each)
    /// - `TimeLocked`: 8 bytes (i64 in little-endian) - unlock timestamp
    pub config: Vec<u8>,
}

impl Policy {
    /// Creates a new policy
    pub fn new(policy_type: PolicyType, config: Vec<u8>) -> Self {
        Self {
            policy_type,
            config,
        }
    }

    /// Creates an open policy (no restrictions)
    pub fn open() -> Self {
        Self {
            policy_type: PolicyType::Open,
            config: Vec::new(),
        }
    }

    /// Creates a spending limit policy
    pub fn spending_limit(max_amount_lamports: u64) -> Self {
        let config = max_amount_lamports.to_le_bytes().to_vec();
        Self {
            policy_type: PolicyType::SpendingLimit,
            config,
        }
    }

    /// Creates a daily limit policy
    pub fn daily_limit(max_amount_lamports: u64, reset_timestamp: i64) -> Self {
        let mut config = Vec::with_capacity(16);
        config.extend_from_slice(&max_amount_lamports.to_le_bytes());
        config.extend_from_slice(&reset_timestamp.to_le_bytes());
        Self {
            policy_type: PolicyType::DailyLimit,
            config,
        }
    }

    /// Creates a multi-sig policy
    pub fn multi_sig(required_signers: Vec<Pubkey>) -> Self {
        let mut config = Vec::with_capacity(required_signers.len() * 32);
        for signer in required_signers {
            config.extend_from_slice(signer.as_ref());
        }
        Self {
            policy_type: PolicyType::MultiSig,
            config,
        }
    }

    /// Creates a time-locked policy
    pub fn time_locked(unlock_timestamp: i64) -> Self {
        let config = unlock_timestamp.to_le_bytes().to_vec();
        Self {
            policy_type: PolicyType::TimeLocked,
            config,
        }
    }

    /// Checks if a transaction is allowed by this policy
    ///
    /// This function looks at the transaction amount and current time,
    /// then decides if the policy allows it.
    ///
    /// # Parameters
    /// - `transaction_amount`: How much the transaction wants to spend (in lamports)
    /// - `current_timestamp`: The current time (Unix timestamp)
    ///
    /// # Returns
    /// - `true` if the policy allows the transaction
    /// - `false` if the policy blocks it
    ///
    /// # Note
    /// For `DailyLimit`, this checks per-transaction limits but doesn't track
    /// daily totals. In production, you'd need to track spending separately.
    pub fn evaluate(&self, transaction_amount: u64, current_timestamp: i64) -> bool {
        match self.policy_type {
            PolicyType::Open => {
                // No restrictions - always allow
                true
            }
            
            PolicyType::SpendingLimit => {
                // Check if transaction amount is within the limit
                const U64_SIZE: usize = 8;
                if self.config.len() < U64_SIZE {
                    // Invalid config - be safe and deny
                    return false;
                }
                
                // Extract the maximum allowed amount (first 8 bytes)
                let max_amount = u64::from_le_bytes([
                    self.config[0], self.config[1], self.config[2], self.config[3],
                    self.config[4], self.config[5], self.config[6], self.config[7],
                ]);
                
                // Allow if amount is within limit
                transaction_amount <= max_amount
            }
            
            PolicyType::DailyLimit => {
                // Check both the per-transaction limit and daily total
                const DAILY_CONFIG_SIZE: usize = 16; // 8 bytes amount + 8 bytes timestamp
                if self.config.len() < DAILY_CONFIG_SIZE {
                    return false;
                }
                
                // Extract max amount (first 8 bytes)
                let max_amount = u64::from_le_bytes([
                    self.config[0], self.config[1], self.config[2], self.config[3],
                    self.config[4], self.config[5], self.config[6], self.config[7],
                ]);
                
                // Extract reset timestamp (next 8 bytes)
                let reset_timestamp = i64::from_le_bytes([
                    self.config[8], self.config[9], self.config[10], self.config[11],
                    self.config[12], self.config[13], self.config[14], self.config[15],
                ]);
                
                // If we're past the reset time, the daily limit has reset
                // TODO: In production, also check if daily total + this transaction <= limit
                if current_timestamp > reset_timestamp {
                    // Limit has reset - check per-transaction limit only
                    transaction_amount <= max_amount
                } else {
                    // Still in the same day - check per-transaction limit
                    // Note: We should also check daily total, but that requires tracking
                    transaction_amount <= max_amount
                }
            }
            
            PolicyType::TimeLocked => {
                // Check if we're past the unlock time
                const I64_SIZE: usize = 8;
                if self.config.len() < I64_SIZE {
                    return false;
                }
                
                // Extract unlock timestamp
                let unlock_timestamp = i64::from_le_bytes([
                    self.config[0], self.config[1], self.config[2], self.config[3],
                    self.config[4], self.config[5], self.config[6], self.config[7],
                ]);
                
                // Allow only if current time is past unlock time
                current_timestamp >= unlock_timestamp
            }
            
            PolicyType::MultiSig => {
                // Multi-sig policies require checking multiple signatures
                // The signature checking happens in the execution layer,
                // so we just return true here (assuming signatures will be checked)
                // TODO: In production, verify that enough signatures are present
                true
            }
        }
    }

    /// Serializes the policy to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        borsh::to_vec(self)
    }

    /// Deserializes bytes into a Policy
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        borsh::from_slice(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_policy() {
        let policy = Policy::open();
        assert!(policy.evaluate(1000, 1234567890));
        assert!(policy.evaluate(1_000_000_000, 1234567890));
    }

    #[test]
    fn test_spending_limit_policy() {
        let policy = Policy::spending_limit(1_000_000_000); // 1 SOL
        
        assert!(policy.evaluate(500_000_000, 1234567890)); // 0.5 SOL - allowed
        assert!(policy.evaluate(1_000_000_000, 1234567890)); // 1 SOL - allowed (at limit)
        assert!(!policy.evaluate(1_000_000_001, 1234567890)); // More than 1 SOL - denied
    }

    #[test]
    fn test_time_locked_policy() {
        let unlock_time = 2000000000i64;
        let policy = Policy::time_locked(unlock_time);
        
        assert!(!policy.evaluate(1000, 1000000000)); // Before unlock - denied
        assert!(policy.evaluate(1000, unlock_time)); // At unlock time - allowed
        assert!(policy.evaluate(1000, 3000000000)); // After unlock - allowed
    }

    #[test]
    fn test_daily_limit_policy() {
        let reset_time = 2000000000i64;
        let policy = Policy::daily_limit(1_000_000_000, reset_time);
        
        // Before reset time - check per-transaction limit
        assert!(policy.evaluate(500_000_000, 1000000000));
        assert!(!policy.evaluate(1_000_000_001, 1000000000));
        
        // After reset time - limit has reset
        assert!(policy.evaluate(500_000_000, reset_time + 1));
    }

    #[test]
    fn test_serialize_deserialize() {
        let policy = Policy::spending_limit(1_000_000_000);
        let bytes = policy.to_bytes().unwrap();
        let deserialized = Policy::from_bytes(&bytes).unwrap();
        
        assert_eq!(policy.policy_type, deserialized.policy_type);
        assert_eq!(policy.config, deserialized.config);
    }
}
