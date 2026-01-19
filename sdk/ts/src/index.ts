/**
 * Attesta SDK - TypeScript client for Attesta account abstraction
 * 
 * This SDK provides client-side functionality for:
 * - Passkey/WebAuthn authentication
 * - Account registration and management
 * - Transaction authorization and execution
 */

export * from './register';
export * from './pay';
export * from './withdraw';
export * from './account';
export * from './instructions';
export * from './config';
export * from './webauthn-utils';

// Types
export interface AttestaAccount {
  owner: string; // Pubkey as base58 string
  passkeyPublicKey: Uint8Array;
  credentialId: Uint8Array;
  nonce: number;
  policy: Uint8Array;
  createdAt: number;
  updatedAt: number;
}

export interface WebAuthnCredential {
  id: string; // Base64url encoded credential ID
  publicKey: Uint8Array; // 64 bytes uncompressed P-256 public key
  credentialId: Uint8Array; // Raw credential ID
}

export interface AuthorizationProof {
  webauthnSignature: {
    authenticatorData: Uint8Array;
    clientDataJSON: Uint8Array;
    signature: Uint8Array;
    credentialId: Uint8Array;
  };
  nonce: number;
  messageHash: Uint8Array;
}
