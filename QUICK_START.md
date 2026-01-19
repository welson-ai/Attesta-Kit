# Quick Start - Deploy to Solana

## Yes, you need to deploy to Solana! 🚀

Attesta is a **Solana program** (smart contract), so it must be deployed to the Solana blockchain to work.

## What Was Created

✅ **All missing TOML files:**
- `Cargo.toml` - Workspace configuration
- `Anchor.toml` - Anchor framework configuration
- `programs/attesta/Cargo.toml` - Program dependencies

✅ **Anchor Program:**
- `programs/attesta/src/lib.rs` - Main program entry point

✅ **Deployment Scripts:**
- `scripts/build.sh` - Build the program
- `scripts/deploy.sh` - Deploy to Solana
- `scripts/test.sh` - Run tests

✅ **Documentation:**
- `DEPLOYMENT.md` - Complete deployment guide
- `.gitignore` - Git ignore rules

## Quick Deployment (3 Steps)

### Step 1: Install Prerequisites

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

### Step 2: Set Up Wallet

```bash
# Generate keypair (if needed)
solana-keygen new

# Set to devnet
solana config set --url devnet

# Get test SOL
solana airdrop 2
```

### Step 3: Deploy

```bash
# Make scripts executable (already done)
chmod +x scripts/*.sh

# Build
./scripts/build.sh

# Deploy to devnet
./scripts/deploy.sh devnet
```

## What Happens When You Deploy

1. **Build**: Compiles your Rust code into a `.so` binary
2. **Upload**: Uploads the binary to Solana
3. **Deploy**: Creates the program on-chain
4. **Get Program ID**: Returns your program's address

## After Deployment

You'll get a **Program ID** (like `Attesta11111111111111111111111111111111`).

Update this in:
- `programs/attesta/src/lib.rs` - `declare_id!()`
- `Anchor.toml` - `[programs.devnet]` section
- Your TypeScript SDK - Program ID constant

## Networks

- **Devnet** (Testing): Free, fast, safe for development
- **Mainnet** (Production): Real SOL, permanent, requires careful testing

## Cost

- **Devnet**: Free (test SOL)
- **Mainnet**: ~2-3 SOL for deployment

## Next Steps

1. Read `DEPLOYMENT.md` for detailed instructions
2. Test on devnet first
3. Update program IDs in your code
4. Deploy to mainnet when ready

## Need Help?

- Check `DEPLOYMENT.md` for troubleshooting
- Review `README.md` for architecture
- All scripts have error messages

---

**Ready to deploy?** Run `./scripts/deploy.sh devnet` 🚀
