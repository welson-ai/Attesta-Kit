/**
 * Configuration for Attesta SDK
 */

import { PublicKey } from '@solana/web3.js';

/**
 * Get the Attesta program ID based on network
 */
export function getAttestaProgramId(network?: 'devnet' | 'mainnet' | 'localnet'): PublicKey {
  // Check environment variable first
  const envProgramId = process.env.ATTESTA_PROGRAM_ID || process.env.NEXT_PUBLIC_ATTESTA_PROGRAM_ID;
  if (envProgramId) {
    try {
      return new PublicKey(envProgramId);
    } catch {
      // Invalid program ID in env, fall through to defaults
    }
  }

  // Default program IDs by network
  const networkId = network || (process.env.SOLANA_NETWORK as any) || 'devnet';
  
  switch (networkId) {
    case 'mainnet':
      // TODO: Replace with actual mainnet program ID after deployment
      return new PublicKey('Attesta11111111111111111111111111111111');
    case 'devnet':
      return new PublicKey('Attesta11111111111111111111111111111111');
    case 'localnet':
      return new PublicKey('Attesta11111111111111111111111111111111');
    default:
      return new PublicKey('Attesta11111111111111111111111111111111');
  }
}

/**
 * Network configuration
 */
export interface NetworkConfig {
  rpcUrl: string;
  programId: PublicKey;
}

/**
 * Get network configuration
 */
export function getNetworkConfig(network?: 'devnet' | 'mainnet' | 'localnet'): NetworkConfig {
  const networkId = network || (process.env.SOLANA_NETWORK as any) || 'devnet';
  
  let rpcUrl: string;
  switch (networkId) {
    case 'mainnet':
      rpcUrl = process.env.SOLANA_RPC_URL || 'https://api.mainnet-beta.solana.com';
      break;
    case 'devnet':
      rpcUrl = process.env.SOLANA_RPC_URL || 'https://api.devnet.solana.com';
      break;
    case 'localnet':
      rpcUrl = process.env.SOLANA_RPC_URL || 'http://127.0.0.1:8899';
      break;
    default:
      rpcUrl = process.env.SOLANA_RPC_URL || 'https://api.devnet.solana.com';
  }

  return {
    rpcUrl,
    programId: getAttestaProgramId(networkId),
  };
}
