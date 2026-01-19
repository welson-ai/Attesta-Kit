import { Connection, PublicKey, Transaction, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import { Program, AnchorProvider, Wallet, BN } from '@coral-xyz/anchor';
import { WebAuthnCredential } from './index';
import { extractPublicKeyFromCredential } from './webauthn-utils';
import { getAttestaProgramId, getNetworkConfig } from './config';

// IDL type - will be generated from Anchor program
// For now, using a minimal interface
interface AttestaIDL {
  version: string;
  name: string;
  instructions: Array<{
    name: string;
    accounts: Array<any>;
    args: Array<any>;
  }>;
}

/**
 * Registers a new Attesta account using WebAuthn/passkey
 */
export async function registerAttestaAccount(
  connection: Connection,
  ownerPublicKey: PublicKey,
  credential: WebAuthnCredential,
  policy?: Uint8Array,
  programId?: PublicKey
): Promise<{
  accountAddress: PublicKey;
  transaction: Transaction;
}> {
  const attestaProgramId = programId || getAttestaProgramId();
  
  // Derive the Attesta account PDA
  // Using owner public key and first 32 bytes of credential ID as seeds
  const credentialIdSeed = credential.credentialId.slice(0, 32);
  const [attestaAccountPDA, bump] = PublicKey.findProgramAddressSync(
    [
      Buffer.from('attesta'),
      ownerPublicKey.toBuffer(),
      credentialIdSeed,
    ],
    attestaProgramId
  );

  // Create initialization transaction
  const transaction = new Transaction();

  // Create instruction to initialize Attesta account
  const instruction = createInitializeAttestaAccountInstruction(
    attestaAccountPDA,
    ownerPublicKey,
    credential,
    policy || new Uint8Array(),
    attestaProgramId
  );

  transaction.add(instruction);

  return {
    accountAddress: attestaAccountPDA,
    transaction,
  };
}

/**
 * Creates a WebAuthn credential (passkey) for the user
 * This should be called from a secure context (HTTPS)
 */
export async function createWebAuthnCredential(
  challenge: Uint8Array,
  userId: string,
  userName: string
): Promise<WebAuthnCredential> {
  // Create credential creation options
  const publicKeyCredentialCreationOptions: PublicKeyCredentialCreationOptions = {
    challenge: challenge,
    rp: {
      name: 'Attesta',
      id: typeof window !== 'undefined' ? window.location.hostname : 'localhost',
    },
    user: {
      id: new TextEncoder().encode(userId),
      name: userName,
      displayName: userName,
    },
    pubKeyCredParams: [
      {
        type: 'public-key',
        alg: -7, // ES256 (P-256)
      },
    ],
    authenticatorSelection: {
      authenticatorAttachment: 'platform', // Use platform authenticator (TouchID, FaceID, etc.)
      userVerification: 'required',
      requireResidentKey: true,
    },
    timeout: 60000,
    attestation: 'direct',
  };

  // Request credential creation from the browser
  if (typeof navigator === 'undefined' || !navigator.credentials) {
    throw new Error('WebAuthn API not available. This must run in a browser with WebAuthn support.');
  }

  const credential = await navigator.credentials.create({
    publicKey: publicKeyCredentialCreationOptions,
  }) as PublicKeyCredential;

  if (!credential || !(credential.response instanceof AuthenticatorAttestationResponse)) {
    throw new Error('Failed to create WebAuthn credential');
  }

  const response = credential.response;
  
  // Extract public key using CBOR parsing
  const publicKey = extractPublicKeyFromCredential(response);

  return {
    id: credential.id,
    publicKey,
    credentialId: new Uint8Array(credential.rawId),
  };
}

/**
 * Creates an instruction to initialize an Attesta account
 * 
 * This creates a proper Anchor instruction. If you have the IDL, you can use
 * the Anchor client for better type safety.
 */
function createInitializeAttestaAccountInstruction(
  accountPDA: PublicKey,
  owner: PublicKey,
  credential: WebAuthnCredential,
  policy: Uint8Array,
  programId: PublicKey
): TransactionInstruction {
  // Ensure public key is exactly 64 bytes (uncompressed P-256)
  if (credential.publicKey.length !== 64) {
    throw new Error(`Invalid public key length: expected 64 bytes, got ${credential.publicKey.length}`);
  }

  // Convert public key to [u8; 64] format
  const passkeyPublicKey = Array.from(credential.publicKey) as any as [number, ...number[]];
  if (passkeyPublicKey.length !== 64) {
    throw new Error('Public key must be exactly 64 bytes');
  }

  // Instruction discriminator for 'initialize' (first 8 bytes of sha256("global:initialize"))
  // This is a placeholder - in production, use the actual discriminator from your IDL
  // You can get it by: anchor build, then check target/idl/attesta.json
  const discriminator = Buffer.from([0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91]); // Placeholder
  
  // Serialize instruction data
  // Format: [discriminator (8 bytes)] [passkey_public_key (64 bytes)] [credential_id_len (4 bytes)] [credential_id] [policy_len (4 bytes)] [policy]
  const credentialIdLen = credential.credentialId.length;
  const policyLen = policy.length;
  
  const data = Buffer.allocUnsafe(8 + 64 + 4 + credentialIdLen + 4 + policyLen);
  let offset = 0;
  
  // Write discriminator
  data.set(discriminator, offset);
  offset += 8;
  
  // Write public key (64 bytes)
  data.set(credential.publicKey, offset);
  offset += 64;
  
  // Write credential ID length and data
  data.writeUInt32LE(credentialIdLen, offset);
  offset += 4;
  data.set(credential.credentialId, offset);
  offset += credentialIdLen;
  
  // Write policy length and data
  data.writeUInt32LE(policyLen, offset);
  offset += 4;
  data.set(policy, offset);
  
  return new TransactionInstruction({
    keys: [
      { pubkey: accountPDA, isSigner: false, isWritable: true },
      { pubkey: owner, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: programId,
    data,
  });
}

/**
 * Base64url encoding helper
 */
function base64UrlEncode(buffer: Uint8Array): string {
  const base64 = btoa(String.fromCharCode(...buffer));
  return base64.replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');
}
