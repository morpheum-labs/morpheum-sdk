//! Domain types for the Marketplace module.
//!
//! These are clean, idiomatic Rust representations of the marketplace protobuf
//! messages. They provide type safety, ergonomic APIs, and full round-trip
//! conversion to/from protobuf while remaining strictly `no_std` compatible.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::marketplace::v1 as proto;

// ====================== LISTING TYPE ======================

/// The type of an agent listing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ListingType {
    /// Full ownership transfer.
    #[default]
    FullOwnership = 0,
    /// Co-ownership arrangement.
    CoOwnership = 1,
    /// Rental/lease access.
    Rental = 2,
    /// Evaluation access only.
    EvaluationOnly = 3,
}

impl ListingType {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::CoOwnership,
            2 => Self::Rental,
            3 => Self::EvaluationOnly,
            _ => Self::FullOwnership,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for ListingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FullOwnership => f.write_str("FULL_OWNERSHIP"),
            Self::CoOwnership => f.write_str("CO_OWNERSHIP"),
            Self::Rental => f.write_str("RENTAL"),
            Self::EvaluationOnly => f.write_str("EVALUATION_ONLY"),
        }
    }
}

// ====================== LISTING STATUS ======================

/// The status of a listing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ListingStatus {
    /// Listing is active and accepting bids.
    #[default]
    Active = 0,
    /// Listing has been sold.
    Sold = 1,
    /// Listing was cancelled by the seller.
    Cancelled = 2,
    /// Listing has expired.
    Expired = 3,
}

impl ListingStatus {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Sold,
            2 => Self::Cancelled,
            3 => Self::Expired,
            _ => Self::Active,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }

    /// Returns `true` if the listing is still active.
    pub fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns `true` if the listing has reached a terminal state.
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Sold | Self::Cancelled | Self::Expired)
    }
}

impl fmt::Display for ListingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => f.write_str("ACTIVE"),
            Self::Sold => f.write_str("SOLD"),
            Self::Cancelled => f.write_str("CANCELLED"),
            Self::Expired => f.write_str("EXPIRED"),
        }
    }
}

// ====================== REVENUE SHARE CONFIG ======================

/// Revenue share configuration for a marketplace listing.
///
/// All values are in basis points (1 bps = 0.01%).
/// The four cuts should sum to 10000 (100%).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RevenueShareConfig {
    /// Creator's cut in basis points.
    pub creator_cut_bps: u32,
    /// Seller's cut in basis points.
    pub seller_cut_bps: u32,
    /// Evaluator's cut in basis points.
    pub evaluator_cut_bps: u32,
    /// Platform's cut in basis points.
    pub platform_cut_bps: u32,
}

impl RevenueShareConfig {
    /// Returns the total basis points across all cuts.
    pub fn total_bps(&self) -> u32 {
        self.creator_cut_bps + self.seller_cut_bps + self.evaluator_cut_bps + self.platform_cut_bps
    }

    /// Validates that the total basis points equal 10000 (100%).
    pub fn is_valid(&self) -> bool {
        self.total_bps() == 10_000
    }
}

impl From<proto::RevenueShareConfig> for RevenueShareConfig {
    fn from(p: proto::RevenueShareConfig) -> Self {
        Self {
            creator_cut_bps: p.creator_cut_bps,
            seller_cut_bps: p.seller_cut_bps,
            evaluator_cut_bps: p.evaluator_cut_bps,
            platform_cut_bps: p.platform_cut_bps,
        }
    }
}

impl From<RevenueShareConfig> for proto::RevenueShareConfig {
    fn from(c: RevenueShareConfig) -> Self {
        Self {
            creator_cut_bps: c.creator_cut_bps,
            seller_cut_bps: c.seller_cut_bps,
            evaluator_cut_bps: c.evaluator_cut_bps,
            platform_cut_bps: c.platform_cut_bps,
        }
    }
}

// ====================== AGENT LISTING ======================

/// An agent listing on the marketplace.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentListing {
    /// Unique listing identifier.
    pub listing_id: String,
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// Seller's agent hash.
    pub seller_agent_hash: String,
    /// Type of listing.
    pub listing_type: ListingType,
    /// Price in USD (scaled).
    pub price_usd: u64,
    /// Revenue share configuration.
    pub revenue_share_config: Option<RevenueShareConfig>,
    /// Duration in seconds for rentals (0 = permanent).
    pub duration_seconds: u64,
    /// Hash of the agent's memory snapshot.
    pub metadata_hash: String,
    /// Current listing status.
    pub status: ListingStatus,
    /// Timestamp when the listing was created.
    pub created_at: u64,
    /// Timestamp when the listing expires.
    pub expires_at: u64,
}

impl From<proto::AgentListing> for AgentListing {
    fn from(p: proto::AgentListing) -> Self {
        Self {
            listing_id: p.listing_id,
            agent_hash: p.agent_hash,
            seller_agent_hash: p.seller_agent_hash,
            listing_type: ListingType::from_proto(p.listing_type),
            price_usd: p.price_usd,
            revenue_share_config: p.revenue_share_config.map(Into::into),
            duration_seconds: p.duration_seconds,
            metadata_hash: p.metadata_hash,
            status: ListingStatus::from_proto(p.status),
            created_at: p.created_at,
            expires_at: p.expires_at,
        }
    }
}

impl From<AgentListing> for proto::AgentListing {
    fn from(l: AgentListing) -> Self {
        Self {
            listing_id: l.listing_id,
            agent_hash: l.agent_hash,
            seller_agent_hash: l.seller_agent_hash,
            listing_type: l.listing_type.to_proto(),
            price_usd: l.price_usd,
            revenue_share_config: l.revenue_share_config.map(Into::into),
            duration_seconds: l.duration_seconds,
            metadata_hash: l.metadata_hash,
            status: l.status.to_proto(),
            created_at: l.created_at,
            expires_at: l.expires_at,
        }
    }
}

// ====================== BID ======================

/// A bid placed on an agent listing.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bid {
    /// Unique bid identifier.
    pub bid_id: String,
    /// Listing identifier this bid targets.
    pub listing_id: String,
    /// Bidder's agent hash.
    pub bidder_agent_hash: String,
    /// Bid amount in USD (scaled).
    pub amount_usd: u64,
    /// Timestamp when the bid was placed.
    pub timestamp: u64,
    /// Bidder's signature.
    pub bidder_signature: Vec<u8>,
}

impl From<proto::Bid> for Bid {
    fn from(p: proto::Bid) -> Self {
        Self {
            bid_id: p.bid_id,
            listing_id: p.listing_id,
            bidder_agent_hash: p.bidder_agent_hash,
            amount_usd: p.amount_usd,
            timestamp: p.timestamp,
            bidder_signature: p.bidder_signature,
        }
    }
}

impl From<Bid> for proto::Bid {
    fn from(b: Bid) -> Self {
        Self {
            bid_id: b.bid_id,
            listing_id: b.listing_id,
            bidder_agent_hash: b.bidder_agent_hash,
            amount_usd: b.amount_usd,
            timestamp: b.timestamp,
            bidder_signature: b.bidder_signature,
        }
    }
}

// ====================== ESCROW STATE ======================

/// Escrow state for a completed marketplace transaction.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EscrowState {
    /// Listing identifier.
    pub listing_id: String,
    /// Bid identifier.
    pub bid_id: String,
    /// Buyer's agent hash.
    pub buyer_agent_hash: String,
    /// Amount locked in escrow.
    pub locked_amount: u64,
    /// Timestamp when funds were locked.
    pub locked_at: u64,
    /// Whether funds have been released.
    pub released: bool,
    /// Timestamp when funds were released.
    pub released_at: u64,
}

impl EscrowState {
    /// Returns `true` if the escrow is still pending (not released).
    pub fn is_pending(&self) -> bool {
        !self.released
    }
}

impl From<proto::EscrowState> for EscrowState {
    fn from(p: proto::EscrowState) -> Self {
        Self {
            listing_id: p.listing_id,
            bid_id: p.bid_id,
            buyer_agent_hash: p.buyer_agent_hash,
            locked_amount: p.locked_amount,
            locked_at: p.locked_at,
            released: p.released,
            released_at: p.released_at,
        }
    }
}

impl From<EscrowState> for proto::EscrowState {
    fn from(e: EscrowState) -> Self {
        Self {
            listing_id: e.listing_id,
            bid_id: e.bid_id,
            buyer_agent_hash: e.buyer_agent_hash,
            locked_amount: e.locked_amount,
            locked_at: e.locked_at,
            released: e.released,
            released_at: e.released_at,
        }
    }
}

// ====================== EVALUATION REPORT ======================

/// Evaluation report submitted by an evaluator agent.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EvaluationReport {
    /// Unique evaluation identifier.
    pub evaluation_id: String,
    /// Agent hash of the agent being evaluated.
    pub agent_hash: String,
    /// Evaluator's agent hash.
    pub evaluator_agent_hash: String,
    /// Score (0–10000 basis points).
    pub score: u32,
    /// Short summary of the evaluation.
    pub summary: String,
    /// Hash of the full report stored in Persistent Memory.
    pub detailed_report_hash: String,
    /// Timestamp when the evaluation was submitted.
    pub submitted_at: u64,
}

impl EvaluationReport {
    /// Returns the score as a floating-point percentage (e.g. 95.00).
    pub fn score_percent(&self) -> f64 {
        f64::from(self.score) / 100.0
    }
}

impl From<proto::EvaluationReport> for EvaluationReport {
    fn from(p: proto::EvaluationReport) -> Self {
        Self {
            evaluation_id: p.evaluation_id,
            agent_hash: p.agent_hash,
            evaluator_agent_hash: p.evaluator_agent_hash,
            score: p.score,
            summary: p.summary,
            detailed_report_hash: p.detailed_report_hash,
            submitted_at: p.submitted_at,
        }
    }
}

impl From<EvaluationReport> for proto::EvaluationReport {
    fn from(r: EvaluationReport) -> Self {
        Self {
            evaluation_id: r.evaluation_id,
            agent_hash: r.agent_hash,
            evaluator_agent_hash: r.evaluator_agent_hash,
            score: r.score,
            summary: r.summary,
            detailed_report_hash: r.detailed_report_hash,
            submitted_at: r.submitted_at,
        }
    }
}

// ====================== PARAMS ======================

/// Module parameters (governance-controlled).
///
/// Provides sensible defaults:
/// - `default_platform_cut_bps`: 250 (2.5%)
/// - `default_escrow_timeout_seconds`: 86400 (24 hours)
/// - `min_reputation_to_list`: 0
/// - `listings_enabled`: true
/// - `max_listings_per_agent`: 50
/// - `default_evaluation_fee_usd`: 100
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params {
    /// Default platform cut in basis points.
    pub default_platform_cut_bps: u32,
    /// Default escrow timeout in seconds.
    pub default_escrow_timeout_seconds: u64,
    /// Minimum reputation required to list an agent.
    pub min_reputation_to_list: u64,
    /// Whether new listings are enabled.
    pub listings_enabled: bool,
    /// Maximum active listings per agent.
    pub max_listings_per_agent: u32,
    /// Default evaluation fee in USD (scaled).
    pub default_evaluation_fee_usd: u64,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            default_platform_cut_bps: 250,
            default_escrow_timeout_seconds: 86_400,
            min_reputation_to_list: 0,
            listings_enabled: true,
            max_listings_per_agent: 50,
            default_evaluation_fee_usd: 100,
        }
    }
}

impl From<proto::Params> for Params {
    fn from(p: proto::Params) -> Self {
        Self {
            default_platform_cut_bps: p.default_platform_cut_bps,
            default_escrow_timeout_seconds: p.default_escrow_timeout_seconds,
            min_reputation_to_list: p.min_reputation_to_list,
            listings_enabled: p.listings_enabled,
            max_listings_per_agent: p.max_listings_per_agent,
            default_evaluation_fee_usd: p.default_evaluation_fee_usd,
        }
    }
}

impl From<Params> for proto::Params {
    fn from(p: Params) -> Self {
        Self {
            default_platform_cut_bps: p.default_platform_cut_bps,
            default_escrow_timeout_seconds: p.default_escrow_timeout_seconds,
            min_reputation_to_list: p.min_reputation_to_list,
            listings_enabled: p.listings_enabled,
            max_listings_per_agent: p.max_listings_per_agent,
            default_evaluation_fee_usd: p.default_evaluation_fee_usd,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn listing_type_roundtrip() {
        for t in [
            ListingType::FullOwnership,
            ListingType::CoOwnership,
            ListingType::Rental,
            ListingType::EvaluationOnly,
        ] {
            assert_eq!(ListingType::from_proto(t.to_proto()), t);
        }
    }

    #[test]
    fn listing_type_display() {
        assert_eq!(ListingType::FullOwnership.to_string(), "FULL_OWNERSHIP");
        assert_eq!(ListingType::CoOwnership.to_string(), "CO_OWNERSHIP");
        assert_eq!(ListingType::Rental.to_string(), "RENTAL");
        assert_eq!(ListingType::EvaluationOnly.to_string(), "EVALUATION_ONLY");
    }

    #[test]
    fn listing_type_unknown_defaults() {
        assert_eq!(ListingType::from_proto(999), ListingType::FullOwnership);
    }

    #[test]
    fn listing_status_roundtrip() {
        for s in [
            ListingStatus::Active,
            ListingStatus::Sold,
            ListingStatus::Cancelled,
            ListingStatus::Expired,
        ] {
            assert_eq!(ListingStatus::from_proto(s.to_proto()), s);
        }
    }

    #[test]
    fn listing_status_display() {
        assert_eq!(ListingStatus::Active.to_string(), "ACTIVE");
        assert_eq!(ListingStatus::Sold.to_string(), "SOLD");
        assert_eq!(ListingStatus::Cancelled.to_string(), "CANCELLED");
        assert_eq!(ListingStatus::Expired.to_string(), "EXPIRED");
    }

    #[test]
    fn listing_status_helpers() {
        assert!(ListingStatus::Active.is_active());
        assert!(!ListingStatus::Active.is_terminal());
        assert!(!ListingStatus::Sold.is_active());
        assert!(ListingStatus::Sold.is_terminal());
        assert!(ListingStatus::Cancelled.is_terminal());
        assert!(ListingStatus::Expired.is_terminal());
    }

    #[test]
    fn revenue_share_config_validation() {
        let valid = RevenueShareConfig {
            creator_cut_bps: 2000,
            seller_cut_bps: 6000,
            evaluator_cut_bps: 1000,
            platform_cut_bps: 1000,
        };
        assert!(valid.is_valid());
        assert_eq!(valid.total_bps(), 10_000);

        let invalid = RevenueShareConfig {
            creator_cut_bps: 5000,
            seller_cut_bps: 5000,
            evaluator_cut_bps: 1000,
            platform_cut_bps: 0,
        };
        assert!(!invalid.is_valid());
    }

    #[test]
    fn revenue_share_config_roundtrip() {
        let cfg = RevenueShareConfig {
            creator_cut_bps: 2000,
            seller_cut_bps: 6000,
            evaluator_cut_bps: 1000,
            platform_cut_bps: 1000,
        };
        let proto: proto::RevenueShareConfig = cfg.clone().into();
        let back: RevenueShareConfig = proto.into();
        assert_eq!(cfg, back);
    }

    #[test]
    fn agent_listing_roundtrip() {
        let listing = AgentListing {
            listing_id: "listing-001".into(),
            agent_hash: "agent-abc".into(),
            seller_agent_hash: "seller-xyz".into(),
            listing_type: ListingType::Rental,
            price_usd: 500_000,
            revenue_share_config: Some(RevenueShareConfig {
                creator_cut_bps: 2000,
                seller_cut_bps: 6000,
                evaluator_cut_bps: 1000,
                platform_cut_bps: 1000,
            }),
            duration_seconds: 2_592_000,
            metadata_hash: "meta-hash".into(),
            status: ListingStatus::Active,
            created_at: 1_700_000_000,
            expires_at: 1_702_592_000,
        };
        let proto: proto::AgentListing = listing.clone().into();
        let back: AgentListing = proto.into();
        assert_eq!(listing, back);
    }

    #[test]
    fn bid_roundtrip() {
        let bid = Bid {
            bid_id: "bid-001".into(),
            listing_id: "listing-001".into(),
            bidder_agent_hash: "bidder-abc".into(),
            amount_usd: 450_000,
            timestamp: 1_700_001_000,
            bidder_signature: vec![0u8; 64],
        };
        let proto: proto::Bid = bid.clone().into();
        let back: Bid = proto.into();
        assert_eq!(bid, back);
    }

    #[test]
    fn escrow_state_roundtrip() {
        let escrow = EscrowState {
            listing_id: "listing-001".into(),
            bid_id: "bid-001".into(),
            buyer_agent_hash: "buyer-abc".into(),
            locked_amount: 500_000,
            locked_at: 1_700_002_000,
            released: false,
            released_at: 0,
        };
        let proto: proto::EscrowState = escrow.clone().into();
        let back: EscrowState = proto.into();
        assert_eq!(escrow, back);
    }

    #[test]
    fn escrow_state_pending() {
        let pending = EscrowState { released: false, ..Default::default() };
        assert!(pending.is_pending());

        let released = EscrowState { released: true, released_at: 123, ..Default::default() };
        assert!(!released.is_pending());
    }

    #[test]
    fn evaluation_report_roundtrip() {
        let report = EvaluationReport {
            evaluation_id: "eval-001".into(),
            agent_hash: "agent-abc".into(),
            evaluator_agent_hash: "evaluator-xyz".into(),
            score: 8500,
            summary: "Excellent trading performance".into(),
            detailed_report_hash: "report-hash".into(),
            submitted_at: 1_700_003_000,
        };
        let proto: proto::EvaluationReport = report.clone().into();
        let back: EvaluationReport = proto.into();
        assert_eq!(report, back);
    }

    #[test]
    fn evaluation_report_score_percent() {
        let report = EvaluationReport { score: 8500, ..Default::default() };
        assert!((report.score_percent() - 85.0).abs() < f64::EPSILON);
    }

    #[test]
    fn params_defaults() {
        let params = Params::default();
        assert_eq!(params.default_platform_cut_bps, 250);
        assert_eq!(params.default_escrow_timeout_seconds, 86_400);
        assert_eq!(params.min_reputation_to_list, 0);
        assert!(params.listings_enabled);
        assert_eq!(params.max_listings_per_agent, 50);
        assert_eq!(params.default_evaluation_fee_usd, 100);
    }

    #[test]
    fn params_roundtrip() {
        let params = Params {
            default_platform_cut_bps: 500,
            default_escrow_timeout_seconds: 172_800,
            min_reputation_to_list: 10_000,
            listings_enabled: false,
            max_listings_per_agent: 20,
            default_evaluation_fee_usd: 200,
        };
        let proto: proto::Params = params.clone().into();
        let back: Params = proto.into();
        assert_eq!(params, back);
    }
}
