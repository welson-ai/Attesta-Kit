# Smart Account

The smart account abstraction program for Attesta. This is the core Solana program that manages accounts that use passkeys instead of traditional private keys.

## Overview

This crate implements the account abstraction logic on Solana. It allows users to:
- Create accounts that use passkeys (biometric authentication) instead of seed phrases
- Authorize transactions using their device's authenticator (TouchID, FaceID, etc.)
- Enforce policies on transactions (spending limits, time locks, etc.)

## Key Components

### `account.rs`
The main `AttestaAccount` struct that represents an account on-chain. It stores:
- The user's passkey public key
- The credential ID
- The current nonce (for replay protection)
- Policy settings

### `auth.rs`
Authentication and authorization logic. Verifies that signatures are valid and come from the account owner's passkey.

### `execute.rs`
Transaction execution logic. Handles the full flow:
1. Verify the authorization proof
2. Check if the policy allows the transaction
3. Execute and update the account

### `storage.rs`
On-chain storage utilities. Functions for reading and writing Attesta accounts to Solana accounts.

## How It Works

1. **Registration**: User creates a passkey on their device, and we store the public key on-chain
2. **Transaction**: User wants to make a transaction, so they sign it with their passkey
3. **Verification**: We verify the signature on-chain using the stored public key
4. **Policy Check**: We check if the transaction is allowed by the user's policy
5. **Execution**: If everything checks out, we execute the transaction

## Security Features

- **Replay Protection**: Each transaction uses a unique nonce
- **Signature Verification**: All signatures are verified on-chain
- **Policy Enforcement**: Transactions are checked against user policies before execution
- **Privacy**: Biometric data never leaves the user's device
