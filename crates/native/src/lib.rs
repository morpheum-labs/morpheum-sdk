//! Morpheum Native SDK — Full Rust SDK for CLI tools, trading bots, and autonomous AI agents.
//!
//! This is the main entry point for the native SDK. It provides:
//! - `MorpheumSdk` — the central, ergonomic facade
//! - Convenience constructors: `native()` and `agent()`
//! - Feature-gated access to all modules (`market`, `vc`, `auth`)
//! - Seamless integration with the official `morpheum-signing-native` library
//!
//! Recommended usage:
//! ```rust,ignore
//! use morpheum_sdk_native::prelude::*;
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;

// ==================== RE-EXPORTS ====================

// Core SDK foundation
pub use morpheum_sdk_core as core;

// Official signing library (native version with full crypto + BIP-39)
pub use morpheum_signing_native as signing;

// Feature-gated module clients
#[cfg(feature = "identity")]
pub use morpheum_sdk_identity as identity;

#[cfg(feature = "market")]
pub use morpheum_sdk_market as market;

#[cfg(feature = "vc")]
pub use morpheum_sdk_vc as vc;

#[cfg(feature = "auth")]
pub use morpheum_sdk_auth as auth;

#[cfg(feature = "agentreg")]
pub use morpheum_sdk_agentreg as agentreg;

#[cfg(feature = "inferreg")]
pub use morpheum_sdk_inferreg as inferreg;

#[cfg(feature = "interop")]
pub use morpheum_sdk_interop as interop;

#[cfg(feature = "job")]
pub use morpheum_sdk_job as job;

#[cfg(feature = "bank")]
pub use morpheum_sdk_bank as bank;

#[cfg(feature = "staking")]
pub use morpheum_sdk_staking as staking;

#[cfg(feature = "reputation")]
pub use morpheum_sdk_reputation as reputation;

#[cfg(feature = "validation")]
pub use morpheum_sdk_validation as validation;

#[cfg(feature = "memory")]
pub use morpheum_sdk_memory as memory;

#[cfg(feature = "intent")]
pub use morpheum_sdk_intent as intent;

#[cfg(feature = "marketplace")]
pub use morpheum_sdk_marketplace as marketplace;

#[cfg(feature = "directory")]
pub use morpheum_sdk_directory as directory;

#[cfg(feature = "x402")]
pub use morpheum_sdk_x402 as x402;

#[cfg(feature = "gmp")]
pub use morpheum_sdk_gmp as gmp;

#[cfg(feature = "evm")]
pub use morpheum_sdk_evm as evm;

#[cfg(feature = "fundingrate")]
pub use morpheum_sdk_fundingrate as fundingrate;

#[cfg(feature = "insurance")]
pub use morpheum_sdk_insurance as insurance;

#[cfg(feature = "kline")]
pub use morpheum_sdk_kline as kline;

#[cfg(feature = "liquidity")]
pub use morpheum_sdk_liquidity as liquidity;

#[cfg(feature = "markprice")]
pub use morpheum_sdk_markprice as markprice;

#[cfg(feature = "osa")]
pub use morpheum_sdk_osa as osa;

#[cfg(feature = "outcomefeed")]
pub use morpheum_sdk_outcomefeed as outcomefeed;

#[cfg(feature = "prediction")]
pub use morpheum_sdk_prediction as prediction;

#[cfg(feature = "pricefeed")]
pub use morpheum_sdk_pricefeed as pricefeed;

#[cfg(feature = "risk")]
pub use morpheum_sdk_risk as risk;

#[cfg(feature = "token")]
pub use morpheum_sdk_token as token;

#[cfg(feature = "treasury")]
pub use morpheum_sdk_treasury as treasury;

#[cfg(feature = "twap")]
pub use morpheum_sdk_twap as twap;

#[cfg(feature = "vault")]
pub use morpheum_sdk_vault as vault;

#[cfg(feature = "vesting")]
pub use morpheum_sdk_vesting as vesting;

#[cfg(feature = "svm")]
pub use morpheum_sdk_svm as svm;

#[cfg(feature = "bondingcurve")]
pub use morpheum_sdk_bondingcurve as bondingcurve;

#[cfg(feature = "bucket")]
pub use morpheum_sdk_bucket as bucket;

#[cfg(feature = "clmm")]
pub use morpheum_sdk_clmm as clmm;

#[cfg(feature = "clmmgrad")]
pub use morpheum_sdk_clmmgrad as clmmgrad;

#[cfg(feature = "clob")]
pub use morpheum_sdk_clob as clob;

#[cfg(feature = "cosmwasm")]
pub use morpheum_sdk_cosmwasm as cosmwasm;

#[cfg(feature = "position")]
pub use morpheum_sdk_position as position;

#[cfg(feature = "ws")]
pub use morpheum_sdk_ws as ws;

// Re-export commonly used core types
pub use core::{
    AccountId,
    ChainId,
    SdkConfig,
    SdkError,
    SignedTx,
};

// Re-export commonly used signing types
pub use signing::{
    AgentSigner,
    NativeSigner,
};

// Claim types are in signing-core, re-exported through native
pub use signing::claim::{TradingKeyClaim, VcClaimBuilder};

// ==================== gRPC TRANSPORT ====================

#[cfg(feature = "grpc")]
pub mod grpc_transport;

#[cfg(feature = "grpc")]
pub use grpc_transport::GrpcTransport;

// ==================== PLACEHOLDER TRANSPORT ====================

/// A placeholder transport that returns errors for all operations.
///
/// Used as the default when no concrete transport (gRPC, HTTP, etc.) is provided.
/// In production, replace with a real transport implementation.
struct PlaceholderTransport;

#[async_trait(?Send)]
impl core::Transport for PlaceholderTransport {
    async fn broadcast_tx(&self, _tx_bytes: Vec<u8>) -> Result<core::BroadcastResult, SdkError> {
        Err(SdkError::transport(
            "no transport configured — provide a concrete transport implementation",
        ))
    }

    async fn query(&self, _path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
        Err(SdkError::transport(
            "no transport configured — provide a concrete transport implementation",
        ))
    }
}

// ==================== MAIN FACADE ====================

/// The main Morpheum SDK facade for native Rust applications.
///
/// This is the recommended entry point for most users. It holds configuration
/// and provides access to all enabled module clients.
pub struct MorpheumSdk {
    config: SdkConfig,
    transport: Box<dyn core::Transport>,
}

impl MorpheumSdk {
    /// Creates a new SDK with the given RPC endpoint and default chain ID.
    pub fn new(rpc_endpoint: impl Into<String>, chain_id: impl Into<ChainId>) -> Self {
        let config = SdkConfig::new(rpc_endpoint, chain_id);
        Self {
            config,
            transport: Box::new(PlaceholderTransport),
        }
    }

    /// Creates a new SDK with a custom transport and configuration.
    pub fn with_transport(
        config: SdkConfig,
        transport: Box<dyn core::Transport>,
    ) -> Self {
        Self { config, transport }
    }

    /// Returns the current SDK configuration.
    pub fn config(&self) -> &SdkConfig {
        &self.config
    }

    /// Returns a reference to the underlying transport.
    pub fn transport(&self) -> &dyn core::Transport {
        &*self.transport
    }
}

// ==================== CONVENIENCE CONSTRUCTORS ====================

/// Creates a new SDK instance using a `NativeSigner` (recommended for humans).
///
/// This is a convenience function. The signer is **not** stored in the SDK —
/// it is passed to `TxBuilder` per-transaction for maximum flexibility.
pub fn native(signer: NativeSigner) -> MorpheumSdk {
    let _ = signer; // Signer is used in TxBuilder, not stored in SDK
    MorpheumSdk::new("https://sentry.morpheum.xyz", "morpheum-1")
}

/// Creates a new SDK instance using an `AgentSigner` (recommended for autonomous agents).
pub fn agent(signer: AgentSigner) -> MorpheumSdk {
    let _ = signer; // Signer is used in TxBuilder, not stored in SDK
    MorpheumSdk::new("https://sentry.morpheum.xyz", "morpheum-1")
}

// ==================== PRELUDE ====================

/// Recommended prelude for the native SDK.
///
/// Most users should start with:
/// ```rust,ignore
/// use morpheum_sdk_native::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        MorpheumSdk,
        native,
        agent,
        AccountId,
        ChainId,
        SdkConfig,
        SdkError,
        SignedTx,
        NativeSigner,
        AgentSigner,
        TradingKeyClaim,
        VcClaimBuilder,
    };

    // Transaction builder and the canonical `Any` type for constructing messages.
    pub use super::core::builder::TxBuilder;
    pub use super::core::prelude::Any;

    // Feature-gated module re-exports
    #[cfg(feature = "market")]
    pub use super::market::MarketClient;

    #[cfg(feature = "vc")]
    pub use super::vc::VcClient;

    #[cfg(feature = "auth")]
    pub use super::auth::AuthClient;

    #[cfg(feature = "agentreg")]
    pub use super::agentreg::AgentRegistryClient;

    #[cfg(feature = "inferreg")]
    pub use super::inferreg::InferenceRegistryClient;

    #[cfg(feature = "interop")]
    pub use super::interop::InteropClient;

    #[cfg(feature = "job")]
    pub use super::job::JobClient;

    #[cfg(feature = "bank")]
    pub use super::bank::BankClient;

    #[cfg(feature = "staking")]
    pub use super::staking::StakingClient;

    #[cfg(feature = "fundingrate")]
    pub use super::fundingrate::FundingRateClient;

    #[cfg(feature = "insurance")]
    pub use super::insurance::InsuranceClient;

    #[cfg(feature = "kline")]
    pub use super::kline::KlineClient;

    #[cfg(feature = "liquidity")]
    pub use super::liquidity::LiquidityClient;

    #[cfg(feature = "markprice")]
    pub use super::markprice::MarkPriceClient;

    #[cfg(feature = "osa")]
    pub use super::osa::OsaClient;

    #[cfg(feature = "outcomefeed")]
    pub use super::outcomefeed::OutcomeFeedClient;

    #[cfg(feature = "prediction")]
    pub use super::prediction::PredictionClient;

    #[cfg(feature = "pricefeed")]
    pub use super::pricefeed::PriceFeedClient;

    #[cfg(feature = "risk")]
    pub use super::risk::RiskClient;

    #[cfg(feature = "token")]
    pub use super::token::TokenClient;

    #[cfg(feature = "treasury")]
    pub use super::treasury::TreasuryClient;

    #[cfg(feature = "twap")]
    pub use super::twap::TwapClient;

    #[cfg(feature = "vault")]
    pub use super::vault::VaultClient;

    #[cfg(feature = "vesting")]
    pub use super::vesting::VestingClient;

    #[cfg(feature = "grpc")]
    pub use super::GrpcTransport;

    #[cfg(feature = "bondingcurve")]
    pub use super::bondingcurve::BondingCurveClient;

    #[cfg(feature = "bucket")]
    pub use super::bucket::BucketClient;

    #[cfg(feature = "clmm")]
    pub use super::clmm::ClmmClient;

    #[cfg(feature = "clmmgrad")]
    pub use super::clmmgrad::ClmmGradClient;

    #[cfg(feature = "clob")]
    pub use super::clob::ClobClient;

    #[cfg(feature = "position")]
    pub use super::position::PositionClient;

    #[cfg(feature = "ws")]
    pub use super::ws::WsClient;
}

// Current version of the native SDK
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn sdk_creates_cleanly() {
        let sdk = MorpheumSdk::new("https://test.morpheum.xyz", "morpheum-test-1");
        assert_eq!(sdk.config().default_chain_id.as_str(), "morpheum-test-1");
    }

    #[test]
    fn convenience_functions_work() {
        let _sdk = native(NativeSigner::from_seed(&[0u8; 32]));
        let _sdk2 = agent(AgentSigner::new(&[0u8; 32], signing::types::AccountId([0u8; 32]), None));
    }

    #[test]
    fn prelude_compiles_cleanly() {
        #[allow(unused_imports)]
        use prelude::*;
        let _ = VERSION;
    }
}
