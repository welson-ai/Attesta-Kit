# Passkey Authentication

Attesta replaces traditional private keys with WebAuthn passkeys to enable secure, recoverable, and policy-controlled smart accounts on Solana.

This document describes the authentication, cryptography, replay protection, recovery, and security model implemented in the Attesta smart account system.

## Table of Contents

- Overview
- Design Goals
- System Architecture
- Cryptographic Error Model
- P-256 Signature Verification
- WebAuthn Authentication
- Replay Protection
- Smart Account Security Model
- Policy Enforcement Engine
- Multi-Passkey & Social Recovery
- Encrypted Backup & Account Recovery
- Security Guarantees
- Threat Model Coverage

## 1. Overview

Attesta enables users to control on-chain accounts using device-bound passkeys (TouchID, FaceID, hardware keys) instead of seed phrases.

All authentication is:

- Verified on-chain
- Backed by standard cryptography (P-256, SHA-256)
- Resistant to replay attacks
- Recoverable without custodians

## 2. Design Goals

- Eliminate seed phrases and raw private keys
- Use WebAuthn-compatible passkeys
- Support biometric and hardware authenticators
- Enforce transaction policies on-chain
- Prevent replay attacks
- Enable multi-device and social recovery
- Preserve decentralization and self-custody

## 3. System Architecture

### High-Level Flow

```text
+---------------------------+
|   User Device / Browser   |
|  (Passkey / WebAuthn)     |
+-------------+-------------+
              |
              v
+---------------------------+
|        core-crypto        |
|  - Signature verification |
|  - Replay protection      |
+-------------+-------------+
              |
              v
+---------------------------+
|      smart-account        |
|  - Account state          |
|  - Policy enforcement    |
|  - Tx authorization      |
+-------------+-------------+
              |
              v
+---------------------------+
|      Solana Runtime       |
|  - Ledger & programs     |
+-------------+-------------+
              ^
              |
+---------------------------+
|         recovery          |
|  - Guardians              |
|  - Multi-passkey restore  |
|  - Time-locked recovery   |
+---------------------------+

```


### Key Properties

- Private keys never leave the device
- Biometric data is never transmitted
- All verification is deterministic and auditable

## 4. Cryptographic Error Model (`errors.rs`)

All cryptographic failures are represented by a single error enum.

### Covered Error Cases

- Invalid WebAuthn signatures
- Invalid P-256 public keys
- Signature verification failures
- Malformed signatures
- Replay attacks
- Invalid or reused nonces
- Challenge mismatches
- Invalid credential IDs
- Invalid authenticator data

### Design Notes

- Errors map cleanly to Solana `ProgramError`
- Prevents ambiguous failure states
- Safe for on-chain execution

## 5. P-256 Signature Verification (`p256_verify.rs`)

Attesta uses ECDSA over P-256, the same curve mandated by WebAuthn.

### Verification Process

1. Validate public key format
2. Hash the message with SHA-256
3. Normalize signature length (64 or 65 bytes)
4. Verify ECDSA signature

### Supported Key Formats

- Uncompressed public keys (64 bytes: X || Y)
- Compressed public keys (33 bytes, decompressed on-chain)

### Security Properties

- Strong cryptographic guarantees
- Hardware-backed key compatibility
- Deterministic verification

## 6. WebAuthn Authentication (`webauthn.rs`)

### WebAuthnSignature Structure

This structure contains all data required to verify a passkey authentication on-chain.

#### Stored Fields

- Authenticator data
- Client data JSON
- ECDSA signature
- Credential ID

#### Serialization Format

Each field is length-prefixed for safe parsing:


#### Verification Steps

1. Validate authenticator data length
2. Ensure expected challenge exists in client data
3. Hash client data JSON (SHA-256)
4. Concatenate:



5. Verify P-256 signature

### Guarantees

- Signature is device-bound
- Challenge prevents cross-session replay
- No private or biometric data exposed

## 7. Replay Protection (`replay.rs`)

Replay attacks are prevented using cryptographic nonces.

### Nonce Generation

Nonce is derived as:


### Replay Protection Flow

1. Generate nonce
2. Validate nonce format
3. Check nonce has not been used
4. Execute transaction
5. Mark nonce as used

### Notes

- Designed for on-chain persistence
- Deterministic and user-specific
- Prevents transaction reuse

## 8. Smart Account Security Model (`account.rs`)

### AttestaAccount Responsibilities

- Store passkey public key
- Track WebAuthn credential ID
- Enforce replay protection via nonce
- Store policy configuration
- Track lifecycle timestamps

### Built-In Protections

- Replay resistance
- Policy enforcement
- Deterministic Borsh serialization
- Forward-compatible policy storage

## 9. Policy Enforcement Engine (`policy.rs`)

Policies limit account behavior even if a passkey is compromised.

### Supported Policy Types

- Open (no restrictions)
- Per-transaction spending limit
- Daily spending limit
- Multi-signature approval
- Time-locked accounts

### Policy Evaluation

- Policies are evaluated before execution, blocking invalid transactions early.

### Design Notes

- Policy configuration stored as raw bytes
- Forward compatible with new policy types
- No breaking changes to existing accounts

## 10. Multi-Passkey & Social Recovery (`multi_passkey.rs`)

Accounts can register multiple passkeys for redundancy and recovery.

### Capabilities

- Multi-device access
- Social recovery
- Hardware key backups
- Threshold-based recovery

### Recovery Rules

- Primary passkey cannot be removed
- Recovery requires N-of-M enabled passkeys
- Disabled passkeys are ignored

### Security Benefits

- Prevents single-device lockout
- Enables gradual key rotation
- No custodians required

## 11. Encrypted Backup & Account Recovery (`encrypted_backup.rs`)

Encrypted backups enable recovery even if all devices are lost.

### Encrypted Contents

- Passkey public keys
- Credential IDs
- Policy configurations
- Account metadata

### Encryption Model

- Key derived from recovery phrase
- Key hash stored for verification
- AES-GCM intended for production use
- Versioned backup format

### Important Notes

- No plaintext secrets on-chain
- Client-controlled encryption
- Forward-compatible backup format

## 12. Security Guarantees

- ✔ No private keys on-chain
- ✔ No biometric data transmitted
- ✔ Device-bound authentication
- ✔ Replay attack prevention
- ✔ Policy-based transaction limits
- ✔ Multi-passkey recovery
- ✔ Deterministic verification

## 13. Threat Model Coverage

| Threat            | Mitigation                     |
|------------------|--------------------------------|
| Replay attacks    | Nonces + counters              |
| Device theft      | Policies + limits              |
| Key extraction    | Hardware-backed passkeys       |
| Signature forgery | P-256 verification             |
| Lost devices      | Multi-passkey recovery         |
| Malformed input   | Strict parsing & errors        |

### Final Notes

This system is designed to be:

- Auditable
- Composable
- Future-proof
- Self-custodial
- Solana-native

Attesta provides smart account abstraction without sacrificing decentralization.
