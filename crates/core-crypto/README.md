# Core Crypto

The core cryptographic library for Attesta. This crate provides all the cryptographic primitives needed for passkey-based authentication on Solana.

## Overview

This crate handles:
- **WebAuthn signature verification**: Verifies signatures created by user devices (TouchID, FaceID, hardware keys)
- **P-256 elliptic curve cryptography**: Uses the P-256 curve (also called secp256r1) for signature verification
- **Replay protection**: Prevents the same transaction from being executed twice using nonces

## Key Components

### `p256_verify.rs`
Functions for verifying P-256 ECDSA signatures. This is what we use to verify that a signature was created by the private key matching a public key.

### `webauthn.rs`
WebAuthn-specific code. Handles the WebAuthn signature structure and verifies signatures according to the WebAuthn specification.

### `replay.rs`
Replay attack protection using nonces. Each transaction must use a unique nonce to prevent someone from submitting the same transaction twice.

### `errors.rs`
All error types used throughout the crypto library.

## Usage

```rust
use core_crypto::{verify_p256_signature, verify_webauthn_signature, WebAuthnSignature};

// Verify a WebAuthn signature
let webauthn_sig = WebAuthnSignature::new(/* ... */);
verify_webauthn_signature(&webauthn_sig, &public_key, &challenge)?;
```

## Security Notes

- All private keys stay on the user's device - we never see them
- We only verify signatures using public keys
- Nonces prevent replay attacks
- Uses industry-standard P-256 cryptography
