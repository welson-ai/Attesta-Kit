#!/bin/bash
# Build script for Attesta SDKs

set -e

echo "📦 Building Attesta SDKs..."

# Build TypeScript SDK
echo "🔷 Building TypeScript SDK..."
cd sdk/ts

if [ ! -d "node_modules" ]; then
    echo "📥 Installing TypeScript SDK dependencies..."
    npm install
fi

echo "🔨 Building TypeScript SDK..."
npm run build

if [ $? -eq 0 ]; then
    echo "✅ TypeScript SDK built successfully!"
    echo "   - CommonJS: dist/index.js"
    echo "   - ES Modules: dist/index.esm.js"
    echo "   - UMD (Browser): dist/index.umd.js"
    echo "   - Types: dist/index.d.ts"
else
    echo "❌ TypeScript SDK build failed!"
    exit 1
fi

cd ../..

# Build Rust SDK
echo "🦀 Building Rust SDK..."
cd sdk/rust

echo "🔨 Building Rust SDK..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Rust SDK built successfully!"
    echo "   - Library: target/release/libattesta_sdk.rlib"
else
    echo "❌ Rust SDK build failed!"
    exit 1
fi

cd ../..

echo ""
echo "✅ All SDKs built successfully!"
echo ""
echo "📦 SDK Outputs:"
echo "   TypeScript: sdk/ts/dist/"
echo "   Rust: sdk/rust/target/release/"
