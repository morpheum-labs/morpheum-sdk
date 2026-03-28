//! Vesting module for the Morpheum SDK.
//!
//! Provides support for creating vesting schedules (linear, cliff+linear, step),
//! claiming releasable tokens, revoking schedules, querying summaries
//! and entries, and receiving streaming vesting events.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::{VestingClient, VestingEntriesPage, VestingEntryResult};

pub use types::{
    ScheduleType,
    VestingCategory,
    VestingEntry,
    VestingEvent,
    VestingParams,
    VestingSummary,
};

pub use requests::{
    ClaimRequest,
    CreateVestingRequest,
    QueryParamsRequest,
    QueryVestingEntriesRequest,
    QueryVestingEntryRequest,
    QueryVestingSummaryRequest,
    RevokeVestingRequest,
    UpdateParamsRequest,
};

pub use builder::{
    ClaimBuilder,
    CreateVestingBuilder,
    RevokeVestingBuilder,
    UpdateParamsBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the vesting module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_vesting::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        VestingClient,
        VestingEntry,
        VestingSummary,
        ScheduleType,
        VestingCategory,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the vesting module (synchronized with workspace version).
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
