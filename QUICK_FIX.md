# Quick Fix to Make Integration Work

## Step 1: Generate IDL (2 minutes)

```bash
# Build the Anchor program (generates IDL)
anchor build

# Copy IDL to SDK
./scripts/generate-idl.sh
```

## Step 2: Get Discriminators (1 minute)

```bash
# View the IDL to find discriminators
cat sdk/ts/idl/attesta.json | jq '.instructions[] | {name: .name, discriminator: .discriminator}'
```

Or manually:
```bash
cat sdk/ts/idl/attesta.json | grep -A 2 "discriminator"
```

You'll see something like:
```json
"discriminator": {
  "type": "u8",
  "value": [123, 45, 67, 89, ...]  // 8 bytes
}
```

## Step 3: Update Discriminators (2 minutes)

### In `sdk/ts/src/register.ts` (line ~154):

Find:
```typescript
const discriminator = Buffer.from([0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91]); // Placeholder
```

Replace with the actual discriminator from IDL for `initialize`:
```typescript
const discriminator = Buffer.from([/* actual 8 bytes from IDL */]);
```

### In `sdk/ts/src/instructions.ts`:

**For `execute` (line ~30):**
```typescript
const discriminator = Buffer.from([/* actual 8 bytes from IDL for execute */]);
```

**For `updatePolicy` (line ~70):**
```typescript
const discriminator = Buffer.from([/* actual 8 bytes from IDL for updatePolicy */]);
```

## Step 4: Test (Optional but Recommended)

```bash
cd sdk/ts
npm install
npm run build
```

## That's It!

After updating the discriminators, your integration should work! 🎉

## Alternative: Use Anchor Client (More Type-Safe)

Instead of manual discriminators, you can use the Anchor client which handles this automatically:

```typescript
import { Program, AnchorProvider } from '@coral-xyz/anchor';
import { Connection, Keypair } from '@solana/web3.js';
import idl from './idl/attesta.json';

const program = new Program(idl as any, programId, provider);

// No need for manual discriminators!
const tx = await program.methods
  .initialize(passkeyPublicKey, credentialId, policy)
  .accounts({
    attestaAccount: accountPDA,
    owner: ownerKeypair.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

This is the recommended approach for production!
