import { Connection, PublicKey, Transaction, SystemProgram } from '@solana/web3.js';
import { AuthorizationProof } from './index';

/**
 * Creates an authorization proof using WebAuthn/passkey
 * This signs a challenge (transaction hash) with the user's passkey
 */
export async function createAuthorizationProof(
  challenge: Uint8Array,
  credentialId: Uint8Array,
  messageHash: Uint8Array
): Promise<AuthorizationProof> {
  // Convert challenge to base64url
  const challengeBase64 = base64UrlEncode(challenge);

  // Convert credential ID to base64url
  const credentialIdBase64 = base64UrlEncode(credentialId);

  // Create credential request options
  const publicKeyCredentialRequestOptions: PublicKeyCredentialRequestOptions = {
    challenge: challenge,
    allowCredentials: [
      {
        id: credentialId,
        type: 'public-key',
      },
    ],
    timeout: 60000,
    userVerification: 'required',
  };

  // Request credential assertion from the browser
  const assertion = await navigator.credentials.get({
    publicKey: publicKeyCredentialRequestOptions,
  }) as PublicKeyCredential;

  if (!assertion || !(assertion.response instanceof AuthenticatorAssertionResponse)) {
    throw new Error('Failed to get WebAuthn assertion');
  }

  const response = assertion.response;

  // Extract WebAuthn signature components
  const authenticatorData = new Uint8Array(response.authenticatorData);
  const clientDataJSON = new Uint8Array(response.clientDataJSON);
  const signature = new Uint8Array(response.signature);

  // Get the current nonce (in production, fetch from on-chain account)
  // For now, use a placeholder
  const nonce = Date.now(); // In production, use account's current nonce + 1

  return {
    webauthnSignature: {
      authenticatorData,
      clientDataJSON,
      signature,
      credentialId,
    },
    nonce,
    messageHash,
  };
}

/**
 * Creates a payment transaction authorized by a passkey
 */
export async function createPasskeyPayment(
  connection: Connection,
  fromAccount: PublicKey,
  toAccount: PublicKey,
  amount: number, // lamports
  credentialId: Uint8Array
): Promise<{
  transaction: Transaction;
  authorizationProof: AuthorizationProof;
}> {
  // Create the payment transaction
  const transaction = new Transaction();

  // Add transfer instruction
  transaction.add(
    SystemProgram.transfer({
      fromPubkey: fromAccount,
      toPubkey: toAccount,
      lamports: amount,
    })
  );

  // Get recent blockhash
  const { blockhash } = await connection.getLatestBlockhash();
  transaction.recentBlockhash = blockhash;
  transaction.feePayer = fromAccount;

  // Create a challenge from the transaction hash
  // In production, you'd serialize the transaction and hash it
  const transactionHash = await hashTransaction(transaction);
  
  // Generate a nonce for this transaction
  const nonce = await getNextNonce(connection, fromAccount);

  // Create authorization proof with WebAuthn
  const authorizationProof = await createAuthorizationProof(
    transactionHash,
    credentialId,
    transactionHash
  );

  return {
    transaction,
    authorizationProof,
  };
}

/**
 * Hashes a transaction to create a challenge for WebAuthn
 */
async function hashTransaction(transaction: Transaction): Promise<Uint8Array> {
  // Serialize transaction
  const serialized = transaction.serialize({
    requireAllSignatures: false,
    verifySignatures: false,
  });

  // Hash using SHA-256
  const hashBuffer = await crypto.subtle.digest('SHA-256', serialized);
  return new Uint8Array(hashBuffer);
}

/**
 * Gets the next nonce for an account
 * In production, fetch from on-chain account state
 */
async function getNextNonce(
  connection: Connection,
  account: PublicKey
): Promise<number> {
  // Import getNextNonce from account module
  const { getNextNonce: fetchNextNonce } = await import('./account');
  return fetchNextNonce(connection, account);
}

/**
 * Base64url encoding helper
 */
function base64UrlEncode(buffer: Uint8Array): string {
  const base64 = btoa(String.fromCharCode(...buffer));
  return base64.replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');
}
