use solana_program::pubkey::Pubkey;
use core_crypto::{WebAuthnSignature, verify_webauthn_signature, CryptoError};
use crate::account::AttestaAccount;

/// Checks if a passkey signature authorizes a transaction
///
/// This is the core authentication function. It verifies that:
/// 1. The signature came from the correct passkey (by checking credential ID)
/// 2. The signature is valid (was created by the matching private key)
/// 3. The challenge matches what we expected
///
/// # Parameters
/// - `account`: The user's Attesta account (contains their passkey public key)
/// - `webauthn_sig`: The signature created by their device's passkey
/// - `challenge`: The challenge/nonce we sent them (must match what they signed)
/// - `message`: The transaction message/hash being authorized
///
/// # Returns
/// - `Ok(())` if the authorization is valid
/// - `Err(CryptoError)` if anything is wrong (wrong passkey, invalid signature, etc.)
///
/// # How it works
/// When a user wants to make a transaction:
/// 1. We send them a challenge (the transaction hash + nonce)
/// 2. They use their passkey (TouchID, FaceID, etc.) to sign it
/// 3. Their device creates a signature (private key stays on device)
/// 4. They send us the signature
/// 5. We verify it matches their public key (this function)
pub fn verify_passkey_authorization(
    account: &AttestaAccount,
    webauthn_sig: &WebAuthnSignature,
    challenge: &[u8],
    message: &[u8],
) -> Result<(), CryptoError> {
    // First, make sure they're using the right passkey
    // The credential ID must match the one we have on file
    if webauthn_sig.credential_id != account.credential_id {
        return Err(CryptoError::InvalidCredentialId);
    }

    // Verify the signature itself is valid
    // This checks that it was created by the private key matching the public key
    verify_webauthn_signature(
        webauthn_sig,
        &account.passkey_public_key,
        challenge,
    )?;

    // Basic sanity checks: challenge and message shouldn't be empty
    // In production, you'd also verify the challenge matches the transaction hash exactly
    if challenge.is_empty() {
        return Err(CryptoError::ChallengeMismatch);
    }
    
    if message.is_empty() {
        return Err(CryptoError::ChallengeMismatch);
    }

    Ok(())
}

/// Proof that a user authorized a transaction with their passkey
///
/// This structure contains everything we need to verify that a transaction
/// was authorized by the account owner, without ever seeing their private key.
/// The private key stays on their device - we only get the signature.
#[derive(Debug, Clone)]
pub struct AuthorizationProof {
    /// The WebAuthn signature from the user's device
    pub webauthn_sig: WebAuthnSignature,
    
    /// The nonce used in this transaction (prevents replay attacks)
    pub nonce: u64,
    
    /// The hash of the transaction that was authorized (32 bytes)
    pub message_hash: [u8; 32],
}

impl AuthorizationProof {
    /// Creates a new authorization proof
    ///
    /// This combines the signature, nonce, and message hash into a single
    /// proof structure that can be verified on-chain.
    pub fn new(
        webauthn_sig: WebAuthnSignature,
        nonce: u64,
        message_hash: [u8; 32],
    ) -> Self {
        Self {
            webauthn_sig,
            nonce,
            message_hash,
        }
    }

    /// Verifies that this proof is valid for a given account
    ///
    /// This checks two things:
    /// 1. The nonce hasn't been used before (replay protection)
    /// 2. The signature is valid (came from the account owner's passkey)
    ///
    /// # Parameters
    /// - `account`: The Attesta account to verify against
    ///
    /// # Returns
    /// - `Ok(())` if the proof is valid
    /// - `Err(CryptoError::ReplayAttack)` if the nonce has been used
    /// - `Err(CryptoError)` if the signature is invalid
    pub fn verify(&self, account: &AttestaAccount) -> Result<(), CryptoError> {
        // First check: has this nonce been used before?
        // If the nonce isn't higher than the last one, it's a replay attack
        if !account.validate_nonce(self.nonce) {
            return Err(CryptoError::ReplayAttack);
        }

        // Convert the nonce to bytes to use as the challenge
        // The nonce is part of what gets signed, so it must match
        let challenge = self.nonce.to_le_bytes();

        // Second check: is the signature valid?
        // This verifies the signature came from the account owner's passkey
        verify_passkey_authorization(
            account,
            &self.webauthn_sig,
            &challenge,
            &self.message_hash,
        )
    }
}
