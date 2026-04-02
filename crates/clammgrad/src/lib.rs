//! CLAMM Graduation module for the Morpheum SDK.
//!
//! Provides support for the CLAMM-to-CLOB venue transition lifecycle:
//! initiating graduation, cancellation, querying graduation state, listing eligible tokens,
//! and reading module parameters.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::{ClammGradClient, EligibleTokensPage};

pub use types::{
    ClammGraduationParams,
    GraduationCheckpoint,
    GraduationComplete,
    GraduationFailed,
    GraduationInitiated,
    GraduationRollbackAttempted,
    GraduationState,
    GraduationStatus,
    LiquidityDrained,
    PerpMarketCreated,
    SpotMarketCreated,
};

pub use requests::{
    CancelGraduationRequest,
    GetGraduationStateRequest,
    GetParamsRequest,
    InitiateGraduationRequest,
    ListEligibleTokensRequest,
    UpdateParamsRequest,
};

pub use builder::{
    CancelGraduationBuilder,
    InitiateGraduationBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the CLAMM Graduation module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_clammgrad::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        ClammGradClient,
        GraduationState,
        GraduationStatus,
        InitiateGraduationBuilder,
        CancelGraduationBuilder,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the CLAMM Graduation module (synchronized with workspace version).
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
