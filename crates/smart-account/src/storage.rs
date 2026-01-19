use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use crate::account::{AttestaAccount, ATTESTA_ACCOUNT_DISCRIMINATOR};

/// Finds the address where an Attesta account is stored (PDA)
///
/// In Solana, we use Program Derived Addresses (PDAs) to create accounts
/// that are controlled by our program. This function calculates the address
/// where a user's Attesta account will be stored.
///
/// # Parameters
/// - `program_id`: The ID of our Attesta program
/// - `owner`: The user's wallet address
/// - `seed`: Additional seed data (e.g., credential ID) to make it unique
///
/// # Returns
/// A tuple of (Pubkey, bump_seed) where:
/// - The Pubkey is the address of the account
/// - The bump_seed is needed to derive this address again
///
/// # How PDAs work
/// PDAs are addresses that don't have a private key. Instead, they're
/// derived deterministically from seeds and the program ID. This means
/// we can calculate the address without needing to generate a keypair.
pub fn derive_attesta_account(
    program_id: &Pubkey,
    owner: &Pubkey,
    seed: &[u8],
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"attesta",           // Prefix to identify Attesta accounts
            owner.as_ref(),        // Owner's public key
            seed,                  // Additional seed (e.g., credential ID)
        ],
        program_id,
    )
}

/// Reads an Attesta account from on-chain storage
///
/// This function takes a Solana account and reads the Attesta account
/// data from it. It validates the discriminator to make sure it's
/// actually an Attesta account, then deserializes the data.
///
/// # Parameters
/// - `account_info`: The Solana account to read from
///
/// # Returns
/// - `Ok(AttestaAccount)` if the account is valid and readable
/// - `Err(ProgramError::InvalidAccountData)` if the data is corrupted or wrong type
pub fn load_attesta_account(
    account_info: &AccountInfo,
) -> Result<AttestaAccount, ProgramError> {
    let data = account_info.data.borrow();
    
    // First, check the discriminator (first 8 bytes)
    // This is like a file type indicator - makes sure it's actually an Attesta account
    const DISCRIMINATOR_SIZE: usize = 8;
    if data.len() < DISCRIMINATOR_SIZE {
        return Err(ProgramError::InvalidAccountData);
    }
    
    if data[..DISCRIMINATOR_SIZE] != ATTESTA_ACCOUNT_DISCRIMINATOR {
        return Err(ProgramError::InvalidAccountData);
    }

    // Skip past the discriminator and deserialize the actual account data
    let account_data = data.get(DISCRIMINATOR_SIZE..)
        .ok_or(ProgramError::InvalidAccountData)?;
    
    let account = AttestaAccount::from_bytes(account_data)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    Ok(account)
}

/// Saves an Attesta account to on-chain storage
///
/// This function takes an Attesta account and writes it to a Solana account.
/// It writes the discriminator first, then the serialized account data.
///
/// # Parameters
/// - `account`: The Attesta account to save
/// - `account_info`: The Solana account to write to
///
/// # Returns
/// - `Ok(())` if the save was successful
/// - `Err(ProgramError::InvalidAccountData)` if there's not enough space or serialization fails
///
/// # Safety
/// This function will overwrite any existing data in the account.
/// Make sure you're writing to the right account!
pub fn save_attesta_account(
    account: &AttestaAccount,
    account_info: &AccountInfo,
) -> Result<(), ProgramError> {
    let mut data = account_info.data.borrow_mut();
    
    // Serialize the account to bytes
    let serialized = account.to_bytes()
        .map_err(|_| ProgramError::InvalidAccountData)?;
    
    // Calculate how much space we need
    const DISCRIMINATOR_SIZE: usize = 8;
    let total_size = DISCRIMINATOR_SIZE + serialized.len();
    
    // Make sure the account is big enough
    if data.len() < total_size {
        return Err(ProgramError::InvalidAccountData);
    }

    // Write the discriminator (first 8 bytes)
    data[..DISCRIMINATOR_SIZE].copy_from_slice(&ATTESTA_ACCOUNT_DISCRIMINATOR);
    
    // Write the account data (after the discriminator)
    let data_slice = data.get_mut(DISCRIMINATOR_SIZE..total_size)
        .ok_or(ProgramError::InvalidAccountData)?;
    data_slice.copy_from_slice(&serialized);

    Ok(())
}

/// Creates a new Attesta account and saves it to storage
///
/// This is a convenience function that combines creating a new account
/// with saving it. It's used when a user first registers with Attesta.
///
/// # Parameters
/// - `account_info`: The Solana account to write to (must be initialized first)
/// - `owner`: The user's wallet address
/// - `passkey_public_key`: The public key from their passkey (64 bytes)
/// - `credential_id`: The credential ID from WebAuthn
/// - `policy`: Their policy settings (can be empty for default)
///
/// # Returns
/// - `Ok(())` if the account was created and saved successfully
/// - `Err(ProgramError::InvalidAccountData)` if something goes wrong
pub fn init_attesta_account(
    account_info: &AccountInfo,
    owner: &Pubkey,
    passkey_public_key: [u8; 64],
    credential_id: Vec<u8>,
    policy: Vec<u8>,
) -> Result<(), ProgramError> {
    // Get the current time for the creation timestamp
    let clock = solana_program::clock::Clock::get()
        .map_err(|_| ProgramError::InvalidAccountData)?;
    
    // Create the new account
    let account = AttestaAccount::new(
        *owner,
        passkey_public_key,
        credential_id,
        policy,
        clock.unix_timestamp,
    );

    // Save it to storage
    save_attesta_account(&account, account_info)
}
