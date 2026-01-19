# Deployment Guide

This guide explains how to deploy Attesta to Solana.

## Prerequisites

1. **Solana CLI** - Install from [solana.com](https://docs.solana.com/cli/install-solana-cli-tools)
2. **Anchor Framework** - Install from [anchor-lang.com](https://www.anchor-lang.com/docs/installation)
3. **Rust** - Install from [rustup.rs](https://rustup.rs/)

## Quick Start

### 1. Install Dependencies

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

### 2. Set Up Wallet

```bash
# Generate a new keypair (if you don't have one)
solana-keygen new

# Or use an existing one
solana config set --keypair ~/.config/solana/id.json
```

### 3. Configure Network

```bash
# For devnet (recommended for testing)
solana config set --url devnet

# For mainnet (production)
solana config set --url mainnet
```

### 4. Get Test SOL (Devnet Only)

```bash
# Airdrop test SOL on devnet
solana airdrop 2
```

### 5. Build the Program

```bash
# Make scripts executable
chmod +x scripts/*.sh

# Build the program
./scripts/build.sh

# Or manually:
anchor build
```

### 6. Deploy

```bash
# Deploy to devnet
./scripts/deploy.sh devnet

# Or manually:
anchor deploy --provider.cluster devnet
```

## Deployment Steps Explained

### Step 1: Update Program ID

Before deploying, you need to set your program ID:

1. Generate a new keypair:
   ```bash
   solana-keygen new -o target/deploy/attesta-keypair.json
   ```

2. Get the public key:
   ```bash
   solana address -k target/deploy/attesta-keypair.json
   ```

3. Update `programs/attesta/src/lib.rs`:
   ```rust
   declare_id!("YOUR_PROGRAM_ID_HERE");
   ```

4. Update `Anchor.toml`:
   ```toml
   [programs.devnet]
   attesta = "YOUR_PROGRAM_ID_HERE"
   ```

### Step 2: Build

```bash
anchor build
```

This will:
- Compile the Rust code
- Generate the IDL (Interface Definition Language)
- Create the `.so` binary file

### Step 3: Deploy

```bash
anchor deploy --provider.cluster devnet
```

This will:
- Upload the program to Solana
- Deploy it to the specified cluster
- Return the program ID

### Step 4: Verify

```bash
# Check program info
solana program show YOUR_PROGRAM_ID

# Check your account
solana account YOUR_PROGRAM_ID
```

## Network Options

### Devnet (Recommended for Testing)

- Free test SOL available
- Fast deployment
- No real money at risk
- Good for development and testing

```bash
solana config set --url devnet
anchor deploy --provider.cluster devnet
```

### Mainnet (Production)

- Real SOL required
- Permanent deployment
- Real money at risk
- Requires careful testing first

```bash
solana config set --url mainnet
anchor deploy --provider.cluster mainnet
```

## Cost Estimation

### Devnet
- Free (test SOL)

### Mainnet
- Program deployment: ~2-3 SOL
- Account creation: ~0.001 SOL per account
- Transaction fees: ~0.000005 SOL per transaction

## Troubleshooting

### Build Errors

```bash
# Clean and rebuild
anchor clean
anchor build
```

### Deployment Errors

```bash
# Check your balance
solana balance

# Get more SOL (devnet only)
solana airdrop 2

# Check program size
ls -lh target/deploy/attesta.so
```

### Program ID Mismatch

If you get a program ID mismatch error:

1. Make sure `declare_id!()` matches your keypair
2. Make sure `Anchor.toml` has the correct program ID
3. Run `anchor keys list` to check

## Post-Deployment

After deploying, you'll need to:

1. **Update SDK**: Update the program ID in your TypeScript SDK
2. **Update Clients**: Update any client applications with the new program ID
3. **Test**: Run integration tests against the deployed program
4. **Monitor**: Set up monitoring for your program

## Security Checklist

Before deploying to mainnet:

- [ ] Code reviewed and audited
- [ ] All tests passing
- [ ] Tested on devnet thoroughly
- [ ] Program ID is correct
- [ ] Upgrade authority is secure
- [ ] Documentation is complete
- [ ] Monitoring is set up

## Next Steps

1. **Initialize Accounts**: Use the `initialize` instruction to create accounts
2. **Test Transactions**: Test the `execute` instruction with real passkeys
3. **Monitor**: Set up monitoring and alerts
4. **Iterate**: Make improvements based on usage

## Support

For issues or questions:
- Check the [README.md](README.md)
- Review [ARCHITECTURE.md](ARCHITECTURE.md)
- Open an issue on GitHub
