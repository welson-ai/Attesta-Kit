# Rust SDK

Rust client SDK for Attesta. This SDK provides a Rust interface for interacting with Attesta accounts and the Attesta program on Solana.

## Overview

The Rust SDK enables Rust applications to:
- **Create and manage** Attesta accounts
- **Interact with** the Attesta program on Solana
- **Configure policies** for accounts
- **Execute transactions** with passkey authorization
- **Query account state** from the blockchain

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
attesta-sdk = { path = "../../sdk/rust" }
solana-sdk = "1.18"
anchor-client = "0.29"
```

Or from crates.io (when published):

```toml
[dependencies]
attesta-sdk = "0.1.0"
```

## Quick Start

```rust
use attesta_sdk::AttestaClient;
use anchor_client::Cluster;

// Create a client
let client = AttestaClient::new(Cluster::Devnet, program_id);

// Get an account
let account = client.get_account(&account_address)?;

// Check account state
println!("Account owner: {}", account.owner);
println!("Current nonce: {}", account.nonce);
```

## Features

### Account Management

```rust
use attesta_sdk::AttestaClient;

let client = AttestaClient::new(Cluster::Devnet, program_id);

// Get account information
let account = client.get_account(&account_address)?;

// Check if account exists
if let Some(account) = client.try_get_account(&account_address)? {
    println!("Account found: {:?}", account);
}
```

### Policy Configuration

```rust
use attesta_sdk::{AttestaClient, Policy, PolicyType};

let client = AttestaClient::new(Cluster::Devnet, program_id);

// Create a policy
let policy = Policy::spending_limit(1_000_000_000); // 1 SOL max

// Update account policy
client.update_policy(&account_address, &policy, &signer)?;
```

### Transaction Execution

```rust
use attesta_sdk::AttestaClient;
use solana_sdk::transaction::Transaction;

let client = AttestaClient::new(Cluster::Devnet, program_id);

// Create a transaction
let transaction = create_payment_transaction(...)?;

// Execute with authorization proof
let signature = client.execute_transaction(
    &account_address,
    &transaction,
    &authorization_proof,
    &signer
)?;
```

## API Reference

### `AttestaClient`

Main client for interacting with Attesta.

```rust
impl AttestaClient {
    /// Create a new client
    pub fn new(cluster: Cluster, program_id: Pubkey) -> Self;
    
    /// Get an account
    pub fn get_account(&self, address: &Pubkey) -> Result<AttestaAccount>;
    
    /// Try to get an account (returns None if not found)
    pub fn try_get_account(&self, address: &Pubkey) -> Result<Option<AttestaAccount>>;
    
    /// Update account policy
    pub fn update_policy(
        &self,
        account: &Pubkey,
        policy: &Policy,
        signer: &Keypair
    ) -> Result<Signature>;
    
    /// Execute a transaction
    pub fn execute_transaction(
        &self,
        account: &Pubkey,
        transaction: &Transaction,
        proof: &AuthorizationProof,
        signer: &Keypair
    ) -> Result<Signature>;
}
```

## Types

### `AttestaAccount`

Account state structure:

```rust
pub struct AttestaAccount {
    pub owner: Pubkey,
    pub passkey_public_key: [u8; 64],
    pub credential_id: Vec<u8>,
    pub nonce: u64,
    pub policy: Vec<u8>,
    pub created_at: i64,
    pub updated_at: i64,
}
```

### `Policy`

Policy configuration:

```rust
pub enum PolicyType {
    Open,
    SpendingLimit,
    DailyLimit,
    TimeLocked,
    MultiSig,
}

pub struct Policy {
    pub policy_type: PolicyType,
    pub config: Vec<u8>,
}
```

### `AuthorizationProof`

Transaction authorization proof:

```rust
pub struct AuthorizationProof {
    pub webauthn_signature: WebAuthnSignature,
    pub nonce: u64,
    pub message_hash: [u8; 32],
}
```

## Examples

### Complete Registration Flow

```rust
use attesta_sdk::AttestaClient;
use anchor_client::Cluster;
use solana_sdk::keypair::Keypair;

let client = AttestaClient::new(Cluster::Devnet, program_id);
let user_keypair = Keypair::new();

// Register account (implementation depends on WebAuthn integration)
let account_address = register_account(&client, &user_keypair)?;
```

### Making a Payment

```rust
use attesta_sdk::AttestaClient;

let client = AttestaClient::new(Cluster::Devnet, program_id);

// Create payment transaction
let transaction = create_payment_transaction(
    &from_account,
    &to_account,
    amount
)?;

// Get authorization proof (from WebAuthn)
let proof = get_authorization_proof(&transaction)?;

// Execute
let signature = client.execute_transaction(
    &from_account,
    &transaction,
    &proof,
    &signer
)?;
```

## WebAuthn Integration

**Note**: WebAuthn requires browser APIs. For Rust applications:

1. **Use a browser-based flow**: Integrate with a web component for WebAuthn
2. **Use a bridge**: Create a bridge between Rust and a WebAuthn-capable environment
3. **Use the TypeScript SDK**: For full WebAuthn support, use the TypeScript SDK

## Error Handling

```rust
use attesta_sdk::{AttestaClient, AttestaError};

match client.get_account(&address) {
    Ok(account) => println!("Account: {:?}", account),
    Err(AttestaError::AccountNotFound) => println!("Account not found"),
    Err(AttestaError::NetworkError(e)) => println!("Network error: {}", e),
    Err(e) => println!("Error: {}", e),
}
```

## Testing

```bash
# Run tests
cargo test

# Run with output
cargo test -- --nocapture
```

## Building

```bash
# Build the SDK
cargo build --release

# Build with docs
cargo doc --open
```

## Documentation

Generate documentation:

```bash
cargo doc --no-deps --open
```

## Comparison with TypeScript SDK

| Feature | Rust SDK | TypeScript SDK |
|---------|----------|----------------|
| WebAuthn Support | Limited (requires bridge) | Full |
| Browser Support | No | Yes |
| Server-side | Yes | Limited |
| Performance | High | Good |
| Type Safety | Strong | Strong |

**Recommendation**: Use TypeScript SDK for client-side applications, Rust SDK for server-side or CLI applications.

## Related Documentation

- [TypeScript SDK](../ts/README.md) - TypeScript client SDK
- [SDK Overview](../README.md) - General SDK documentation
- [API Documentation](../../documentation/my-website/docs/sdk-and-integration/javascript-typescript-sdk.md) - Detailed API docs

## License

See [LICENSE](../../LICENSE) for details.
