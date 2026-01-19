//! Rust SDK for Attesta
//!
//! This SDK provides Rust client functionality for interacting with
//! Attesta accounts on Solana.

pub mod client;

pub use client::AttestaClient;

// Re-export commonly used types
pub use smart_account::AttestaAccount;
pub use core_crypto::WebAuthnSignature;
pub use recovery::{Policy, PolicyType, MultiPasskey};
