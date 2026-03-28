//! Token module for the Morpheum SDK.
//!
//! Provides support for creating tokens, managing tradability and metadata,
//! configuring programmable hooks (MWVM), setting cross-chain origin metadata,
//! querying token info, and consuming streaming token events.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::{ProgrammableLogicResult, TokenClient, TokensPage};

pub use types::{
    HookDisabled,
    HookExecuted,
    HookPoint,
    ProgrammableLogicConfig,
    SimulateHookResult,
    TokenCreated,
    TokenEvent,
    TokenInfo,
    TokenMetadataUpdated,
    TokenOriginSummary,
    TokenSummary,
    TokenTradabilityChanged,
};

pub use requests::{
    CreateTokenRequest,
    DisableHookRequest,
    GetProgrammableLogicRequest,
    GetTokenInfoRequest,
    ListTokensRequest,
    SetOriginMetadataRequest,
    SetTradableRequest,
    SimulateHookRequest,
    UpdateMetadataRequest,
};

pub use builder::{
    CreateTokenBuilder,
    DisableHookBuilder,
    SetOriginMetadataBuilder,
    SetTradableBuilder,
    UpdateMetadataBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the token module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_token::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        TokenClient,
        TokenInfo,
        TokenSummary,
        HookPoint,
        ProgrammableLogicConfig,
        TokenEvent,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the token module (synchronized with workspace version).
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
