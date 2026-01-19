#!/bin/bash
# Deployment script for Attesta Solana program

set -e

NETWORK=${1:-devnet}

echo "🚀 Deploying Attesta to $NETWORK..."

# Check if Anchor is installed
if ! command -v anchor &> /dev/null; then
    echo "❌ Anchor CLI not found. Please install Anchor first."
    exit 1
fi

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI not found. Please install Solana CLI first."
    exit 1
fi

# Set the cluster
echo "🌐 Setting cluster to $NETWORK..."
solana config set --url $NETWORK

# Check wallet balance
echo "💰 Checking wallet balance..."
BALANCE=$(solana balance | awk '{print $1}')
echo "Current balance: $BALANCE SOL"

# Build first
echo "🔨 Building program..."
anchor build

# Deploy
echo "📤 Deploying program..."
anchor deploy --provider.cluster $NETWORK

if [ $? -eq 0 ]; then
    echo "✅ Deployment successful!"
    echo "📋 Program ID: $(solana address -k target/deploy/attesta-keypair.json)"
    echo ""
    echo "🔍 Verify deployment:"
    echo "   solana program show $(solana address -k target/deploy/attesta-keypair.json)"
else
    echo "❌ Deployment failed!"
    exit 1
fi
