//! Morpheum SDK — CCTP contract interaction.
//!
//! Provides canonical types and typed query helpers for the `hpl-cctp-handler`
//! CosmWasm contract. Types are re-exported directly from the contract crate
//! (via `library` feature), guaranteeing a single source of truth and
//! eliminating type duplication across consumers.
//!
//! - **`types`** — Re-exported contract types (`PendingTransfer`, `ConfigResponse`,
//!   `PostMintHookMsg`, message enums)
//! - **`client`** — Typed query helpers (`query_pending_transfers`, `query_config`,
//!   `query_pending_by_nonce`, `query_routes`)
//! - **`error`** — `CctpError` for CCTP-specific failures

#![forbid(unsafe_code)]

pub mod builder;
pub mod client;
pub mod error;
#[cfg(feature = "iris")]
pub mod fulfill;
pub mod types;

pub use builder::{
    EnrollRemoteRouterBuilder, FulfillCctpBuilder, SetPostMintHookBuilder, UpdateAttestersBuilder,
};
pub use client::{query_config, query_pending_by_nonce, query_pending_transfers, query_routes};
pub use error::CctpError;
pub use types::{
    ConfigResponse, ExecuteMsg, HandleMsg, HexBinary, InstantiateMsg, PendingTransfer,
    PendingTransferResponse, PendingTransfersResponse, PostMintHookMsg, QueryMsg, RouteResponse,
    RoutesResponse,
};

pub use hpl_cctp_handler as handler;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
