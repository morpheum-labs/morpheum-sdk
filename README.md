# Morpheum SDK

**Official Rust + TypeScript SDK for Morpheum — built for humans and AI agents.**

[![Crates.io](https://img.shields.io/crates/v/morpheum-sdk-native)](https://crates.io/crates/morpheum-sdk-native)
[![npm](https://img.shields.io/npm/v/@morpheum/sdk)](https://www.npmjs.com/package/@morpheum/sdk)

Sign transactions, issue/revoke VCs, create markets, and more — with a single elegant API that works identically in native Rust and the browser.

**Documentation:** [API Reference](API_REFERENCE.md) · [Full Guide](DOCUMENTATION.md) · [Architecture](ARCHITECTURE.md)

---

## Features

- Unified `MorpheumSdk` facade
- Full support for **NativeSigner**, **AgentSigner** (with TradingKeyClaim), MetaMask, Phantom, and Taproot
- Fluent builders for every operation
- Dynamic `SignerInfo` + `TradingKeyClaim` embedding (via `morpheum-signing-native`)
- Dual target: Native (CLI/bots/agents) + WASM/TypeScript (browser)
- Pluggable transport (gRPC + HTTP)
- Feature-gated modules (`market`, `vc`, `auth`)

---

## Installation

**Native Rust**

```toml
[dependencies]
morpheum-sdk-native = { version = "0.1", features = ["full"] }
```

