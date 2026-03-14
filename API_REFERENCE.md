# Morpheum SDK API Reference

Structured reference for the Morpheum SDK public API. For usage guides and examples, see [DOCUMENTATION](DOCUMENTATION.md).

## Table of Contents

- [Core (`morpheum-sdk-core`)](#core-morpheum-sdk-core)
- [Native SDK (`morpheum-sdk-native`)](#native-sdk-morpheum-sdk-native)
- [WASM SDK (`morpheum-sdk-wasm`)](#wasm-sdk-morpheum-sdk-wasm)
- [Market Module](#market-module)
- [VC Module](#vc-module)
- [Auth Module](#auth-module)
- [Other Modules](#other-modules)

---

## Core (`morpheum-sdk-core`)

Foundation crate shared by native and WASM targets. `no_std` compatible.

### Types

| Type | Description |
|------|-------------|
| `AccountId` | 32-byte account identifier. `new(bytes)`, `as_bytes()` |
| `ChainId` | Chain identifier string. `new(s)`, `as_str()` |
| `SignedTx` | Signed transaction. `raw_bytes()`, `txhash_hex()`, `tx()`, `tx_raw_bytes()` |
| `BroadcastResult` | Result of broadcast. `txhash`, `raw_response` |
| `PublicKey` | Re-exported from signing. |
| `Signature` | Re-exported from signing. |
| `WalletType` | Re-exported from signing. |
| `TradingKeyClaim` | Agent delegation claim. Re-exported from signing. |
| `VcClaimBuilder` | Builder for `TradingKeyClaim`. Re-exported from signing. |

### Traits

#### `Transport`

```rust
#[async_trait(?Send)]
pub trait Transport: Send + Sync + 'static {
    async fn broadcast_tx(&self, tx_bytes: Vec<u8>) -> Result<BroadcastResult, SdkError>;
    async fn query(&self, path: &str, data: Vec<u8>) -> Result<Vec<u8>, SdkError>;
}
```

#### `MorpheumClient`

```rust
#[async_trait(?Send)]
pub trait MorpheumClient: Send + Sync + 'static {
    fn config(&self) -> &SdkConfig;
    fn transport(&self) -> &dyn Transport;
    async fn broadcast(&self, tx_bytes: Vec<u8>) -> Result<BroadcastResult, SdkError>;
    async fn query(&self, path: &str, data: Vec<u8>) -> Result<Vec<u8>, SdkError>;
}
```

### Configuration

#### `SdkConfig`

| Field | Type | Description |
|-------|------|-------------|
| `rpc_endpoint` | `String` | Primary RPC endpoint |
| `default_chain_id` | `ChainId` | Default chain ID |
| `timeout_secs` | `u64` | Request timeout (default: 60) |
| `user_agent` | `Option<String>` | Custom user-agent |

**Methods:** `new(endpoint, chain_id)`, `builder()`

#### `SdkConfigBuilder`

Fluent builder: `rpc_endpoint()`, `chain_id()`, `timeout_secs()`, `user_agent()`, `build()`

### Transaction Builder

#### `TxBuilder<S: Signer>`

| Method | Description |
|--------|-------------|
| `new(signer)` | Create with signer |
| `chain_id(id)` | Set chain ID |
| `memo(memo)` | Set memo |
| `add_message(msg)` | Add protobuf `Any` message |
| `add_typed_message(type_url, msg)` | Add typed message |
| `with_nonce(nonce)` | Set pre-built nonce |
| `with_nonce_provider(provider)` | Set nonce provider |
| `with_trading_key_claim(claim)` | Attach agent claim |
| `sign()` | Sign and return `SignedTx` |

### Error

#### `SdkError`

| Variant | Description |
|---------|-------------|
| `Signing(SigningError)` | Signing library error |
| `Transport(String)` | Network/transport error |
| `Encode(EncodeError)` | Protobuf encode error |
| `Decode(DecodeError)` | Protobuf decode error |
| `Config(String)` | Configuration error |
| `InvalidInput(String)` | Invalid user input |
| `Other(String)` | Catch-all |

**Constructors:** `transport()`, `config()`, `invalid_input()`, `other()`

### Prelude

```rust
use morpheum_sdk_core::prelude::*;
// Exports: MorpheumClient, SdkConfig, SdkError, AccountId, ChainId, SignedTx,
// Transport, BroadcastResult, TxBuilder, Any, PublicKey, Signature, WalletType,
// TradingKeyClaim, VcClaimBuilder
```

---

## Native SDK (`morpheum-sdk-native`)

Full Rust SDK for CLI, bots, and agents.

### Facade

#### `MorpheumSdk`

| Method | Description |
|--------|-------------|
| `new(rpc_endpoint, chain_id)` | Create with default transport |
| `with_transport(config, transport)` | Create with custom transport |
| `config()` | Get configuration |
| `transport()` | Get transport |

### Constructors

| Function | Description |
|----------|-------------|
| `native(signer: NativeSigner)` | SDK for human wallet |
| `agent(signer: AgentSigner)` | SDK for autonomous agent |

### Re-exports

- **Core:** `core`, `AccountId`, `ChainId`, `SdkConfig`, `SdkError`, `SignedTx`
- **Signing:** `NativeSigner`, `AgentSigner`, `TradingKeyClaim`, `VcClaimBuilder`
- **Modules (feature-gated):** `market`, `vc`, `auth`, `identity`, `agent_registry`, `inference_registry`, `interop`, `job`, `bank`, `staking`

### Prelude

```rust
use morpheum_sdk_native::prelude::*;
// Exports: MorpheumSdk, native, agent, AccountId, ChainId, SdkConfig, SdkError,
// SignedTx, NativeSigner, AgentSigner, TradingKeyClaim, VcClaimBuilder, TxBuilder,
// Any, MarketClient, VcClient, AuthClient, ... (feature-gated)
```

---

## WASM SDK (`morpheum-sdk-wasm`)

Browser-ready bindings for TypeScript/JavaScript.

### `MorpheumSdkWasm`

| Method | Description |
|--------|-------------|
| `new(sentry_url, chain_id)` | Constructor |
| `version` | SDK version (getter) |

### Functions

| Function | Description |
|----------|-------------|
| `set_panic_hook()` | Install panic hook for console errors |
| `version()` | Return SDK version string |

### Re-exports

- **Types:** `AccountId`, `ChainId`, `SdkError`, `SignedTx`, `TradingKeyClaim`, `VcClaimBuilder`
- **Modules (feature-gated):** `market`, `vc`, `auth`

---

## Market Module

**Crate:** `morpheum-sdk-market`  
**Feature (native):** `market`

### `MarketClient`

| Method | Description |
|--------|-------------|
| `new(config, transport)` | Create client |
| `query_market(market_index)` | Query single market |
| `query_markets(limit, offset, status_filter, type_filter)` | Query markets with filters |
| `query_active_markets(limit, offset)` | Query active markets |
| `query_market_stats(market_index, time_range)` | Query market statistics |

### Builders

| Builder | Purpose |
|---------|---------|
| `MarketCreateBuilder` | Create market. `from_address()`, `base_asset_index()`, `quote_asset_index()`, `market_type()`, `orderbook_type()`, `params()`, `governance_proposal_id()`, `build()` |
| `MarketActivateBuilder` | Activate market |
| `MarketSuspendBuilder` | Suspend market |
| `MarketUpdateBuilder` | Update market |
| `ChangeMarketMarginRatioBuilder` | Change margin ratio |

### Types

| Type | Description |
|------|-------------|
| `Market` | Market data |
| `MarketParams` | Market parameters |
| `MarketStats` | Market statistics |
| `MarketType` | Spot, Perp, Future, Option |
| `MarketStatus` | Status enum |
| `MarketCategory` | Category enum |
| `PerpConfig` | Perpetual config |

### Request Types

| Request | `to_any()` | Use with TxBuilder |
|---------|------------|--------------------|
| `CreateMarketRequest` | ✓ | Market creation |
| `ActivateMarketRequest` | ✓ | Market activation |
| `SuspendMarketRequest` | ✓ | Market suspension |
| `UpdateMarketRequest` | ✓ | Market update |
| `ChangeMarketMarginRatioRequest` | ✓ | Margin change |

---

## VC Module

**Crate:** `morpheum-sdk-vc`  
**Feature (native):** `vc`

### `VcClient`

Client for Verifiable Credentials: issue, revoke, query, status.

### Builders

| Builder | Purpose |
|---------|---------|
| `VcIssueBuilder` | Issue VC. `issuer()`, `subject()`, `claims()`, `expiry()`, `issuer_signature()`, `build()` |
| `VcRevokeBuilder` | Revoke VC (issuer-initiated) |
| `VcSelfRevokeBuilder` | Self-revoke (subject-initiated) |
| `UpdateClaimsBuilder` | Update VC claims |
| `VcUpdateParamsBuilder` | Update module params |

### Types

| Type | Description |
|------|-------------|
| `Vc` | Verifiable Credential |
| `VcClaims` | Claims payload |
| `Vp` | Verifiable Presentation |
| `VcStatus` | Status enum |
| `Params` | Module params |
| `ActiveVc` | Active VC record |

---

## Auth Module

**Crate:** `morpheum-sdk-auth`  
**Feature (native):** `auth`

### `AuthClient`

Client for nonce queries, TradingKey management, account state.

### Builders

| Builder | Purpose |
|---------|---------|
| `ApproveTradingKeyBuilder` | Approve trading key |
| `RevokeTradingKeyBuilder` | Revoke trading key |
| `UpdateParamsBuilder` | Update auth params |

### Types

| Type | Description |
|------|-------------|
| `BaseAccount` | Base account |
| `ModuleAccount` | Module account |
| `ModuleCredential` | Module credential |
| `NonceState` | Nonce state |
| `Params` | Auth params |

---

## Other Modules

### Identity (`morpheum-sdk-identity`)

- **Client:** `IdentityClient`
- **Requests:** `RegisterAgentRequest`, `TransferOwnershipRequest`, `UpdateMetadataRequest`, `UpdateStatusRequest`, `BurnAgentRequest`
- **Queries:** `QueryAgentRequest`, `QueryAgentByOwnerRequest`, `QueryMetadataCardRequest`, `QueryAgentStatusRequest`

### Agent Registry (`morpheum-sdk-agent-registry`)

- **Client:** `AgentRegistryClient`

### Inference Registry (`morpheum-sdk-inference-registry`)

- **Client:** `InferenceRegistryClient`

### Interop (`morpheum-sdk-interop`)

- **Client:** `InteropClient`

### Job (`morpheum-sdk-job`)

- **Client:** `JobClient`
- **Builders:** `CreateJobBuilder`, `FundJobBuilder`, `SubmitDeliverableBuilder`, `AttestBuilder`, `ClaimRefundBuilder`

### Bank (`morpheum-sdk-bank`)

- **Client:** `BankClient`

### Staking (`morpheum-sdk-staking`)

- **Client:** `StakingClient`

### Gov (`morpheum-sdk-gov`)

- **Client:** `GovClient`
- **Builders:** `SubmitProposalBuilder`, `ScheduleUpgradeBuilder`, `ProposalDepositBuilder`, `ProposalVoteBuilder`, `CancelProposalBuilder`

### DAO (`morpheum-sdk-dao`)

- **Client:** `DaoClient`
- **Builders:** `CreateDaoBuilder`, `CreateDaoProposalBuilder`, `DaoVoteBuilder`, `SignDaoProposalBuilder`, `DaoDepositBuilder`, `CancelDaoProposalBuilder`, `ExecuteDaoProposalBuilder`, `WithdrawDaoDepositBuilder`

### Directory (`morpheum-sdk-directory`)

- **Client:** `DirectoryClient`
- **Requests:** `UpdateProfileRequest`, `UpdateVisibilityRequest`, `UpdateParamsRequest`

### Reputation (`morpheum-sdk-reputation`)

- **Client:** `ReputationClient`

### Memory (`morpheum-sdk-memory`)

- **Client:** `MemoryClient`
- **Types:** `MemoryEntry`, `MemoryEntryType`, `VectorEmbedding`, `MemoryRoot`, `MemorySnapshot`

### Validation (`morpheum-sdk-validation`)

- **Client:** `ValidationClient`

### Marketplace (`morpheum-sdk-marketplace`)

- **Client:** `MarketplaceClient`

### Upgrade (`morpheum-sdk-upgrade`)

- **Client:** `UpgradeClient`

### Intent (`morpheum-sdk-intent`)

- **Client:** `IntentClient`

---

## Request Pattern

All transaction request types follow this pattern:

```rust
// Build request
let req = SomeBuilder::new()
    .from_address(account_id)
    .some_field(value)
    .build()?;

// Convert to protobuf Any for TxBuilder
let any = req.to_any();

// Add to transaction
let signed = TxBuilder::new(signer)
    .chain_id("morpheum-1")
    .add_message(any)
    .sign()
    .await?;
```

---

## Generating Full API Docs

```bash
cargo doc --no-deps --open
```

For a specific crate:

```bash
cargo doc -p morpheum-sdk-core --no-deps --open
cargo doc -p morpheum-sdk-native --no-deps --open
```

---

## See Also

- [DOCUMENTATION](DOCUMENTATION.md) — Usage guide and examples
- [ARCHITECTURE](ARCHITECTURE.md) — Design and structure
- [CONTRIBUTING](CONTRIBUTING.md) — Contribution guidelines
