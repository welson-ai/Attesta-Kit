//! Recovery and Policy Management for Attesta
//!
//! This crate provides tools for account recovery and policy management. It helps
//! users recover their accounts if they lose devices, and configure policies
//! that control what transactions are allowed.
//!
//! # Features
//!
//! - **Multi-passkey support**: Use multiple devices (phone, laptop, hardware key)
//! - **Social recovery**: Recover your account using other passkeys
//! - **Policy management**: Set spending limits, time locks, and more
//! - **Encrypted backups**: Securely backup account information for recovery
//!
//! # Policy Types
//!
//! - `Open`: No restrictions (default)
//! - `SpendingLimit`: Maximum amount per transaction
//! - `DailyLimit`: Maximum amount per day
//! - `TimeLocked`: Transactions only allowed after a certain time
//! - `MultiSig`: Requires multiple passkeys to sign
//!
//! # Example
//!
//! ```ignore
//! use recovery::{Policy, MultiPasskey};
//!
//! // Create a spending limit policy
//! let policy = Policy::spending_limit(1_000_000_000); // 1 SOL max
//!
//! // Set up multi-passkey recovery
//! let multi_passkey = MultiPasskey::new(/* ... */);
//! ```

pub mod encrypted_backup;
pub mod multi_passkey;
pub mod policies;

pub use encrypted_backup::EncryptedBackup;
pub use multi_passkey::{MultiPasskey, PasskeyEntry};
pub use policies::{Policy, PolicyType};
