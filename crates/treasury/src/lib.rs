//! Treasury module for the Morpheum SDK.
//!
//! Provides support for querying protocol reserve states, treasury metrics,
//! individual category reserves, governance parameters, allocation history,
//! as well as sweeping revenue and allocating funds.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::{AllocationHistoryPage, CategoryReserveResult, TreasuryClient};

pub use types::{
    AllocationRecord,
    CategoryReserve,
    ReserveCategory,
    ReservesState,
    TreasuryMetrics,
    TreasuryParams,
};

pub use requests::{
    AllocateFundsRequest,
    QueryAllocationHistoryRequest,
    QueryCategoryReserveRequest,
    QueryParamsRequest,
    QueryReservesStateRequest,
    QueryTreasuryMetricsRequest,
    SweepRevenueRequest,
    UpdateParamsRequest,
};

pub use builder::{AllocateFundsBuilder, SweepRevenueBuilder, UpdateParamsBuilder};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the treasury module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_treasury::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        TreasuryClient,
        ReserveCategory,
        ReservesState,
        TreasuryMetrics,
        TreasuryParams,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the treasury module (synchronized with workspace version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn public_api_compiles_cleanly() {
        #[allow(unused_imports)]
        use prelude::*;
        let _ = VERSION;
    }
}
