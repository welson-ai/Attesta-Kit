# Policy Configuration

Policies are security rules that control what transactions are allowed on your Attesta account. This guide explains how to configure and use policies.

## What are Policies?

Policies are on-chain rules that restrict account behavior, providing an additional security layer beyond passkey authentication. Even if someone gains access to your passkey, policies can limit the damage they can do.

**Example Use Cases:**
- Limit spending to 1 SOL per transaction
- Lock account until a specific date (time-locked savings)
- Require multiple passkeys for large transactions
- Set daily spending limits

## Policy Types

### 1. Open Policy

No restrictions - all transactions are allowed (default).

```typescript
// No policy configuration needed - this is the default
const policy = new Uint8Array(0);
```

**Use Case:** Maximum flexibility, trust your passkey completely.

### 2. Spending Limit Policy

Limits the maximum amount per transaction.

```typescript
// Limit to 1 SOL per transaction
const maxAmountLamports = 1_000_000_000; // 1 SOL

// Serialize policy (format: [policy_type, ...amount_bytes])
const policyBytes = new Uint8Array(9);
policyBytes[0] = 1; // PolicyType.SpendingLimit = 1
const amountBytes = new Uint8Array(new BigUint64Array([BigInt(maxAmountLamports)]).buffer);
policyBytes.set(amountBytes, 1);

const policy = policyBytes;
```

**Use Case:** Protect against large unauthorized transactions.

**Example:**
```typescript
// Create spending limit of 0.5 SOL
const spendingLimit = 500_000_000; // 0.5 SOL in lamports
const policy = createSpendingLimitPolicy(spendingLimit);

// Register account with policy
const { accountAddress } = await registerAttestaAccount(
  connection,
  userPublicKey,
  credential,
  policy
);
```

### 3. Daily Limit Policy

Limits the maximum amount per day.

```typescript
// Limit to 10 SOL per day
const maxAmountLamports = 10_000_000_000; // 10 SOL
const resetTimestamp = Math.floor(Date.now() / 1000) + 86400; // Reset in 24 hours

// Serialize policy
// Format: [policy_type, ...amount_bytes (8), ...reset_timestamp_bytes (8)]
const policyBytes = new Uint8Array(17);
policyBytes[0] = 2; // PolicyType.DailyLimit = 2

const amountBytes = new Uint8Array(new BigUint64Array([BigInt(maxAmountLamports)]).buffer);
policyBytes.set(amountBytes, 1);

const resetBytes = new Uint8Array(new BigInt64Array([BigInt(resetTimestamp)]).buffer);
policyBytes.set(resetBytes, 9);

const policy = policyBytes;
```

**Use Case:** Prevent excessive daily spending.

**Example:**
```typescript
// Daily limit of 5 SOL, resets at midnight
const dailyLimit = 5_000_000_000;
const tomorrowMidnight = new Date();
tomorrowMidnight.setHours(24, 0, 0, 0);
const resetTimestamp = Math.floor(tomorrowMidnight.getTime() / 1000);

const policy = createDailyLimitPolicy(dailyLimit, resetTimestamp);
```

### 4. Time-Locked Policy

Locks the account until a specific timestamp.

```typescript
// Lock until January 1, 2025
const unlockTimestamp = Math.floor(new Date('2025-01-01').getTime() / 1000);

// Serialize policy
// Format: [policy_type, ...unlock_timestamp_bytes (8)]
const policyBytes = new Uint8Array(9);
policyBytes[0] = 4; // PolicyType.TimeLocked = 4
const timestampBytes = new Uint8Array(new BigInt64Array([BigInt(unlockTimestamp)]).buffer);
policyBytes.set(timestampBytes, 1);

const policy = policyBytes;
```

**Use Case:** Lock savings account until a specific date.

**Example:**
```typescript
// Lock account for 1 year
const oneYearFromNow = Math.floor(Date.now() / 1000) + (365 * 24 * 60 * 60);
const policy = createTimeLockedPolicy(oneYearFromNow);
```

### 5. Multi-Signature Policy

Requires multiple passkeys to sign the same transaction.

```typescript
// Require 2 out of 3 passkeys
const requiredSigners = [
  new PublicKey('Signer1PublicKey...'),
  new PublicKey('Signer2PublicKey...'),
  new PublicKey('Signer3PublicKey...')
];

// Serialize policy
// Format: [policy_type, ...signer_count, ...signer_pubkeys (32 bytes each)]
const policyBytes = new Uint8Array(1 + 1 + (requiredSigners.length * 32));
policyBytes[0] = 3; // PolicyType.MultiSig = 3
policyBytes[1] = requiredSigners.length;

let offset = 2;
for (const signer of requiredSigners) {
  policyBytes.set(signer.toBuffer(), offset);
  offset += 32;
}

const policy = policyBytes;
```

**Use Case:** Shared accounts, high-security transactions.

**Example:**
```typescript
// Require both phone and laptop passkeys
const phonePasskey = new PublicKey('...');
const laptopPasskey = new PublicKey('...');
const policy = createMultiSigPolicy([phonePasskey, laptopPasskey]);
```

## Policy Helper Functions

Here are some helper functions to create policies:

```typescript
/**
 * Creates a spending limit policy
 */
function createSpendingLimitPolicy(maxAmountLamports: number): Uint8Array {
  const policyBytes = new Uint8Array(9);
  policyBytes[0] = 1; // SpendingLimit
  const amountBytes = new Uint8Array(
    new BigUint64Array([BigInt(maxAmountLamports)]).buffer
  );
  policyBytes.set(amountBytes, 1);
  return policyBytes;
}

/**
 * Creates a daily limit policy
 */
function createDailyLimitPolicy(
  maxAmountLamports: number,
  resetTimestamp: number
): Uint8Array {
  const policyBytes = new Uint8Array(17);
  policyBytes[0] = 2; // DailyLimit
  const amountBytes = new Uint8Array(
    new BigUint64Array([BigInt(maxAmountLamports)]).buffer
  );
  policyBytes.set(amountBytes, 1);
  const resetBytes = new Uint8Array(
    new BigInt64Array([BigInt(resetTimestamp)]).buffer
  );
  policyBytes.set(resetBytes, 9);
  return policyBytes;
}

/**
 * Creates a time-locked policy
 */
function createTimeLockedPolicy(unlockTimestamp: number): Uint8Array {
  const policyBytes = new Uint8Array(9);
  policyBytes[0] = 4; // TimeLocked
  const timestampBytes = new Uint8Array(
    new BigInt64Array([BigInt(unlockTimestamp)]).buffer
  );
  policyBytes.set(timestampBytes, 1);
  return policyBytes;
}

/**
 * Creates a multi-signature policy
 */
function createMultiSigPolicy(requiredSigners: PublicKey[]): Uint8Array {
  const policyBytes = new Uint8Array(1 + 1 + (requiredSigners.length * 32));
  policyBytes[0] = 3; // MultiSig
  policyBytes[1] = requiredSigners.length;
  let offset = 2;
  for (const signer of requiredSigners) {
    policyBytes.set(signer.toBuffer(), offset);
    offset += 32;
  }
  return policyBytes;
}
```

## Setting Policies During Registration

Set a policy when registering a new account:

```typescript
import { registerAttestaAccount, createWebAuthnCredential } from '@attesta/sdk';

async function registerWithPolicy() {
  const connection = new Connection('https://api.devnet.solana.com');
  const userKeypair = Keypair.generate();
  
  // Create credential
  const credential = await createWebAuthnCredential(
    crypto.getRandomValues(new Uint8Array(32)),
    userKeypair.publicKey.toBase58(),
    'Alice'
  );
  
  // Create policy (spending limit of 1 SOL)
  const policy = createSpendingLimitPolicy(1_000_000_000);
  
  // Register with policy
  const { accountAddress } = await registerAttestaAccount(
    connection,
    userKeypair.publicKey,
    credential,
    policy // Pass policy here
  );
  
  return accountAddress;
}
```

## Updating Policies

To update a policy on an existing account, you'll need to call the Attesta program's update policy instruction:

```typescript
// This is a conceptual example - actual implementation depends on your program
async function updatePolicy(
  accountAddress: PublicKey,
  newPolicy: Uint8Array,
  credentialId: Uint8Array
) {
  const connection = new Connection('https://api.devnet.solana.com');
  
  // Create update policy transaction
  const transaction = new Transaction();
  
  // Add update policy instruction
  // (Implementation depends on your program's instruction format)
  transaction.add(
    createUpdatePolicyInstruction(accountAddress, newPolicy)
  );
  
  // Get recent blockhash
  const { blockhash } = await connection.getLatestBlockhash();
  transaction.recentBlockhash = blockhash;
  transaction.feePayer = accountAddress;
  
  // Create authorization proof
  const transactionHash = await hashTransaction(transaction);
  const authorizationProof = await createAuthorizationProof(
    transactionHash,
    credentialId,
    transactionHash
  );
  
  // Submit transaction with proof
  // (Implementation depends on your program)
  
  return transaction;
}
```

## Policy Evaluation

Policies are evaluated on-chain when transactions are executed. The program checks:

1. **Spending Limit**: Transaction amount ≤ limit
2. **Daily Limit**: Transaction amount ≤ limit AND daily total ≤ limit
3. **Time Lock**: Current timestamp ≥ unlock timestamp
4. **Multi-Sig**: Required number of signatures present

If a policy check fails, the transaction is rejected.

## Policy Examples

### Example 1: Conservative User

```typescript
// Small spending limit for daily use
const policy = createSpendingLimitPolicy(100_000_000); // 0.1 SOL
```

### Example 2: Savings Account

```typescript
// Lock for 1 year
const oneYear = Math.floor(Date.now() / 1000) + (365 * 24 * 60 * 60);
const policy = createTimeLockedPolicy(oneYear);
```

### Example 3: Family Account

```typescript
// Require both parents' passkeys
const parent1 = new PublicKey('...');
const parent2 = new PublicKey('...');
const policy = createMultiSigPolicy([parent1, parent2]);
```

### Example 4: Business Account

```typescript
// Daily limit of 50 SOL
const dailyLimit = 50_000_000_000;
const resetAtMidnight = Math.floor(
  new Date().setHours(24, 0, 0, 0) / 1000
);
const policy = createDailyLimitPolicy(dailyLimit, resetAtMidnight);
```

## Best Practices

1. **Start Conservative**: Begin with lower limits and increase as needed
2. **Document Policies**: Keep track of what policies you've set
3. **Test Policies**: Test policy enforcement in devnet before mainnet
4. **Review Regularly**: Periodically review and update policies
5. **Backup Recovery**: Ensure you have recovery options if policies lock you out

## Policy Serialization Format

For reference, here's the binary format for each policy type:

```
Open: []
SpendingLimit: [0x01, ...amount (u64, little-endian)]
DailyLimit: [0x02, ...amount (u64), ...reset_timestamp (i64)]
MultiSig: [0x03, ...count (u8), ...pubkey1 (32 bytes), ...pubkey2 (32 bytes), ...]
TimeLocked: [0x04, ...unlock_timestamp (i64, little-endian)]
```

## Troubleshooting

### Policy Too Restrictive

If a policy is blocking legitimate transactions:

1. Check the policy type and values
2. Verify timestamps are correct
3. Ensure transaction amounts are within limits
4. Update the policy if needed

### Policy Not Enforcing

If policies aren't being enforced:

1. Verify the policy was set correctly during registration
2. Check that the program is evaluating policies
3. Ensure policy bytes are correctly serialized

## Next Steps

- [Transaction and Signing](./transaction-and-signing.md) - Learn how transactions work with policies
- [Error Handling and Security](./error-handling-and-security.md) - Security best practices
- [DApp Integration](../developer-guides/dapp-integration.md) - Integrate policies into your DApp

---

**Ready to configure policies?** Check out the [Quickstart Guide](../developer-guides/quickstart.md) to see policies in action!
