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
#[cfg(feature = "relay")]
pub mod relay;
pub mod requests;
pub mod types;

pub use builder::{
    HyperlaneParamsBuilder, SettleGmpPaymentBuilder,
    UpdateGmpParamsBuilder, WarpRouteConfigBuilder, WarpRouteTransferBuilder,
};
pub use client::GmpClient;
pub use requests::{
    QueryGmpParamsRequest, SettleGmpPaymentRequest, UpdateGmpParamsRequest,
    WarpRouteTransferRequest,
};
pub use types::{
    GmpParams, HyperlaneDeliveryStatus, HyperlaneParams, ProtocolInfo,
    SettleGmpPaymentResult, WarpRouteConfig, WarpRouteToken, WarpRouteTransferResult,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
