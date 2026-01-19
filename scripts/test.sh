#!/bin/bash
# Test script for Attesta

set -e

echo "🧪 Running Attesta tests..."

# Run Rust unit tests
echo "📦 Running Rust unit tests..."
cargo test --lib --workspace

# Run Anchor tests if they exist
if [ -d "tests" ]; then
    echo "🔍 Running Anchor integration tests..."
    anchor test
fi

echo "✅ All tests passed!"
