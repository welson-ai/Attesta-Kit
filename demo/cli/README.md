# CLI Demo

Command-line interface demo application for Attesta. This demonstrates how to use Attesta's passkey-based account abstraction from a terminal application.

## Overview

The CLI demo provides a simple command-line interface for:
- Registering Attesta accounts
- Creating passkey credentials
- Making payments with passkey authorization
- Managing account policies

## Prerequisites

- Rust (latest stable)
- Solana CLI installed
- Access to a Solana RPC endpoint (devnet or local)
- A WebAuthn-compatible environment (note: CLI may have limited WebAuthn support)

## Building

```bash
cd demo/cli
cargo build --release
```

## Usage

```bash
# Run the CLI demo
cargo run --release

# Or run with specific command
cargo run --release -- register
cargo run --release -- pay <recipient> <amount>
```

## Features

### Account Registration

Register a new Attesta account with a passkey:

```bash
cargo run --release -- register --name "Alice"
```

### Payment

Make a payment using passkey authorization:

```bash
cargo run --release -- pay <recipient_address> <amount_in_lamports>
```

### Policy Management

Configure account policies:

```bash
cargo run --release -- set-policy --spending-limit 1000000000
```

## Architecture

The CLI demo uses:
- **Attesta SDK (Rust)**: For account and transaction management
- **Solana Web3**: For blockchain interaction
- **WebAuthn**: For passkey authentication (may require browser integration)

## Limitations

**Note**: WebAuthn requires browser APIs, so the CLI demo may need to:
- Use a browser-based authentication flow
- Integrate with a local web server for WebAuthn
- Use alternative authentication methods for CLI

## Development

To extend the CLI demo:

1. Add new commands in `src/main.rs`
2. Implement command handlers
3. Use the Attesta SDK for blockchain operations

## Example Workflow

```bash
# 1. Register an account
cargo run --release -- register --name "Demo User"

# 2. Check account balance
cargo run --release -- balance

# 3. Make a payment
cargo run --release -- pay <recipient> 1000000000

# 4. View transaction history
cargo run --release -- history
```

## Integration with Web Demo

For full WebAuthn support, consider using the [web demo](../web/README.md) which runs in a browser and has full WebAuthn API access.

## Troubleshooting

### WebAuthn Not Available

The CLI environment may not have WebAuthn support. Consider:
- Using the web demo for full functionality
- Implementing a browser-based auth flow
- Using alternative authentication for CLI

### Connection Issues

Ensure your Solana RPC endpoint is accessible:
```bash
# Test connection
solana cluster-version
```

### Build Errors

Make sure all dependencies are installed:
```bash
cargo update
cargo build
```

## Next Steps

- Check out the [web demo](../web/README.md) for browser-based usage
- Read the [SDK documentation](../../sdk/README.md)
- See [Quickstart Guide](../../QUICK_START.md) for deployment
