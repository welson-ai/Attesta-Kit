//! Core cryptographic library for Attesta
//!
//! This crate provides all the cryptographic functions needed for passkey-based
//! authentication on Solana. It handles WebAuthn signatures, P-256 verification,
//! and replay protection.
//!
//! # What is passkey authentication?
//!
//! Instead of using seed phrases or private keys stored in wallets, users can
//! authenticate with their device's biometric authenticator (TouchID, FaceID,
//! hardware keys, etc.). The private key never leaves their device - only the
//! signature comes to us for verification.
//!
//! # Key Features
//!
//! - **WebAuthn signature verification**: Verifies signatures from user devices
//! - **P-256 cryptography**: Uses industry-standard elliptic curve cryptography
//! - **Replay protection**: Prevents the same transaction from being executed twice
//!
//! # Example
//!
//! ```ignore
//! use core_crypto::{verify_webauthn_signature, WebAuthnSignature};
//!
//! // Verify a WebAuthn signature
//! let webauthn_sig = WebAuthnSignature::new(/* ... */);
//! verify_webauthn_signature(&webauthn_sig, &public_key, &challenge)?;
//! ```

pub mod errors;
pub mod p256_verify;
pub mod replay;
pub mod webauthn;

pub use errors::CryptoError;
pub use p256_verify::verify_p256_signature;
pub use replay::ReplayProtection;
pub use webauthn::{WebAuthnSignature, verify_webauthn_signature};
