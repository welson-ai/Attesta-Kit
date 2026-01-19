---
sidebar_position: 1
---

# Attesta Kit — Introduction

Welcome to **Attesta Kit**, a **passkey-first infrastructure kit on Solana** for building **secure, policy-based, and recoverable smart accounts**.

Attesta Kit replaces seed phrases and private keys with **passkeys (WebAuthn)**, enforced through **on-chain policies**, **non-custodial recovery mechanisms**, and **developer-friendly SDKs**.

It is designed as a foundational layer for wallets, payments, embedded Web3, and consumer-scale Solana applications.

## What is Attesta Kit?

Attesta Kit is a modular authentication and authorization infrastructure that enables **smart accounts controlled by passkeys**, rather than raw private keys.

It provides:
- **Passkey-based authentication**
- **Policy-driven authorization**
- **Built-in account recovery**
- **Solana-native smart account programs**
- **SDKs and tooling for developers**

Together, these components allow users to interact with Solana securely using biometrics or device credentials, without ever handling a seed phrase.

## Core Components

### 1. Passkey Authentication

- WebAuthn / Passkeys for cryptographic authentication  
- Device-bound keys secured by hardware (Secure Enclave / TPM)  
- Phishing-resistant and passwordless  
- Works across mobile and desktop environments  

### 2. Policy Engine (On-Chain)

Attesta Kit introduces an **on-chain policy engine** that defines **how, when, and by whom** an account can be used.

Supported policy primitives include:
- Spending and transaction limits  
- Session-based permissions  
- Multi-passkey requirements  
- App- or domain-scoped access  
- Role-based authorization (user, guardian, relayer)  

All policies are enforced on-chain, making them transparent, auditable, and composable.

### 3. Smart Account Architecture

- Program-owned smart accounts on Solana  
- Supports multiple passkeys per account  
- Fully composable with existing Solana programs  
- Designed for wallets, payments, and embedded use cases  

### 4. Recovery Framework

Attesta Kit provides **non-custodial recovery** without seed phrases:

- Guardian-based recovery  
- Multi-device / multi-passkey recovery  
- Time-delayed recovery policies  
- Policy-governed account restoration  

Recovery uses the same policy engine, ensuring security even during account recovery.

### 5. SDKs & Developer Tooling

Attesta Kit ships with SDKs that abstract cryptography and on-chain complexity:

- **JavaScript / TypeScript SDK** (web & mobile)  
- **Rust SDK** for Solana programs  
- Passkey registration & verification utilities  
- Policy configuration helpers  
- Transaction creation and signing APIs  

This allows developers to integrate passkey-based smart accounts quickly and safely.

## Why Attesta Kit on Solana?

Solana enables:
- ⚡ High-throughput authentication flows  
- 💸 Low-cost policy enforcement  
- 📱 Mobile-first user experiences  
- 🔐 Secure, scalable consumer applications  

Attesta Kit turns Solana into a **consumer-ready identity and authorization layer**.

## Use Cases

- Passkey-based smart wallets  
- Embedded wallets for Web2 & fintech apps  
- Secure payments and subscriptions  
- DAO, enterprise, and team access control  
- Consumer dApps with account abstraction  

## Getting Started

### Requirements

- [Node.js](https://nodejs.org/en/download/) version **20.0 or above**  
  - Enable all recommended dependencies during installation  

### Run the Documentation Locally

```bash
npm install
npm run start
