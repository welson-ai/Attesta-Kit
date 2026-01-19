# Installation

This guide will help you install and set up the Attesta SDK in your project.

## Prerequisites

Before installing the SDK, make sure you have:

- **Node.js** 16.0 or higher
- **npm** 7.0 or higher (or **yarn** 1.22+, **pnpm** 6.0+)
- A **modern browser** with WebAuthn support (for browser-based projects)
- **HTTPS** or **localhost** (required for WebAuthn API)

## Installation Methods

### npm

```bash
npm install @attesta/sdk @solana/web3.js
```

### yarn

```bash
yarn add @attesta/sdk @solana/web3.js
```

### pnpm

```bash
pnpm add @attesta/sdk @solana/web3.js
```

## Peer Dependencies

The Attesta SDK requires `@solana/web3.js` as a peer dependency. Make sure to install a compatible version:

```bash
npm install @solana/web3.js@^1.87.6
```

## Browser Installation (UMD)

For projects that don't use a bundler, you can include the SDK via a `<script>` tag:

```html
<!DOCTYPE html>
<html>
<head>
  <title>Attesta DApp</title>
</head>
<body>
  <!-- Include Solana Web3.js first -->
  <script src="https://unpkg.com/@solana/web3.js@latest/lib/index.iife.min.js"></script>
  
  <!-- Then include Attesta SDK -->
  <script src="https://unpkg.com/@attesta/sdk/dist/index.umd.js"></script>
  
  <script>
    // SDK is available as AttestaSDK global
    const { registerAttestaAccount } = AttestaSDK;
    // Use the SDK...
  </script>
</body>
</html>
```

## Framework-Specific Setup

### React / Next.js

```bash
npm install @attesta/sdk @solana/web3.js
```

```typescript
// In your component or page
import { registerAttestaAccount } from '@attesta/sdk';
import { Connection } from '@solana/web3.js';
```

**Note for Next.js**: If you're using Server-Side Rendering (SSR), you may need to dynamically import the SDK:

```typescript
// pages/payment.tsx or app/payment/page.tsx
import dynamic from 'next/dynamic';

const AttestaSDK = dynamic(() => import('@attesta/sdk'), {
  ssr: false, // WebAuthn requires browser APIs
});
```

### Vue.js

```bash
npm install @attesta/sdk @solana/web3.js
```

```vue
<script setup lang="ts">
import { registerAttestaAccount } from '@attesta/sdk';
import { Connection } from '@solana/web3.js';
</script>
```

### Angular

```bash
npm install @attesta/sdk @solana/web3.js
```

```typescript
// In your component
import { registerAttestaAccount } from '@attesta/sdk';
import { Connection } from '@solana/web3.js';
```

### Svelte / SvelteKit

```bash
npm install @attesta/sdk @solana/web3.js
```

```svelte
<script lang="ts">
  import { registerAttestaAccount } from '@attesta/sdk';
  import { Connection } from '@solana/web3.js';
</script>
```

## TypeScript Configuration

If you're using TypeScript, the SDK includes type definitions. Make sure your `tsconfig.json` includes:

```json
{
  "compilerOptions": {
    "module": "ESNext",
    "moduleResolution": "node",
    "target": "ES2020",
    "lib": ["ES2020", "DOM"],
    "types": ["node"]
  }
}
```

## Verifying Installation

Create a simple test file to verify the installation:

```typescript
// test-installation.ts
import { registerAttestaAccount } from '@attesta/sdk';
import { Connection, PublicKey } from '@solana/web3.js';

console.log('Attesta SDK installed successfully!');
console.log('registerAttestaAccount:', typeof registerAttestaAccount);
```

Run it:

```bash
# If using TypeScript
npx ts-node test-installation.ts

# Or compile and run
tsc test-installation.ts && node test-installation.js
```

## Environment Setup

### Development (Devnet)

For development, connect to Solana devnet:

```typescript
import { Connection } from '@solana/web3.js';

const connection = new Connection('https://api.devnet.solana.com');
```

### Production (Mainnet)

For production, use Solana mainnet:

```typescript
import { Connection } from '@solana/web3.js';

const connection = new Connection('https://api.mainnet-beta.solana.com');
// Or use a custom RPC endpoint
const connection = new Connection('https://your-rpc-endpoint.com');
```

### Local Development

If running a local Solana validator:

```typescript
const connection = new Connection('http://127.0.0.1:8899');
```

## Program ID Configuration

You'll need to configure the Attesta program ID. This is typically done via environment variables:

```typescript
// config.ts
import { PublicKey } from '@solana/web3.js';

export const ATTESTA_PROGRAM_ID = new PublicKey(
  process.env.NEXT_PUBLIC_ATTESTA_PROGRAM_ID || 
  'YourProgramIdHere1111111111111111111111111'
);
```

Create a `.env` file:

```env
NEXT_PUBLIC_ATTESTA_PROGRAM_ID=YourProgramIdHere1111111111111111111111111
```

## HTTPS Requirement

**Important**: WebAuthn requires HTTPS (or localhost). For production:

1. **Use HTTPS**: Deploy your app with SSL/TLS
2. **Development**: `localhost` and `127.0.0.1` work without HTTPS
3. **Testing**: Use tools like `ngrok` for HTTPS in local development

## Troubleshooting

### "Module not found" Error

If you see module resolution errors:

1. **Clear cache**: `npm cache clean --force`
2. **Delete node_modules**: `rm -rf node_modules package-lock.json`
3. **Reinstall**: `npm install`

### WebAuthn Not Available

If WebAuthn APIs are not available:

1. **Check HTTPS**: Ensure you're on HTTPS or localhost
2. **Browser Support**: Verify your browser supports WebAuthn
3. **Context**: WebAuthn only works in browser context, not Node.js

### TypeScript Errors

If you see TypeScript errors:

1. **Install types**: `npm install --save-dev @types/node`
2. **Check tsconfig**: Ensure proper module resolution
3. **Restart IDE**: Sometimes IDEs need a restart after installation

### Peer Dependency Warnings

If you see peer dependency warnings:

```bash
npm install @solana/web3.js@^1.87.6 --save
```

## Next Steps

Now that you have the SDK installed:

1. [JavaScript/TypeScript SDK Guide](./javascript-typescript-sdk.md) - Learn the API
2. [Quickstart](../developer-guides/quickstart.md) - Build your first integration
3. [DApp Integration](../developer-guides/dapp-integration.md) - Full integration guide

## Additional Resources

- [SDK Overview](./sdk-overview.md) - Understand the SDK architecture
- [Error Handling](./error-handling-and-security.md) - Handle errors properly
- [GitHub Repository](https://github.com/attesta/attesta-solana) - Source code and issues

---

**Installation complete!** Ready to start building? Check out the [Quickstart Guide](../developer-guides/quickstart.md).
