//! Attesta - Account Abstraction on Solana with Passkeys
//!
//! This is the main Anchor program that implements account abstraction
//! on Solana, enabling passkey-based authorization and policy-driven execution.

use anchor_lang::prelude::*;
use smart_account::{AttestaAccount, AuthorizationProof, execute_transaction, PolicyResult};
use smart_account::storage::{load_attesta_account, save_attesta_account, init_attesta_account};
use core_crypto::WebAuthnSignature;

// TODO: Replace with your actual program ID after generating keypair
// Generate with: solana-keygen new -o target/deploy/attesta-keypair.json
declare_id!("Attesta11111111111111111111111111111111");

#[program]
pub mod attesta {
    use super::*;

    /// Initializes a new Attesta account
    ///
    /// This instruction creates a new smart account that uses passkeys
    /// instead of traditional private keys. The user provides their passkey's
    /// public key and we store it on-chain.
    ///
    /// # Accounts
    /// - `attesta_account`: The account to initialize (must be a PDA)
    /// - `owner`: The user who owns this account (signer)
    /// - `system_program`: The Solana system program
    ///
    /// # Arguments
    /// - `passkey_public_key`: The public key from the user's passkey (64 bytes)
    /// - `credential_id`: The credential ID from WebAuthn
    /// - `policy`: Policy configuration (can be empty for default)
    pub fn initialize(
        ctx: Context<Initialize>,
        passkey_public_key: [u8; 64],
        credential_id: Vec<u8>,
        policy: Vec<u8>,
    ) -> Result<()> {
        let clock = Clock::get()?;
        
        // Create the AttestaAccount
        let account = AttestaAccount::new(
            *ctx.accounts.owner.key,
            passkey_public_key,
            credential_id,
            policy,
            clock.unix_timestamp,
        );

        // Serialize and store
        let account_data = account.to_bytes()
            .map_err(|_| AttestaError::SerializationFailed)?;
        
        ctx.accounts.attesta_account.data = account_data;

        msg!("Attesta account initialized for owner: {}", ctx.accounts.owner.key());
        Ok(())
    }

    /// Executes a transaction using passkey authorization
    ///
    /// This is the main instruction that processes transactions. It verifies
    /// the passkey signature, checks the policy, and executes the transaction.
    ///
    /// # Accounts
    /// - `attesta_account`: The user's Attesta account (mut)
    /// - `authority`: The transaction authority (can be the owner or a program)
    ///
    /// # Arguments
    /// - `webauthn_sig`: The WebAuthn signature from the user's device
    /// - `nonce`: The nonce for this transaction (must be > account's current nonce)
    /// - `message_hash`: The hash of the transaction being authorized
    /// - `transaction_data`: The transaction data to execute
    pub fn execute(
        ctx: Context<Execute>,
        webauthn_sig: Vec<u8>, // Serialized WebAuthnSignature
        nonce: u64,
        message_hash: [u8; 32],
        transaction_data: Vec<u8>,
    ) -> Result<()> {
        // Deserialize the account from the account data
        let mut account = AttestaAccount::from_bytes(&ctx.accounts.attesta_account.data)
            .map_err(|_| AttestaError::InvalidAccountData)?;

        // Deserialize the WebAuthn signature
        let webauthn_signature = WebAuthnSignature::from_bytes(&webauthn_sig)
            .map_err(|_| AttestaError::InvalidSignature)?;

        // Create the authorization proof
        let proof = AuthorizationProof::new(
            webauthn_signature,
            nonce,
            message_hash,
        );

        // Execute the transaction
        let result = execute_transaction(&mut account, &proof, &transaction_data)
            .map_err(|e| AttestaError::ExecutionFailed)?;

        match result {
            PolicyResult::Allowed => {
                // Serialize and save the updated account
                let account_data = account.to_bytes()
                    .map_err(|_| AttestaError::SerializationFailed)?;
                ctx.accounts.attesta_account.data = account_data;
                msg!("Transaction executed successfully");
                Ok(())
            }
            PolicyResult::RequiresApproval => {
                msg!("Transaction requires additional approvals");
                Err(AttestaError::RequiresApproval.into())
            }
            PolicyResult::Denied => {
                msg!("Transaction denied by policy");
                Err(AttestaError::PolicyDenied.into())
            }
        }
    }

    /// Updates the policy for an account
    ///
    /// Allows the account owner to change their policy settings (spending limits, etc.)
    ///
    /// # Accounts
    /// - `attesta_account`: The account to update (mut)
    /// - `owner`: The account owner (signer)
    ///
    /// # Arguments
    /// - `new_policy`: The new policy configuration
    pub fn update_policy(
        ctx: Context<UpdatePolicy>,
        new_policy: Vec<u8>,
    ) -> Result<()> {
        // Deserialize the account
        let mut account = AttestaAccount::from_bytes(&ctx.accounts.attesta_account.data)
            .map_err(|_| AttestaError::InvalidAccountData)?;

        // Verify the owner
        require!(
            account.owner == *ctx.accounts.owner.key,
            AttestaError::Unauthorized
        );

        // Update the policy
        account.policy = new_policy;
        
        // Serialize and save
        let account_data = account.to_bytes()
            .map_err(|_| AttestaError::SerializationFailed)?;
        ctx.accounts.attesta_account.data = account_data;

        msg!("Policy updated for account: {}", ctx.accounts.attesta_account.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 64 + 4 + 256 + 4 + 256 + 8 + 8 + 8, // discriminator + account data
        seeds = [b"attesta", owner.key.as_ref()],
        bump
    )]
    pub attesta_account: Account<'info, AttestaAccountData>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    // Helper to get the seed for PDA derivation
    pub fn get_seed(&self) -> Vec<u8> {
        // Use first 32 bytes of owner key as seed
        self.owner.key().as_ref()[..32].to_vec()
    }
}

#[derive(Accounts)]
pub struct Execute<'info> {
    #[account(mut)]
    pub attesta_account: Account<'info, AttestaAccountData>,
    
    /// CHECK: Can be the owner or a program that's authorized to execute
    pub authority: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct UpdatePolicy<'info> {
    #[account(mut)]
    pub attesta_account: Account<'info, AttestaAccountData>,
    
    pub owner: Signer<'info>,
}

/// Wrapper account type for Anchor
/// This wraps our AttestaAccount so Anchor can manage it
#[account]
pub struct AttestaAccountData {
    pub data: Vec<u8>, // Serialized AttestaAccount
}

#[error_code]
pub enum AttestaError {
    #[msg("Invalid signature format")]
    InvalidSignature,
    
    #[msg("Transaction execution failed")]
    ExecutionFailed,
    
    #[msg("Transaction requires additional approvals")]
    RequiresApproval,
    
    #[msg("Transaction denied by policy")]
    PolicyDenied,
    
    #[msg("Unauthorized: not the account owner")]
    Unauthorized,
    
    #[msg("Failed to serialize account data")]
    SerializationFailed,
    
    #[msg("Invalid account data format")]
    InvalidAccountData,
}
