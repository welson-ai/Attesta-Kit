# Scripts

Utility scripts for building, deploying, and testing the Attesta project.

## Overview

This directory contains scripts for:
- **Building** the project (Rust crates, Anchor program, SDKs)
- **Deploying** to Solana networks (devnet, mainnet)
- **Testing** components
- **Publishing** SDKs
- **Development** utilities

## Scripts

### Build Scripts

#### `build.sh`

Builds all Rust components of the project.

```bash
./scripts/build.sh
```

**What it does:**
- Builds all Rust crates (`core-crypto`, `smart-account`, `recovery`)
- Builds the Anchor program (`programs/attesta`)
- Builds the Rust SDK
- Runs tests

#### `build-sdk.sh`

Builds the TypeScript SDK.

```bash
./scripts/build-sdk.sh
```

**What it does:**
- Installs npm dependencies
- Builds TypeScript SDK (CommonJS, ESM, UMD)
- Generates TypeScript definitions

### Deployment Scripts

#### `deploy.sh`

Deploys the Attesta program to Solana.

```bash
# Deploy to devnet
./scripts/deploy.sh devnet

# Deploy to mainnet
./scripts/deploy.sh mainnet
```

**What it does:**
- Builds the program
- Deploys to specified network
- Updates program ID if needed
- Verifies deployment

**Prerequisites:**
- Solana CLI installed
- Wallet configured
- Sufficient SOL for deployment

#### `deploy.rs`

Rust-based deployment script with more control.

```bash
cargo run --bin deploy -- --network devnet
```

### Testing Scripts

#### `test.sh`

Runs all tests for the project.

```bash
./scripts/test.sh
```

**What it does:**
- Runs Rust unit tests
- Runs Anchor integration tests
- Runs SDK tests

#### `test_keys.rs`

Tests key generation and management.

```bash
cargo run --bin test_keys
```

#### `simulate_payment.rs`

Simulates a payment transaction for testing.

```bash
cargo run --bin simulate_payment
```

### Publishing Scripts

#### `publish-sdk.sh`

Publishes the TypeScript SDK to npm.

```bash
./scripts/publish-sdk.sh
```

**What it does:**
- Builds the SDK
- Runs tests
- Publishes to npm (if version changed)
- Tags git release

**Prerequisites:**
- npm account configured
- Version bumped in `package.json`

## Usage Examples

### Complete Build and Deploy Workflow

```bash
# 1. Build everything
./scripts/build.sh

# 2. Run tests
./scripts/test.sh

# 3. Deploy to devnet
./scripts/deploy.sh devnet

# 4. Test on devnet
cargo run --bin simulate_payment
```

### Development Workflow

```bash
# Build and watch for changes
cargo watch -x 'run --bin test_keys'

# Deploy to local validator
solana-test-validator &
./scripts/deploy.sh localhost
```

### SDK Development

```bash
# Build SDK
./scripts/build-sdk.sh

# Test SDK locally
cd sdk/ts
npm link
cd ../../demo/web
npm link @attesta/sdk
```

## Script Details

### `build.sh`

```bash
#!/bin/bash
set -e

echo "Building Attesta project..."

# Build Rust crates
cargo build --release

# Build Anchor program
anchor build

# Build Rust SDK
cd sdk/rust
cargo build --release
cd ../..

echo "Build complete!"
```

### `deploy.sh`

```bash
#!/bin/bash
set -e

NETWORK=${1:-devnet}

echo "Deploying to $NETWORK..."

# Build first
./scripts/build.sh

# Deploy
anchor deploy --provider.cluster $NETWORK

echo "Deployment complete!"
```

## Environment Variables

Some scripts use environment variables:

- `SOLANA_NETWORK`: Network to use (devnet, mainnet, localhost)
- `SOLANA_KEYPAIR`: Path to keypair file
- `RPC_URL`: Custom RPC endpoint

## Troubleshooting

### Build Failures

```bash
# Clean and rebuild
cargo clean
./scripts/build.sh
```

### Deployment Failures

```bash
# Check Solana config
solana config get

# Check balance
solana balance

# Request airdrop (devnet)
solana airdrop 2
```

### Permission Errors

```bash
# Make scripts executable
chmod +x scripts/*.sh
```

## Contributing

When adding new scripts:

1. Use `#!/bin/bash` shebang
2. Add `set -e` for error handling
3. Add comments explaining what the script does
4. Update this README
5. Test on multiple platforms if possible

## Script Maintenance

- Keep scripts simple and focused
- Use consistent error handling
- Document prerequisites
- Test before committing
- Update README when adding scripts

## Related Documentation

- [DEPLOYMENT.md](../DEPLOYMENT.md) - Deployment guide
- [QUICK_START.md](../QUICK_START.md) - Quick start guide
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guidelines
