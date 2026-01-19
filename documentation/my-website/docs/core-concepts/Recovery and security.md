# Recovery & Security

Attesta is designed to be **secure by default** while remaining **recoverable**
without trusted intermediaries. This document explains how Attesta achieves
this using **multi-passkey recovery**, **encrypted backups**, and **on-chain
security guarantees**.

---

## Design Principles

Attesta’s recovery and security model follows these principles:

- No seed phrases or private keys
- No trusted custodians or admins
- No plaintext recovery data on-chain
- Explicit, auditable authorization
- Safe recovery even after total device loss

---

## Recovery Overview

Attesta supports **two complementary recovery mechanisms**:

1. **Multi-passkey recovery** (social / multi-device)
2. **Encrypted backups** (last-resort recovery)

These mechanisms are independent but composable.

---

## Multi-Passkey Recovery

### What Is Multi-Passkey?

An account can be controlled by **multiple passkeys**, each backed by a
separate WebAuthn credential. This enables:

- Multi-device access (phone, laptop, hardware key)
- Social recovery (trusted contacts)
- Threshold-based recovery

---

### Passkey Entry Model

Each passkey is represented by a `PasskeyEntry`.

| Field | Description |
|-----|------------|
| `public_key` | P-256 public key (64 bytes, uncompressed) |
| `credential_id` | WebAuthn credential identifier |
| `name` | Human-readable label (UTF-8) |
| `enabled` | Whether the passkey is active |
| `added_at` | Timestamp when the passkey was added |

Passkeys can be disabled without being removed, allowing soft revocation.

---

### MultiPasskey Structure

The `MultiPasskey` container manages all passkeys for an account.

| Field | Description |
|------|------------|
| `primary` | Primary authentication passkey |
| `additional` | Additional recovery / device passkeys |
| `recovery_threshold` | Minimum enabled passkeys required for recovery |
| `max_passkeys` | Maximum allowed passkeys |

---

### Recovery Thresholds

Recovery requires **N enabled passkeys**, where:

- `N = recovery_threshold`
- Threshold is clamped between `1` and `max_passkeys`
- Primary passkey **cannot be removed**

This ensures the account is never locked by mistake.

---

### Recovery Eligibility

Recovery is allowed if:


---

### Security Properties

- No single device can hijack recovery
- Passkeys can be rotated safely
- Social recovery without custody
- Explicit opt-in recovery configuration

---

## Encrypted Backup Recovery

### Purpose

Encrypted backups act as a **last-resort recovery mechanism** if **all
passkeys are lost**.

Backups contain everything required to reconstruct an account **except
the encryption key**.

---

### EncryptedBackup Structure

| Field | Description |
|------|------------|
| `key_hash` | SHA-256 hash of encryption key |
| `encrypted_data` | Encrypted recovery payload |
| `nonce` | 96-bit nonce (AES-GCM compatible) |
| `created_at` | Backup creation timestamp |
| `version` | Backup format version |

---

### What Is Encrypted?

The encrypted payload may include:

- Passkey public keys
- Credential IDs
- Policy configurations
- Account metadata

---

### Key Derivation

Encryption keys are derived from a **user-provided recovery phrase**:


**Properties:**

- Deterministic
- Never stored
- Never transmitted
- Known only to the user

---

### Encryption Model

- Designed for **AES-GCM**
- Authenticated encryption
- 96-bit nonce
- Tamper-resistant

> **Implementation Note**  
> Current implementations may include a placeholder encryption routine.
> Production deployments **must** use real AES-GCM encryption with constant-time
> operations.

---

### Backup Creation Flow

1. User generates a recovery phrase
2. SDK derives encryption key
3. Account recovery data is serialized
4. Data is encrypted client-side
5. Encrypted backup is stored

Backups can be stored:
- On-chain
- Decentralized storage
- Local device storage
- Cloud backups

---

### Backup Recovery Flow

1. User provides recovery phrase
2. Encryption key is derived
3. Key hash is verified
4. Encrypted data is decrypted
5. Account state is restored

If the key is incorrect, recovery **fails safely** with no partial state
application.

---

## Security Model

### Authorization Guarantees

Attesta enforces the following invariants:

- All actions require passkey authorization
- All signatures are verified on-chain
- Replay attacks are prevented using nonces
- Policies are enforced before execution

---

### Threats Defended Against

| Threat | Mitigation |
|------|-----------|
| Replay attacks | Monotonic nonces |
| Key theft | Hardware-backed WebAuthn keys |
| Policy bypass | On-chain policy evaluation |
| Unauthorized recovery | Threshold-based passkey checks |
| Backup leakage | Strong encryption + key secrecy |

---

### Threats Out of Scope

The following are **explicitly acknowledged** but not prevented:

- Compromised authenticator devices
- Users sharing recovery phrases
- Malicious or modified client software

---

## Failure Behavior

If any security check fails:

- Transaction is aborted
- No on-chain state is mutated
- Nonce is not incremented
- Recovery does not partially apply

This ensures **atomic safety**.

---

## Versioning & Forward Compatibility

- Backup format versioning
- Passkey structure extensibility
- Policy encoding flexibility

This allows future upgrades without breaking existing accounts.

---

## Developer Notes

- All encryption occurs client-side
- Programs operate on opaque byte arrays
- SDKs provide helpers for:
  - Passkey management
  - Backup encryption / decryption
  - Recovery simulation
- No trusted recovery actors exist

---

## Documentation Roadmap

The following documents are recommended to complete the Attesta
documentation set:

1. `sdk-integration.md` — Rust & TypeScript usage
2. `transaction-flow.md` — End-to-end execution lifecycle
3. `threat-model.md` — Formal security analysis (audit-ready)
4. `glossary.md` — Terminology reference

---

## Summary

Attesta’s recovery and security model provides:

- Seedless self-custody
- Hardware-backed authentication
- Social and cryptographic recovery
- Strong on-chain enforcement
- Long-term extensibility

Users remain in control — even in worst-case scenarios.

