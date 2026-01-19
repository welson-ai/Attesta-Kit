//! Smart Account Abstraction for Solana
//!
//! This crate implements account abstraction on Solana, allowing users to use
//! passkeys (biometric authentication) instead of traditional private keys.
//!
//! # What is account abstraction?
//!
//! Account abstraction means your account can be smarter than a regular wallet.
//! Instead of just checking a signature, it can:
//! - Verify passkey signatures (TouchID, FaceID, etc.)
//! - Enforce policies (spending limits, time locks, etc.)
//! - Support multiple passkeys for recovery
//!
//! # How it works
//!
//! 1. **Registration**: User creates a passkey, we store their public key on-chain
//! 2. **Transaction**: User signs a transaction with their passkey
//! 3. **Verification**: We verify the signature on-chain using their public key
//! 4. **Policy Check**: We check if their policy allows the transaction
//! 5. **Execution**: If everything checks out, we execute the transaction
//!
//! # Key Components
//!
//! - `account.rs`: The main `AttestaAccount` struct that represents an account
//! - `auth.rs`: Functions for verifying passkey signatures
//! - `execute.rs`: Transaction execution logic with policy enforcement
//! - `storage.rs`: Utilities for reading and writing accounts on-chain
//!
//! # Example
//!
//! ```ignore
//! use smart_account::{AttestaAccount, execute_transaction, AuthorizationProof};
//!
//! // Execute a transaction with an authorization proof
//! let result = execute_transaction(&mut account, &proof, &transaction_data)?;
//! ```

pub mod account;
pub mod auth;
pub mod execute;
pub mod storage;

pub use account::AttestaAccount;
pub use auth::{verify_passkey_authorization, AuthorizationProof};
pub use execute::{execute_transaction, PolicyResult};
pub use storage::{load_attesta_account, save_attesta_account, init_attesta_account};
