//! Attesta Client for Rust
//!
//! This client provides a high-level interface for interacting with
//! Attesta accounts on Solana.

use anchor_client::{
    Client,
    Cluster,
};
use solana_program::pubkey::Pubkey;
use smart_account::AttestaAccount;
use core_crypto::WebAuthnSignature;
use thiserror::Error;

/// Client for interacting with Attesta program
pub struct AttestaClient {
    /// The Anchor client
    client: Client,
    
    /// The Attesta program ID
    program_id: Pubkey,
}

impl AttestaClient {
    /// Creates a new Attesta client
    ///
    /// # Parameters
    /// - `cluster`: The Solana cluster to connect to (Devnet, Mainnet, etc.)
    /// - `program_id`: The Attesta program ID
    ///
    /// # Returns
    /// A new AttestaClient instance
    pub fn new(cluster: Cluster, program_id: Pubkey) -> Self {
        let client = Client::new(cluster, None);
        
        Self {
            client,
            program_id,
        }
    }

    /// Gets an Attesta account
    ///
    /// # Parameters
    /// - `account_address`: The address of the Attesta account
    ///
    /// # Returns
    /// The AttestaAccount if found, or an error
    pub fn get_account(&self, account_address: &Pubkey) -> Result<AttestaAccount, AttestaError> {
        // TODO: Implement account fetching from on-chain
        // This would use the Anchor client to fetch and deserialize the account
        Err(AttestaError::NotImplemented)
    }

    /// Derives the Attesta account PDA for a user
    ///
    /// # Parameters
    /// - `owner`: The owner's public key
    /// - `seed`: Additional seed (e.g., credential ID)
    ///
    /// # Returns
    /// The PDA address and bump seed
    pub fn derive_account_address(&self, owner: &Pubkey, seed: &[u8]) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                b"attesta",
                owner.as_ref(),
                seed,
            ],
            &self.program_id,
        )
    }
}

/// Errors that can occur when using the Attesta client
#[derive(Error, Debug)]
pub enum AttestaError {
    #[error("Not implemented yet")]
    NotImplemented,
    
    #[error("Account not found")]
    AccountNotFound,
    
    #[error("Invalid account data")]
    InvalidAccountData,
    
    #[error("RPC error: {0}")]
    RpcError(String),
}
