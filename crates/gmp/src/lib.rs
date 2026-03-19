//! Morpheum SDK — GMP (General Message Passing) module.
//!
//! Provides builders, query client, and domain types for cross-chain operations
//! via Hyperlane, Axelar, Wormhole, and LayerZero.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod builder;
pub mod client;
pub mod requests;
pub mod types;

pub use builder::{
    HyperlaneParamsBuilder, ProcessHyperlaneMessageBuilder, SettleGmpPaymentBuilder,
    UpdateGmpParamsBuilder, WarpRouteConfigBuilder, WarpRouteTransferBuilder,
};
pub use client::GmpClient;
pub use requests::{
    ProcessHyperlaneMessageRequest, QueryGmpParamsRequest, QueryHyperlaneDeliveryRequest,
    QueryHyperlaneNonceRequest, SettleGmpPaymentRequest, UpdateGmpParamsRequest,
    WarpRouteTransferRequest,
};
pub use types::{
    GmpParams, HyperlaneParams, ProcessHyperlaneResult, ProtocolInfo,
    SettleGmpPaymentResult, WarpRouteConfig, WarpRouteToken, WarpRouteTransferResult,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
