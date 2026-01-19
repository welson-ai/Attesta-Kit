---
sidebar_position: 2
---

# Architecture Overview

Attesta Kit is a **passkey-first infrastructure protocol on Solana**, designed to enable **secure, policy-driven smart accounts** with **non-custodial recovery**.  
This document describes the **system architecture, core components, data flows, and security model**.

---

## Overview

Attesta Kit brings **WebAuthn / passkey authentication** to Solana, allowing users to:

- Authenticate using **biometrics or hardware keys** instead of seed phrases  
- Keep biometric data **fully on-device** (never transmitted or stored on-chain)  
- Define **policies** for spending, permissions, and access control  
- Recover accounts using **multiple passkeys or guardians**  

The protocol abstracts cryptography and account logic while exposing **clean SDKs** for developers.

---

## System Architecture

At a high level, Attesta Kit sits between **user devices** and the **Solana runtime**, acting as a secure authorization and policy enforcement layer.

### High-Level Architecture
The core-crypto module provides the cryptographic foundation of Attesta Kit.

Responsibilities:

-WebAuthn / passkey signature verification

-P-256 (secp256r1) elliptic curve support

-Nonce-based replay protection

-Deterministic, auditable verification logic



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


---


### Flow Summary

1. User registers a passkey via WebAuthn → linked to a smart account  
2. User creates or executes a transaction → smart-account program validates against policies  
3. Transaction signed using passkey → core-crypto verifies signature and replay protection  
4. Optional recovery triggered → recovery module enforces policy and restores access  




---



### Passkey Authentication Flow

The smart-account program is the on-chain authority controlling assets and transactions.

Responsibilities:

-Owns user funds and account state

-Maps passkeys → account permissions

-Enforces on-chain policies

-Executes transactions on behalf of users

```text

User initiates action
        |
        v
Device prompts passkey
        |
        v
WebAuthn signature created
        |
        v
core-crypto verifies signature
        |
        v
smart-account enforces policy
        |
        v
Transaction executed on Solana

```

---

### Policy & Recovery Flow
The recovery module ensures accounts are recoverable without seed phrases.

Responsibilities:

-Multi-passkey recovery

-Guardian-based recovery flows

-Time-delayed or threshold-based restores

-Encrypted backup support

```text

User loses access
        |
        v
Recovery initiated
        |
        v
Guardians / Passkeys verified
        |
        v
Policy delay enforced
        |
        v
Account access restored

```

---

##  Project Structure
```text
attesta-solana/
├── crates/
│   ├── core-crypto/      # Cryptographic primitives
│   ├── smart-account/   # Solana account abstraction program
│   └── recovery/        # Recovery & policy logic
├── sdk/
│   ├── rust/            # Rust SDK (WIP)
│   └── ts/              # TypeScript SDK
├── demo/
│   ├── cli/             # CLI demo
│   └── web/             # Web demo
└── docs/                # Documentation


```