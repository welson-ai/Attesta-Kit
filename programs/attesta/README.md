# Attesta Program

The main Solana program (smart contract) for Attesta account abstraction. This Anchor program implements passkey-based authentication and policy-driven transaction execution on Solana.

## Overview

The Attesta program is an Anchor-based Solana program that enables:
- **Passkey-based accounts**: Users can create accounts using WebAuthn/passkeys instead of seed phrases
- **Policy enforcement**: On-chain policies control what transactions are allowed
- **Transaction execution**: Secure execution of transactions with passkey authorization
- **Account management**: Initialize, update, and manage Attesta accounts

## Program Instructions

### `initialize`

Creates a new Attesta account with a passkey.

**Accounts:**
- `attesta_account`: The PDA account to initialize
- `owner`: The account owner (signer)
- `system_program`: Solana system program

**Arguments:**
- `passkey_public_key`: P-256 public key from user's passkey (64 bytes)
- `credential_id`: WebAuthn credential ID
- `policy`: Policy configuration (can be empty for default)

**Example:**
```rust
attesta::initialize(
    ctx,
    passkey_public_key,
    credential_id,
    policy
)?;
```

### `execute`

Executes a transaction using passkey authorization.

**Accounts:**
- `attesta_account`: The user's Attesta account (mutable)
- `authority`: Transaction authority (can be owner or program)

**Arguments:**
- `webauthn_sig`: Serialized WebAuthn signature
- `nonce`: Transaction nonce (must be > current nonce)
- `message_hash`: Hash of the transaction being authorized
- `transaction_data`: Transaction data to execute

**Example:**
```rust
attesta::execute(
    ctx,
    webauthn_sig,
    nonce,
    message_hash,
    transaction_data
)?;
```

### `update_policy`

Updates the policy for an account.

**Accounts:**
- `attesta_account`: The account to update (mutable)
- `owner`: The account owner (signer)

**Arguments:**
- `new_policy`: New policy configuration

**Example:**
```rust
attesta::update_policy(
    ctx,
    new_policy
)?;
```

## Program Structure

```
programs/attesta/
├── Cargo.toml          # Dependencies
├── README.md           # This file
└── src/
    └── lib.rs          # Main program code
```

## Dependencies

The program depends on:
- `anchor-lang`: Anchor framework
- `smart-account`: Account abstraction logic
- `core-crypto`: Cryptographic verification
- `recovery`: Policy management

## Building

```bash
# Build the program
anchor build

# Or use the build script
../scripts/build.sh
```

## Deployment

See [DEPLOYMENT.md](../../DEPLOYMENT.md) for detailed deployment instructions.

Quick deploy to devnet:
```bash
./scripts/deploy.sh devnet
```

## Program ID

After deployment, update the program ID in:
- `src/lib.rs`: `declare_id!()` macro
- `Anchor.toml`: `[programs.devnet]` section
- SDK configuration: Program ID constant

## Account Structure

Attesta accounts are stored as PDAs (Program Derived Addresses) with the following structure:

```rust
pub struct AttestaAccount {
    pub owner: Pubkey,              // Account owner
    pub passkey_public_key: [u8; 64], // P-256 public key
    pub credential_id: Vec<u8>,     // WebAuthn credential ID
    pub nonce: u64,                 // Current nonce (replay protection)
    pub policy: Vec<u8>,            // Policy configuration
    pub created_at: i64,            // Creation timestamp
    pub updated_at: i64,            // Last update timestamp
}
```

## Security Features

- **Replay Protection**: Nonce-based system prevents transaction replay
- **Signature Verification**: All WebAuthn signatures are verified on-chain
- **Policy Enforcement**: Policies are evaluated before transaction execution
- **PDA Accounts**: Accounts are PDAs, ensuring proper ownership

## Error Codes

The program defines the following error codes:

- `InvalidSignature`: Invalid signature format
- `ExecutionFailed`: Transaction execution failed
- `RequiresApproval`: Transaction requires additional approvals
- `PolicyDenied`: Transaction denied by policy
- `Unauthorized`: Not the account owner
- `SerializationFailed`: Failed to serialize account data
- `InvalidAccountData`: Invalid account data format

## Testing

```bash
# Run Anchor tests
anchor test

# Run with specific test file
anchor test -- --test test_name
```

## Integration

The program is used by:
- **TypeScript SDK**: Client-side integration
- **Rust SDK**: Rust client integration
- **Demo applications**: Example usage

## Development

### Adding New Instructions

1. Add instruction function in `lib.rs`
2. Define accounts struct with `#[derive(Accounts)]`
3. Implement instruction logic
4. Add tests
5. Update documentation

### Modifying Account Structure

If you need to change the account structure:
1. Update `AttestaAccount` in `smart-account` crate
2. Update serialization/deserialization
3. Handle migration for existing accounts
4. Update SDKs

## Documentation

- [Architecture Overview](../../ARCHITECTURE.md)
- [Deployment Guide](../../DEPLOYMENT.md)
- [SDK Documentation](../../sdk/README.md)

## License

See [LICENSE](../../LICENSE) for details.
