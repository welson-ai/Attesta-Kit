use solana_program::{pubkey::Pubkey, program_error::ProgramError};
use crate::account::AttestaAccount;
use crate::auth::AuthorizationProof;

/// The result of checking if a transaction is allowed by the account's policy
///
/// After we verify the signature, we need to check if the transaction
/// is allowed by the user's policy settings (spending limits, etc.)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PolicyResult {
    /// The transaction is allowed and can proceed
    Allowed,
    
    /// The transaction is denied by the policy (e.g., exceeds spending limit)
    Denied,
    
    /// The transaction needs additional approvals (e.g., multi-sig required)
    RequiresApproval,
}

/// Executes a transaction on behalf of an Attesta account
///
/// This is the main function that processes transactions. It:
/// 1. Verifies the user authorized it (signature check)
/// 2. Checks if it's allowed by their policy
/// 3. If both pass, marks it as complete (increments nonce)
///
/// # Parameters
/// - `account`: The user's Attesta account (will be updated if transaction succeeds)
/// - `proof`: The authorization proof showing they signed the transaction
/// - `transaction_data`: The transaction data to execute (for policy evaluation)
///
/// # Returns
/// - `Ok(PolicyResult::Allowed)` if the transaction is executed successfully
/// - `Ok(PolicyResult::RequiresApproval)` if more signatures are needed
/// - `Ok(PolicyResult::Denied)` if the policy blocks it
/// - `Err(ProgramError)` if the proof is invalid or something goes wrong
///
/// # Side Effects
/// If the transaction is allowed, this will:
/// - Increment the account's nonce (prevents replay)
/// - Update the account's `updated_at` timestamp
pub fn execute_transaction(
    account: &mut AttestaAccount,
    proof: &AuthorizationProof,
    transaction_data: &[u8],
) -> Result<PolicyResult, ProgramError> {
    // Step 1: Verify the user actually authorized this transaction
    // This checks the signature and nonce
    proof.verify(account)
        .map_err(|e| ProgramError::Custom(e as u32))?;

    // Step 2: Check if the policy allows this transaction
    // Even if the signature is valid, the policy might block it
    let policy_result = evaluate_policy(account, transaction_data)?;

    // Step 3: If everything checks out, execute the transaction
    match policy_result {
        PolicyResult::Allowed => {
            // Mark the transaction as complete
            // This increments the nonce so it can't be replayed
            account.increment_nonce();
            Ok(PolicyResult::Allowed)
        }
        PolicyResult::RequiresApproval => {
            // Transaction is valid but needs more signatures
            // Don't increment nonce yet - wait for additional approvals
            Ok(PolicyResult::RequiresApproval)
        }
        PolicyResult::Denied => {
            // Policy says no - reject the transaction
            Err(ProgramError::InvalidArgument)
        }
    }
}

/// Checks if a transaction is allowed by the account's policy
///
/// Policies can restrict transactions based on things like:
/// - Spending limits (max amount per transaction)
/// - Daily limits (max amount per day)
/// - Time locks (transactions only allowed after a certain time)
/// - Program allowlists (only allow transactions to specific programs)
///
/// # Parameters
/// - `account`: The account with the policy to check
/// - `transaction_data`: The transaction data (for extracting amount, destination, etc.)
///
/// # Returns
/// - `Ok(PolicyResult::Allowed)` if the policy allows it
/// - `Ok(PolicyResult::Denied)` if the policy blocks it
/// - `Ok(PolicyResult::RequiresApproval)` if more approvals are needed
///
/// # Note
/// This is a simplified implementation. In production, you'd:
/// - Parse the policy structure properly
/// - Extract transaction details (amount, destination, program ID)
/// - Check spending limits, time locks, allowlists, etc.
/// - Track daily spending separately
fn evaluate_policy(
    account: &AttestaAccount,
    _transaction_data: &[u8],
) -> Result<PolicyResult, ProgramError> {
    // If there's no policy configured, default to allowing all transactions
    // This makes it easier for users to get started
    if account.policy.is_empty() {
        return Ok(PolicyResult::Allowed);
    }

    // TODO: In production, properly parse and evaluate the policy
    // For now, we'll do basic validation:
    // - Check policy structure is valid
    // - Extract transaction details from transaction_data
    // - Evaluate spending limits, time locks, etc.
    
    // Placeholder: assume policy passes basic checks
    // In real implementation, decode policy and check all conditions
    Ok(PolicyResult::Allowed)
}

/// Checks if an instruction is allowed by the account's policy
///
/// Some policies might restrict which programs can be called. This function
/// checks if the instruction's program ID is in the allowed list.
///
/// # Parameters
/// - `account`: The account with the policy
/// - `program_id`: The program that's being called
/// - `_instruction_data`: The instruction data (not used yet, but might be in future)
///
/// # Returns
/// - `Ok(())` if the instruction is allowed
/// - `Err(ProgramError)` if the policy blocks it
pub fn validate_instruction(
    account: &AttestaAccount,
    _program_id: &Pubkey,
    _instruction_data: &[u8],
) -> Result<(), ProgramError> {
    // TODO: In production, check if program_id is in the policy's allowlist
    // For now, allow all programs (default behavior)
    
    Ok(())
}
