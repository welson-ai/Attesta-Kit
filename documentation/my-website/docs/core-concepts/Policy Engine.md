# Policy Engine

The **Attesta Policy Engine** defines rules that govern which transactions
are allowed on a smart account. Policies are **on-chain, enforceable rules**
that work in tandem with passkey-based authorization to secure user assets.

---

## What is a Policy?

A **policy** is a set of restrictions that control account behavior. It allows
users to:

- Limit spending per transaction or per day  
- Require multiple signatures for approval  
- Lock accounts until a certain time  
- Define open accounts with no restrictions  

Policies help protect users even if a passkey is compromised.

---

## Policy Types

Attesta currently supports the following policy types:

| Type | Description |
|------|-------------|
| `Open` | No restrictions; all transactions are allowed |
| `SpendingLimit` | Maximum amount allowed per transaction |
| `DailyLimit` | Maximum cumulative amount allowed per day |
| `MultiSig` | Requires multiple passkeys to sign a transaction |
| `TimeLocked` | Transactions allowed only after a specific unlock time |

Policies are **Borsh-encoded** and stored as bytes, allowing future
extensions without breaking old accounts.

---

## Policy Structure

Each policy contains:

- `policy_type` — The type of restriction (`Open`, `SpendingLimit`, etc.)  
- `config` — Type-specific configuration bytes

### Config Details

| Policy Type | Config |
|------------|--------|
| `Open` | Empty |
| `SpendingLimit` | 8 bytes: maximum amount in lamports (u64) |
| `DailyLimit` | 16 bytes: max amount (u64) + reset timestamp (i64) |
| `MultiSig` | Variable length: list of required signer public keys (32 bytes each) |
| `TimeLocked` | 8 bytes: unlock timestamp (i64) |

---

## How Policies Are Evaluated

1. **After signature verification** and before executing the transaction  
2. Policy engine reads the account’s `policy` field  
3. Depending on the policy type, checks are performed:

- **Open:** always allow  
- **SpendingLimit:** transaction amount ≤ max allowed  
- **DailyLimit:** transaction amount ≤ max per-transaction limit; reset handled via timestamp  
- **MultiSig:** execution layer ensures enough signatures  
- **TimeLocked:** current time ≥ unlock timestamp  

> **Note:** DailyLimit requires tracking daily totals in production; current implementation checks per-transaction only.

---

## Failure Behavior

If a transaction violates the policy:

- Execution is **aborted immediately**  
- **No state changes** occur  
- The nonce is **not incremented**  

This ensures failed transactions do not compromise account security.

---

## Updating Policies

- Only the **account owner** can update policies  
- Policy updates require **passkey authorization**  
- Policies can be updated at any time without modifying the core account structure  

This design allows users to adapt account restrictions dynamically.

---

## SDK Responsibilities

The SDKs help developers:

- Encode policies into the correct Borsh format  
- Decode policies for client-side inspection  
- Simulate policy checks before submitting transactions  
- Provide helper functions for creating common policies (spending limit, multi-sig, etc.)

---

## Security Considerations

- Policies are **enforced on-chain**, so they cannot be bypassed  
- Passkey authorization is **required** for any policy update  
- Combined with nonce-based replay protection, policies ensure safe and auditable transactions  

---

## Example Policies

```rust
// Open account
let policy = Policy::open();

// Spending limit: 1 SOL
let policy = Policy::spending_limit(1_000_000_000);

// Time-locked: unlock at Unix timestamp 2_000_000_000
let policy = Policy::time_locked(2_000_000_000);

// Multi-sig: require two signers
let policy = Policy::multi_sig(vec![signer1_pubkey, signer2_pubkey]);

// Daily limit: 10 SOL per day, reset at Unix timestamp 2_000_000_000
let policy = Policy::daily_limit(10_000_000_000, 2_000_000_000);
