# DApp Integration

Complete guide for integrating Attesta into your decentralized application (DApp). This guide covers architecture patterns, best practices, and real-world examples.

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Integration Patterns](#integration-patterns)
- [User Flow](#user-flow)
- [State Management](#state-management)
- [Error Handling](#error-handling)
- [UI/UX Best Practices](#uiux-best-practices)
- [Complete Example](#complete-example)
- [Production Checklist](#production-checklist)

## Architecture Overview

### Recommended DApp Architecture

```
┌─────────────────────────────────────┐
│         User Interface              │
│  (React/Vue/Angular/etc.)          │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│      Attesta SDK Integration       │
│  - Registration                     │
│  - Authentication                   │
│  - Transactions                     │
└──────────────┬──────────────────────┘
               │
        ┌──────┴──────┐
        ▼             ▼
┌──────────────┐  ┌──────────────┐
│  WebAuthn    │  │  Solana RPC  │
│  API         │  │  Connection   │
└──────────────┘  └──────┬───────┘
                         │
                         ▼
                  ┌──────────────┐
                  │  Attesta     │
                  │  Program     │
                  │  (On-chain)  │
                  └──────────────┘
```

## Integration Patterns

### Pattern 1: Simple Integration

For small DApps or prototypes:

```typescript
// app.ts
import { 
  registerAttestaAccount, 
  createWebAuthnCredential,
  createPasskeyPayment 
} from '@attesta/sdk';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';

class AttestaService {
  private connection: Connection;
  private accountAddress: PublicKey | null = null;
  private credential: any = null;
  
  constructor(rpcUrl: string) {
    this.connection = new Connection(rpcUrl);
  }
  
  async register(userName: string) {
    const keypair = Keypair.generate();
    const challenge = crypto.getRandomValues(new Uint8Array(32));
    
    const credential = await createWebAuthnCredential(
      challenge,
      keypair.publicKey.toBase58(),
      userName
    );
    
    const { accountAddress, transaction } = await registerAttestaAccount(
      this.connection,
      keypair.publicKey,
      credential
    );
    
    transaction.sign(keypair);
    const signature = await this.connection.sendRawTransaction(
      transaction.serialize()
    );
    await this.connection.confirmTransaction(signature);
    
    this.accountAddress = accountAddress;
    this.credential = credential;
    
    return { accountAddress, keypair, credential };
  }
  
  async pay(to: PublicKey, amount: number) {
    if (!this.accountAddress || !this.credential) {
      throw new Error('Account not registered');
    }
    
    const { transaction, authorizationProof } = await createPasskeyPayment(
      this.connection,
      this.accountAddress,
      to,
      amount,
      this.credential.credentialId
    );
    
    return { transaction, authorizationProof };
  }
}
```

### Pattern 2: React Hook Integration

For React applications:

```typescript
// useAttesta.ts
import { useState, useCallback } from 'react';
import { 
  registerAttestaAccount, 
  createWebAuthnCredential,
  createPasskeyPayment 
} from '@attesta/sdk';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';

interface AttestaState {
  accountAddress: PublicKey | null;
  credential: any | null;
  isRegistered: boolean;
  isLoading: boolean;
  error: string | null;
}

export function useAttesta(connection: Connection) {
  const [state, setState] = useState<AttestaState>({
    accountAddress: null,
    credential: null,
    isRegistered: false,
    isLoading: false,
    error: null,
  });
  
  const register = useCallback(async (userName: string) => {
    setState(prev => ({ ...prev, isLoading: true, error: null }));
    
    try {
      const keypair = Keypair.generate();
      const challenge = crypto.getRandomValues(new Uint8Array(32));
      
      const credential = await createWebAuthnCredential(
        challenge,
        keypair.publicKey.toBase58(),
        userName
      );
      
      const { accountAddress, transaction } = await registerAttestaAccount(
        connection,
        keypair.publicKey,
        credential
      );
      
      transaction.sign(keypair);
      const signature = await connection.sendRawTransaction(
        transaction.serialize()
      );
      await connection.confirmTransaction(signature);
      
      setState({
        accountAddress,
        credential,
        isRegistered: true,
        isLoading: false,
        error: null,
      });
      
      return { accountAddress, keypair, credential };
    } catch (error: any) {
      setState(prev => ({
        ...prev,
        isLoading: false,
        error: error.message,
      }));
      throw error;
    }
  }, [connection]);
  
  const pay = useCallback(async (to: PublicKey, amount: number) => {
    if (!state.accountAddress || !state.credential) {
      throw new Error('Account not registered');
    }
    
    setState(prev => ({ ...prev, isLoading: true, error: null }));
    
    try {
      const { transaction, authorizationProof } = await createPasskeyPayment(
        connection,
        state.accountAddress,
        to,
        amount,
        state.credential.credentialId
      );
      
      setState(prev => ({ ...prev, isLoading: false }));
      return { transaction, authorizationProof };
    } catch (error: any) {
      setState(prev => ({
        ...prev,
        isLoading: false,
        error: error.message,
      }));
      throw error;
    }
  }, [connection, state.accountAddress, state.credential]);
  
  return {
    ...state,
    register,
    pay,
  };
}
```

**Usage in React component:**

```tsx
// PaymentComponent.tsx
import { useAttesta } from './useAttesta';
import { Connection, PublicKey } from '@solana/web3.js';

function PaymentComponent() {
  const connection = new Connection('https://api.devnet.solana.com');
  const { accountAddress, isRegistered, register, pay, isLoading, error } = useAttesta(connection);
  
  const handleRegister = async () => {
    try {
      await register('Alice');
    } catch (err) {
      console.error('Registration failed:', err);
    }
  };
  
  const handlePay = async () => {
    try {
      const recipient = new PublicKey('...');
      const { transaction, authorizationProof } = await pay(recipient, 1e9);
      // Submit transaction...
    } catch (err) {
      console.error('Payment failed:', err);
    }
  };
  
  if (!isRegistered) {
    return (
      <div>
        <button onClick={handleRegister} disabled={isLoading}>
          {isLoading ? 'Registering...' : 'Register Account'}
        </button>
        {error && <p>Error: {error}</p>}
      </div>
    );
  }
  
  return (
    <div>
      <p>Account: {accountAddress?.toBase58()}</p>
      <button onClick={handlePay} disabled={isLoading}>
        {isLoading ? 'Processing...' : 'Make Payment'}
      </button>
      {error && <p>Error: {error}</p>}
    </div>
  );
}
```

### Pattern 3: Context Provider Pattern

For applications that need Attesta throughout:

```typescript
// AttestaContext.tsx
import React, { createContext, useContext, useState, useCallback } from 'react';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { 
  registerAttestaAccount, 
  createWebAuthnCredential,
  createPasskeyPayment 
} from '@attesta/sdk';

interface AttestaContextType {
  accountAddress: PublicKey | null;
  credential: any | null;
  isRegistered: boolean;
  register: (userName: string) => Promise<void>;
  pay: (to: PublicKey, amount: number) => Promise<any>;
  isLoading: boolean;
  error: string | null;
}

const AttestaContext = createContext<AttestaContextType | null>(null);

export function AttestaProvider({ 
  children, 
  connection 
}: { 
  children: React.ReactNode;
  connection: Connection;
}) {
  const [accountAddress, setAccountAddress] = useState<PublicKey | null>(null);
  const [credential, setCredential] = useState<any | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const register = useCallback(async (userName: string) => {
    setIsLoading(true);
    setError(null);
    
    try {
      const keypair = Keypair.generate();
      const challenge = crypto.getRandomValues(new Uint8Array(32));
      
      const cred = await createWebAuthnCredential(
        challenge,
        keypair.publicKey.toBase58(),
        userName
      );
      
      const { accountAddress: addr, transaction } = await registerAttestaAccount(
        connection,
        keypair.publicKey,
        cred
      );
      
      transaction.sign(keypair);
      const signature = await connection.sendRawTransaction(
        transaction.serialize()
      );
      await connection.confirmTransaction(signature);
      
      setAccountAddress(addr);
      setCredential(cred);
    } catch (err: any) {
      setError(err.message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [connection]);
  
  const pay = useCallback(async (to: PublicKey, amount: number) => {
    if (!accountAddress || !credential) {
      throw new Error('Account not registered');
    }
    
    setIsLoading(true);
    setError(null);
    
    try {
      const { transaction, authorizationProof } = await createPasskeyPayment(
        connection,
        accountAddress,
        to,
        amount,
        credential.credentialId
      );
      
      return { transaction, authorizationProof };
    } catch (err: any) {
      setError(err.message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [connection, accountAddress, credential]);
  
  return (
    <AttestaContext.Provider
      value={{
        accountAddress,
        credential,
        isRegistered: accountAddress !== null,
        register,
        pay,
        isLoading,
        error,
      }}
    >
      {children}
    </AttestaContext.Provider>
  );
}

export function useAttestaContext() {
  const context = useContext(AttestaContext);
  if (!context) {
    throw new Error('useAttestaContext must be used within AttestaProvider');
  }
  return context;
}
```

## User Flow

### Registration Flow

```
1. User clicks "Create Account"
   ↓
2. Generate Solana keypair
   ↓
3. Prompt for passkey creation
   ↓
4. User authenticates (biometric)
   ↓
5. Create WebAuthn credential
   ↓
6. Register account on-chain
   ↓
7. Show success / account address
```

### Payment Flow

```
1. User enters payment details
   ↓
2. Validate inputs
   ↓
3. Show transaction summary
   ↓
4. User confirms
   ↓
5. Create transaction
   ↓
6. Prompt for passkey authentication
   ↓
7. User authenticates (biometric)
   ↓
8. Generate authorization proof
   ↓
9. Submit to Attesta program
   ↓
10. Show transaction status
```

## State Management

### Local Storage

Store user data securely:

```typescript
const STORAGE_KEY = 'attesta_user';

interface StoredUser {
  accountAddress: string;
  credentialId: string;
  userName: string;
}

function saveUser(user: StoredUser) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(user));
}

function loadUser(): StoredUser | null {
  const stored = localStorage.getItem(STORAGE_KEY);
  return stored ? JSON.parse(stored) : null;
}

function clearUser() {
  localStorage.removeItem(STORAGE_KEY);
}
```

### Session Management

```typescript
class AttestaSession {
  private accountAddress: PublicKey | null = null;
  private credential: any | null = null;
  
  async initialize() {
    const stored = loadUser();
    if (stored) {
      this.accountAddress = new PublicKey(stored.accountAddress);
      // Load credential (implementation depends on storage)
    }
  }
  
  isAuthenticated(): boolean {
    return this.accountAddress !== null && this.credential !== null;
  }
  
  async logout() {
    this.accountAddress = null;
    this.credential = null;
    clearUser();
  }
}
```

## Error Handling

### Global Error Handler

```typescript
class AttestaErrorHandler {
  static handle(error: any): string {
    if (error instanceof DOMException) {
      switch (error.name) {
        case 'NotAllowedError':
          return 'Authentication cancelled. Please try again.';
        case 'NotSupportedError':
          return 'WebAuthn is not supported in this browser.';
        case 'SecurityError':
          return 'WebAuthn requires HTTPS. Please use a secure connection.';
        default:
          return `Authentication error: ${error.name}`;
      }
    }
    
    if (error.message.includes('insufficient funds')) {
      return 'Insufficient SOL balance. Please add funds.';
    }
    
    if (error.message.includes('nonce')) {
      return 'Transaction conflict. Please try again.';
    }
    
    if (error.message.includes('policy')) {
      return 'Transaction blocked by account policy.';
    }
    
    return error.message || 'An unexpected error occurred.';
  }
}
```

### Error Boundaries (React)

```tsx
class AttestaErrorBoundary extends React.Component {
  state = { hasError: false, error: null };
  
  static getDerivedStateFromError(error: any) {
    return { hasError: true, error };
  }
  
  componentDidCatch(error: any, errorInfo: any) {
    console.error('Attesta error:', error, errorInfo);
  }
  
  render() {
    if (this.state.hasError) {
      return (
        <div>
          <h2>Something went wrong</h2>
          <p>{AttestaErrorHandler.handle(this.state.error)}</p>
          <button onClick={() => this.setState({ hasError: false, error: null })}>
            Try Again
          </button>
        </div>
      );
    }
    
    return this.props.children;
  }
}
```

## UI/UX Best Practices

### Loading States

```tsx
function PaymentButton({ onClick, isLoading }: { onClick: () => void; isLoading: boolean }) {
  return (
    <button onClick={onClick} disabled={isLoading}>
      {isLoading ? (
        <>
          <Spinner />
          Processing...
        </>
      ) : (
        'Make Payment'
      )}
    </button>
  );
}
```

### Transaction Confirmation

```tsx
function TransactionConfirmation({
  recipient,
  amount,
  onConfirm,
  onCancel,
}: {
  recipient: string;
  amount: number;
  onConfirm: () => void;
  onCancel: () => void;
}) {
  return (
    <div className="modal">
      <h3>Confirm Transaction</h3>
      <div>
        <p>Recipient: {recipient}</p>
        <p>Amount: {amount / 1e9} SOL</p>
      </div>
      <div>
        <button onClick={onConfirm}>Confirm</button>
        <button onClick={onCancel}>Cancel</button>
      </div>
    </div>
  );
}
```

### Passkey Prompt Guidance

```tsx
function PasskeyPrompt({ onAuthenticate }: { onAuthenticate: () => void }) {
  const [showHint, setShowHint] = useState(false);
  
  return (
    <div>
      <p>Authenticate with your passkey</p>
      {showHint && (
        <p className="hint">
          Use TouchID, FaceID, or your security key to authenticate
        </p>
      )}
      <button onClick={() => {
        setShowHint(true);
        onAuthenticate();
      }}>
        Authenticate
      </button>
    </div>
  );
}
```

## Complete Example

See the [Quickstart Guide](./quickstart.md) for a complete working example.

## Production Checklist

Before deploying to production:

- [ ] HTTPS enabled (required for WebAuthn)
- [ ] Error handling implemented
- [ ] Loading states for all async operations
- [ ] Transaction confirmation UI
- [ ] User-friendly error messages
- [ ] Proper state management
- [ ] Credential storage secured
- [ ] Policies configured
- [ ] Nonce management implemented
- [ ] Transaction status tracking
- [ ] Rate limiting (if needed)
- [ ] Analytics/monitoring
- [ ] Testing on devnet
- [ ] Security audit
- [ ] User documentation

## Next Steps

- [Quickstart Guide](./quickstart.md) - Get started quickly
- [SDK Documentation](../sdk-and-integration/javascript-typescript-sdk.md) - Complete API reference
- [Policy Configuration](../sdk-and-integration/policy-configuration.md) - Configure security policies
- [Error Handling](../sdk-and-integration/error-handling-and-security.md) - Security best practices

---

**Ready to build?** Start with the [Quickstart Guide](./quickstart.md) and then integrate using the patterns above!
