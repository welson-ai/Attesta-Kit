# Transaction and Signing

This guide explains how transactions work in Attesta, how to create them, sign them with passkeys, and submit them to the blockchain.

## Overview

Attesta transactions work differently from traditional Solana transactions:

1. **Transaction Creation**: Build a Solana transaction
2. **Challenge Generation**: Hash the transaction to create a challenge
3. **Passkey Signing**: User authenticates with passkey (biometric)
4. **Authorization Proof**: WebAuthn signature is created
5. **On-Chain Verification**: Attesta program verifies the signature
6. **Policy Check**: Program checks if policy allows the transaction
7. **Execution**: Transaction executes if all checks pass

## Transaction Flow

```
┌─────────────────┐
│ Create Transaction │
└────────┬──────────┘
         │
         ▼
┌─────────────────┐
│ Hash Transaction │
└────────┬──────────┘
         │
         ▼
┌─────────────────┐
│ Passkey Prompt  │
│ (Biometric)     │
└────────┬──────────┘
         │
         ▼
┌─────────────────┐
│ WebAuthn        │
│ Signature       │
└────────┬──────────┘
         │
         ▼
┌─────────────────┐
│ Submit to       │
│ Attesta Program │
└────────┬──────────┘
         │
         ▼
┌─────────────────┐
│ Verify & Execute│
└─────────────────┘
```

## Creating Transactions

### Basic Payment Transaction

```typescript
import { createPasskeyPayment } from '@attesta/sdk';
import { Connection, PublicKey } from '@solana/web3.js';

async function createPayment() {
  const connection = new Connection('https://api.devnet.solana.com');
  const fromAccount = new PublicKey('...');
  const toAccount = new PublicKey('...');
  const amount = 1_000_000_000; // 1 SOL
  const credentialId = new Uint8Array([...]); // User's credential ID
  
  const { transaction, authorizationProof } = await createPasskeyPayment(
    connection,
    fromAccount,
    toAccount,
    amount,
    credentialId
  );
  
  return { transaction, authorizationProof };
}
```

### Custom Transactions

You can create custom transactions with any Solana instructions:

```typescript
import { createAuthorizationProof } from '@attesta/sdk';
import { Connection, PublicKey, Transaction, SystemProgram } from '@solana/web3.js';

async function createCustomTransaction(
  fromAccount: PublicKey,
  instructions: TransactionInstruction[]
) {
  const connection = new Connection('https://api.devnet.solana.com');
  
  // Create transaction
  const transaction = new Transaction();
  transaction.add(...instructions);
  
  // Get recent blockhash
  const { blockhash } = await connection.getLatestBlockhash();
  transaction.recentBlockhash = blockhash;
  transaction.feePayer = fromAccount;
  
  // Hash transaction for challenge
  const transactionHash = await hashTransaction(transaction);
  
  // Get next nonce
  const nonce = await getNextNonce(connection, fromAccount);
  
  // Create authorization proof
  const credentialId = new Uint8Array([...]);
  const authorizationProof = await createAuthorizationProof(
    transactionHash,
    credentialId,
    transactionHash
  );
  
  return { transaction, authorizationProof };
}

async function hashTransaction(transaction: Transaction): Promise<Uint8Array> {
  const serialized = transaction.serialize({
    requireAllSignatures: false,
    verifySignatures: false,
  });
  const hashBuffer = await crypto.subtle.digest('SHA-256', serialized);
  return new Uint8Array(hashBuffer);
}
```

## Transaction Hashing

The transaction hash serves as the WebAuthn challenge. It must be:

1. **Deterministic**: Same transaction = same hash
2. **Unique**: Different transactions = different hashes
3. **Secure**: SHA-256 hash of serialized transaction

```typescript
async function hashTransaction(transaction: Transaction): Promise<Uint8Array> {
  // Serialize transaction without signatures
  const serialized = transaction.serialize({
    requireAllSignatures: false,
    verifySignatures: false,
  });
  
  // Hash with SHA-256
  const hashBuffer = await crypto.subtle.digest('SHA-256', serialized);
  return new Uint8Array(hashBuffer);
}
```

## Passkey Signing

### Creating Authorization Proofs

The authorization proof contains the WebAuthn signature:

```typescript
import { createAuthorizationProof } from '@attesta/sdk';

async function signTransaction(
  transactionHash: Uint8Array,
  credentialId: Uint8Array
): Promise<AuthorizationProof> {
  // This will prompt the user for biometric authentication
  const proof = await createAuthorizationProof(
    transactionHash,  // Challenge
    credentialId,      // User's credential
    transactionHash    // Message to sign
  );
  
  return proof;
}
```

### What Happens During Signing

1. **Browser Prompt**: User sees biometric prompt (TouchID, FaceID, etc.)
2. **Local Authentication**: Biometric happens on device (never leaves device)
3. **Signature Creation**: WebAuthn creates cryptographic signature
4. **Proof Generation**: SDK packages signature into authorization proof

**Important**: The private key never leaves the secure element. Only the signature is created.

## Authorization Proof Structure

```typescript
interface AuthorizationProof {
  webauthnSignature: {
    authenticatorData: Uint8Array;  // Authenticator state
    clientDataJSON: Uint8Array;      // Client data (challenge, origin, etc.)
    signature: Uint8Array;           // ECDSA P-256 signature
    credentialId: Uint8Array;        // Credential identifier
  };
  nonce: number;                     // Replay protection
  messageHash: Uint8Array;          // Transaction hash
}
```

### Understanding the Components

**authenticatorData**: Contains:
- RP ID hash
- Flags (user present, user verified, etc.)
- Signature counter
- Attested credential data

**clientDataJSON**: JSON string containing:
- `type`: "webauthn.get"
- `challenge`: Base64url encoded challenge
- `origin`: Origin of the request
- `crossOrigin`: Whether cross-origin

**signature**: ECDSA P-256 signature over:
- `authenticatorData || SHA256(clientDataJSON)`

## Nonce Management

Nonces prevent replay attacks. Each transaction must use a unique, incrementing nonce.

```typescript
async function getNextNonce(
  connection: Connection,
  accountAddress: PublicKey
): Promise<number> {
  // Fetch account from blockchain
  const accountInfo = await connection.getAccountInfo(accountAddress);
  
  if (!accountInfo) {
    throw new Error('Account not found');
  }
  
  // Deserialize account (implementation depends on your program)
  // const account = deserializeAttestaAccount(accountInfo.data);
  // return account.nonce + 1;
  
  // Placeholder
  return Date.now();
}
```

**Best Practices:**
- Always fetch current nonce from blockchain
- Increment nonce for each transaction
- Never reuse nonces
- Handle nonce conflicts (transaction may have been submitted)

## Submitting Transactions

### Submitting to Attesta Program

After creating a transaction and authorization proof, submit them to the Attesta program:

```typescript
async function submitTransaction(
  connection: Connection,
  transaction: Transaction,
  authorizationProof: AuthorizationProof,
  accountAddress: PublicKey
): Promise<string> {
  // Create instruction to execute transaction with proof
  const executeInstruction = createExecuteInstruction(
    accountAddress,
    transaction,
    authorizationProof
  );
  
  // Create wrapper transaction
  const wrapperTransaction = new Transaction();
  wrapperTransaction.add(executeInstruction);
  
  // Get recent blockhash
  const { blockhash } = await connection.getLatestBlockhash();
  wrapperTransaction.recentBlockhash = blockhash;
  
  // Sign with fee payer (or use a relayer)
  // wrapperTransaction.sign(feePayer);
  
  // Send transaction
  const signature = await connection.sendRawTransaction(
    wrapperTransaction.serialize()
  );
  
  // Confirm transaction
  await connection.confirmTransaction(signature);
  
  return signature;
}
```

### Transaction Execution Instruction

The Attesta program needs an instruction that includes:

1. Account address
2. Transaction to execute
3. Authorization proof
4. Policy verification

```typescript
function createExecuteInstruction(
  accountAddress: PublicKey,
  transaction: Transaction,
  proof: AuthorizationProof
): TransactionInstruction {
  // Serialize transaction
  const transactionBytes = transaction.serialize({
    requireAllSignatures: false,
    verifySignatures: false,
  });
  
  // Serialize authorization proof
  const proofBytes = serializeAuthorizationProof(proof);
  
  // Create instruction data
  const data = Buffer.concat([
    Buffer.from([0]), // Instruction discriminator
    transactionBytes,
    proofBytes,
  ]);
  
  return new TransactionInstruction({
    keys: [
      { pubkey: accountAddress, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      // Add other required accounts
    ],
    programId: ATTESTA_PROGRAM_ID,
    data,
  });
}
```

## Complete Transaction Example

Here's a complete example of creating and submitting a payment:

```typescript
import {
  createPasskeyPayment,
  createAuthorizationProof
} from '@attesta/sdk';
import { Connection, PublicKey } from '@solana/web3.js';

async function completePaymentFlow() {
  const connection = new Connection('https://api.devnet.solana.com');
  const fromAccount = new PublicKey('...');
  const toAccount = new PublicKey('...');
  const amount = 1_000_000_000; // 1 SOL
  const credentialId = new Uint8Array([...]);
  
  try {
    // Step 1: Create payment transaction
    console.log('Creating payment transaction...');
    const { transaction, authorizationProof } = await createPasskeyPayment(
      connection,
      fromAccount,
      toAccount,
      amount,
      credentialId
    );
    
    console.log('Transaction created');
    console.log('Authorization proof:', authorizationProof);
    
    // Step 2: Submit to Attesta program
    console.log('Submitting transaction...');
    const signature = await submitTransaction(
      connection,
      transaction,
      authorizationProof,
      fromAccount
    );
    
    console.log('Transaction submitted:', signature);
    
    // Step 3: Wait for confirmation
    console.log('Waiting for confirmation...');
    await connection.confirmTransaction(signature);
    
    console.log('Payment successful!');
    return signature;
    
  } catch (error) {
    console.error('Payment failed:', error);
    throw error;
  }
}
```

## Transaction Fees

Attesta transactions require Solana transaction fees. You have several options:

### 1. User Pays Fees

User's account pays the fees:

```typescript
transaction.feePayer = userAccount;
```

### 2. Relayer Pays Fees

A relayer service pays fees on behalf of users:

```typescript
// Relayer signs and pays fees
transaction.feePayer = relayerPublicKey;
transaction.sign(relayerKeypair);
```

### 3. Fee Subsidy

Some applications subsidize fees for users.

## Error Handling

### Common Transaction Errors

```typescript
try {
  const signature = await submitTransaction(...);
} catch (error) {
  if (error.message.includes('insufficient funds')) {
    console.error('Not enough SOL for transaction');
  } else if (error.message.includes('nonce')) {
    console.error('Nonce conflict - transaction may have been submitted');
  } else if (error.message.includes('policy')) {
    console.error('Transaction blocked by policy');
  } else if (error.message.includes('signature')) {
    console.error('Invalid authorization proof');
  } else {
    console.error('Transaction failed:', error);
  }
}
```

## Best Practices

1. **Always Hash Consistently**: Use the same hashing method for challenge and verification
2. **Handle User Cancellation**: Users can cancel biometric prompts
3. **Validate Before Signing**: Check amounts, recipients, and policies before prompting
4. **Manage Nonces**: Always fetch current nonce from blockchain
5. **Error Handling**: Provide clear error messages to users
6. **Transaction Status**: Show transaction status and confirmations
7. **Retry Logic**: Implement retry for transient failures

## Security Considerations

1. **Replay Protection**: Nonces prevent transaction replay
2. **Challenge Binding**: Transaction hash is bound to signature via challenge
3. **Policy Enforcement**: Policies are checked on-chain
4. **Signature Verification**: P-256 signatures are verified on-chain
5. **Biometric Privacy**: Biometric data never leaves device

## Next Steps

- [Error Handling and Security](./error-handling-and-security.md) - Security best practices
- [Policy Configuration](./policy-configuration.md) - Configure transaction policies
- [DApp Integration](../developer-guides/dapp-integration.md) - Full integration guide

---

**Ready to build?** Check out the [Quickstart Guide](../developer-guides/quickstart.md)!
