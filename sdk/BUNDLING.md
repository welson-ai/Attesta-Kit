# SDK Bundling Guide

This document explains how the Attesta SDKs are bundled and distributed.

## TypeScript SDK

The TypeScript SDK is bundled in multiple formats for different use cases:

### Build Formats

1. **CommonJS** (`dist/index.js`)
   - For Node.js and bundlers that use CommonJS
   - Format: `require('@attesta/sdk')`

2. **ES Modules** (`dist/index.esm.js`)
   - For modern bundlers and ES module imports
   - Format: `import { ... } from '@attesta/sdk'`

3. **UMD** (`dist/index.umd.js`)
   - For direct browser use via `<script>` tag
   - Format: `AttestaSDK` global variable

4. **Type Definitions** (`dist/index.d.ts`)
   - TypeScript type definitions
   - Auto-generated from source

### Building

```bash
# Build all formats
cd sdk/ts
npm install
npm run build

# Or use the build script
./scripts/build-sdk.sh
```

### Publishing

```bash
# Publish to npm
cd sdk/ts
npm publish --access public

# Or use the publish script
./scripts/publish-sdk.sh
```

### Bundle Size

- CommonJS: ~50KB (minified)
- ES Modules: ~50KB (minified)
- UMD: ~55KB (minified, includes IIFE wrapper)

**Note**: `@solana/web3.js` is externalized (not bundled) to avoid duplication.

## Rust SDK

The Rust SDK is built as a library crate that can be used in Rust projects.

### Building

```bash
cd sdk/rust
cargo build --release
```

### Publishing

```bash
# Publish to crates.io
cd sdk/rust
cargo publish
```

### Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
attesta-sdk = "0.1.0"
```

## Distribution

### NPM Package

The TypeScript SDK is distributed via npm:

```json
{
  "name": "@attesta/sdk",
  "version": "0.1.0",
  "main": "dist/index.js",
  "module": "dist/index.esm.js",
  "browser": "dist/index.umd.js",
  "types": "dist/index.d.ts"
}
```

### Crates.io

The Rust SDK is distributed via crates.io:

```toml
[dependencies]
attesta-sdk = "0.1.0"
```

## Development

### Local Development

```bash
# TypeScript SDK
cd sdk/ts
npm install
npm run dev  # Watch mode

# Rust SDK
cd sdk/rust
cargo build
```

### Testing

```bash
# TypeScript SDK
cd sdk/ts
npm test  # When tests are added

# Rust SDK
cd sdk/rust
cargo test
```

## Bundle Optimization

### Tree Shaking

The ES module build supports tree shaking. Import only what you need:

```typescript
// Good - tree shakeable
import { registerAttestaAccount } from '@attesta/sdk';

// Less optimal - imports everything
import * as AttestaSDK from '@attesta/sdk';
```

### External Dependencies

`@solana/web3.js` is marked as external to:
- Reduce bundle size
- Avoid version conflicts
- Allow users to use their preferred version

## Versioning

SDKs follow semantic versioning:
- **Major**: Breaking changes
- **Minor**: New features, backward compatible
- **Patch**: Bug fixes, backward compatible

## CI/CD

SDKs should be built and published automatically via CI/CD:

1. Build on every commit
2. Publish on version tags
3. Run tests before publishing
4. Generate changelog
