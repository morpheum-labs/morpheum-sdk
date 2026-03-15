//! # morpheum-sdk-marketplace
//!
//! Marketplace module client for the Morpheum SDK.
//!
//! This crate provides:
//! - **Domain types**: `AgentListing`, `Bid`, `EscrowState`, `EvaluationReport`,
//!   `ListingType`, `ListingStatus`, `RevenueShareConfig`, `Params`.
//! - **Request/response wrappers**: `ListAgentRequest`, `PlaceBidRequest`,
//!   `AcceptBidRequest`, `RequestEvaluationRequest`,
//!   plus all query request/response pairs.
//! - **Fluent builders**: `ListAgentBuilder`, `PlaceBidBuilder`,
//!   `AcceptBidBuilder`, `RequestEvaluationBuilder`.
//! - **Client**: `MarketplaceClient` with high-level query methods.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use morpheum_sdk_marketplace::prelude::*;
//!
//! // Build a list-agent transaction
//! let request = ListAgentBuilder::new()
//!     .agent_hash("agent-abc")
//!     .seller_agent_hash("seller-xyz")
//!     .listing_type(ListingType::FullOwnership)
//!     .price_usd(1_000_000)
//!     .metadata_hash("meta-hash")
//!     .seller_signature(sig_bytes)
//!     .build()?;
//!
//! let any = request.to_any();
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
)]

extern crate alloc;

pub mod builder;
pub mod client;
pub mod requests;
pub mod types;

// Re-export core domain types at crate root for convenience
pub use builder::{
    AcceptBidBuilder, ListAgentBuilder, PlaceBidBuilder, RequestEvaluationBuilder,
};
pub use client::MarketplaceClient;
pub use types::{
    AgentListing, Bid, EscrowState, EvaluationReport, ListingStatus, ListingType, Params,
    RevenueShareConfig,
};

/// Prelude module — import everything you need with a single `use`.
pub mod prelude {
    pub use crate::builder::{
        AcceptBidBuilder, ListAgentBuilder, PlaceBidBuilder, RequestEvaluationBuilder,
    };
    pub use crate::client::MarketplaceClient;
    pub use crate::requests::{
        AcceptBidRequest, AcceptBidResponse, ListAgentRequest, ListAgentResponse,
        PlaceBidRequest, PlaceBidResponse, QueryActiveListingsRequest, QueryActiveListingsResponse,
        QueryBidsByListingRequest, QueryBidsByListingResponse, QueryListingRequest,
        QueryListingResponse, QueryListingsRequest, QueryListingsResponse, QueryParamsRequest,
        QueryParamsResponse, RequestEvaluationRequest, RequestEvaluationResponse,
    };
    pub use crate::types::{
        AgentListing, Bid, EscrowState, EvaluationReport, ListingStatus, ListingType, Params,
        RevenueShareConfig,
    };
}
