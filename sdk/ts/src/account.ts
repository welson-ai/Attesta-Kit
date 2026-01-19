/**
 * Account state management utilities
 */

import { Connection, PublicKey } from '@solana/web3.js';
import { AttestaAccount } from './index';

/**
 * Fetches an Attesta account from the blockchain
 */
export async function getAttestaAccount(
  connection: Connection,
  accountAddress: PublicKey
): Promise<AttestaAccount | null> {
  try {
    const accountInfo = await connection.getAccountInfo(accountAddress);
    
    if (!accountInfo || !accountInfo.data) {
      return null;
    }
    
    // Deserialize account data
    // The account data format depends on your Anchor program structure
    // For now, this is a placeholder - you'll need to implement proper deserialization
    // based on your AttestaAccount structure
    return deserializeAttestaAccount(accountInfo.data);
  } catch (error) {
    console.error('Error fetching Attesta account:', error);
    return null;
  }
}

/**
 * Deserializes account data into an AttestaAccount
 * 
 * This needs to match the serialization format used in your Rust program.
 * The format should match AttestaAccountData structure.
 */
function deserializeAttestaAccount(data: Buffer): AttestaAccount {
  // Skip discriminator (8 bytes for Anchor accounts)
  let offset = 8;
  
  // Read owner (32 bytes - PublicKey)
  const ownerBytes = data.slice(offset, offset + 32);
  const owner = new PublicKey(ownerBytes).toBase58();
  offset += 32;
  
  // Read passkey public key (64 bytes)
  const passkeyPublicKey = new Uint8Array(data.slice(offset, offset + 64));
  offset += 64;
  
  // Read credential ID (length-prefixed)
  const credentialIdLen = data.readUInt32LE(offset);
  offset += 4;
  const credentialId = new Uint8Array(data.slice(offset, offset + credentialIdLen));
  offset += credentialIdLen;
  
  // Read nonce (8 bytes - u64)
  const nonce = data.readUInt32LE(offset); // Reading as u32 for now, adjust if needed
  offset += 8;
  
  // Read policy (length-prefixed)
  const policyLen = data.readUInt32LE(offset);
  offset += 4;
  const policy = new Uint8Array(data.slice(offset, offset + policyLen));
  offset += policyLen;
  
  // Read timestamps (8 bytes each - i64)
  const createdAt = Number(data.readBigInt64LE(offset));
  offset += 8;
  const updatedAt = Number(data.readBigInt64LE(offset));
  
  return {
    owner,
    passkeyPublicKey,
    credentialId,
    nonce,
    policy,
    createdAt,
    updatedAt,
  };
}

/**
 * Gets the next nonce for an account
 */
export async function getNextNonce(
  connection: Connection,
  accountAddress: PublicKey
): Promise<number> {
  const account = await getAttestaAccount(connection, accountAddress);
  
  if (!account) {
    throw new Error('Account not found');
  }
  
  // Return current nonce + 1
  return account.nonce + 1;
}

/**
 * Checks if an Attesta account exists
 */
export async function accountExists(
  connection: Connection,
  accountAddress: PublicKey
): Promise<boolean> {
  const accountInfo = await connection.getAccountInfo(accountAddress);
  return accountInfo !== null;
}
