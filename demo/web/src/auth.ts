/**
 * Authentication utilities for Attesta demo
 * Handles WebAuthn/passkey authentication
 */

import { Connection, PublicKey } from '@solana/web3.js';
import { registerAttestaAccount, createWebAuthnCredential } from '../../../sdk/ts/src/register';

/**
 * Registers a new user with Attesta using passkey
 */
export async function registerUser(
  connection: Connection,
  userPublicKey: PublicKey,
  userName: string
): Promise<{
  accountAddress: PublicKey;
  credentialId: string;
  transaction: any;
}> {
  // Generate a random challenge
  const challenge = crypto.getRandomValues(new Uint8Array(32));
  
  // Create WebAuthn credential
  const credential = await createWebAuthnCredential(
    challenge,
    userPublicKey.toBase58(),
    userName
  );

  // Register Attesta account
  const { accountAddress, transaction } = await registerAttestaAccount(
    connection,
    userPublicKey,
    credential
  );

  return {
    accountAddress,
    credentialId: credential.id,
    transaction,
  };
}

/**
 * Authenticates a user with their passkey
 */
export async function authenticateUser(
  challenge: Uint8Array,
  credentialId: Uint8Array
): Promise<{
  authenticatorData: Uint8Array;
  clientDataJSON: Uint8Array;
  signature: Uint8Array;
}> {
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

  const assertion = await navigator.credentials.get({
    publicKey: publicKeyCredentialRequestOptions,
  }) as PublicKeyCredential;

  if (!assertion || !(assertion.response instanceof AuthenticatorAssertionResponse)) {
    throw new Error('Failed to authenticate with passkey');
  }

  const response = assertion.response;

  return {
    authenticatorData: new Uint8Array(response.authenticatorData),
    clientDataJSON: new Uint8Array(response.clientDataJSON),
    signature: new Uint8Array(response.signature),
  };
}
