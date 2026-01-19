# Error Handling and Security

This guide covers error handling patterns, security best practices, and common pitfalls when using the Attesta SDK.

## Error Handling

### WebAuthn Errors

WebAuthn operations can fail for various reasons. Always handle errors gracefully:

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
      switch (error.name) {
        case 'NotAllowedError':
          throw new Error('User cancelled passkey creation or denied permission');
        case 'NotSupportedError':
          throw new Error('WebAuthn is not supported in this browser');
        case 'InvalidStateError':
          throw new Error('A passkey already exists for this user');
        case 'SecurityError':
          throw new Error('WebAuthn requires HTTPS (or localhost)');
        case 'UnknownError':
          throw new Error('An unknown error occurred during passkey creation');
        default:
          throw new Error(`WebAuthn error: ${error.name}`);
      }
    }
    throw error;
  }
}
```

### Transaction Errors

```typescript
import { createPasskeyPayment } from '@attesta/sdk';
import { Connection, PublicKey } from '@solana/web3.js';

async function safePayment() {
  try {
    const { transaction, authorizationProof } = await createPasskeyPayment(
      connection,
      fromAccount,
      toAccount,
      amount,
      credentialId
    );
    
    const signature = await submitTransaction(transaction, authorizationProof);
    return signature;
  } catch (error) {
    // Handle specific error types
    if (error.message.includes('insufficient funds')) {
      throw new Error('Insufficient SOL balance');
    } else if (error.message.includes('nonce')) {
      throw new Error('Nonce conflict - please try again');
    } else if (error.message.includes('policy')) {
      throw new Error('Transaction blocked by account policy');
    } else if (error.message.includes('signature')) {
      throw new Error('Invalid authorization proof');
    } else if (error.message.includes('network')) {
      throw new Error('Network error - please check your connection');
    } else {
      throw new Error(`Transaction failed: ${error.message}`);
    }
  }
}
```

### Network Errors

```typescript
async function safeConnection(connection: Connection) {
  try {
    const blockhash = await connection.getLatestBlockhash();
    return blockhash;
  } catch (error) {
    if (error.message.includes('fetch')) {
      throw new Error('Failed to connect to Solana network');
    } else if (error.message.includes('timeout')) {
      throw new Error('Network request timed out');
    }
    throw error;
  }
}
```

## Security Best Practices

### 1. Always Use HTTPS

WebAuthn **requires** HTTPS (except for localhost). Never use HTTP in production:

```typescript
// ✅ Good
const isSecure = window.location.protocol === 'https:' || 
                 window.location.hostname === 'localhost';

if (!isSecure) {
  throw new Error('WebAuthn requires HTTPS');
}
```

### 2. Validate User Input

Always validate transaction parameters before creating transactions:

```typescript
function validatePayment(
  toAccount: string,
  amount: number
): { valid: boolean; error?: string } {
  // Validate recipient
  try {
    new PublicKey(toAccount);
  } catch {
    return { valid: false, error: 'Invalid recipient address' };
  }
  
  // Validate amount
  if (amount <= 0) {
    return { valid: false, error: 'Amount must be positive' };
  }
  
  if (amount > MAX_AMOUNT) {
    return { valid: false, error: 'Amount exceeds maximum' };
  }
  
  return { valid: true };
}
```

### 3. Store Credentials Securely

**Never store private keys** - passkeys are hardware-backed and don't expose private keys.

Store only:
- ✅ Credential ID (public identifier)
- ✅ Account address (public key)
- ✅ User preferences

Never store:
- ❌ Private keys (don't exist for passkeys)
- ❌ Biometric data (never leaves device)
- ❌ WebAuthn secrets (handled by browser)

```typescript
// ✅ Good - Store only public data
const userData = {
  accountAddress: accountAddress.toBase58(),
  credentialId: base64UrlEncode(credential.credentialId),
  userName: 'Alice'
};

localStorage.setItem('attesta_user', JSON.stringify(userData));

// ❌ Bad - Never store sensitive data
// localStorage.setItem('private_key', privateKey); // Don't do this!
```

### 4. Implement Replay Protection

Always use proper nonce management:

```typescript
async function getNextNonce(
  connection: Connection,
  accountAddress: PublicKey
): Promise<number> {
  // Always fetch from blockchain
  const accountInfo = await connection.getAccountInfo(accountAddress);
  
  if (!accountInfo) {
    throw new Error('Account not found');
  }
  
  // Deserialize and get current nonce
  const account = deserializeAccount(accountInfo.data);
  
  // Return incremented nonce
  return account.nonce + 1;
}
```

### 5. Verify Transaction Details

Always show transaction details to users before signing:

```typescript
async function confirmTransaction(
  transaction: Transaction,
  authorizationProof: AuthorizationProof
) {
  // Extract transaction details
  const details = {
    recipient: transaction.instructions[0].keys[1].pubkey.toBase58(),
    amount: extractAmount(transaction),
    fee: transaction.feePayer ? 'User pays' : 'Relayer pays',
  };
  
  // Show to user
  const confirmed = await showTransactionConfirmation(details);
  
  if (!confirmed) {
    throw new Error('User cancelled transaction');
  }
  
  // Proceed with submission
  return submitTransaction(transaction, authorizationProof);
}
```

### 6. Handle User Cancellation

Users can cancel WebAuthn prompts. Handle gracefully:

```typescript
async function safeAuthorization(challenge: Uint8Array, credentialId: Uint8Array) {
  try {
    const proof = await createAuthorizationProof(challenge, credentialId, challenge);
    return proof;
  } catch (error) {
    if (error.name === 'NotAllowedError') {
      // User cancelled - this is normal, don't treat as error
      return null;
    }
    throw error;
  }
}
```

### 7. Use Secure Random Challenges

Always use cryptographically secure random values for challenges:

```typescript
// ✅ Good
const challenge = crypto.getRandomValues(new Uint8Array(32));

// ❌ Bad - Predictable
const challenge = new Uint8Array([1, 2, 3, ...]);
```

### 8. Validate Authorization Proofs

Before submitting, validate authorization proofs:

```typescript
function validateAuthorizationProof(proof: AuthorizationProof): boolean {
  // Check nonce is present
  if (!proof.nonce || proof.nonce <= 0) {
    return false;
  }
  
  // Check signature components
  if (!proof.webauthnSignature.signature || 
      proof.webauthnSignature.signature.length === 0) {
    return false;
  }
  
  if (!proof.webauthnSignature.authenticatorData ||
      proof.webauthnSignature.authenticatorData.length < 37) {
    return false;
  }
  
  if (!proof.webauthnSignature.clientDataJSON ||
      proof.webauthnSignature.clientDataJSON.length === 0) {
    return false;
  }
  
  return true;
}
```

## Common Security Pitfalls

### ❌ Pitfall 1: Trusting Client-Side Validation

**Problem:**
```typescript
// Client-side only validation
if (amount > userBalance) {
  throw new Error('Insufficient funds');
}
// Transaction still submitted
```

**Solution:**
Always validate on-chain. Client-side validation is for UX only.

### ❌ Pitfall 2: Reusing Nonces

**Problem:**
```typescript
// Reusing same nonce
const nonce = 12345; // Fixed nonce
```

**Solution:**
Always fetch and increment nonces from blockchain.

### ❌ Pitfall 3: Not Verifying Signatures

**Problem:**
```typescript
// Trusting client-provided signature without verification
submitTransaction(transaction, clientProvidedProof);
```

**Solution:**
On-chain program must verify all signatures.

### ❌ Pitfall 4: Exposing Credential IDs

**Problem:**
```typescript
// Logging credential IDs in production
console.log('Credential ID:', credentialId);
```

**Solution:**
Credential IDs are public, but avoid unnecessary exposure.

### ❌ Pitfall 5: Not Handling Policy Failures

**Problem:**
```typescript
// Not checking policy before transaction
const { transaction } = await createPasskeyPayment(...);
// Transaction fails on-chain due to policy
```

**Solution:**
Check policies before creating transactions when possible.

## Security Checklist

Before deploying to production:

- [ ] HTTPS enabled (or localhost for development)
- [ ] Input validation on all user inputs
- [ ] Proper error handling for all operations
- [ ] Nonce management implemented correctly
- [ ] Transaction details shown to users
- [ ] Policies configured appropriately
- [ ] No sensitive data in logs
- [ ] Secure random challenges
- [ ] Authorization proofs validated
- [ ] Network errors handled gracefully
- [ ] User cancellation handled properly
- [ ] Transaction fees considered
- [ ] Rate limiting implemented (if needed)

## Threat Model

### What Attesta Protects Against

✅ **Seed Phrase Theft**: No seed phrases needed
✅ **Keyloggers**: Biometric authentication bypasses keyboards
✅ **Phishing**: WebAuthn verifies origin
✅ **Replay Attacks**: Nonce-based protection
✅ **Unauthorized Spending**: Policy enforcement

### What You Still Need to Protect

⚠️ **Malicious DApps**: Users must trust the DApp
⚠️ **Social Engineering**: Users can be tricked into approving transactions
⚠️ **Device Compromise**: If device is compromised, passkeys may be at risk
⚠️ **Network Attacks**: Use secure RPC endpoints

## Reporting Security Issues

If you discover a security vulnerability:

1. **Do not** open a public issue
2. Email security@attesta.io
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

## Additional Resources

- [WebAuthn Security](https://www.w3.org/TR/webauthn-2/#sctn-security-considerations)
- [Solana Security Best Practices](https://docs.solana.com/developing/programming-model/security)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)

## Next Steps

- [Transaction and Signing](./transaction-and-signing.md) - Learn transaction security
- [Policy Configuration](./policy-configuration.md) - Configure security policies
- [DApp Integration](../developer-guides/dapp-integration.md) - Secure integration patterns

---

**Security is a shared responsibility.** Follow these practices to keep your users safe!
