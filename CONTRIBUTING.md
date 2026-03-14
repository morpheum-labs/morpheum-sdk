# Contributing to Morpheum SDK

Thank you for your interest in contributing to the Morpheum SDK. This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Coding Standards](#coding-standards)
- [Pull Request Process](#pull-request-process)
- [Testing](#testing)
- [Documentation](#documentation)

---

## Code of Conduct

By participating in this project, you agree to uphold a respectful and inclusive environment. Be constructive, collaborative, and professional in all interactions.

---

## Getting Started

### Prerequisites

- **Rust** 1.80 or later (`rustup` recommended)
- **Git** for version control
- **Sibling repositories** (for full builds):
  - `morpheum-proto` — canonical protobuf definitions
  - `morpheum-signing` — official signing library

Clone the SDK and sibling repos into a common parent directory:

```
morpheumlabs/
├── morpheum-sdk/      # this repo
├── morpheum-proto/
└── morpheum-signing/
```

### Quick Start

```bash
# Verify Rust version
rustup show

# Build the workspace
cargo build

# Run all tests
cargo test

# Run examples
cargo run -p morpheum-sdk-examples --example native_market
```

---

## Development Setup

### Building

```bash
# Build everything (requires morpheum-proto and morpheum-signing siblings)
cargo build

# Build specific crates
cargo build -p morpheum-sdk-core
cargo build -p morpheum-sdk-native --features full
cargo build -p morpheum-sdk-wasm --target wasm32-unknown-unknown
```

### Feature Flags

The SDK uses feature flags for modular builds. Key features:

| Crate | Feature | Description |
|-------|---------|-------------|
| `morpheum-sdk-native` | `full` | All modules + gRPC + HTTP transport |
| `morpheum-sdk-native` | `market`, `vc`, `auth`, etc. | Individual module clients |
| `morpheum-sdk-native` | `grpc`, `http` | Transport backends |
| `morpheum-sdk-wasm` | `full-wasm` | Browser build with all modules |

### WASM Build

```bash
# Install wasm32 target
rustup target add wasm32-unknown-unknown

# Build WASM
cargo build -p morpheum-sdk-wasm --target wasm32-unknown-unknown --features full-wasm
```

---

## Project Structure

```
morpheum-sdk/
├── crates/
│   ├── core/           # no_std foundation, types, transport trait, TxBuilder
│   ├── native/         # Full Rust SDK facade (CLI, bots, agents)
│   ├── wasm/           # WASM + TypeScript bindings (browser)
│   ├── market/         # Market module client
│   ├── vc/             # Verifiable Credentials module
│   ├── auth/           # Auth module
│   ├── identity/       # Identity module
│   └── ...             # Other domain modules
├── examples/           # Runnable examples
├── tests/              # Integration tests
├── Cargo.toml          # Workspace manifest
└── README.md
```

---

## Coding Standards

### General Principles

1. **SOLID** — Single responsibility, clear interfaces, dependency injection via traits
2. **no_std first** — Keep `core` crate `no_std`-compatible; use `alloc` for collections
3. **Feature-gated modules** — Optional dependencies via Cargo features
4. **Fluent builders** — Use builder pattern for complex request construction
5. **`#![forbid(unsafe_code)]`** — No unsafe code in `native` and `wasm` crates

### Module Pattern

Each domain module (market, vc, auth, etc.) follows this structure:

- `client.rs` — Implements `MorpheumClient`, provides query/transaction methods
- `builder.rs` — Fluent builders for transaction messages
- `requests.rs` — Request types (convert to protobuf)
- `types.rs` — Domain types (convert from protobuf)

### Naming Conventions

- **Crates**: `morpheum-sdk-<module>` (kebab-case)
- **Types**: `PascalCase` (e.g. `MarketClient`, `TxBuilder`)
- **Functions**: `snake_case` (e.g. `query_market`, `add_message`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g. `TEST_CHAIN_ID`)

### Error Handling

- Use `SdkError` from `morpheum-sdk-core` for all public APIs
- Prefer specific variants: `Transport`, `Config`, `InvalidInput`, etc.
- Use `thiserror` for derive-based error types

### Async Traits

- Use `#[async_trait(?Send)]` for WASM compatibility (single-threaded)
- All `Transport` and `MorpheumClient` implementations must use this

---

## Pull Request Process

1. **Fork** the repository and create a branch from `main`
2. **Implement** your changes following the coding standards
3. **Add tests** for new functionality
4. **Update documentation** if the public API changes
5. **Run** `cargo test` and `cargo clippy` before submitting
6. **Submit** a PR with a clear description and reference any issues

### PR Checklist

- [ ] Code compiles without warnings
- [ ] All tests pass (`cargo test`)
- [ ] Clippy passes (`cargo clippy --all-targets`)
- [ ] Documentation is updated for public API changes
- [ ] Changelog updated (if applicable)

---

## Testing

### Unit Tests

Each crate has inline `#[cfg(test)]` modules. Run:

```bash
cargo test
```

### Integration Tests

Integration tests live in the `tests/` package:

```bash
cargo test -p morpheum-sdk-tests
```

### Test Utilities

Shared helpers are in `tests/common.rs`:

- `test_native_signer()`, `test_agent_signer()`
- `test_sdk()`, `test_trading_key_claim()`
- `TestNonceProvider` for deterministic nonce tests

### Mocking

Use `DummyTransport` or custom transport implementations for tests that don't require network access.

---

## Documentation

### Doc Comments

- All public items must have doc comments
- Use `///` for items and `//!` for module-level docs
- Include usage examples in doc comments where helpful

### Building Docs

```bash
cargo doc --no-deps --open
```

### README Updates

Update `README.md` when adding new features or changing installation instructions.

---

## Questions?

- Open an issue for bugs or feature requests
- Check existing issues and discussions before creating new ones
- Be specific and provide reproduction steps for bug reports

## Related Documentation

- [API_REFERENCE](API_REFERENCE.md) — Structured API reference for all public types and methods
- [DOCUMENTATION](DOCUMENTATION.md) — Usage guide and examples
- [ARCHITECTURE](ARCHITECTURE.md) — Design and structure

Thank you for contributing to Morpheum!
