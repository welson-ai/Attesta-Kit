#!/bin/bash
# Generate IDL from Anchor program and copy to SDK

set -e

echo "🔨 Building Anchor program to generate IDL..."
anchor build

echo "📋 Copying IDL to SDK..."
mkdir -p sdk/ts/idl
cp target/idl/attesta.json sdk/ts/idl/attesta.json

echo "✅ IDL generated and copied to sdk/ts/idl/attesta.json"
echo ""
echo "📝 Next steps:"
echo "1. Update instruction discriminators in:"
echo "   - sdk/ts/src/register.ts"
echo "   - sdk/ts/src/instructions.ts"
echo ""
echo "2. Extract discriminators from IDL:"
echo "   cat sdk/ts/idl/attesta.json | grep -A 5 discriminator"
echo ""
echo "3. Or use Anchor client with IDL for automatic handling"
