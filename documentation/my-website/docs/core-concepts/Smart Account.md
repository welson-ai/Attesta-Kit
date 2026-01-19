# Smart Account

The **Attesta Smart Account** is the core on-chain abstraction that enables
**passkey-based authentication** on Solana.

Instead of relying on traditional private keys or seed phrases, Attesta
accounts authorize transactions using **WebAuthn passkeys**
(biometrics or hardware-backed keys).

This program is responsible for **account state**, **authorization**,
**policy enforcement**, and **replay protection**.

---

## What Makes an Attesta Account “Smart”

An Attesta Smart Account differs from a standard Solana account in several
important ways:

- Passkey-based authorization (WebAuthn / P-256)
- Policy-aware transaction execution
- Built-in replay protection using nonces
- Recoverable by design (multi-passkey & social recovery)
- Composable with other Solana programs via CPI

---

## On-Chain Account Model

Each Attesta account is represented on-chain by the `AttestaAccount` struct
and serialized using **Borsh**.

### Account Fields

| Field | Type | Description |
|------|------|------------|
| `owner` | `Pubkey` | Solana address that owns this smart account |
| `passkey_public_key` | `[u8; 64]` | P-256 public key (X + Y) from WebAuthn |
| `credential_id` | `Vec<u8>` | WebAuthn credential identifier |
| `nonce` | `u64` | Monotonically increasing counter for replay protection |
| `policy` | `Vec<u8>` | Encoded policy configuration |
| `created_at` | `i64` | Account creation timestamp |
| `updated_at` | `i64` | Last execution timestamp |

### Design Rationale

- No private keys are ever stored on-chain
- Policies are stored as bytes to allow forward compatibility
- Nonce-based replay protection is simple, deterministic, and auditable
- Timestamps enable time-based policies and auditing

---

## Account Lifecycle

### 1. Account Creation (Registration)

When a user first registers with Attesta:

1. A passkey is created on the user’s device
2. The public key and credential ID are extracted
3. A new `AttestaAccount` is created on-chain
4. The nonce is initialized to `0`

The account is now ready to authorize transactions.

---

### 2. Transaction Authorization

To authorize a transaction:

1. The client constructs a transaction intent
2. The intent includes:
   - Instruction data
   - Account address
   - Proposed nonce
3. The intent is signed using the user’s passkey
4. The signature is submitted on-chain

Only the signature is sent on-chain — biometric data never leaves the device.

---

### 3. Signature Verification

On-chain verification ensures that:

- The WebAuthn signature is valid (P-256)
- The signature matches the stored passkey public key
- The credential ID is correct
- The authorization request is well-formed

Invalid signatures are rejected immediately.

---

### 4. Replay Protection

Each transaction includes a nonce supplied by the user.

Rules:

- The provided nonce must be **strictly greater** than the stored nonce
- Equal or lower nonces are rejected
- After successful execution, the nonce is incremented

This prevents replay attacks and invalidates old signatures permanently.

---

### 5. Policy Enforcement

Before execution, the account’s policy is evaluated.

Policies may include:

- Spending limits
- Time locks
- Transaction restrictions
- Multi-approval requirements

If a policy check fails, execution is aborted before any state changes occur.

Policies are stored as raw bytes to support future upgrades without breaking
existing accounts.

---

### 6. Transaction Execution

If all checks pass:

1. The transaction is executed
2. The nonce is incremented
3. The `updated_at` timestamp is refreshed
4. Any downstream CPI calls are performed

The smart account acts as an execution gateway for other Solana programs.

---

## Serialization & Storage

Attesta accounts use **Borsh serialization**, which provides:

- Deterministic layouts
- Efficient encoding
- Strong ecosystem support in Solana tooling

This ensures safe upgrades and stable SDK integrations.

---

## Security Model

### Defended Against

- Replay attacks
- Unauthorized transaction execution
- Policy bypass attempts
- Signature forgery

### Assumptions

- The user’s authenticator device is trusted
- WebAuthn private keys cannot be extracted
- Solana runtime isolation is enforced

---

## Recovery & Extensibility

While the smart account stores the active passkey, recovery mechanisms can:

- Add or rotate passkeys
- Restore access via social recovery
- Update or replace policy configurations

These features integrate without modifying the core account model.

---

## Developer Notes

- SDK-first design
- Clients never manage private keys
- Authorization flows are explicit and auditable
- Built to support future policy and recovery upgrades without migrations
