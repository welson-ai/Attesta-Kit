/**
 * Payment utilities for Attesta demo
 * Handles passkey-authorized payments
 */

import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import { createPasskeyPayment } from '../../../sdk/ts/src/pay';
import { authenticateUser } from './auth';

/**
 * Makes a payment using passkey authorization
 */
export async function makePayment(
  connection: Connection,
  fromAccount: PublicKey,
  toAccount: PublicKey,
  amount: number, // lamports
  credentialId: Uint8Array
): Promise<Transaction> {
  // Create payment with authorization proof
  const { transaction, authorizationProof } = await createPasskeyPayment(
    connection,
    fromAccount,
    toAccount,
    amount,
    credentialId
  );

  // In production, you'd submit the authorization proof to the program
  // along with the transaction. For now, return the transaction.
  return transaction;
}

/**
 * Shows payment status
 */
export function showPaymentStatus(status: 'pending' | 'success' | 'error', message?: string) {
  console.log(`Payment ${status}: ${message || ''}`);
  
  // In a real app, update UI
  const statusElement = document.getElementById('payment-status');
  if (statusElement) {
    statusElement.textContent = `Payment ${status}: ${message || ''}`;
    statusElement.className = `payment-status ${status}`;
  }
}
