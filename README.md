# Attesta - Account Abstraction on Solana with Passkeys

Attesta is an account abstraction protocol implemented on Solana, enabling passkey-based authorization and policy-driven execution while preserving user privacy at the authorization layer.

## Overview

Attesta brings WebAuthn/passkey authentication to Solana, allowing users to:

- **Use biometric authentication** (TouchID, FaceID, hardware keys) instead of seed phrases
- **Maintain privacy** - biometric data never leaves the user's device
- **Configure policies** - set spending limits, transaction requirements, and recovery options
- **Social recovery** - use multiple passkeys for account recovery

## Architecture

Attesta consists of three core components:

### 1. **core-crypto** - Cryptographic Primitives
- WebAuthn signature verification (P-256 elliptic curve)
- Replay protection (nonce-based)
- Secure cryptographic operations

### 2. **smart-account** - Account Abstraction Program
- Account state management
- Passkey authorization verification
- Policy-driven transaction execution
- On-chain account storage

### 3. **recovery** - Recovery & Policy Management
- Multi-passkey support
- Policy configuration (spending limits, time locks, etc.)
- Encrypted backup functionality

## 📦 Project Structure

```
attesta-solana/
├── crates/
│   ├── core-crypto/          # Cryptographic primitives
│   ├── smart-account/         # Solana program (account abstraction)
│   └── recovery/              # Recovery & policy management
├── sdk/
│   ├── rust/                  # Rust SDK (WIP)
│   └── ts/                    # TypeScript SDK
├── demo/
│   ├── cli/                   # CLI demo application
│   └── web/                   # Web demo application
└── docs/                      # Documentation
```

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable)
- Solana CLI (1.18+) - [Install](https://docs.solana.com/cli/install-solana-cli-tools)
- Anchor Framework (0.29+) - [Install](https://www.anchor-lang.com/docs/installation)
- Node.js & npm (for TypeScript SDK)
- A WebAuthn-compatible browser (Chrome, Firefox, Safari, Edge)

### Quick Deploy

```bash
# Install dependencies (see DEPLOYMENT.md for details)
# Then deploy:
./scripts/deploy.sh devnet
```

See [QUICK_START.md](QUICK_START.md) for a 3-step deployment guide.

### Building the Crates

```bash
# Build all Rust crates
cargo build --release

# Build the Anchor program
anchor build

# Or use the build script
./scripts/build.sh
```

### TypeScript SDK

```bash
# Build the SDK (bundles for CommonJS, ESM, and UMD)
cd sdk/ts
npm install
npm run build

# Or use the build script
./scripts/build-sdk.sh
```

The SDK is bundled in multiple formats:
- **CommonJS**: `dist/index.js` (Node.js)
- **ES Modules**: `dist/index.esm.js` (Modern bundlers)
- **UMD**: `dist/index.umd.js` (Browser `<script>` tag)
- **Types**: `dist/index.d.ts` (TypeScript definitions)

### Rust SDK

```bash
# Build the SDK
cd sdk/rust
cargo build --release

# Or use the build script
./scripts/build-sdk.sh
```

### Running the Demo

```bash
cd demo/web
npm install
npm start
```

## 🔐 How It Works

### 1. Registration

1. User generates a Solana keypair
2. User creates a WebAuthn credential (passkey) using their device's biometric authenticator
3. Passkey's public key is registered on-chain in an Attesta account
4. User can now use their passkey to authorize transactions

### 2. Transaction Authorization

1. User creates a transaction
2. Transaction hash is used as a challenge for WebAuthn
3. User authenticates with their passkey (TouchID, FaceID, etc.)
4. WebAuthn signature is created (stays on device - private key never exposed)
5. Signature is submitted to the Attesta program for verification
6. Program verifies the P-256 signature on-chain
7. If valid and policy allows, transaction executes

### 3. Privacy Guarantee

- **Biometric data never leaves the device** - authentication happens locally
- **Only the signature is sent on-chain** - no fingerprint, face scan, or device info
- **Replay protection** - each transaction uses a unique nonce

## 📚 API Documentation

### TypeScript SDK

**Installation:**
```bash
npm install @attesta/sdk
```

**Usage:**
```typescript
import { registerAttestaAccount, createWebAuthnCredential } from '@attesta/sdk';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';

// Register a new account
const connection = new Connection('https://api.devnet.solana.com');
const userKeypair = Keypair.generate();
const credential = await createWebAuthnCredential(challenge, userId, userName);
const { accountAddress } = await registerAttestaAccount(
  connection,
  userKeypair.publicKey,
  credential
);

// Make a payment
import { createPasskeyPayment } from '@attesta/sdk';
const { transaction, authorizationProof } = await createPasskeyPayment(
  connection,
  fromAccount,
  toAccount,
  amountLamports,
  credentialId
);
```

**Browser (UMD):**
```html
<script src="https://unpkg.com/@attesta/sdk/dist/index.umd.js"></script>
<script>
  const { registerAttestaAccount } = AttestaSDK;
</script>
```

### Rust SDK

**Installation:**
```toml
[dependencies]
attesta-sdk = "0.1.0"
```

**Usage:**
```rust
use attesta_sdk::AttestaClient;
use anchor_client::Cluster;
use solana_program::pubkey::Pubkey;

// Create client
let client = AttestaClient::new(Cluster::Devnet, program_id);

// Get account
let account = client.get_account(&account_address)?;
```

See [sdk/BUNDLING.md](sdk/BUNDLING.md) for bundling details.

## 🛡️ Security Considerations

- **WebAuthn Standard**: Uses FIDO2/WebAuthn for industry-standard security
- **P-256 Cryptography**: ECDSA with P-256 curve (widely trusted)
- **Replay Protection**: Nonce-based system prevents transaction replay
- **Policy Enforcement**: On-chain policies ensure transaction compliance
- **Privacy First**: No biometric data or device information is ever transmitted

## 🔮 Roadmap

- [x] Full Anchor program implementation
- [x] Deployment scripts and configuration
- [ ] Multi-signature policies (structure ready)
- [x] Time-locked transactions (policy ready)
- [x] Social recovery flows (multi-passkey ready)
- [ ] Encrypted backup restore (structure ready)
- [ ] Mobile SDK (iOS/Android)
- [ ] Production deployment (ready for mainnet)

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- Solana Labs for the amazing blockchain platform
- FIDO Alliance for WebAuthn/FIDO2 standards
- The Rust and TypeScript communities

---

**Note**: This is a prototype implementation. Do not use in production without thorough security auditing.
