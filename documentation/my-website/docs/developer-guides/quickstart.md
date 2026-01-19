# Quickstart

Get started with Attesta in 5 minutes. This guide will walk you through creating your first passkey-authenticated Solana account and making a payment.

## Prerequisites

- Node.js 16+ installed
- A modern browser with WebAuthn support (Chrome, Firefox, Safari, Edge)
- Basic knowledge of JavaScript/TypeScript
- A Solana devnet connection (we'll use a public RPC)

## Step 1: Install the SDK

Create a new project and install dependencies:

```bash
mkdir attesta-quickstart
cd attesta-quickstart
npm init -y
npm install @attesta/sdk @solana/web3.js
```

## Step 2: Set Up Your Project

Create an `index.html` file:

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Attesta Quickstart</title>
  <style>
    body {
      font-family: Arial, sans-serif;
      max-width: 800px;
      margin: 50px auto;
      padding: 20px;
    }
    button {
      background: #007bff;
      color: white;
      border: none;
      padding: 10px 20px;
      border-radius: 5px;
      cursor: pointer;
      margin: 10px 5px;
    }
    button:hover {
      background: #0056b3;
    }
    #output {
      background: #f5f5f5;
      padding: 15px;
      border-radius: 5px;
      margin-top: 20px;
      white-space: pre-wrap;
      font-family: monospace;
    }
  </style>
</head>
<body>
  <h1>Attesta Quickstart</h1>
  
  <div>
    <button onclick="registerAccount()">1. Register Account</button>
    <button onclick="makePayment()">2. Make Payment</button>
  </div>
  
  <div id="output"></div>
  
  <script type="module" src="app.js"></script>
</body>
</html>
```

Create an `app.js` file:

```javascript
import { 
  registerAttestaAccount, 
  createWebAuthnCredential,
  createPasskeyPayment 
} from 'https://unpkg.com/@attesta/sdk@latest/dist/index.esm.js';
import { 
  Connection, 
  PublicKey, 
  Keypair,
  SystemProgram 
} from 'https://unpkg.com/@solana/web3.js@latest/lib/index.esm.js';

// Connect to Solana devnet
const connection = new Connection('https://api.devnet.solana.com');

// Store user data (in production, use secure storage)
let userData = {
  keypair: null,
  accountAddress: null,
  credential: null
};

// Load saved data
function loadUserData() {
  const saved = localStorage.getItem('attesta_user');
  if (saved) {
    userData = JSON.parse(saved);
    // Reconstruct objects
    if (userData.keypair) {
      userData.keypair = Keypair.fromSecretKey(
        new Uint8Array(Object.values(userData.keypair.secretKey))
      );
    }
    if (userData.accountAddress) {
      userData.accountAddress = new PublicKey(userData.accountAddress);
    }
  }
}

// Save user data
function saveUserData() {
  const toSave = {
    keypair: userData.keypair ? {
      secretKey: Array.from(userData.keypair.secretKey)
    } : null,
    accountAddress: userData.accountAddress?.toBase58(),
    credential: userData.credential
  };
  localStorage.setItem('attesta_user', JSON.stringify(toSave));
}

// Output helper
function output(message) {
  const outputEl = document.getElementById('output');
  outputEl.textContent += message + '\n';
  console.log(message);
}

// Step 1: Register Account
window.registerAccount = async function() {
  try {
    output('🚀 Starting account registration...');
    
    // Generate user keypair
    output('📝 Generating keypair...');
    userData.keypair = Keypair.generate();
    output(`✅ Keypair generated: ${userData.keypair.publicKey.toBase58()}`);
    
    // Create WebAuthn credential (passkey)
    output('🔐 Creating passkey...');
    output('   (You will be prompted for biometric authentication)');
    
    const challenge = crypto.getRandomValues(new Uint8Array(32));
    userData.credential = await createWebAuthnCredential(
      challenge,
      userData.keypair.publicKey.toBase58(),
      'Quickstart User'
    );
    
    output(`✅ Passkey created: ${userData.credential.id.substring(0, 20)}...`);
    
    // Register Attesta account
    output('📝 Registering Attesta account...');
    const { accountAddress, transaction } = await registerAttestaAccount(
      connection,
      userData.keypair.publicKey,
      userData.credential
    );
    
    userData.accountAddress = accountAddress;
    output(`✅ Account address: ${accountAddress.toBase58()}`);
    
    // Sign and send transaction
    output('📤 Sending registration transaction...');
    transaction.sign(userData.keypair);
    
    // For devnet, we need SOL for fees
    output('💡 Requesting airdrop for transaction fees...');
    try {
      const airdropSignature = await connection.requestAirdrop(
        userData.keypair.publicKey,
        2e9 // 2 SOL
      );
      await connection.confirmTransaction(airdropSignature);
      output('✅ Airdrop received');
    } catch (e) {
      output('⚠️ Airdrop failed (may need to request manually)');
    }
    
    const signature = await connection.sendRawTransaction(
      transaction.serialize()
    );
    output(`📤 Transaction sent: ${signature}`);
    
    await connection.confirmTransaction(signature);
    output('✅ Account registered successfully!');
    
    saveUserData();
    output('\n🎉 Registration complete! You can now make payments.');
    
  } catch (error) {
    output(`❌ Error: ${error.message}`);
    console.error(error);
  }
};

// Step 2: Make Payment
window.makePayment = async function() {
  try {
    loadUserData();
    
    if (!userData.accountAddress || !userData.credential) {
      output('❌ Please register an account first!');
      return;
    }
    
    output('💸 Creating payment...');
    
    // Create a recipient (for demo, use a random address)
    // In production, get this from user input
    const recipient = Keypair.generate().publicKey;
    output(`📤 Recipient: ${recipient.toBase58()}`);
    
    const amount = 0.1e9; // 0.1 SOL
    output(`💰 Amount: ${amount / 1e9} SOL`);
    
    // Create payment transaction
    output('🔐 Authenticating with passkey...');
    output('   (You will be prompted for biometric authentication)');
    
    const { transaction, authorizationProof } = await createPasskeyPayment(
      connection,
      userData.accountAddress,
      recipient,
      amount,
      userData.credential.credentialId
    );
    
    output('✅ Payment transaction created');
    output(`📝 Authorization proof generated`);
    output(`   Nonce: ${authorizationProof.nonce}`);
    
    // In a real app, you would submit this to the Attesta program
    // For this quickstart, we'll just show what was created
    output('\n📋 Transaction Details:');
    output(`   From: ${userData.accountAddress.toBase58()}`);
    output(`   To: ${recipient.toBase58()}`);
    output(`   Amount: ${amount / 1e9} SOL`);
    output(`   Instructions: ${transaction.instructions.length}`);
    
    output('\n💡 Note: In production, submit this transaction and');
    output('   authorization proof to the Attesta program.');
    
    output('\n✅ Payment transaction ready!');
    
  } catch (error) {
    output(`❌ Error: ${error.message}`);
    console.error(error);
  }
};

// Initialize
loadUserData();
if (userData.accountAddress) {
  output('✅ Found existing account');
  output(`   Address: ${userData.accountAddress.toBase58()}`);
}
```

## Step 3: Run Your App

For a simple setup, use a local HTTP server:

```bash
# Using Python
python3 -m http.server 8000

# Using Node.js (install http-server)
npx http-server -p 8000

# Using PHP
php -S localhost:8000
```

Then open `http://localhost:8000` in your browser.

**Note**: WebAuthn requires HTTPS (or localhost). The above will work on localhost.

## Step 4: Test It Out

1. **Click "1. Register Account"**
   - You'll be prompted to create a passkey (TouchID, FaceID, etc.)
   - The account will be registered on Solana devnet
   - Your account address will be displayed

2. **Click "2. Make Payment"**
   - You'll be prompted to authenticate with your passkey
   - A payment transaction will be created
   - Transaction details will be displayed

## What Just Happened?

### Registration Flow

1. **Keypair Generation**: Created a Solana keypair for the user
2. **Passkey Creation**: Created a WebAuthn credential (passkey) using biometrics
3. **Account Registration**: Registered the passkey on-chain in an Attesta account
4. **Transaction Submission**: Sent the registration transaction to Solana

### Payment Flow

1. **Transaction Creation**: Created a payment transaction
2. **Challenge Generation**: Hashed the transaction to create a WebAuthn challenge
3. **Passkey Authentication**: User authenticated with biometrics
4. **Authorization Proof**: Generated WebAuthn signature (authorization proof)
5. **Transaction Ready**: Transaction and proof are ready to submit

## Next Steps

Now that you have a working quickstart:

1. **Explore the SDK**: Check out the [JavaScript/TypeScript SDK Guide](../sdk-and-integration/javascript-typescript-sdk.md)
2. **Configure Policies**: Learn about [Policy Configuration](../sdk-and-integration/policy-configuration.md)
3. **Build a DApp**: Follow the [DApp Integration Guide](./dapp-integration.md)
4. **Understand Security**: Read [Error Handling and Security](../sdk-and-integration/error-handling-and-security.md)

## Troubleshooting

### "WebAuthn not supported"

- Ensure you're using HTTPS or localhost
- Use a modern browser (Chrome, Firefox, Safari, Edge)
- Check browser settings for WebAuthn support

### "User cancelled"

- This is normal - users can cancel biometric prompts
- Handle gracefully in your UI

### "Insufficient funds"

- Request a devnet airdrop: `solana airdrop 2 <your-address>`
- Or use the airdrop in the code (may be rate-limited)

### "Transaction failed"

- Check Solana network status
- Verify you're connected to devnet
- Ensure account has sufficient balance

## Production Considerations

Before deploying to production:

- [ ] Use HTTPS (required for WebAuthn)
- [ ] Implement proper error handling
- [ ] Add transaction confirmation UI
- [ ] Store credentials securely
- [ ] Configure policies
- [ ] Test on mainnet with small amounts first
- [ ] Implement proper nonce management
- [ ] Add rate limiting if needed

## Additional Resources

- [SDK Overview](../sdk-and-integration/sdk-overview.md) - Understand the SDK architecture
- [Installation Guide](../sdk-and-integration/installation.md) - Detailed installation instructions
- [API Reference](../sdk-and-integration/javascript-typescript-sdk.md) - Complete API documentation

---

**Congratulations!** You've created your first Attesta account and payment. Ready to build more? Check out the [DApp Integration Guide](./dapp-integration.md)!
