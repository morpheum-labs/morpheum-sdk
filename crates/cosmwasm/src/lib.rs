//! Morpheum SDK — CosmWasm contract interaction.
//!
//! Provides builders and helpers for interacting with CosmWasm contracts
//! deployed on Morpheum's embedded CosmWasm VM:
//!
//! - **`builder`** — Fluent builders (`ExecuteContractBuilder`, `InstantiateContractBuilder`,
//!   `StoreCodeBuilder`, `WarpRouteTransferBuilder`)
//! - **`client`** — `CosmWasmClient` query methods (smart contract state, raw state, contract info)
//! - **`types`** — Domain types (`ContractInfo`, `CosmWasmError`)
//! - **`requests`** — Request/response types with `to_any()` serialization
//!
//! Messages are encoded in Cosmos wire format and submitted via the GMP protocol
//! bridge (`gmp_compat`), which translates them to Morpheum-native operations.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod builder;
pub mod client;
#[cfg(feature = "grpc")]
pub mod grpc;
pub mod requests;
pub mod types;

pub use builder::{
    ExecuteContractBuilder, InstantiateContractBuilder, StoreCodeBuilder,
    WarpRouteTransferBuilder,
};
pub use client::CosmWasmClient;
pub use requests::{
    ExecuteContractRequest, InstantiateContractRequest, QueryRawRequest,
    QuerySmartRequest, StoreCodeRequest,
};
pub use types::{ContractInfo, CosmWasmError};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
