# Recovery

Recovery and Policy Management crate for Attesta. This crate provides tools for account recovery, multi-passkey support, and policy configuration.

## Overview

The recovery crate enables users to:
- **Recover accounts** using multiple passkeys (social recovery)
- **Configure policies** that control what transactions are allowed
- **Create encrypted backups** for account recovery
- **Manage multiple passkeys** across different devices

## Features

### Multi-Passkey Support

Use multiple devices (phone, laptop, hardware key) to access your account. If you lose one device, you can still access your account using another.

```rust
use recovery::MultiPasskey;

// Add multiple passkeys to your account
let multi_passkey = MultiPasskey::new();
multi_passkey.add_passkey(phone_credential_id, phone_public_key)?;
multi_passkey.add_passkey(laptop_credential_id, laptop_public_key)?;
```

### Policy Management

Configure security policies that control transaction behavior:

- **Open**: No restrictions (default)
- **SpendingLimit**: Maximum amount per transaction
- **DailyLimit**: Maximum amount per day
- **TimeLocked**: Transactions only allowed after a certain time
- **MultiSig**: Requires multiple passkeys to sign

```rust
use recovery::{Policy, PolicyType};

// Create a spending limit policy (1 SOL max per transaction)
let policy = Policy::spending_limit(1_000_000_000);

// Create a time-locked policy (unlocks on Jan 1, 2025)
let unlock_timestamp = 1735689600; // Unix timestamp
let policy = Policy::time_locked(unlock_timestamp);

// Create a daily limit policy (10 SOL per day)
let daily_limit = 10_000_000_000;
let reset_timestamp = get_midnight_timestamp();
let policy = Policy::daily_limit(daily_limit, reset_timestamp);
```

### Encrypted Backups

Create encrypted backups of account information for recovery:

```rust
use recovery::EncryptedBackup;

// Create an encrypted backup
let backup = EncryptedBackup::create(
    account_data,
    encryption_key
)?;

// Restore from backup
let account_data = backup.decrypt(encryption_key)?;
```

## Key Components

### `policies.rs`

Policy types and evaluation logic. Defines all supported policy types and provides functions to evaluate whether transactions are allowed.

**Policy Types:**
- `PolicyType::Open` - No restrictions
- `PolicyType::SpendingLimit` - Per-transaction limit
- `PolicyType::DailyLimit` - Daily spending limit
- `PolicyType::TimeLocked` - Time-based lock
- `PolicyType::MultiSig` - Multi-signature requirement

### `multi_passkey.rs`

Multi-passkey management. Allows accounts to have multiple passkeys registered, enabling recovery if one device is lost.

**Features:**
- Add/remove passkeys
- List registered passkeys
- Verify passkey ownership
- Recovery workflows

### `encrypted_backup.rs`

Encrypted backup functionality. Allows users to create encrypted backups of their account data for recovery purposes.

**Security:**
- AES-256 encryption
- User-controlled encryption keys
- Secure backup storage

## Usage Example

```rust
use recovery::{Policy, MultiPasskey, EncryptedBackup};

// 1. Create a policy
let policy = Policy::spending_limit(1_000_000_000); // 1 SOL max

// 2. Set up multi-passkey recovery
let mut multi_passkey = MultiPasskey::new();
multi_passkey.add_passkey(phone_credential_id, phone_public_key)?;
multi_passkey.add_passkey(laptop_credential_id, laptop_public_key)?;

// 3. Create encrypted backup
let backup = EncryptedBackup::create(account_data, encryption_key)?;
```

## Policy Evaluation

Policies are evaluated on-chain when transactions are executed:

1. **Transaction Amount**: Checked against spending/daily limits
2. **Timestamp**: Checked for time-locked policies
3. **Signatures**: Verified for multi-sig policies
4. **Result**: Transaction allowed, denied, or requires approval

## Security Considerations

- **Policy Enforcement**: Policies are enforced on-chain, not client-side
- **Multi-Passkey**: Each passkey is independently verified
- **Backup Encryption**: Backups use strong encryption (AES-256)
- **Recovery**: Recovery requires proper authentication

## Integration

This crate is used by:
- **smart-account**: For policy evaluation during transaction execution
- **programs/attesta**: For on-chain policy management
- **SDKs**: For client-side policy configuration

## Testing

```bash
cargo test
```

## Documentation

See the [Policy Configuration Guide](../../documentation/my-website/docs/sdk-and-integration/policy-configuration.md) for detailed usage examples.
