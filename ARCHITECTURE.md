# Morpheum SDK Architecture

This document describes the design, structure, and key architectural decisions of the Morpheum SDK.

## Table of Contents

- [Overview](#overview)
- [Layered Architecture](#layered-architecture)
- [Crate Dependency Graph](#crate-dependency-graph)
- [Core Abstractions](#core-abstractions)
- [Module Pattern](#module-pattern)
- [Dual Target: Native vs WASM](#dual-target-native-vs-wasm)
- [Signing Integration](#signing-integration)
- [Transport Layer](#transport-layer)
- [Feature Gating](#feature-gating)
- [Design Principles](#design-principles)

---

## Overview

The Morpheum SDK is a **modular, dual-target** SDK that provides a unified API for both native Rust applications (CLI tools, trading bots, autonomous agents) and browser-based applications (React, Vue, Svelte, Next.js).

**Key design goals:**

- **Unified API** — Same concepts and patterns across targets
- **Minimal core** — `no_std`-compatible foundation
- **Pluggable transport** — Swap gRPC, HTTP, or custom backends
- **Feature-gated modules** — Pay only for what you use
- **Zero unsafe code** — In `native` and `wasm` crates

---

## Layered Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Application Layer                            │
│  (CLI tools, bots, agents, React/Vue/Svelte apps)               │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Facade Layer                                  │
│  morpheum-sdk-native (Rust)  │  morpheum-sdk-wasm (TypeScript)   │
│  MorpheumSdk, native(), agent()  │  MorpheumSdkWasm, setPanicHook │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Module Layer                                  │
│  market │ vc │ auth │ identity │ agent_registry │ bank │ ...    │
│  (MarketClient, VcClient, AuthClient, etc.)                       │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Core Layer                                    │
│  morpheum-sdk-core                                               │
│  Transport, MorpheumClient, TxBuilder, SdkConfig, SdkError        │
└─────────────────────────────────────────────────────────────────┘
                                │
                ┌───────────────┴───────────────┐
                ▼                               ▼
┌───────────────────────────┐   ┌───────────────────────────┐
│   morpheum-signing        │   │   morpheum-proto           │
│   (NativeSigner,          │   │   (Protobuf definitions)  │
│   AgentSigner, TxBuilder) │   │                            │
└───────────────────────────┘   └───────────────────────────┘
```

---

## Crate Dependency Graph

```
                    morpheum-sdk-native
                    morpheum-sdk-wasm
                           │
                           ▼
                    morpheum-sdk-core
                    (no_std foundation)
                    /              \
                   /                \
                  ▼                  ▼
         morpheum-signing      morpheum-proto
         (native / core)       (protobuf)

    morpheum-sdk-market, morpheum-sdk-vc, morpheum-sdk-auth, ...
    (each depends on core + proto, optional in native/wasm)
```

**Workspace members:**

| Crate | Purpose |
|-------|---------|
| `morpheum-sdk-core` | Foundation: types, errors, transport trait, TxBuilder, MorpheumClient |
| `morpheum-sdk-native` | Full Rust facade: MorpheumSdk, native(), agent() |
| `morpheum-sdk-wasm` | WASM bindings: MorpheumSdkWasm, wallet adapters |
| `morpheum-sdk-market` | Market module client + builders |
| `morpheum-sdk-vc` | Verifiable Credentials client + builders |
| `morpheum-sdk-auth` | Auth client + builders |
| `morpheum-sdk-identity` | Identity client |
| ... | Additional domain modules |

---

## Core Abstractions

### Transport Trait

The `Transport` trait is the **single point of network I/O** in the SDK:

```rust
#[async_trait(?Send)]
pub trait Transport: Send + Sync + 'static {
    async fn broadcast_tx(&self, tx_bytes: Vec<u8>) -> Result<BroadcastResult, SdkError>;
    async fn query(&self, path: &str, data: Vec<u8>) -> Result<Vec<u8>, SdkError>;
}
```

- **Object-safe** — Can be used as `Box<dyn Transport>`
- **`async_trait(?Send)`** — Enables WASM (single-threaded, no `Send` required)
- **Pluggable** — gRPC, HTTP, or custom implementations

### MorpheumClient Trait

All module clients implement this base trait:

```rust
#[async_trait(?Send)]
pub trait MorpheumClient: Send + Sync + 'static {
    fn config(&self) -> &SdkConfig;
    fn transport(&self) -> &dyn Transport;
    async fn broadcast(&self, tx_bytes: Vec<u8>) -> Result<BroadcastResult, SdkError>;
    async fn query(&self, path: &str, data: Vec<u8>) -> Result<Vec<u8>, SdkError>;
}
```

Default implementations for `broadcast` and `query` delegate to the transport, keeping module code DRY.

### TxBuilder

Thin wrapper around `morpheum-signing`'s TxBuilder:

- Accepts `Signer` (NativeSigner, AgentSigner, etc.)
- Fluent API: `chain_id()`, `memo()`, `add_message()`, `with_trading_key_claim()`
- Delegates signing, nonce, and SignerInfo to the signing library

---

## Module Pattern

Each domain module follows a consistent structure:

```
crates/<module>/
├── src/
│   ├── lib.rs       # Public exports
│   ├── client.rs   # Implements MorpheumClient, query/transaction methods
│   ├── builder.rs  # Fluent builders (MarketCreateBuilder, VcIssueBuilder, etc.)
│   ├── requests.rs # Request types → protobuf
│   └── types.rs    # Domain types ← protobuf
└── Cargo.toml
```

**Data flow:**

1. **Query:** `Client::query_*()` → encode request → `transport.query()` → decode response → domain type
2. **Transaction:** `Builder::build()` → request type → `to_any()` → `TxBuilder::add_message()` → sign → broadcast

---

## Dual Target: Native vs WASM

### Native (morpheum-sdk-native)

- Full Rust standard library
- `morpheum-signing-native` (BIP-39, full crypto)
- gRPC and HTTP transport
- All modules available via features

### WASM (morpheum-sdk-wasm)

- `wasm32-unknown-unknown` target
- `wasm-bindgen` for JS interop
- `tsify` for TypeScript types
- Wallet adapters (MetaMask, Phantom, Taproot)
- `#[async_trait(?Send)]` — No `Send` bound for single-threaded JS

### Shared Core

`morpheum-sdk-core` is `no_std` by default:

- Uses `alloc` for `String`, `Vec`, `Box`
- `std` feature for full standard library
- `wasm` feature for `wasm-bindgen` / `tsify`

---

## Signing Integration

The SDK **does not implement signing**. It delegates to the official `morpheum-signing` library:

- **morpheum-signing-core** — `no_std` types, TxBuilder, claim verification
- **morpheum-signing-native** — NativeSigner, AgentSigner, BIP-39, full crypto

**Key types re-exported:**

- `NativeSigner`, `AgentSigner`
- `TradingKeyClaim`, `VcClaimBuilder`
- `Signer` trait
- `Any` (prost_types::Any) for message packing

**Claim flow:**

1. Owner creates `TradingKeyClaim` via `VcClaimBuilder`
2. Agent attaches claim via `TxBuilder::with_trading_key_claim()`
3. Signing library embeds claim in `SignerInfo.signing_options`
4. Signature covers the claim (tamper-evident)

---

## Transport Layer

### Placeholder Transport

The native SDK uses a `PlaceholderTransport` by default that returns errors for all operations. Production applications must provide a concrete transport:

```rust
let sdk = MorpheumSdk::with_transport(config, Box::new(GrpcTransport::new(endpoint)?));
```

### gRPC Transport

When `grpc` feature is enabled, `tonic` is used for gRPC calls. Path format: `/package.Service/Method` (e.g. `/market.v1.Query/QueryMarket`).

### HTTP Transport

When `http` feature is enabled, `reqwest` is used for REST/HTTP calls.

---

## Feature Gating

### Native Crate

```toml
full = ["market", "vc", "auth", "grpc", "http", "std"]
market = ["dep:morpheum-sdk-market"]
vc = ["dep:morpheum-sdk-vc"]
grpc = ["dep:tonic", "dep:tokio"]
http = ["dep:reqwest", "dep:tokio"]
```

### WASM Crate

```toml
full-wasm = ["std", "console_error_panic_hook", "morpheum-sdk-market?", "morpheum-sdk-vc?", "morpheum-sdk-auth?"]
```

Optional dependencies (`?`) allow modules to be excluded for smaller WASM bundles.

---

## Design Principles

### 1. SOLID

- **Single responsibility** — Each crate has one clear purpose
- **Open/closed** — Extend via new modules, not by modifying core
- **Liskov substitution** — Any `Transport` or `MorpheumClient` implementation is interchangeable
- **Interface segregation** — Minimal traits; optional methods have defaults
- **Dependency inversion** — Depend on `Transport` trait, not concrete implementations

### 2. no_std First

- Core crate stays `no_std`-compatible
- Enables embedded and constrained environments
- `std` is a feature, not a requirement

### 3. Zero Unsafe Code

- `#![forbid(unsafe_code)]` in `native` and `wasm`
- Security-critical code lives in `morpheum-signing` (audited separately)

### 4. Fluent Builders

- Complex requests use builder pattern
- Type-safe, self-documenting API
- Validation at `build()` time

### 5. Protobuf as Boundary

- All wire formats are protobuf (via `morpheum-proto`)
- SDK types convert to/from protobuf at module boundaries
- Keeps SDK decoupled from protocol evolution

---

## File Layout Summary

```
morpheum-sdk/
├── Cargo.toml                 # Workspace
├── crates/
│   ├── core/                  # Foundation
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── client.rs      # MorpheumClient trait
│   │   │   ├── transport.rs   # Transport trait
│   │   │   ├── builder.rs     # TxBuilder
│   │   │   ├── config.rs
│   │   │   ├── error.rs
│   │   │   └── types.rs
│   │   └── Cargo.toml
│   ├── native/                # Rust facade
│   ├── wasm/                  # Browser facade
│   ├── market/                # Module: client + builder + types + requests
│   ├── vc/
│   ├── auth/
│   └── ...
├── examples/
├── tests/
└── README.md
```

---

## Future Considerations

- **Streaming** — gRPC streaming for real-time data
- **Retry/backoff** — Transport-level retry policies
- **Caching** — Query result caching for repeated reads
- **Telemetry** — OpenTelemetry integration for observability

---

## See Also

- [API_REFERENCE](API_REFERENCE.md) — Structured API reference
- [DOCUMENTATION](DOCUMENTATION.md) — Usage guide
