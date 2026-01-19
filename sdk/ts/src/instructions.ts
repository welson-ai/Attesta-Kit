/**
 * Instruction builders for Attesta program
 * 
 * These functions create proper Anchor instructions for interacting with
 * the Attesta program on-chain.
 */

import { 
  PublicKey, 
  TransactionInstruction, 
  SystemProgram 
} from '@solana/web3.js';
import { serializeWebAuthnSignature } from './webauthn-utils';
import { AuthorizationProof } from './index';

/**
 * Creates an instruction to execute a transaction with passkey authorization
 */
export function createExecuteInstruction(
  accountAddress: PublicKey,
  authorizationProof: AuthorizationProof,
  transactionData: Uint8Array,
  programId: PublicKey
): TransactionInstruction {
  // Serialize WebAuthn signature
  const webauthnSig = serializeWebAuthnSignature(authorizationProof.webauthnSignature);
  
  // Instruction discriminator for 'execute'
  // Placeholder - replace with actual discriminator from IDL
  const discriminator = Buffer.from([0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1]);
  
  // Serialize instruction data
  // Format: [discriminator] [webauthn_sig_len] [webauthn_sig] [nonce (8 bytes)] [message_hash (32 bytes)] [tx_data_len] [tx_data]
  const webauthnSigLen = webauthnSig.length;
  const nonce = authorizationProof.nonce;
  const messageHash = authorizationProof.messageHash;
  const txDataLen = transactionData.length;
  
  const data = Buffer.allocUnsafe(8 + 4 + webauthnSigLen + 8 + 32 + 4 + txDataLen);
  let offset = 0;
  
  // Write discriminator
  data.set(discriminator, offset);
  offset += 8;
  
  // Write WebAuthn signature length and data
  data.writeUInt32LE(webauthnSigLen, offset);
  offset += 4;
  data.set(webauthnSig, offset);
  offset += webauthnSigLen;
  
  // Write nonce (u64, little-endian)
  data.writeBigUInt64LE(BigInt(nonce), offset);
  offset += 8;
  
  // Write message hash (32 bytes)
  data.set(messageHash, offset);
  offset += 32;
  
  // Write transaction data length and data
  data.writeUInt32LE(txDataLen, offset);
  offset += 4;
  data.set(transactionData, offset);
  
  return new TransactionInstruction({
    keys: [
      { pubkey: accountAddress, isSigner: false, isWritable: true },
      // Add other required accounts based on your program
    ],
    programId: programId,
    data,
  });
}

/**
 * Creates an instruction to update account policy
 */
export function createUpdatePolicyInstruction(
  accountAddress: PublicKey,
  owner: PublicKey,
  newPolicy: Uint8Array,
  programId: PublicKey
): TransactionInstruction {
  // Instruction discriminator for 'update_policy'
  // Placeholder - replace with actual discriminator from IDL
  const discriminator = Buffer.from([0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1]);
  
  // Serialize instruction data
  // Format: [discriminator] [policy_len] [policy]
  const policyLen = newPolicy.length;
  const data = Buffer.allocUnsafe(8 + 4 + policyLen);
  
  // Write discriminator
  data.set(discriminator, 0);
  
  // Write policy length and data
  data.writeUInt32LE(policyLen, 8);
  data.set(newPolicy, 12);
  
  return new TransactionInstruction({
    keys: [
      { pubkey: accountAddress, isSigner: false, isWritable: true },
      { pubkey: owner, isSigner: true, isWritable: false },
    ],
    programId: programId,
    data,
  });
}

/**
 * Helper to get instruction discriminator from instruction name
 * 
 * In production, use the actual discriminators from your IDL.
 * You can get them by running `anchor build` and checking the IDL file.
 */
export function getInstructionDiscriminator(instructionName: string): Buffer {
  // These are placeholders - replace with actual discriminators
  const discriminators: Record<string, Buffer> = {
    initialize: Buffer.from([0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91]),
    execute: Buffer.from([0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1]),
    updatePolicy: Buffer.from([0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1]),
  };
  
  const discriminator = discriminators[instructionName];
  if (!discriminator) {
    throw new Error(`Unknown instruction: ${instructionName}`);
  }
  
  return discriminator;
}
