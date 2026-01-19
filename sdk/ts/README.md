# Attesta TypeScript SDK

TypeScript/JavaScript SDK for integrating Attesta passkey-based account abstraction into your Solana applications.

## Installation

```bash
npm install @attesta/sdk @solana/web3.js @coral-xyz/anchor
```

## Quick Start

```typescript
import { 
  registerAttestaAccount, 
  createWebAuthnCredential,
  createPasskeyPayment,
  getAttestaAccount,
  getNextNonce
} from '@attesta/sdk';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';

// Connect to Solana
const connection = new Connection('https://api.devnet.solana.com');

// 1. Register an account
const userKeypair = Keypair.generate();
const challenge = crypto.getRandomValues(new Uint8Array(32));
const credential = await createWebAuthnCredential(
  challenge,
  userKeypair.publicKey.toBase58(),
  'Alice'
);

const { accountAddress, transaction } = await registerAttestaAccount(
  connection,
  userKeypair.publicKey,
  credential
);

transaction.sign(userKeypair);
const signature = await connection.sendRawTransaction(transaction.serialize());
await connection.confirmTransaction(signature);

// 2. Make a payment
const recipient = new PublicKey('...');
const { transaction: paymentTx, authorizationProof } = await createPasskeyPayment(
  connection,
  accountAddress,
  recipient,
  1_000_000_000, // 1 SOL
  credential.credentialId
);

// Submit payment transaction with authorization proof
// (Implementation depends on your program's execute instruction)
```

## Features

- ✅ **WebAuthn Passkey Support**: Create and use passkeys for authentication
- ✅ **Account Registration**: Register Attesta accounts on-chain
- ✅ **Transaction Authorization**: Create authorization proofs with passkeys
- ✅ **Account State Management**: Fetch and manage account state
- ✅ **Policy Configuration**: Configure account policies
- ✅ **CBOR Parsing**: Extract public keys from WebAuthn credentials

## Configuration

Set environment variables for program ID and network:

```bash
# Program ID (required after deployment)
ATTESTA_PROGRAM_ID=YourProgramIdHere...

# Network (optional, defaults to devnet)
SOLANA_NETWORK=devnet
SOLANA_RPC_URL=https://api.devnet.solana.com
```

Or use the config functions:

```typescript
import { getAttestaProgramId, getNetworkConfig } from '@attesta/sdk';

const programId = getAttestaProgramId('devnet');
const config = getNetworkConfig('devnet');
```

## API Reference

### Registration

```typescript
// Create WebAuthn credential
const credential = await createWebAuthnCredential(
  challenge: Uint8Array,
  userId: string,
  userName: string
): Promise<WebAuthnCredential>

// Register account
const { accountAddress, transaction } = await registerAttestaAccount(
  connection: Connection,
  ownerPublicKey: PublicKey,
  credential: WebAuthnCredential,
  policy?: Uint8Array
): Promise<{ accountAddress: PublicKey, transaction: Transaction }>
```

### Account Management

```typescript
// Get account state
const account = await getAttestaAccount(
  connection: Connection,
  accountAddress: PublicKey
): Promise<AttestaAccount | null>

// Get next nonce
const nonce = await getNextNonce(
  connection: Connection,
  accountAddress: PublicKey
): Promise<number>

// Check if account exists
const exists = await accountExists(
  connection: Connection,
  accountAddress: PublicKey
): Promise<boolean>
```

### Transactions

```typescript
// Create payment
const { transaction, authorizationProof } = await createPasskeyPayment(
  connection: Connection,
  fromAccount: PublicKey,
  toAccount: PublicKey,
  amount: number,
  credentialId: Uint8Array
): Promise<{ transaction: Transaction, authorizationProof: AuthorizationProof }>

// Create execute instruction
const instruction = createExecuteInstruction(
  accountAddress: PublicKey,
  authorizationProof: AuthorizationProof,
  transactionData: Uint8Array,
  programId: PublicKey
): TransactionInstruction
```

## Important Notes

### Instruction Discriminators

The SDK uses placeholder instruction discriminators. After building your Anchor program, you need to:

1. Run `anchor build` to generate the IDL
2. Copy the actual discriminators from `target/idl/attesta.json`
3. Update the discriminators in `src/instructions.ts` and `src/register.ts`

### IDL Integration

For full type safety, integrate the generated IDL:

```typescript
import { Program, AnchorProvider } from '@coral-xyz/anchor';
import { Connection, Keypair } from '@solana/web3.js';
import idl from './idl/attesta.json';

const programId = new PublicKey('YourProgramId');
const provider = new AnchorProvider(
  connection,
  wallet,
  { commitment: 'confirmed' }
);

const program = new Program(idl as any, programId, provider);
```

### WebAuthn Requirements

- **HTTPS Required**: WebAuthn only works over HTTPS (or localhost)
- **Browser Only**: WebAuthn APIs are only available in browsers
- **User Interaction**: Credential creation requires user interaction

## Building

```bash
# Install dependencies
npm install

# Build all formats
npm run build

# Build specific format
npm run build:cjs   # CommonJS
npm run build:esm   # ES Modules
npm run build:umd    # UMD (browser)
npm run build:types # TypeScript definitions
```

## Development

```bash
# Watch mode
npm run dev

# Type checking
npx tsc --noEmit
```

## Troubleshooting

### "WebAuthn API not available"
- Ensure you're running in a browser
- Check that you're on HTTPS or localhost
- Verify browser supports WebAuthn

### "Public key extraction failed"
- Check that CBOR library is installed: `npm install cbor`
- Verify the credential was created with P-256 algorithm

### "Invalid instruction discriminator"
- Run `anchor build` to generate IDL
- Update discriminators in instruction builders
- Or use Anchor client with IDL for automatic handling

## Documentation

- [SDK Overview](../../documentation/my-website/docs/sdk-and-integration/sdk-overview.md)
- [Installation Guide](../../documentation/my-website/docs/sdk-and-integration/installation.md)
- [API Reference](../../documentation/my-website/docs/sdk-and-integration/javascript-typescript-sdk.md)
- [Integration Guide](../../documentation/my-website/docs/developer-guides/dapp-integration.md)

## License

MIT
