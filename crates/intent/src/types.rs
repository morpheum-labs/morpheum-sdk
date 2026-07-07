//! Domain types for the Intent module.
//!
//! These are clean, idiomatic Rust representations of the intent protobuf
//! messages. They provide type safety, ergonomic APIs, and full round-trip
//! conversion to/from protobuf while remaining strictly `no_std` compatible.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::intent::v1 as proto;

// ====================== INTENT TYPE ======================

/// Type of agent intent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum IntentType {
    /// Conditional: execute an action when a condition is met.
    #[default]
    Conditional = 0,
    /// TWAP: time-weighted average price execution across slices.
    Twap = 1,
    /// Multi-leg: atomic execution of multiple correlated actions.
    MultiLeg = 2,
    /// Declarative: high-level goal decomposed by the runtime.
    Declarative = 3,
    /// RFQ: sealed-bid request-for-quote auction (zkRFQ, ADR-ZK-002).
    Rfq = 4,
    /// POV: volume-participation execution (WS-BF).
    Pov = 5,
}

impl IntentType {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Twap,
            2 => Self::MultiLeg,
            3 => Self::Declarative,
            4 => Self::Rfq,
            5 => Self::Pov,
            _ => Self::Conditional,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for IntentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conditional => f.write_str("CONDITIONAL"),
            Self::Twap => f.write_str("TWAP"),
            Self::MultiLeg => f.write_str("MULTI_LEG"),
            Self::Declarative => f.write_str("DECLARATIVE"),
            Self::Rfq => f.write_str("RFQ"),
            Self::Pov => f.write_str("POV"),
        }
    }
}

// ====================== INTENT STATUS ======================

/// Lifecycle status of an agent intent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum IntentStatus {
    /// Awaiting execution or condition check.
    #[default]
    Pending = 0,
    /// Currently being executed.
    Executing = 1,
    /// Successfully completed.
    Completed = 2,
    /// Execution failed.
    Failed = 3,
    /// Explicitly cancelled by the agent.
    Cancelled = 4,
    /// Expired (past `expiry_timestamp`).
    Expired = 5,
    /// zkRFQ: a sealed bid has been accepted and escrow locked; the RFQ awaits
    /// the winning maker's reveal-and-settle (ADR-ZK-002 Phase 4c).
    AwaitingReveal = 6,
}

impl IntentStatus {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Executing,
            2 => Self::Completed,
            3 => Self::Failed,
            4 => Self::Cancelled,
            5 => Self::Expired,
            6 => Self::AwaitingReveal,
            _ => Self::Pending,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }

    /// Returns `true` if the intent is in a terminal state.
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Cancelled | Self::Expired
        )
    }

    /// Returns `true` if the intent is still live (pending, executing, or
    /// awaiting a zkRFQ reveal-and-settle).
    pub fn is_active(self) -> bool {
        matches!(self, Self::Pending | Self::Executing | Self::AwaitingReveal)
    }
}

impl fmt::Display for IntentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => f.write_str("PENDING"),
            Self::Executing => f.write_str("EXECUTING"),
            Self::Completed => f.write_str("COMPLETED"),
            Self::Failed => f.write_str("FAILED"),
            Self::Cancelled => f.write_str("CANCELLED"),
            Self::Expired => f.write_str("EXPIRED"),
            Self::AwaitingReveal => f.write_str("AWAITING_REVEAL"),
        }
    }
}

// ====================== EXECUTION-ENGINE ENUMS (E6, WS-G G1) ======================
//
// Feed the intent-execution engine's typed orders/triggers. Every field is
// deterministic and float-free: integer indices/sizes and 1e8 fixed-point
// prices as decimal strings, matching the CLOB `batch_execute` SSOT.

/// Order side for an execution-engine order.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum Side {
    /// Buy (bid).
    #[default]
    Buy = 0,
    /// Sell (ask).
    Sell = 1,
}

impl Side {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Sell,
            _ => Self::Buy,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Buy => f.write_str("SIDE_BUY"),
            Self::Sell => f.write_str("SIDE_SELL"),
        }
    }
}

/// Time-in-force for an execution-engine order. GTC rests as a maker; IOC/FOK
/// are marketable against a crossing limit price.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum Tif {
    /// Good-till-cancelled.
    #[default]
    Gtc = 0,
    /// Immediate-or-cancel.
    Ioc = 1,
    /// Fill-or-kill.
    Fok = 2,
}

impl Tif {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Ioc,
            2 => Self::Fok,
            _ => Self::Gtc,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for Tif {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gtc => f.write_str("TIF_GTC"),
            Self::Ioc => f.write_str("TIF_IOC"),
            Self::Fok => f.write_str("TIF_FOK"),
        }
    }
}

/// TWAP slice-size distribution. Only `Uniform` (equal slices, dust on the
/// last) is supported on the consensus path today; the enum keeps the schema
/// forward-compatible without a wire break.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum SliceCurve {
    /// Equal-sized slices (dust absorbed by the last slice).
    #[default]
    Uniform = 0,
}

impl SliceCurve {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(_value: i32) -> Self {
        Self::Uniform
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for SliceCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uniform => f.write_str("SLICE_CURVE_UNIFORM"),
        }
    }
}

/// Comparator for a conditional trigger evaluated against the committed mark.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum Comparator {
    /// Fire when the committed mark >= `trigger_price_e8`.
    #[default]
    Above = 0,
    /// Fire when the committed mark <= `trigger_price_e8`.
    Below = 1,
}

impl Comparator {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Below,
            _ => Self::Above,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for Comparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Above => f.write_str("COMPARATOR_ABOVE"),
            Self::Below => f.write_str("COMPARATOR_BELOW"),
        }
    }
}

// ====================== EXECUTION-ENGINE ORDERS/TRIGGERS ======================

/// A single execution-ready CLOB order. The engine authorizes the intent
/// owner against `bucket_id` (an agent must not spend another agent's bucket
/// margin) before placing, then lowers this into one order placement.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OrderAction {
    pub market_index: u64,
    /// Bucket the order trades against (margin + position attribution).
    pub bucket_id: u64,
    pub side: Side,
    /// Order quantity (1e8 satoshi-scale).
    pub quantity: u64,
    /// Limit price (1e8 fixed-point) as a decimal string. Required, non-zero —
    /// the engine posts deterministic limit orders only.
    pub price_e8: String,
    pub tif: Tif,
}

impl From<proto::OrderAction> for OrderAction {
    fn from(p: proto::OrderAction) -> Self {
        Self {
            market_index: p.market_index,
            bucket_id: p.bucket_id,
            side: Side::from_proto(p.side),
            quantity: p.quantity,
            price_e8: p.price_e8,
            tif: Tif::from_proto(p.tif),
        }
    }
}

impl From<OrderAction> for proto::OrderAction {
    fn from(a: OrderAction) -> Self {
        Self {
            market_index: a.market_index,
            bucket_id: a.bucket_id,
            side: a.side.to_proto(),
            quantity: a.quantity,
            price_e8: a.price_e8,
            tif: a.tif.to_proto(),
        }
    }
}

/// A conditional trigger: when the committed mark for `market_index` crosses
/// `trigger_price_e8` per `cmp`, the intent's action fires.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TriggerCondition {
    pub market_index: u64,
    pub cmp: Comparator,
    /// Committed-mark trigger price (1e8) as a decimal string.
    pub trigger_price_e8: String,
}

impl From<proto::TriggerCondition> for TriggerCondition {
    fn from(p: proto::TriggerCondition) -> Self {
        Self {
            market_index: p.market_index,
            cmp: Comparator::from_proto(p.cmp),
            trigger_price_e8: p.trigger_price_e8,
        }
    }
}

impl From<TriggerCondition> for proto::TriggerCondition {
    fn from(t: TriggerCondition) -> Self {
        Self {
            market_index: t.market_index,
            cmp: t.cmp.to_proto(),
            trigger_price_e8: t.trigger_price_e8,
        }
    }
}

// ====================== INTENT PARAMETER TYPES ======================

/// Conditional intent parameters: a committed-mark trigger plus the order to
/// place when it fires.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConditionalParams {
    /// The committed-mark trigger.
    pub condition: TriggerCondition,
    /// The order to place when the trigger fires.
    pub action: OrderAction,
}

impl From<proto::ConditionalParams> for ConditionalParams {
    fn from(p: proto::ConditionalParams) -> Self {
        Self {
            condition: p.condition.unwrap_or_default().into(),
            action: p.action.unwrap_or_default().into(),
        }
    }
}

impl From<ConditionalParams> for proto::ConditionalParams {
    fn from(c: ConditionalParams) -> Self {
        Self {
            condition: Some(c.condition.into()),
            action: Some(c.action.into()),
        }
    }
}

/// TWAP (Time-Weighted Average Price) intent parameters.
///
/// Slices `total_size` into `num_slices` equal child orders spread across
/// `duration_ms`, each posted at `limit_price_e8` with `tif`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TwapParams {
    pub market_index: u64,
    pub bucket_id: u64,
    pub side: Side,
    /// Total order size (1e8 satoshi-scale).
    pub total_size: u64,
    /// Number of slices to divide the order into.
    pub num_slices: u32,
    /// Duration over which to execute (milliseconds).
    pub duration_ms: u64,
    pub curve: SliceCurve,
    pub tif: Tif,
    /// Per-slice limit price (1e8) as a decimal string. Required, non-zero.
    pub limit_price_e8: String,
}

impl From<proto::TwapParams> for TwapParams {
    fn from(p: proto::TwapParams) -> Self {
        Self {
            market_index: p.market_index,
            bucket_id: p.bucket_id,
            side: Side::from_proto(p.side),
            total_size: p.total_size,
            num_slices: p.num_slices,
            duration_ms: p.duration_ms,
            curve: SliceCurve::from_proto(p.curve),
            tif: Tif::from_proto(p.tif),
            limit_price_e8: p.limit_price_e8,
        }
    }
}

impl From<TwapParams> for proto::TwapParams {
    fn from(t: TwapParams) -> Self {
        Self {
            market_index: t.market_index,
            bucket_id: t.bucket_id,
            side: t.side.to_proto(),
            total_size: t.total_size,
            num_slices: t.num_slices,
            duration_ms: t.duration_ms,
            curve: t.curve.to_proto(),
            tif: t.tif.to_proto(),
            limit_price_e8: t.limit_price_e8,
            // WS-BG custom volume-profile weights are not yet surfaced on the SDK
            // `SliceCurve` (still `Uniform`-only, as of WS-AP); default empty ⇒ the
            // non-custom curves, byte-identical. Full SDK parity is a follow-on.
            slice_weights: Vec::new(),
        }
    }
}

/// POV (Percentage-of-Volume) / volume-participation intent parameters (WS-BF).
///
/// Each cadence tick sizes the child order to a target participation of the
/// market volume realized since arming, reading the CLOB committed traded-volume
/// SSOT. Prices are 1e8 fixed-point decimal strings; every other field is an
/// integer index/size.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PovParams {
    pub market_index: u64,
    pub bucket_id: u64,
    pub side: Side,
    /// Total order size to execute (1e8 satoshi-scale).
    pub total_size: u64,
    /// Target participation of realized market volume, in basis points
    /// (`1..=10000`).
    pub participation_rate_bps: u32,
    /// Execution window (milliseconds); the remainder is closed by expiry.
    pub duration_ms: u64,
    /// Minimum child-order size (1e8); a below-floor tick is skipped. `0` ⇒ none.
    pub min_slice_size: u64,
    /// Maximum child-order size (1e8), bounding a single tick. `0` ⇒ unbounded.
    pub max_slice_size: u64,
    pub tif: Tif,
    /// Per-slice limit price (1e8) as a decimal string. Required, non-zero.
    pub limit_price_e8: String,
}

impl From<proto::PovParams> for PovParams {
    fn from(p: proto::PovParams) -> Self {
        Self {
            market_index: p.market_index,
            bucket_id: p.bucket_id,
            side: Side::from_proto(p.side),
            total_size: p.total_size,
            participation_rate_bps: p.participation_rate_bps,
            duration_ms: p.duration_ms,
            min_slice_size: p.min_slice_size,
            max_slice_size: p.max_slice_size,
            tif: Tif::from_proto(p.tif),
            limit_price_e8: p.limit_price_e8,
        }
    }
}

impl From<PovParams> for proto::PovParams {
    fn from(p: PovParams) -> Self {
        Self {
            market_index: p.market_index,
            bucket_id: p.bucket_id,
            side: p.side.to_proto(),
            total_size: p.total_size,
            participation_rate_bps: p.participation_rate_bps,
            duration_ms: p.duration_ms,
            min_slice_size: p.min_slice_size,
            max_slice_size: p.max_slice_size,
            tif: p.tif.to_proto(),
            limit_price_e8: p.limit_price_e8,
        }
    }
}

/// Multi-leg atomic intent parameters.
///
/// All legs are executed atomically (all-or-nothing) when `atomic` is `true`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MultiLegParams {
    /// Individual legs of the intent.
    pub legs: Vec<OrderAction>,
    /// Whether this is an all-or-nothing atomic execution.
    pub atomic: bool,
}

impl From<proto::MultiLegParams> for MultiLegParams {
    fn from(p: proto::MultiLegParams) -> Self {
        Self {
            legs: p.legs.into_iter().map(Into::into).collect(),
            atomic: p.atomic,
        }
    }
}

impl From<MultiLegParams> for proto::MultiLegParams {
    fn from(m: MultiLegParams) -> Self {
        Self {
            legs: m.legs.into_iter().map(Into::into).collect(),
            atomic: m.atomic,
        }
    }
}

/// Declarative (high-level goal) intent parameters.
///
/// The runtime decomposes the natural-language goal into executable steps.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeclarativeParams {
    /// Original text goal (e.g. `"Maximise yield on idle USDC"`).
    pub raw_goal: String,
    /// Semantic embedding vector (typically 512-dim).
    pub goal_embedding: Vec<f32>,
    /// JSON-encoded constraints.
    pub constraints: String,
    /// Preferred execution style: `"conservative"`, `"balanced"`, `"aggressive"`.
    pub preferred_style: String,
}

impl From<proto::DeclarativeParams> for DeclarativeParams {
    fn from(p: proto::DeclarativeParams) -> Self {
        Self {
            raw_goal: p.raw_goal,
            goal_embedding: p.goal_embedding,
            constraints: p.constraints,
            preferred_style: p.preferred_style,
        }
    }
}

impl From<DeclarativeParams> for proto::DeclarativeParams {
    fn from(d: DeclarativeParams) -> Self {
        Self {
            raw_goal: d.raw_goal,
            goal_embedding: d.goal_embedding,
            constraints: d.constraints,
            preferred_style: d.preferred_style,
        }
    }
}

// ====================== RFQ ======================

/// Side of an RFQ request — the direction the taker wants quoted.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum RfqSide {
    /// Taker wants to buy.
    #[default]
    Buy = 0,
    /// Taker wants to sell.
    Sell = 1,
}

impl RfqSide {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Sell,
            _ => Self::Buy,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for RfqSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Buy => f.write_str("BUY"),
            Self::Sell => f.write_str("SELL"),
        }
    }
}

/// Public terms of a sealed-bid RFQ auction (zkRFQ, ADR-ZK-002).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RfqParams {
    /// Market the quote is solicited for.
    pub market_id: u32,
    /// Taker's requested maximum size (1e8-scaled).
    pub size: u64,
    /// Direction the taker wants quoted.
    pub side: RfqSide,
    /// Band tolerance in basis points.
    pub tol_bps: u32,
    /// Duration (unix seconds) an accepted maker has to reveal-and-settle,
    /// applied per acceptance: the effective deadline is
    /// `min(accepted_at + reveal_window_secs, expiry_timestamp)`.
    pub reveal_window_secs: u64,
}

impl From<proto::RfqParams> for RfqParams {
    fn from(p: proto::RfqParams) -> Self {
        Self {
            market_id: p.market_id,
            size: p.size,
            side: RfqSide::from_proto(p.side),
            tol_bps: p.tol_bps,
            reveal_window_secs: p.reveal_window_secs,
        }
    }
}

impl From<RfqParams> for proto::RfqParams {
    fn from(p: RfqParams) -> Self {
        Self {
            market_id: p.market_id,
            size: p.size,
            side: p.side.to_proto(),
            tol_bps: p.tol_bps,
            reveal_window_secs: p.reveal_window_secs,
        }
    }
}

/// Typed union of intent parameter variants.
///
/// Maps directly to the protobuf `oneof params` in `AgentIntent`.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IntentParams {
    /// Conditional: execute on condition.
    Conditional(ConditionalParams),
    /// TWAP: time-sliced execution.
    Twap(TwapParams),
    /// POV: volume-participation execution.
    Pov(PovParams),
    /// Multi-leg: atomic correlated actions.
    MultiLeg(MultiLegParams),
    /// Declarative: high-level goal decomposition.
    Declarative(DeclarativeParams),
    /// RFQ: sealed-bid request-for-quote auction.
    Rfq(RfqParams),
}

impl From<proto::agent_intent::Params> for IntentParams {
    fn from(p: proto::agent_intent::Params) -> Self {
        match p {
            proto::agent_intent::Params::Conditional(c) => Self::Conditional(c.into()),
            proto::agent_intent::Params::Twap(t) => Self::Twap(t.into()),
            proto::agent_intent::Params::Pov(p) => Self::Pov(p.into()),
            proto::agent_intent::Params::MultiLeg(m) => Self::MultiLeg(m.into()),
            proto::agent_intent::Params::Declarative(d) => Self::Declarative(d.into()),
            proto::agent_intent::Params::Rfq(r) => Self::Rfq(r.into()),
        }
    }
}

impl From<IntentParams> for proto::agent_intent::Params {
    fn from(p: IntentParams) -> Self {
        match p {
            IntentParams::Conditional(c) => Self::Conditional(c.into()),
            IntentParams::Twap(t) => Self::Twap(t.into()),
            IntentParams::Pov(p) => Self::Pov(p.into()),
            IntentParams::MultiLeg(m) => Self::MultiLeg(m.into()),
            IntentParams::Declarative(d) => Self::Declarative(d.into()),
            IntentParams::Rfq(r) => Self::Rfq(r.into()),
        }
    }
}

// ====================== AGENT INTENT ======================

/// Core agent intent — the primary unit of intent-based execution on Morpheum.
///
/// An `AgentIntent` represents a high-level trading objective submitted by an
/// AI agent, which the runtime decomposes and executes according to its type.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentIntent {
    /// Unique intent identifier.
    pub intent_id: String,
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// Type of intent.
    pub intent_type: IntentType,
    /// Type-specific parameters.
    pub params: Option<IntentParams>,
    /// Hash of the delegation VC proving authorisation.
    pub vc_proof_hash: String,
    /// Expiry timestamp (0 = no expiry).
    pub expiry_timestamp: u64,
    /// Priority boost from reputation/milestones.
    pub priority_boost: u32,
    /// Current lifecycle status.
    pub status: IntentStatus,
    /// Block timestamp when the intent was created.
    pub created_at: u64,
    /// Optional attached context data (memory snapshots, agent state,
    /// validation proofs, model references). Routed via blob storage above
    /// the blob threshold, in which case `blob_merkle_root` is set instead.
    pub context_data: Vec<u8>,
    /// Blob-backed context Merkle root (erasure-coded DAS layer). Set
    /// automatically when `context_data` is routed through blob storage.
    pub blob_merkle_root: Vec<u8>,
}

impl AgentIntent {
    /// Returns `true` if the intent is still active (pending, executing, or
    /// awaiting a zkRFQ reveal-and-settle).
    pub fn is_active(&self) -> bool {
        self.status.is_active()
    }

    /// Returns `true` if the intent has reached a terminal state.
    pub fn is_terminal(&self) -> bool {
        self.status.is_terminal()
    }

    /// Returns `true` if the intent has expired relative to the given timestamp.
    pub fn is_expired(&self, now: u64) -> bool {
        self.expiry_timestamp > 0 && now >= self.expiry_timestamp
    }
}

impl From<proto::AgentIntent> for AgentIntent {
    fn from(p: proto::AgentIntent) -> Self {
        Self {
            intent_id: p.intent_id,
            agent_hash: p.agent_hash,
            intent_type: IntentType::from_proto(p.intent_type),
            params: p.params.map(Into::into),
            vc_proof_hash: p.vc_proof_hash,
            expiry_timestamp: p.expiry_timestamp,
            priority_boost: p.priority_boost,
            status: IntentStatus::from_proto(p.status),
            created_at: p.created_at,
            context_data: p.context_data,
            blob_merkle_root: p.blob_merkle_root,
        }
    }
}

impl From<AgentIntent> for proto::AgentIntent {
    fn from(a: AgentIntent) -> Self {
        Self {
            intent_id: a.intent_id,
            agent_hash: a.agent_hash,
            intent_type: a.intent_type.to_proto(),
            params: a.params.map(Into::into),
            vc_proof_hash: a.vc_proof_hash,
            expiry_timestamp: a.expiry_timestamp,
            priority_boost: a.priority_boost,
            status: a.status.to_proto(),
            created_at: a.created_at,
            context_data: a.context_data,
            blob_merkle_root: a.blob_merkle_root,
        }
    }
}

// ====================== DECOMPOSITION TRACE ======================

/// Audit trail for intent decomposition (for agent learning and transparency).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DecompositionTrace {
    /// Intent ID this trace belongs to.
    pub intent_id: String,
    /// Original high-level goal.
    pub original_goal: String,
    /// Human-readable decomposition steps.
    pub steps: Vec<String>,
    /// Timestamp when decomposition occurred.
    pub decomposed_at: u64,
}

impl From<proto::DecompositionTrace> for DecompositionTrace {
    fn from(p: proto::DecompositionTrace) -> Self {
        Self {
            intent_id: p.intent_id,
            original_goal: p.original_goal,
            steps: p.steps,
            decomposed_at: p.decomposed_at,
        }
    }
}

impl From<DecompositionTrace> for proto::DecompositionTrace {
    fn from(d: DecompositionTrace) -> Self {
        Self {
            intent_id: d.intent_id,
            original_goal: d.original_goal,
            steps: d.steps,
            decomposed_at: d.decomposed_at,
        }
    }
}

// ====================== PARAMS ======================

/// Module parameters (governance-controlled).
///
/// Provides sensible defaults:
/// - `default_expiry_seconds`: 3600 (1 hour)
/// - `max_concurrent_intents_per_agent`: 10
/// - `enable_declarative_decomposition`: true
/// - `scheduler_tick_ms`: 500
/// - `require_simulation`: false
/// - `max_decomposition_steps`: 20
/// - `rfq_enabled`: false
/// - `enable_intent_execution`: false
/// - `max_intents_per_scan`: 0 (module built-in default)
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params {
    /// Default intent expiry in seconds (0 = no expiry).
    pub default_expiry_seconds: u64,
    /// Maximum concurrent intents per agent (0 = unlimited).
    pub max_concurrent_intents_per_agent: u32,
    /// Whether declarative intent decomposition is enabled.
    pub enable_declarative_decomposition: bool,
    /// Scheduler tick interval in milliseconds.
    pub scheduler_tick_ms: u32,
    /// Whether dry-run simulation is required before execution.
    pub require_simulation: bool,
    /// Maximum steps allowed in a single declarative decomposition.
    pub max_decomposition_steps: u32,
    /// Whether the zkRFQ flow (RFQ intent type + sealed-quote messages) is
    /// enabled (ADR-ZK-002). Default-disabled, fail-closed.
    pub rfq_enabled: bool,
    /// WS-G G1 — master switch for the intent-execution engine (TWAP /
    /// conditional / multi-leg). Default-disabled, fail-closed.
    pub enable_intent_execution: bool,
    /// Allowlist of keepers permitted to submit the `MsgExecuteIntents`
    /// cadence. Empty = permissionless.
    pub authorized_execution_signers: Vec<String>,
    /// Per-scan bound on the number of intents one `MsgExecuteIntents`
    /// services (0 = built-in default).
    pub max_intents_per_scan: u64,
    /// WS-AI — whether the zkRFQ reputation admission gate is armed.
    /// Default-disabled, fail-closed.
    pub enable_rfq_reputation_gate: bool,
    /// WS-AI — minimum committed reputation score a market maker must hold to
    /// quote/settle a zkRFQ when the gate is armed (ignored when disarmed).
    pub min_reputation_to_quote: u64,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            default_expiry_seconds: 3600,
            max_concurrent_intents_per_agent: 10,
            enable_declarative_decomposition: true,
            scheduler_tick_ms: 500,
            require_simulation: false,
            max_decomposition_steps: 20,
            rfq_enabled: false,
            enable_intent_execution: false,
            authorized_execution_signers: Vec::new(),
            max_intents_per_scan: 0,
            enable_rfq_reputation_gate: false,
            min_reputation_to_quote: 0,
        }
    }
}

impl From<proto::Params> for Params {
    fn from(p: proto::Params) -> Self {
        Self {
            default_expiry_seconds: p.default_expiry_seconds,
            max_concurrent_intents_per_agent: p.max_concurrent_intents_per_agent,
            enable_declarative_decomposition: p.enable_declarative_decomposition,
            scheduler_tick_ms: p.scheduler_tick_ms,
            require_simulation: p.require_simulation,
            max_decomposition_steps: p.max_decomposition_steps,
            rfq_enabled: p.rfq_enabled,
            enable_intent_execution: p.enable_intent_execution,
            authorized_execution_signers: p.authorized_execution_signers,
            max_intents_per_scan: p.max_intents_per_scan,
            enable_rfq_reputation_gate: p.enable_rfq_reputation_gate,
            min_reputation_to_quote: p.min_reputation_to_quote,
        }
    }
}

impl From<Params> for proto::Params {
    fn from(p: Params) -> Self {
        Self {
            default_expiry_seconds: p.default_expiry_seconds,
            max_concurrent_intents_per_agent: p.max_concurrent_intents_per_agent,
            enable_declarative_decomposition: p.enable_declarative_decomposition,
            scheduler_tick_ms: p.scheduler_tick_ms,
            require_simulation: p.require_simulation,
            max_decomposition_steps: p.max_decomposition_steps,
            rfq_enabled: p.rfq_enabled,
            enable_intent_execution: p.enable_intent_execution,
            authorized_execution_signers: p.authorized_execution_signers,
            max_intents_per_scan: p.max_intents_per_scan,
            enable_rfq_reputation_gate: p.enable_rfq_reputation_gate,
            min_reputation_to_quote: p.min_reputation_to_quote,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn intent_type_roundtrip() {
        for t in [
            IntentType::Conditional,
            IntentType::Twap,
            IntentType::MultiLeg,
            IntentType::Declarative,
            IntentType::Rfq,
            IntentType::Pov,
        ] {
            assert_eq!(IntentType::from_proto(t.to_proto()), t);
        }
    }

    #[test]
    fn intent_status_roundtrip() {
        for s in [
            IntentStatus::Pending,
            IntentStatus::Executing,
            IntentStatus::Completed,
            IntentStatus::Failed,
            IntentStatus::Cancelled,
            IntentStatus::Expired,
            IntentStatus::AwaitingReveal,
        ] {
            assert_eq!(IntentStatus::from_proto(s.to_proto()), s);
        }
    }

    #[test]
    fn intent_status_helpers() {
        assert!(IntentStatus::Pending.is_active());
        assert!(IntentStatus::Executing.is_active());
        assert!(IntentStatus::AwaitingReveal.is_active());
        assert!(!IntentStatus::Completed.is_active());
        assert!(IntentStatus::Completed.is_terminal());
        assert!(IntentStatus::Failed.is_terminal());
        assert!(IntentStatus::Cancelled.is_terminal());
        assert!(IntentStatus::Expired.is_terminal());
        assert!(!IntentStatus::Pending.is_terminal());
        assert!(!IntentStatus::AwaitingReveal.is_terminal());
    }

    #[test]
    fn side_tif_curve_comparator_roundtrip() {
        for s in [Side::Buy, Side::Sell] {
            assert_eq!(Side::from_proto(s.to_proto()), s);
        }
        for t in [Tif::Gtc, Tif::Ioc, Tif::Fok] {
            assert_eq!(Tif::from_proto(t.to_proto()), t);
        }
        assert_eq!(
            SliceCurve::from_proto(SliceCurve::Uniform.to_proto()),
            SliceCurve::Uniform
        );
        for c in [Comparator::Above, Comparator::Below] {
            assert_eq!(Comparator::from_proto(c.to_proto()), c);
        }
    }

    #[test]
    fn order_action_roundtrip() {
        let action = OrderAction {
            market_index: 7,
            bucket_id: 42,
            side: Side::Sell,
            quantity: 1_000_000,
            price_e8: "5000000000000".into(),
            tif: Tif::Ioc,
        };
        let proto: proto::OrderAction = action.clone().into();
        let back: OrderAction = proto.into();
        assert_eq!(action, back);
    }

    #[test]
    fn conditional_params_roundtrip() {
        let params = ConditionalParams {
            condition: TriggerCondition {
                market_index: 7,
                cmp: Comparator::Above,
                trigger_price_e8: "5000000000000".into(),
            },
            action: OrderAction {
                market_index: 7,
                bucket_id: 42,
                side: Side::Buy,
                quantity: 500_000,
                price_e8: "4990000000000".into(),
                tif: Tif::Gtc,
            },
        };
        let proto: proto::ConditionalParams = params.clone().into();
        let back: ConditionalParams = proto.into();
        assert_eq!(params, back);
    }

    #[test]
    fn twap_params_roundtrip() {
        let params = TwapParams {
            market_index: 3,
            bucket_id: 9,
            side: Side::Buy,
            total_size: 100_000,
            num_slices: 10,
            duration_ms: 60_000,
            curve: SliceCurve::Uniform,
            tif: Tif::Gtc,
            limit_price_e8: "5000000000000".into(),
        };
        let proto: proto::TwapParams = params.clone().into();
        let back: TwapParams = proto.into();
        assert_eq!(params, back);
    }

    #[test]
    fn multi_leg_params_roundtrip() {
        let params = MultiLegParams {
            legs: vec![
                OrderAction {
                    market_index: 1,
                    bucket_id: 1,
                    side: Side::Buy,
                    quantity: 1000,
                    price_e8: "100000000".into(),
                    tif: Tif::Gtc,
                },
                OrderAction {
                    market_index: 2,
                    bucket_id: 1,
                    side: Side::Sell,
                    quantity: 500,
                    price_e8: "200000000".into(),
                    tif: Tif::Ioc,
                },
            ],
            atomic: true,
        };
        let proto: proto::MultiLegParams = params.clone().into();
        let back: MultiLegParams = proto.into();
        assert_eq!(params, back);
    }

    #[test]
    fn declarative_params_roundtrip() {
        let params = DeclarativeParams {
            raw_goal: "Maximise yield on idle USDC".into(),
            goal_embedding: vec![0.1, 0.2, 0.3],
            constraints: r#"{"max_risk": "low"}"#.into(),
            preferred_style: "conservative".into(),
        };
        let proto: proto::DeclarativeParams = params.clone().into();
        let back: DeclarativeParams = proto.into();
        assert_eq!(params, back);
    }

    #[test]
    fn agent_intent_roundtrip_conditional() {
        let intent = AgentIntent {
            intent_id: "intent-001".into(),
            agent_hash: "abc123".into(),
            intent_type: IntentType::Conditional,
            params: Some(IntentParams::Conditional(ConditionalParams {
                condition: TriggerCondition {
                    market_index: 1,
                    cmp: Comparator::Above,
                    trigger_price_e8: "5000000000000".into(),
                },
                action: OrderAction {
                    market_index: 1,
                    bucket_id: 1,
                    side: Side::Buy,
                    quantity: 100_000_000,
                    price_e8: "5000000000000".into(),
                    tif: Tif::Gtc,
                },
            })),
            vc_proof_hash: "vc-hash".into(),
            expiry_timestamp: 1_700_003_600,
            priority_boost: 5,
            status: IntentStatus::Pending,
            created_at: 1_700_000_000,
            context_data: vec![1, 2, 3],
            blob_merkle_root: vec![4, 5, 6],
        };
        let proto: proto::AgentIntent = intent.clone().into();
        let back: AgentIntent = proto.into();
        assert_eq!(intent, back);
    }

    #[test]
    fn agent_intent_helpers() {
        let mut intent = AgentIntent {
            status: IntentStatus::Pending,
            expiry_timestamp: 1_700_003_600,
            ..Default::default()
        };
        assert!(intent.is_active());
        assert!(!intent.is_terminal());
        assert!(!intent.is_expired(1_700_000_000));
        assert!(intent.is_expired(1_700_003_600));

        intent.status = IntentStatus::Completed;
        assert!(!intent.is_active());
        assert!(intent.is_terminal());
    }

    #[test]
    fn decomposition_trace_roundtrip() {
        let trace = DecompositionTrace {
            intent_id: "intent-001".into(),
            original_goal: "Maximise yield".into(),
            steps: vec!["Step 1".into(), "Step 2".into()],
            decomposed_at: 1_700_000_100,
        };
        let proto: proto::DecompositionTrace = trace.clone().into();
        let back: DecompositionTrace = proto.into();
        assert_eq!(trace, back);
    }

    #[test]
    fn params_defaults() {
        let params = Params::default();
        assert_eq!(params.default_expiry_seconds, 3600);
        assert_eq!(params.max_concurrent_intents_per_agent, 10);
        assert!(params.enable_declarative_decomposition);
        assert_eq!(params.scheduler_tick_ms, 500);
        assert!(!params.require_simulation);
        assert_eq!(params.max_decomposition_steps, 20);
        assert!(!params.rfq_enabled);
        assert!(!params.enable_intent_execution);
        assert!(params.authorized_execution_signers.is_empty());
        assert_eq!(params.max_intents_per_scan, 0);
    }

    #[test]
    fn params_roundtrip() {
        let params = Params {
            default_expiry_seconds: 7200,
            max_concurrent_intents_per_agent: 5,
            enable_declarative_decomposition: false,
            scheduler_tick_ms: 1000,
            require_simulation: true,
            max_decomposition_steps: 50,
            rfq_enabled: true,
            enable_intent_execution: true,
            authorized_execution_signers: vec!["morpheum1keeper".into()],
            max_intents_per_scan: 100,
            enable_rfq_reputation_gate: true,
            min_reputation_to_quote: 750,
        };
        let proto: proto::Params = params.clone().into();
        let back: Params = proto.into();
        assert_eq!(params, back);
    }
}
