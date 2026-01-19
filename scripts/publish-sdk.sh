#!/bin/bash
# Publish script for Attesta SDKs

set -e

echo "📤 Publishing Attesta SDKs..."

# Check if we're in the right directory
if [ ! -f "package.json" ] && [ ! -f "Cargo.toml" ]; then
    echo "❌ Please run this from the project root"
    exit 1
fi

# Publish TypeScript SDK
echo "🔷 Publishing TypeScript SDK to npm..."
cd sdk/ts

# Check if logged in to npm
if ! npm whoami &> /dev/null; then
    echo "⚠️  Not logged in to npm. Please run: npm login"
    exit 1
fi

# Build first
npm run build

# Publish
read -p "Publish TypeScript SDK to npm? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    npm publish --access public
    echo "✅ TypeScript SDK published!"
else
    echo "⏭️  Skipped TypeScript SDK publish"
fi

cd ../..

# Publish Rust SDK
echo "🦀 Publishing Rust SDK to crates.io..."
cd sdk/rust

# Check if logged in to crates.io
if ! cargo login --check &> /dev/null; then
    echo "⚠️  Not logged in to crates.io. Please run: cargo login"
    exit 1
fi

# Build and check first
cargo build --release
cargo publish --dry-run

read -p "Publish Rust SDK to crates.io? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    cargo publish
    echo "✅ Rust SDK published!"
else
    echo "⏭️  Skipped Rust SDK publish"
fi

cd ../..

echo ""
echo "✅ SDK publishing complete!"
