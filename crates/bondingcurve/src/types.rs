//! Domain types for the bonding-curve module.
//!
//! Covers curve configuration, per-token state, prediction enhancements,
//! and on-chain events (buy, sell, graduation).

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::bonding_curve::v1 as proto;

// ====================== ENUMS ======================

/// Bonding-curve formula type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CurveType {
    Unspecified,
    ConstantProduct,
    Linear,
}

impl From<i32> for CurveType {
    fn from(v: i32) -> Self {
        match v { 1 => Self::ConstantProduct, 2 => Self::Linear, _ => Self::Unspecified }
    }
}

impl From<CurveType> for i32 {
    fn from(t: CurveType) -> Self {
        match t { CurveType::Unspecified => 0, CurveType::ConstantProduct => 1, CurveType::Linear => 2 }
    }
}

/// Lifecycle status of a bonding curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CurveStatus {
    Unspecified,
    Active,
    Graduating,
    Completed,
}

impl From<i32> for CurveStatus {
    fn from(v: i32) -> Self {
        match v { 1 => Self::Active, 2 => Self::Graduating, 3 => Self::Completed, _ => Self::Unspecified }
    }
}

impl From<CurveStatus> for i32 {
    fn from(s: CurveStatus) -> Self {
        match s {
            CurveStatus::Unspecified => 0, CurveStatus::Active => 1,
            CurveStatus::Graduating => 2, CurveStatus::Completed => 3,
        }
    }
}

/// LP anti-rug strategy applied at graduation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LpAntiRugStrategy {
    Unspecified,
    Immediate,
    Timelock,
    AfterClobMigration,
    Fractional,
}

impl From<i32> for LpAntiRugStrategy {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Immediate, 2 => Self::Timelock,
            3 => Self::AfterClobMigration, 4 => Self::Fractional,
            _ => Self::Unspecified,
        }
    }
}

impl From<LpAntiRugStrategy> for i32 {
    fn from(s: LpAntiRugStrategy) -> Self {
        match s {
            LpAntiRugStrategy::Unspecified => 0, LpAntiRugStrategy::Immediate => 1,
            LpAntiRugStrategy::Timelock => 2, LpAntiRugStrategy::AfterClobMigration => 3,
            LpAntiRugStrategy::Fractional => 4,
        }
    }
}

// ====================== HELPERS ======================

fn ts_secs(ts: Option<morpheum_proto::google::protobuf::Timestamp>) -> u64 {
    ts.map(|t| t.seconds as u64).unwrap_or(0)
}

// ====================== PREDICTION ======================

/// Prediction enhancement mode for a bonding-curve token.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PredictionMode {
    /// Default — deterministic bonding-curve only.
    None,
    /// Base curve active; prediction outcome provides a boost.
    Boost(PredictionFeed),
    /// Trading blocked until prediction resolves YES.
    Gated(PredictionFeed),
}

/// Reference to an outcomefeed prediction.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PredictionFeed {
    pub feed_id: String,
    pub expected_resolution_blocks: Option<u64>,
    pub min_reputation_hint: Option<u64>,
}

impl From<proto::PredictionFeed> for PredictionFeed {
    fn from(p: proto::PredictionFeed) -> Self {
        Self {
            feed_id: p.feed_id,
            expected_resolution_blocks: p.expected_resolution_blocks,
            min_reputation_hint: p.min_reputation_hint,
        }
    }
}

/// Optional metadata displayed alongside a prediction-enhanced token.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PredictionMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub external_view_url: Option<String>,
}

impl From<proto::PredictionMetadata> for PredictionMetadata {
    fn from(p: proto::PredictionMetadata) -> Self {
        Self { title: p.title, description: p.description, external_view_url: p.external_view_url }
    }
}

/// Full prediction enhancement configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PredictionEnhancement {
    pub mode: PredictionMode,
    pub metadata: Option<PredictionMetadata>,
}

impl From<proto::PredictionEnhancement> for PredictionEnhancement {
    fn from(p: proto::PredictionEnhancement) -> Self {
        use proto::prediction_enhancement::Variant;
        let mode = match p.variant {
            Some(Variant::Boost(f)) => PredictionMode::Boost(f.into()),
            Some(Variant::Gated(f)) => PredictionMode::Gated(f.into()),
            _ => PredictionMode::None,
        };
        Self { mode, metadata: p.metadata.map(Into::into) }
    }
}

// ====================== PARAMS ======================

/// A reputation-bond tier for token launches.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReputationBondTier {
    pub min_reputation: u64,
    pub bond_amount_satoshi: String,
    pub refund_percentage: u32,
}

impl From<proto::ReputationBondTier> for ReputationBondTier {
    fn from(p: proto::ReputationBondTier) -> Self {
        Self {
            min_reputation: p.min_reputation,
            bond_amount_satoshi: p.bond_amount_satoshi,
            refund_percentage: p.refund_percentage,
        }
    }
}

/// Module-level governance parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BondingCurveParams {
    pub min_reputation_for_launch: u64,
    pub reputation_bond_tiers: Vec<ReputationBondTier>,
    pub graduation_burn_bps: u32,
    pub lp_anti_rug_strategy: LpAntiRugStrategy,
    pub lp_timelock_blocks: u64,
    pub graduation_cooldown_blocks: u64,
    pub utility_multiplier: String,
    pub min_revenue_per_hook: String,
    pub max_utility_points_per_day: String,
    pub non_morm_quote_fee_bps: u32,
    pub min_reputation_for_gated_mode: u64,
    pub max_gated_duration_blocks: u64,
}

impl From<proto::Params> for BondingCurveParams {
    fn from(p: proto::Params) -> Self {
        Self {
            min_reputation_for_launch: p.min_reputation_for_launch,
            reputation_bond_tiers: p.reputation_bond_tiers.into_iter().map(Into::into).collect(),
            graduation_burn_bps: p.graduation_burn_bps,
            lp_anti_rug_strategy: LpAntiRugStrategy::from(p.lp_anti_rug_strategy),
            lp_timelock_blocks: p.lp_timelock_blocks,
            graduation_cooldown_blocks: p.graduation_cooldown_blocks,
            utility_multiplier: p.utility_multiplier,
            min_revenue_per_hook: p.min_revenue_per_hook,
            max_utility_points_per_day: p.max_utility_points_per_day,
            non_morm_quote_fee_bps: p.non_morm_quote_fee_bps,
            min_reputation_for_gated_mode: p.min_reputation_for_gated_mode,
            max_gated_duration_blocks: p.max_gated_duration_blocks,
        }
    }
}

// ====================== STATE ======================

/// Persistent per-token bonding-curve state.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BondingCurveState {
    pub token_index: u64,
    pub creator_did: Option<String>,
    pub status: CurveStatus,
    pub curve_type: CurveType,
    pub k: String,
    pub max_supply: String,
    pub minted_tokens: String,
    pub escrow_morm: String,
    pub utility_points: String,
    pub utility_points_day_block: Option<u64>,
    pub utility_points_today: String,
    pub graduation_mcap: String,
    pub early_grad_mcap: String,
    pub utility_multiplier: String,
    pub threshold_met_at: Option<u64>,
    pub prediction_enhancement: Option<PredictionEnhancement>,
    pub created_at: u64,
    pub graduated_at: Option<u64>,
}

impl From<proto::BondingCurveState> for BondingCurveState {
    fn from(p: proto::BondingCurveState) -> Self {
        Self {
            token_index: p.token_index,
            creator_did: p.creator_did,
            status: CurveStatus::from(p.status),
            curve_type: CurveType::from(p.curve_type),
            k: p.k,
            max_supply: p.max_supply,
            minted_tokens: p.minted_tokens,
            escrow_morm: p.escrow_morm,
            utility_points: p.utility_points,
            utility_points_day_block: p.utility_points_day_block,
            utility_points_today: p.utility_points_today,
            graduation_mcap: p.graduation_mcap,
            early_grad_mcap: p.early_grad_mcap,
            utility_multiplier: p.utility_multiplier,
            threshold_met_at: p.threshold_met_at,
            prediction_enhancement: p.prediction_enhancement.map(Into::into),
            created_at: ts_secs(p.created_at),
            graduated_at: p.graduated_at.map(|t| t.seconds as u64),
        }
    }
}

// ====================== EVENTS ======================

/// Emitted when a buy is executed on the bonding curve.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuyExecuted {
    pub token_index: u64,
    pub buyer: String,
    pub token_amount: String,
    pub morm_amount: String,
    pub price: String,
    pub escrow_delta: String,
    pub timestamp: u64,
}

impl From<proto::BuyExecuted> for BuyExecuted {
    fn from(p: proto::BuyExecuted) -> Self {
        Self {
            token_index: p.token_index, buyer: p.buyer,
            token_amount: p.token_amount, morm_amount: p.morm_amount,
            price: p.price, escrow_delta: p.escrow_delta,
            timestamp: ts_secs(p.timestamp),
        }
    }
}

/// Emitted when a sell is executed on the bonding curve.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SellExecuted {
    pub token_index: u64,
    pub seller: String,
    pub token_amount: String,
    pub morm_amount: String,
    pub price: String,
    pub escrow_delta: String,
    pub timestamp: u64,
}

impl From<proto::SellExecuted> for SellExecuted {
    fn from(p: proto::SellExecuted) -> Self {
        Self {
            token_index: p.token_index, seller: p.seller,
            token_amount: p.token_amount, morm_amount: p.morm_amount,
            price: p.price, escrow_delta: p.escrow_delta,
            timestamp: ts_secs(p.timestamp),
        }
    }
}

/// Emitted when the graduation market-cap threshold is reached.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GraduationThresholdReached {
    pub token_index: u64,
    pub current_mcap: String,
    pub reason: String,
    pub at: u64,
}

impl From<proto::GraduationThresholdReached> for GraduationThresholdReached {
    fn from(p: proto::GraduationThresholdReached) -> Self {
        Self {
            token_index: p.token_index, current_mcap: p.current_mcap,
            reason: p.reason, at: ts_secs(p.at),
        }
    }
}

/// Emitted when graduation to CLAMM is complete.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GraduationComplete {
    pub token_index: u64,
    pub pool_id: String,
    pub mcap_at_grad: String,
    pub burned_morm: String,
    pub lp_strategy_applied: String,
    pub graduated_at: u64,
}

impl From<proto::GraduationComplete> for GraduationComplete {
    fn from(p: proto::GraduationComplete) -> Self {
        Self {
            token_index: p.token_index, pool_id: p.pool_id,
            mcap_at_grad: p.mcap_at_grad, burned_morm: p.burned_morm,
            lp_strategy_applied: p.lp_strategy_applied,
            graduated_at: ts_secs(p.graduated_at),
        }
    }
}

/// Emitted when a prediction enhancement is activated on a token.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PredictionEnhancementActivated {
    pub token_index: u64,
    pub mode: String,
    pub feed_id: String,
    pub activated_at: u64,
}

impl From<proto::PredictionEnhancementActivated> for PredictionEnhancementActivated {
    fn from(p: proto::PredictionEnhancementActivated) -> Self {
        Self {
            token_index: p.token_index, mode: p.mode,
            feed_id: p.feed_id, activated_at: ts_secs(p.activated_at),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_type_roundtrip() {
        for ct in [CurveType::ConstantProduct, CurveType::Linear] {
            let v: i32 = ct.into();
            assert_eq!(ct, CurveType::from(v));
        }
    }

    #[test]
    fn curve_status_roundtrip() {
        for cs in [CurveStatus::Active, CurveStatus::Graduating, CurveStatus::Completed] {
            let v: i32 = cs.into();
            assert_eq!(cs, CurveStatus::from(v));
        }
    }

    #[test]
    fn lp_strategy_roundtrip() {
        for s in [LpAntiRugStrategy::Immediate, LpAntiRugStrategy::Timelock,
                   LpAntiRugStrategy::AfterClobMigration, LpAntiRugStrategy::Fractional] {
            let v: i32 = s.into();
            assert_eq!(s, LpAntiRugStrategy::from(v));
        }
    }

    #[test]
    fn bonding_curve_state_from_proto() {
        let p = proto::BondingCurveState {
            token_index: 42,
            creator_did: Some("did:morpheum:creator".into()),
            status: 1,
            curve_type: 1,
            k: "1000000".into(),
            max_supply: "100000000".into(),
            minted_tokens: "50000000".into(),
            escrow_morm: "500000".into(),
            utility_points: "0".into(),
            utility_points_day_block: None,
            utility_points_today: "0".into(),
            graduation_mcap: "69000".into(),
            early_grad_mcap: "42000".into(),
            utility_multiplier: "1".into(),
            threshold_met_at: None,
            prediction_enhancement: None,
            created_at: Some(morpheum_proto::google::protobuf::Timestamp { seconds: 1_700_000_000, nanos: 0 }),
            graduated_at: None,
        };
        let state: BondingCurveState = p.into();
        assert_eq!(state.token_index, 42);
        assert_eq!(state.status, CurveStatus::Active);
        assert_eq!(state.curve_type, CurveType::ConstantProduct);
        assert_eq!(state.created_at, 1_700_000_000);
        assert!(state.graduated_at.is_none());
    }

    #[test]
    fn prediction_enhancement_boost_from_proto() {
        use proto::prediction_enhancement::Variant;
        let p = proto::PredictionEnhancement {
            variant: Some(Variant::Boost(proto::PredictionFeed {
                feed_id: "feed-123".into(),
                expected_resolution_blocks: Some(100),
                min_reputation_hint: None,
            })),
            metadata: Some(proto::PredictionMetadata {
                title: Some("Will BTC hit 100k?".into()),
                description: None,
                external_view_url: None,
            }),
        };
        let pe: PredictionEnhancement = p.into();
        match &pe.mode {
            PredictionMode::Boost(f) => assert_eq!(f.feed_id, "feed-123"),
            _ => panic!("expected Boost"),
        }
        assert_eq!(pe.metadata.as_ref().unwrap().title.as_deref(), Some("Will BTC hit 100k?"));
    }
}
