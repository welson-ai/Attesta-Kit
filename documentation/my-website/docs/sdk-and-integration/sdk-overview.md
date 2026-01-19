# SDK Overview

The Attesta SDK provides a comprehensive TypeScript/JavaScript library for integrating passkey-based account abstraction into your Solana applications. This guide gives you a high-level understanding of the SDK's capabilities and architecture.

## What is the Attesta SDK?

The Attesta SDK is a client-side library that enables developers to:

- **Register users** with passkey-based authentication
- **Create and authorize transactions** using WebAuthn/passkey signatures
- **Configure security policies** for accounts (spending limits, time locks, etc.)
- **Manage multi-passkey recovery** for account security
- **Interact with Attesta smart accounts** on the Solana blockchain

## Key Features

### 🔐 Passkey Authentication

The SDK handles all WebAuthn interactions, allowing users to authenticate using:
- **Biometric authentication** (TouchID, FaceID, Windows Hello)
- **Hardware security keys** (YubiKey, Titan, etc.)
- **Platform authenticators** (device-based passkeys)

All authentication happens locally on the user's device - biometric data never leaves their device.

### 💼 Account Management

- **Account Registration**: Create new Attesta accounts with passkey credentials
- **Account Retrieval**: Fetch account state from the blockchain
- **Policy Configuration**: Set and update security policies
- **Multi-Passkey Support**: Add multiple passkeys for recovery

### 💸 Transaction Handling

- **Payment Transactions**: Create and authorize SOL transfers
- **Custom Transactions**: Build and sign any Solana transaction
- **Authorization Proofs**: Generate WebAuthn signatures for transactions
- **Nonce Management**: Automatic replay protection

### 🛡️ Security & Privacy

- **Zero-knowledge authentication**: Biometric data never transmitted
- **Replay protection**: Nonce-based system prevents transaction replay
- **Policy enforcement**: On-chain policies ensure transaction compliance
- **Standard cryptography**: Uses FIDO2/WebAuthn and P-256 ECDSA

## Architecture

The SDK is built on top of:

- **@solana/web3.js**: Core Solana blockchain interaction
- **WebAuthn API**: Native browser passkey support
- **Attesta Program**: On-chain account abstraction program

```
┌─────────────────┐
│  Your DApp      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Attesta SDK    │
│  - Registration │
│  - Transactions │
│  - Policies     │
└────────┬────────┘
         │
    ┌────┴────┐
    ▼         ▼
┌─────────┐ ┌──────────────┐
│ WebAuthn│ │ Solana RPC   │
│  API    │ │   Node       │
└─────────┘ └──────┬───────┘
                   │
                   ▼
            ┌──────────────┐
            │ Attesta      │
            │ Program      │
            │ (On-chain)   │
            └──────────────┘
```

## SDK Components

### Core Modules

1. **Registration Module** (`register.ts`)
   - `registerAttestaAccount()`: Register a new account
   - `createWebAuthnCredential()`: Create a passkey credential

2. **Payment Module** (`pay.ts`)
   - `createPasskeyPayment()`: Create a payment transaction
   - `createAuthorizationProof()`: Generate WebAuthn signature

3. **Withdrawal Module** (`withdraw.ts`)
   - `createPasskeyWithdrawal()`: Create withdrawal transactions

4. **Types** (`index.ts`)
   - TypeScript interfaces for all SDK types
   - Account structures
   - Authorization proof formats

## Supported Environments

### Browser
- **Modern browsers** with WebAuthn support (Chrome, Firefox, Safari, Edge)
- **HTTPS required** for WebAuthn (localhost works for development)
- **UMD bundle** available for `<script>` tag inclusion

### Node.js
- **CommonJS** and **ES Modules** support
- Note: WebAuthn requires browser APIs, so registration/auth must happen in browser context

### Frameworks
- **React**: Full support
- **Vue**: Full support
- **Angular**: Full support
- **Next.js**: Full support (with proper SSR handling)
- **Svelte**: Full support

## Bundle Formats

The SDK is distributed in multiple formats:

- **CommonJS** (`dist/index.js`): For Node.js and bundlers
- **ES Modules** (`dist/index.esm.js`): For modern bundlers
- **UMD** (`dist/index.umd.js`): For browser `<script>` tags
- **TypeScript Definitions** (`dist/index.d.ts`): For TypeScript projects

## Prerequisites

Before using the SDK, ensure you have:

1. **Node.js** 16+ or **Browser** with WebAuthn support
2. **@solana/web3.js** installed (peer dependency)
3. **HTTPS** (or localhost) for WebAuthn to work
4. **Attesta Program** deployed on Solana (devnet or mainnet)

## Next Steps

- [Installation Guide](./installation.md) - Set up the SDK in your project
- [JavaScript/TypeScript SDK](./javascript-typescript-sdk.md) - Learn the API
- [Quickstart](../developer-guides/quickstart.md) - Get started in 5 minutes

## Example: Quick Look

```typescript
import { 
  registerAttestaAccount, 
  createWebAuthnCredential,
  createPasskeyPayment 
} from '@attesta/sdk';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';

// 1. Register a new account
const connection = new Connection('https://api.devnet.solana.com');
const userKeypair = Keypair.generate();
const challenge = crypto.getRandomValues(new Uint8Array(32));
const credential = await createWebAuthnCredential(
  challenge,
  userKeypair.publicKey.toBase58(),
  'Alice'
);

const { accountAddress } = await registerAttestaAccount(
  connection,
  userKeypair.publicKey,
  credential
);

// 2. Make a payment
const recipient = new PublicKey('...');
const { transaction, authorizationProof } = await createPasskeyPayment(
  connection,
  accountAddress,
  recipient,
  1_000_000_000, // 1 SOL
  credential.credentialId
);
```

## Support

- **Documentation**: See other guides in this section
- **Issues**: Report on GitHub
- **Community**: Join our Discord

---

**Ready to get started?** Head to the [Installation Guide](./installation.md)!
