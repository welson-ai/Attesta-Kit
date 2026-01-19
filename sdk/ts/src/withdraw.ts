import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import { AuthorizationProof, createAuthorizationProof } from './pay';

/**
 * Creates a withdrawal transaction authorized by a passkey
 * Similar to payment, but may have different policy requirements
 */
export async function createPasskeyWithdrawal(
  connection: Connection,
  fromAccount: PublicKey,
  toAccount: PublicKey,
  amount: number, // lamports
  credentialId: Uint8Array
): Promise<{
  transaction: Transaction;
  authorizationProof: AuthorizationProof;
}> {
  // Create withdrawal transaction
  // In production, this might call a specific withdraw instruction
  // For now, similar to payment
  const transaction = new Transaction();

  // In production, add custom withdraw instruction here
  // For now, use system transfer as placeholder
  transaction.add({
    keys: [
      { pubkey: fromAccount, isSigner: false, isWritable: true },
      { pubkey: toAccount, isSigner: false, isWritable: true },
      { pubkey: getAttestaProgramId(), isSigner: false, isWritable: false },
    ],
    programId: getAttestaProgramId(),
    data: Buffer.from([]), // Withdrawal instruction data
  });

  // Get recent blockhash
  const { blockhash } = await connection.getLatestBlockhash();
  transaction.recentBlockhash = blockhash;
  transaction.feePayer = fromAccount;

  // Hash transaction for challenge
  const transactionHash = await hashTransaction(transaction);

  // Get next nonce
  const nonce = await getNextNonce(connection, fromAccount);

  // Create authorization proof
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
 * Gets the Attesta program ID
 */
function getAttestaProgramId(): PublicKey {
  // Placeholder - replace with actual program ID
  return new PublicKey('11111111111111111111111111111111');
}

/**
 * Hashes a transaction
 */
async function hashTransaction(transaction: Transaction): Promise<Uint8Array> {
  const serialized = transaction.serialize({
    requireAllSignatures: false,
    verifySignatures: false,
  });
  const hashBuffer = await crypto.subtle.digest('SHA-256', serialized);
  return new Uint8Array(hashBuffer);
}

/**
 * Gets the next nonce for an account
 */
async function getNextNonce(
  connection: Connection,
  account: PublicKey
): Promise<number> {
  // Placeholder - in production, fetch from Attesta account
  return Date.now();
}
