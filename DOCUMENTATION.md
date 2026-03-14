# Morpheum SDK Documentation

This document provides a comprehensive guide to the Morpheum SDK API, usage patterns, and examples.

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Core Concepts](#core-concepts)
- [Native SDK](#native-sdk)
- [WASM / TypeScript SDK](#wasm--typescript-sdk)
- [Module Reference](#module-reference)
- [Signing & Wallets](#signing--wallets)
- [Transaction Building](#transaction-building)
- [Error Handling](#error-handling)
- [Examples](#examples)

---

## Overview

The Morpheum SDK is a unified Rust + TypeScript SDK for the Morpheum blockchain. It provides:

- **Unified API** — Same concepts and patterns across native Rust and browser
- **Dual target** — Native (CLI, bots, agents) and WASM (browser, React, Vue, Svelte)
- **Pluggable transport** — gRPC and HTTP backends
- **Feature-gated modules** — Include only what you need

---

## Installation

### Native Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
morpheum-sdk-native = { version = "0.1", features = ["full"] }
```

For minimal builds, enable only the modules you need:

```toml
morpheum-sdk-native = {
    version = "0.1",
    features = ["market", "vc", "auth", "grpc"]
}
```

### TypeScript / npm

```bash
npm install @morpheum/sdk
```

---

## Quick Start

### Native Rust

```rust
use morpheum_sdk_native::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create signer (human wallet)
    let signer = NativeSigner::from_seed(b"your-32-byte-seed-here!!!!!!!!!!!!!!");

    // Create SDK
    let sdk = native(signer);

    // Build and sign a transaction
    let signed_tx = TxBuilder::new(signer)
        .chain_id("morpheum-1")
        .memo("Hello from SDK")
        .add_message(your_message)
        .sign()
        .await?;

    println!("TxHash: {}", signed_tx.txhash_hex());
    Ok(())
}
```

### TypeScript (Browser)

```typescript
import { MorpheumSdkWasm, setPanicHook } from '@morpheum/sdk';

setPanicHook();

const sdk = new MorpheumSdkWasm("https://sentry.morpheum.xyz", "morpheum-1");
console.log("SDK version:", sdk.version);
```

---

## Core Concepts

### SdkConfig

Configuration for RPC endpoint and chain ID:

```rust
let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-1");
```

### Transport

The `Transport` trait abstracts network I/O:

- `broadcast_tx(tx_bytes)` — Submit signed transaction
- `query(path, data)` — ABCI query (optional)

Concrete implementations: gRPC (tonic), HTTP (reqwest). The native SDK uses a placeholder by default; provide a custom transport for production.

### MorpheumClient

All module clients implement `MorpheumClient`:

- `config()` — SDK configuration
- `transport()` — Underlying transport
- `broadcast(tx_bytes)` — Broadcast signed transaction
- `query(path, data)` — ABCI query

### AccountId

32-byte account identifier. Construct with `AccountId::new([u8; 32])`.

### ChainId

String chain identifier (e.g. `"morpheum-1"`).

---

## Native SDK

### MorpheumSdk Facade

The main entry point for native applications:

```rust
use morpheum_sdk_native::prelude::*;

// With default config
let sdk = MorpheumSdk::new("https://sentry.morpheum.xyz", "morpheum-1");

// With custom transport
let sdk = MorpheumSdk::with_transport(config, Box::new(my_transport));
```

### Convenience Constructors

```rust
// Human wallet (NativeSigner)
let sdk = native(signer);

// Autonomous agent (AgentSigner)
let sdk = agent(agent_signer);
```

### Module Access

When features are enabled, construct module clients with the SDK's config and transport:

```rust
use morpheum_sdk_native::prelude::*;

let sdk = MorpheumSdk::new("https://sentry.morpheum.xyz", "morpheum-1");

#[cfg(feature = "market")]
let market_client = morpheum_sdk_market::MarketClient::new(
    sdk.config().clone(),
    // Provide a concrete transport (e.g. GrpcTransport) for production
);

#[cfg(feature = "vc")]
let vc_client = morpheum_sdk_vc::VcClient::new(sdk.config().clone(), transport);

#[cfg(feature = "auth")]
let auth_client = morpheum_sdk_auth::AuthClient::new(sdk.config().clone(), transport);
```

Alternatively, use `NativeClient` (when available) for convenient accessors: `native_client.market()`, `native_client.vc()`, etc.

---

## WASM / TypeScript SDK

### MorpheumSdkWasm

Browser entry point:

```typescript
const sdk = new MorpheumSdkWasm("https://sentry.morpheum.xyz", "morpheum-1");
```

### Wallet Adapters

The WASM crate supports wallet adapters for:

- **MetaMask** — Ethereum-style wallets
- **Phantom** — Solana-style wallets
- **Taproot** — Bitcoin Taproot keys

### Panic Hook

Always call `setPanicHook()` at app startup for better error messages in the console:

```typescript
import { setPanicHook } from '@morpheum/sdk';
setPanicHook();
```

---

## Module Reference

### Market Module

Query and manage markets:

```rust
let market = client.query_market(42).await?;
let markets = client.query_active_markets(10, 0).await?;
let stats = client.query_market_stats(42, None).await?;
```

**Builders:** `MarketCreateBuilder`, `MarketActivateBuilder`, `MarketSuspendBuilder`, etc.

### VC (Verifiable Credentials) Module

Issue and revoke credentials:

```rust
let request = VcIssueBuilder::new()
    .issuer(issuer_account)
    .subject(subject_account)
    .claims(claims)
    .expiry(expiry_ts)
    .build()?;
```

**Builders:** `VcIssueBuilder`, `RevokeVcBuilder`, `SelfRevokeVcBuilder`, etc.

### Auth Module

Authentication and nonce management.

### Identity Module

Identity resolution and management.

### Other Modules

- `agent_registry` — Agent registration
- `inference_registry` — Inference model registry
- `interop` — Cross-chain interoperability
- `job` — Job scheduling
- `bank` — Token transfers
- `staking` — Staking operations

---

## Signing & Wallets

### NativeSigner

For human-operated wallets:

```rust
// From 32-byte seed
let signer = NativeSigner::from_seed(&[0u8; 32]);

// From BIP-39 mnemonic
let signer = NativeSigner::from_mnemonic("abandon abandon ...", "")?;
```

### AgentSigner

For autonomous agents with delegated authority:

```rust
let agent_signer = AgentSigner::new(
    &seed,
    agent_account_id,
    None, // optional metadata
);
```

### TradingKeyClaim

Delegation claim for agent trading:

```rust
let claim = VcClaimBuilder::new()
    .issuer(owner_account)
    .subject(agent_account)
    .permissions(0b0001)  // TRADE
    .max_daily_usd(100_000)
    .expiry(expiry_timestamp)
    .build(now_secs)?;

// Attach to transaction
TxBuilder::new(agent_signer)
    .with_trading_key_claim(claim)
    .add_message(msg)
    .sign()
    .await?;
```

---

## Transaction Building

### TxBuilder

Fluent transaction builder:

```rust
let signed_tx = TxBuilder::new(signer)
    .chain_id("morpheum-1")
    .memo("Optional memo")
    .add_message(any_message)
    .with_trading_key_claim(claim)  // optional, for agents
    .with_nonce_provider(provider)  // optional, for nonce
    .sign()
    .await?;
```

### Adding Messages

```rust
// From module builder (e.g. MarketCreateBuilder)
let create_req = MarketCreateBuilder::new()
    .from_address(addr)
    .base_asset_index(1)
    .quote_asset_index(2)
    .build()?;

signed_tx = TxBuilder::new(signer)
    .add_message(create_req.to_any())
    .sign()
    .await?;
```

### SignedTx

Result of signing:

- `txhash_hex()` — Transaction hash as hex string
- `raw_bytes()` — Raw TxRaw bytes for broadcast

---

## Error Handling

### SdkError Variants

| Variant | Description |
|---------|-------------|
| `Signing` | Key handling, claim verification, signature errors |
| `Transport` | Network, gRPC, HTTP errors |
| `Encode` / `Decode` | Protobuf serialization errors |
| `Config` | Invalid configuration |
| `InvalidInput` | User-provided invalid data |
| `Other` | Catch-all |

### Usage

```rust
match sdk_result {
    Ok(tx) => println!("Success: {}", tx.txhash_hex()),
    Err(SdkError::Transport(msg)) => eprintln!("Network error: {}", msg),
    Err(SdkError::Signing(e)) => eprintln!("Signing error: {:?}", e),
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## Examples

### Run Examples

```bash
# Market creation
cargo run -p morpheum-sdk-examples --example native_market

# Agent trading
cargo run -p morpheum-sdk-examples --example native_agent_trade

# VC issuance
cargo run -p morpheum-sdk-examples --example native_agent_vc_issue

# BIP-39 mnemonic
cargo run -p morpheum-sdk-examples --example native_bip39
```

### WASM Examples

TypeScript examples for browser:

- `examples/wasm_browser_metamask_market.ts`
- `examples/wasm_browser_phantom_vc.ts`
- `examples/wasm_browser_taproot.ts`

---

## API Stability

The SDK is in active development. Public APIs may change between minor versions. Check the [changelog](https://github.com/morpheum-labs/morpheum-sdk/releases) for breaking changes.

---

## Further Resources

- [README](README.md) — Project overview
- [API_REFERENCE](API_REFERENCE.md) — Structured API reference
- [ARCHITECTURE](ARCHITECTURE.md) — Design and structure
- [CONTRIBUTING](CONTRIBUTING.md) — Contribution guidelines
- [Morpheum](https://morpheum.xyz) — Project homepage
