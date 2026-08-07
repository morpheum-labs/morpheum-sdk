<!-- morpheum-claude-framework v2026-08-07 — shared blocks synced by sync.sh; edit prose freely -->
# morpheum-sdk

The official Rust client SDK for the Morpheum L1: a unified `MorpheumSdk` facade over
pluggable signers and transports, with one thin builder crate per on-chain module.
Published as `morpheum-sdk-native` (Rust) with browser examples under `examples/`.

**This repo is PUBLIC on GitHub.** Everything committed here is public content.

## Layout

- `crates/core` — builder, client, transport, config, chain registry
- `crates/native`, `crates/wasm`, `crates/ws` — target/transport surfaces
- `crates/<module>` ×~40 — one thin crate per chain module (bank, clob, staking, …)
- `crates/{evm,svm,cosmwasm}` — chain adapters
- `API_REFERENCE.md`, `ARCHITECTURE.md`, `DOCUMENTATION.md` — keep these current with API
  changes; AI agents are first-class consumers of this SDK and of these docs

## Commands `[host]` (builds on the host; needs sibling checkouts)

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Invariants

- **Module crates are template-shaped.** A structural change (builder signature, error
  type, transport hook) applies uniformly across all module crates in one PR — never
  special-case one crate and leave the rest to drift.
- Signing bytes come from `morpheum-signing`; this SDK wraps and forwards (e.g.
  `with_genesis_hash`) and must never assemble sign-doc bytes itself.
- Public API stability matters: additive changes preferred; breaking changes need the CLI
  and e2e consumers updated in the same batch.
- Known-weak: `crates/wasm` does not currently build for `wasm32` and predates the
  signing-SSOT discipline — do not extend it; repair-or-retire is an open decision. Use
  `crates/gmp` as a reminder to prefer workspace deps over `../../../` paths when touching
  manifests.

<!-- framework:begin ripple -->
## Cross-repo ripple

- Depends on siblings: `../morpheum-proto`, `../morpheum-primitives`,
  `../morpheum-signing/crates/{core,native,wasm-lib}`.
- Dependents: `morpheum-cli` (native, core, evm, svm, cosmwasm, cctp, gov) and the
  `orchestrator` e2e suites (each module suite has an `sdk/` crate driving this SDK).
- Public API changes ripple to the CLI and to dozens of e2e crates — enumerate and compile
  dependents before merging.
<!-- framework:end ripple -->

## Verification

- CI = fmt + workspace clippy `-D warnings` + workspace tests, all `--all-features`, with
  the transitive sibling closure checked out. Run the same shapes locally.
- Integration tests live in `tests/` at the workspace root; module-crate unit tests sit in
  each crate.
