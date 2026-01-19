# JavaScript/TypeScript SDK

Complete API reference and usage guide for the Attesta TypeScript/JavaScript SDK.

## Table of Contents

- [Core Concepts](#core-concepts)
- [Registration](#registration)
- [Authentication](#authentication)
- [Transactions](#transactions)
- [Account Management](#account-management)
- [Types Reference](#types-reference)
- [Examples](#examples)

## Core Concepts

### Connection

All SDK functions require a Solana `Connection` instance:

```typescript
import { Connection } from '@solana/web3.js';

const connection = new Connection('https://api.devnet.solana.com');
```

### Public Keys

The SDK uses Solana `PublicKey` objects:

```typescript
import { PublicKey } from '@solana/web3.js';

const accountAddress = new PublicKey('YourAccountAddressHere...');
```

### Credentials

WebAuthn credentials are represented as `WebAuthnCredential` objects:

```typescript
interface WebAuthnCredential {
  id: string; // Base64url encoded credential ID
  publicKey: Uint8Array; // 64 bytes uncompressed P-256 public key
  credentialId: Uint8Array; // Raw credential ID
}
```

## Registration

### Creating a WebAuthn Credential

Before registering an account, you need to create a passkey credential:

```typescript
import { createWebAuthnCredential } from '@attesta/sdk';

async function createPasskey(userId: string, userName: string) {
  // Generate a random challenge (32 bytes)
  const challenge = crypto.getRandomValues(new Uint8Array(32));
  
  // Create the credential
  const credential = await createWebAuthnCredential(
    challenge,
    userId,        // Unique user identifier
    userName       // Display name
  );
  
  return credential;
}
```

**Parameters:**
- `challenge: Uint8Array` - Random 32-byte challenge for WebAuthn
- `userId: string` - Unique identifier for the user (typically their public key as base58)
- `userName: string` - Display name for the user

**Returns:** `Promise<WebAuthnCredential>`

**Example:**

```typescript
const userKeypair = Keypair.generate();
const credential = await createWebAuthnCredential(
  crypto.getRandomValues(new Uint8Array(32)),
  userKeypair.publicKey.toBase58(),
  'Alice'
);

console.log('Credential ID:', credential.id);
console.log('Public Key:', credential.publicKey);
```

### Registering an Attesta Account

Once you have a credential, register it on-chain:

```typescript
import { registerAttestaAccount } from '@attesta/sdk';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';

async function registerAccount() {
  const connection = new Connection('https://api.devnet.solana.com');
  const userKeypair = Keypair.generate();
  
  // Create credential
  const challenge = crypto.getRandomValues(new Uint8Array(32));
  const credential = await createWebAuthnCredential(
    challenge,
    userKeypair.publicKey.toBase58(),
    'Alice'
  );
  
  // Register account
  const { accountAddress, transaction } = await registerAttestaAccount(
    connection,
    userKeypair.publicKey,
    credential
  );
  
  // Sign and send transaction
  transaction.sign(userKeypair);
  const signature = await connection.sendRawTransaction(
    transaction.serialize()
  );
  
  await connection.confirmTransaction(signature);
  
  return { accountAddress, credential };
}
```

**Parameters:**
- `connection: Connection` - Solana connection
- `ownerPublicKey: PublicKey` - Owner's public key
- `credential: WebAuthnCredential` - WebAuthn credential
- `policy?: Uint8Array` - Optional policy configuration

**Returns:** `Promise<{ accountAddress: PublicKey, transaction: Transaction }>`

## Authentication

### Creating Authorization Proofs

To authorize a transaction, create an authorization proof:

```typescript
import { createAuthorizationProof } from '@attesta/sdk';

async function authorizeTransaction(
  transactionHash: Uint8Array,
  credentialId: Uint8Array
) {
  const proof = await createAuthorizationProof(
    transactionHash,  // Challenge (transaction hash)
    credentialId,      // User's credential ID
    transactionHash    // Message hash
  );
  
  return proof;
}
```

**Parameters:**
- `challenge: Uint8Array` - Challenge bytes (typically transaction hash)
- `credentialId: Uint8Array` - User's credential ID
- `messageHash: Uint8Array` - Message to sign (typically transaction hash)

**Returns:** `Promise<AuthorizationProof>`

**AuthorizationProof Structure:**

```typescript
interface AuthorizationProof {
  webauthnSignature: {
    authenticatorData: Uint8Array;
    clientDataJSON: Uint8Array;
    signature: Uint8Array;
    credentialId: Uint8Array;
  };
  nonce: number;
  messageHash: Uint8Array;
}
```

## Transactions

### Creating Payments

Create a payment transaction authorized by a passkey:

```typescript
import { createPasskeyPayment } from '@attesta/sdk';

async function makePayment(
  fromAccount: PublicKey,
  toAccount: PublicKey,
  amount: number,
  credentialId: Uint8Array
) {
  const connection = new Connection('https://api.devnet.solana.com');
  
  const { transaction, authorizationProof } = await createPasskeyPayment(
    connection,
    fromAccount,
    toAccount,
    amount,        // Amount in lamports (1 SOL = 1_000_000_000 lamports)
    credentialId
  );
  
  // Submit transaction with authorization proof to Attesta program
  // (Implementation depends on your program's instruction format)
  
  return { transaction, authorizationProof };
}
```

**Parameters:**
- `connection: Connection` - Solana connection
- `fromAccount: PublicKey` - Source Attesta account
- `toAccount: PublicKey` - Destination account
- `amount: number` - Amount in lamports
- `credentialId: Uint8Array` - User's credential ID

**Returns:** `Promise<{ transaction: Transaction, authorizationProof: AuthorizationProof }>`

**Example:**

```typescript
const fromAccount = new PublicKey('...');
const toAccount = new PublicKey('...');
const amount = 1_000_000_000; // 1 SOL

const { transaction, authorizationProof } = await createPasskeyPayment(
  connection,
  fromAccount,
  toAccount,
  amount,
  credential.credentialId
);

// The transaction includes the transfer instruction
// The authorizationProof contains the WebAuthn signature
```

### Creating Withdrawals

Similar to payments, but may have different policy requirements:

```typescript
import { createPasskeyWithdrawal } from '@attesta/sdk';

async function withdraw(
  fromAccount: PublicKey,
  toAccount: PublicKey,
  amount: number,
  credentialId: Uint8Array
) {
  const connection = new Connection('https://api.devnet.solana.com');
  
  const { transaction, authorizationProof } = await createPasskeyWithdrawal(
    connection,
    fromAccount,
    toAccount,
    amount,
    credentialId
  );
  
  return { transaction, authorizationProof };
}
```

## Account Management

### Fetching Account State

To get account information from the blockchain:

```typescript
import { Connection, PublicKey } from '@solana/web3.js';

async function getAccountInfo(accountAddress: PublicKey) {
  const connection = new Connection('https://api.devnet.solana.com');
  
  // Fetch account data
  const accountInfo = await connection.getAccountInfo(accountAddress);
  
  if (!accountInfo) {
    throw new Error('Account not found');
  }
  
  // Deserialize account data (implementation depends on your program)
  // const account = deserializeAttestaAccount(accountInfo.data);
  
  return accountInfo;
}
```

### Account Structure

Attesta accounts contain:

```typescript
interface AttestaAccount {
  owner: string;              // Owner's public key (base58)
  passkeyPublicKey: Uint8Array; // P-256 public key
  credentialId: Uint8Array;    // Credential ID
  nonce: number;              // Current nonce (for replay protection)
  policy: Uint8Array;         // Policy configuration
  createdAt: number;          // Creation timestamp
  updatedAt: number;          // Last update timestamp
}
```

## Types Reference

### WebAuthnCredential

```typescript
interface WebAuthnCredential {
  id: string;                 // Base64url encoded credential ID
  publicKey: Uint8Array;      // 64 bytes uncompressed P-256 public key
  credentialId: Uint8Array;   // Raw credential ID
}
```

### AttestaAccount

```typescript
interface AttestaAccount {
  owner: string;              // Pubkey as base58 string
  passkeyPublicKey: Uint8Array;
  credentialId: Uint8Array;
  nonce: number;
  policy: Uint8Array;
  createdAt: number;
  updatedAt: number;
}
```

### AuthorizationProof

```typescript
interface AuthorizationProof {
  webauthnSignature: {
    authenticatorData: Uint8Array;
    clientDataJSON: Uint8Array;
    signature: Uint8Array;
    credentialId: Uint8Array;
  };
  nonce: number;
  messageHash: Uint8Array;
}
```

## Examples

### Complete Registration Flow

```typescript
import {
  registerAttestaAccount,
  createWebAuthnCredential
} from '@attesta/sdk';
import { Connection, Keypair } from '@solana/web3.js';

async function completeRegistration() {
  const connection = new Connection('https://api.devnet.solana.com');
  const userKeypair = Keypair.generate();
  
  // Step 1: Create passkey
  const challenge = crypto.getRandomValues(new Uint8Array(32));
  const credential = await createWebAuthnCredential(
    challenge,
    userKeypair.publicKey.toBase58(),
    'Alice'
  );
  
  console.log('Passkey created:', credential.id);
  
  // Step 2: Register account
  const { accountAddress, transaction } = await registerAttestaAccount(
    connection,
    userKeypair.publicKey,
    credential
  );
  
  console.log('Account address:', accountAddress.toBase58());
  
  // Step 3: Sign and send
  transaction.sign(userKeypair);
  const signature = await connection.sendRawTransaction(
    transaction.serialize()
  );
  
  await connection.confirmTransaction(signature);
  console.log('Account registered!', signature);
  
  return { accountAddress, credential };
}
```

### Complete Payment Flow

```typescript
import { createPasskeyPayment } from '@attesta/sdk';
import { Connection, PublicKey } from '@solana/web3.js';

async function completePayment(
  fromAccount: PublicKey,
  toAccount: PublicKey,
  amount: number,
  credentialId: Uint8Array
) {
  const connection = new Connection('https://api.devnet.solana.com');
  
  // Step 1: Create payment with authorization
  const { transaction, authorizationProof } = await createPasskeyPayment(
    connection,
    fromAccount,
    toAccount,
    amount,
    credentialId
  );
  
  console.log('Payment transaction created');
  console.log('Authorization proof:', authorizationProof);
  
  // Step 2: Submit to Attesta program
  // (Implementation depends on your program's instruction format)
  // const signature = await submitTransactionWithProof(
  //   connection,
  //   transaction,
  //   authorizationProof
  // );
  
  return { transaction, authorizationProof };
}
```

### Error Handling

```typescript
import { createWebAuthnCredential } from '@attesta/sdk';

async function safeCreateCredential() {
  try {
    const credential = await createWebAuthnCredential(
      crypto.getRandomValues(new Uint8Array(32)),
      'user-id',
      'User Name'
    );
    return credential;
  } catch (error) {
    if (error instanceof DOMException) {
      if (error.name === 'NotAllowedError') {
        throw new Error('User cancelled passkey creation');
      } else if (error.name === 'NotSupportedError') {
        throw new Error('WebAuthn not supported in this browser');
      }
    }
    throw error;
  }
}
```

## Best Practices

1. **Store Credentials Securely**: Never store credential private keys (they don't exist - passkeys are hardware-backed)

2. **Handle User Cancellation**: Users can cancel WebAuthn prompts - handle gracefully

3. **Validate Transactions**: Always validate transaction amounts and recipients before signing

4. **Error Handling**: Wrap SDK calls in try-catch blocks

5. **Nonce Management**: Ensure nonces are properly incremented to prevent replay attacks

6. **HTTPS Required**: WebAuthn only works over HTTPS (or localhost)

## Next Steps

- [Policy Configuration](./policy-configuration.md) - Configure account policies
- [Transaction and Signing](./transaction-and-signing.md) - Deep dive into transactions
- [Error Handling and Security](./error-handling-and-security.md) - Security best practices

---

**Need help?** Check out the [Quickstart Guide](../developer-guides/quickstart.md) or [DApp Integration](../developer-guides/dapp-integration.md).
