#!/bin/bash
# Build script for Attesta Solana program

set -e

echo "🔨 Building Attesta program..."

# Check if Anchor is installed
if ! command -v anchor &> /dev/null; then
    echo "❌ Anchor CLI not found. Installing..."
    cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
    avm install latest
    avm use latest
fi

# Build the program
echo "📦 Building with Anchor..."
anchor build

# Check build status
if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Program IDL: target/idl/attesta.json"
    echo "📁 Program binary: target/deploy/attesta.so"
else
    echo "❌ Build failed!"
    exit 1
fi
