import React, { useState, useEffect } from 'react';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { registerUser } from './auth';
import { makePayment, showPaymentStatus } from './pay';

// Use your Solana RPC endpoint
const RPC_ENDPOINT = 'https://api.devnet.solana.com';
const connection = new Connection(RPC_ENDPOINT, 'confirmed');

function App() {
  const [isRegistered, setIsRegistered] = useState(false);
  const [accountAddress, setAccountAddress] = useState<string | null>(null);
  const [credentialId, setCredentialId] = useState<string | null>(null);
  const [userKeypair, setUserKeypair] = useState<Keypair | null>(null);
  const [recipientAddress, setRecipientAddress] = useState('');
  const [paymentAmount, setPaymentAmount] = useState('');

  useEffect(() => {
    // Check if user already has credentials stored
    const storedCredentialId = localStorage.getItem('attesta_credential_id');
    const storedAccountAddress = localStorage.getItem('attesta_account_address');
    const storedPublicKey = localStorage.getItem('attesta_public_key');
    const storedPrivateKey = localStorage.getItem('attesta_private_key');

    if (storedCredentialId && storedAccountAddress && storedPublicKey && storedPrivateKey) {
      setCredentialId(storedCredentialId);
      setAccountAddress(storedAccountAddress);
      const privateKeyArray = JSON.parse(storedPrivateKey);
      const keypair = Keypair.fromSecretKey(new Uint8Array(privateKeyArray));
      setUserKeypair(keypair);
      setIsRegistered(true);
    }
  }, []);

  const handleRegister = async () => {
    try {
      if (!userKeypair) {
        // Generate a new keypair for the user
        const newKeypair = Keypair.generate();
        setUserKeypair(newKeypair);
      }

      if (!userKeypair) {
        throw new Error('Failed to generate keypair');
      }

      // Register user with passkey
      const { accountAddress: accountAddr, credentialId: credId, transaction } = await registerUser(
        connection,
        userKeypair.publicKey,
        'Attesta User'
      );

      // Store credentials locally
      localStorage.setItem('attesta_credential_id', credId);
      localStorage.setItem('attesta_account_address', accountAddr.toBase58());
      localStorage.setItem('attesta_public_key', userKeypair.publicKey.toBase58());
      localStorage.setItem('attesta_private_key', JSON.stringify(Array.from(userKeypair.secretKey)));

      setAccountAddress(accountAddr.toBase58());
      setCredentialId(credId);
      setIsRegistered(true);

      // Sign and send transaction (in production, handle this properly)
      transaction.sign(userKeypair);
      const signature = await connection.sendTransaction(transaction, [userKeypair]);
      await connection.confirmTransaction(signature);

      alert('Registration successful! Account: ' + accountAddr.toBase58());
    } catch (error: any) {
      console.error('Registration error:', error);
      alert('Registration failed: ' + error.message);
    }
  };

  const handlePayment = async () => {
    try {
      if (!userKeypair || !accountAddress || !credentialId) {
        throw new Error('Please register first');
      }

      const recipientPubkey = new PublicKey(recipientAddress);
      const amount = parseFloat(paymentAmount);
      const lamports = amount * 1e9; // Convert SOL to lamports

      if (isNaN(amount) || amount <= 0) {
        throw new Error('Invalid amount');
      }

      showPaymentStatus('pending', 'Creating payment...');

      // Convert credential ID from base64 to Uint8Array
      const credIdBytes = Uint8Array.from(atob(credentialId), c => c.charCodeAt(0));

      const transaction = await makePayment(
        connection,
        userKeypair.publicKey,
        recipientPubkey,
        lamports,
        credIdBytes
      );

      // Sign and send transaction
      transaction.sign(userKeypair);
      const signature = await connection.sendTransaction(transaction, [userKeypair]);
      await connection.confirmTransaction(signature);

      showPaymentStatus('success', `Payment sent! Signature: ${signature}`);
      setRecipientAddress('');
      setPaymentAmount('');
    } catch (error: any) {
      console.error('Payment error:', error);
      showPaymentStatus('error', error.message);
    }
  };

  return (
    <div style={{ maxWidth: '800px', margin: '0 auto', padding: '20px' }}>
      <h1>Attesta - Account Abstraction with Passkeys</h1>
      
      {!isRegistered ? (
        <div>
          <h2>Register with Passkey</h2>
          <p>Click the button below to register a new account using your device's passkey (TouchID, FaceID, etc.)</p>
          <button 
            onClick={handleRegister}
            style={{
              padding: '12px 24px',
              fontSize: '16px',
              backgroundColor: '#007bff',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer'
            }}
          >
            Register with Passkey
          </button>
        </div>
      ) : (
        <div>
          <h2>Your Account</h2>
          <p><strong>Account Address:</strong> {accountAddress}</p>
          <p><strong>Public Key:</strong> {userKeypair?.publicKey.toBase58()}</p>

          <div style={{ marginTop: '40px' }}>
            <h2>Make a Payment</h2>
            <div style={{ marginBottom: '16px' }}>
              <label>
                Recipient Address:
                <input
                  type="text"
                  value={recipientAddress}
                  onChange={(e) => setRecipientAddress(e.target.value)}
                  placeholder="Enter Solana address"
                  style={{
                    width: '100%',
                    padding: '8px',
                    marginTop: '4px',
                    fontSize: '14px'
                  }}
                />
              </label>
            </div>
            <div style={{ marginBottom: '16px' }}>
              <label>
                Amount (SOL):
                <input
                  type="number"
                  value={paymentAmount}
                  onChange={(e) => setPaymentAmount(e.target.value)}
                  placeholder="0.1"
                  min="0"
                  step="0.001"
                  style={{
                    width: '100%',
                    padding: '8px',
                    marginTop: '4px',
                    fontSize: '14px'
                  }}
                />
              </label>
            </div>
            <button
              onClick={handlePayment}
              style={{
                padding: '12px 24px',
                fontSize: '16px',
                backgroundColor: '#28a745',
                color: 'white',
                border: 'none',
                borderRadius: '4px',
                cursor: 'pointer'
              }}
            >
              Pay with Passkey
            </button>
            <div id="payment-status" style={{ marginTop: '16px', padding: '12px', borderRadius: '4px' }}></div>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
