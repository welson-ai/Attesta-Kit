/**
 * WebAuthn utility functions for parsing CBOR and extracting public keys
 */

import * as cbor from 'cbor';

/**
 * Extracts the P-256 public key from a WebAuthn attestation object
 * 
 * The attestation object is CBOR-encoded and contains the credential public key
 * in COSE format. We need to parse it and extract the x and y coordinates.
 */
export function extractPublicKeyFromCredential(
  response: AuthenticatorAttestationResponse
): Uint8Array {
  const attestationObject = new Uint8Array(response.getAttestationObject());
  
  try {
    // Decode the CBOR-encoded attestation object
    const decoded = cbor.decodeFirstSync(attestationObject);
    
    // The structure is: { fmt: string, attStmt: object, authData: Buffer }
    // The public key is in authData, starting at byte 37 (after RP ID hash, flags, counter)
    const authData = decoded.get('authData');
    
    if (!authData || !Buffer.isBuffer(authData)) {
      throw new Error('Invalid attestation object: missing authData');
    }
    
    // Parse authData to find the credential public key
    // Format: [RP ID hash (32 bytes)] [flags (1 byte)] [sign count (4 bytes)] [attested credential data]
    const authDataBytes = new Uint8Array(authData);
    
    if (authDataBytes.length < 37) {
      throw new Error('Invalid authData: too short');
    }
    
    // Skip RP ID hash (32 bytes), flags (1 byte), sign count (4 bytes) = 37 bytes
    let offset = 37;
    
    // Read AAGUID (16 bytes)
    offset += 16;
    
    // Read credential ID length (2 bytes, big-endian)
    if (offset + 2 > authDataBytes.length) {
      throw new Error('Invalid authData: cannot read credential ID length');
    }
    const credentialIdLength = (authDataBytes[offset] << 8) | authDataBytes[offset + 1];
    offset += 2;
    
    // Skip credential ID
    offset += credentialIdLength;
    
    // Now we should be at the credential public key (COSE format)
    if (offset >= authDataBytes.length) {
      throw new Error('Invalid authData: no public key found');
    }
    
    // The public key is in CBOR format (COSE_Key)
    const publicKeyCbor = authDataBytes.slice(offset);
    const publicKeyDecoded = cbor.decodeFirstSync(Buffer.from(publicKeyCbor));
    
    // COSE_Key format: { 1: kty, 3: alg, -1: crv, -2: x, -3: y }
    // For P-256: kty = 2 (EC2), alg = -7 (ES256), crv = 1 (P-256)
    const kty = publicKeyDecoded.get(1);
    const alg = publicKeyDecoded.get(3);
    const crv = publicKeyDecoded.get(-1);
    const x = publicKeyDecoded.get(-2);
    const y = publicKeyDecoded.get(-3);
    
    if (kty !== 2 || alg !== -7 || crv !== 1) {
      throw new Error(`Unsupported key type: kty=${kty}, alg=${alg}, crv=${crv}. Expected P-256 (kty=2, alg=-7, crv=1)`);
    }
    
    if (!x || !y) {
      throw new Error('Invalid public key: missing x or y coordinates');
    }
    
    // Convert x and y to Uint8Array
    const xBytes = Buffer.isBuffer(x) ? new Uint8Array(x) : new Uint8Array(x);
    const yBytes = Buffer.isBuffer(y) ? new Uint8Array(y) : new Uint8Array(y);
    
    // P-256 coordinates are 32 bytes each
    // Ensure they're the right length (pad or truncate if needed)
    const xPadded = padOrTruncate(xBytes, 32);
    const yPadded = padOrTruncate(yBytes, 32);
    
    // Combine into 64-byte uncompressed public key [x (32 bytes) | y (32 bytes)]
    const publicKey = new Uint8Array(64);
    publicKey.set(xPadded, 0);
    publicKey.set(yPadded, 32);
    
    return publicKey;
  } catch (error) {
    throw new Error(`Failed to extract public key from WebAuthn credential: ${error.message}`);
  }
}

/**
 * Pads or truncates a byte array to the specified length
 */
function padOrTruncate(bytes: Uint8Array, length: number): Uint8Array {
  if (bytes.length === length) {
    return bytes;
  }
  
  const result = new Uint8Array(length);
  if (bytes.length > length) {
    // Truncate (take last N bytes)
    result.set(bytes.slice(bytes.length - length), 0);
  } else {
    // Pad with zeros at the beginning
    result.set(bytes, length - bytes.length);
  }
  
  return result;
}

/**
 * Serializes a WebAuthn signature for on-chain verification
 */
export function serializeWebAuthnSignature(signature: {
  authenticatorData: Uint8Array;
  clientDataJSON: Uint8Array;
  signature: Uint8Array;
  credentialId: Uint8Array;
}): Uint8Array {
  // Format: [auth_data_len (4 bytes)] [auth_data] [client_data_len (4 bytes)] [client_data] 
  //         [sig_len (4 bytes)] [signature] [cred_id_len (4 bytes)] [credential_id]
  
  const authDataLen = signature.authenticatorData.length;
  const clientDataLen = signature.clientDataJSON.length;
  const sigLen = signature.signature.length;
  const credIdLen = signature.credentialId.length;
  
  const totalLen = 4 + authDataLen + 4 + clientDataLen + 4 + sigLen + 4 + credIdLen;
  const result = new Uint8Array(totalLen);
  
  let offset = 0;
  
  // Write lengths and data
  const writeUint32 = (value: number) => {
    result[offset++] = value & 0xff;
    result[offset++] = (value >> 8) & 0xff;
    result[offset++] = (value >> 16) & 0xff;
    result[offset++] = (value >> 24) & 0xff;
  };
  
  const writeBytes = (bytes: Uint8Array) => {
    result.set(bytes, offset);
    offset += bytes.length;
  };
  
  writeUint32(authDataLen);
  writeBytes(signature.authenticatorData);
  writeUint32(clientDataLen);
  writeBytes(signature.clientDataJSON);
  writeUint32(sigLen);
  writeBytes(signature.signature);
  writeUint32(credIdLen);
  writeBytes(signature.credentialId);
  
  return result;
}
