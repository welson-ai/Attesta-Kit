use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum CryptoError {
    #[error("Invalid WebAuthn signature")]
    InvalidWebAuthnSignature,

    #[error("Invalid P-256 public key")]
    InvalidP256PublicKey,

    #[error("Signature verification failed")]
    SignatureVerificationFailed,

    #[error("Invalid signature format")]
    InvalidSignatureFormat,

    #[error("Replay attack detected: nonce already used")]
    ReplayAttack,

    #[error("Invalid nonce")]
    InvalidNonce,

    #[error("Challenge mismatch")]
    ChallengeMismatch,

    #[error("Invalid credential ID")]
    InvalidCredentialId,

    #[error("Invalid authenticator data")]
    InvalidAuthenticatorData,
}

impl From<CryptoError> for solana_program::program_error::ProgramError {
    fn from(e: CryptoError) -> Self {
        solana_program::program_error::ProgramError::Custom(e as u32)
    }
}
