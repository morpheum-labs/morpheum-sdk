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
}

impl IntentType {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Twap,
            2 => Self::MultiLeg,
            3 => Self::Declarative,
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
            _ => Self::Pending,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }

    /// Returns `true` if the intent is in a terminal state.
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled | Self::Expired)
    }

    /// Returns `true` if the intent is still live (pending or executing).
    pub fn is_active(self) -> bool {
        matches!(self, Self::Pending | Self::Executing)
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
        }
    }
}

// ====================== INTENT PARAMETER TYPES ======================

/// Conditional intent parameters.
///
/// Execute an action when a condition is met (e.g. "price > 42000").
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConditionalParams {
    /// Condition expression (e.g. `"price > 42000"` or encoded condition).
    pub condition: String,
    /// Action to execute when the condition is met (e.g. `"market_buy 0.5 BTC"`).
    pub action: String,
}

impl From<proto::ConditionalParams> for ConditionalParams {
    fn from(p: proto::ConditionalParams) -> Self {
        Self {
            condition: p.condition,
            action: p.action,
        }
    }
}

impl From<ConditionalParams> for proto::ConditionalParams {
    fn from(c: ConditionalParams) -> Self {
        Self {
            condition: c.condition,
            action: c.action,
        }
    }
}

/// TWAP (Time-Weighted Average Price) intent parameters.
///
/// Splits a large order into time-sliced portions to minimise market impact.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TwapParams {
    /// Direction: `"buy"` or `"sell"`.
    pub direction: String,
    /// Total order size (in base units / satoshis).
    pub total_size: u64,
    /// Duration over which to execute (milliseconds).
    pub duration_ms: u64,
    /// Number of slices to divide the order into.
    pub num_slices: u32,
    /// Slice curve: `"linear"`, `"front_loaded"`, `"back_loaded"`, `"adaptive"`.
    pub slice_curve: String,
    /// Maximum slippage tolerance in basis points.
    pub slippage_tolerance_bps: u32,
    /// Optional condition that triggers rebalancing between slices.
    pub rebalance_trigger: String,
}

impl From<proto::TwapParams> for TwapParams {
    fn from(p: proto::TwapParams) -> Self {
        Self {
            direction: p.direction,
            total_size: p.total_size,
            duration_ms: p.duration_ms,
            num_slices: p.num_slices,
            slice_curve: p.slice_curve,
            slippage_tolerance_bps: p.slippage_tolerance_bps,
            rebalance_trigger: p.rebalance_trigger,
        }
    }
}

impl From<TwapParams> for proto::TwapParams {
    fn from(t: TwapParams) -> Self {
        Self {
            direction: t.direction,
            total_size: t.total_size,
            duration_ms: t.duration_ms,
            num_slices: t.num_slices,
            slice_curve: t.slice_curve,
            slippage_tolerance_bps: t.slippage_tolerance_bps,
            rebalance_trigger: t.rebalance_trigger,
        }
    }
}

/// A single leg in a multi-leg intent.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Leg {
    /// Action to perform (e.g. `"buy"`, `"sell"`).
    pub action: String,
    /// Size of the leg (in base units / satoshis).
    pub size: u64,
    /// Trading pair (e.g. `"BTC-USDC"`).
    pub pair: String,
}

impl From<proto::Leg> for Leg {
    fn from(p: proto::Leg) -> Self {
        Self {
            action: p.action,
            size: p.size,
            pair: p.pair,
        }
    }
}

impl From<Leg> for proto::Leg {
    fn from(l: Leg) -> Self {
        Self {
            action: l.action,
            size: l.size,
            pair: l.pair,
        }
    }
}

/// Multi-leg atomic intent parameters.
///
/// All legs are executed atomically (all-or-nothing) when `atomic` is `true`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MultiLegParams {
    /// Individual legs of the intent.
    pub legs: Vec<Leg>,
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
    /// Multi-leg: atomic correlated actions.
    MultiLeg(MultiLegParams),
    /// Declarative: high-level goal decomposition.
    Declarative(DeclarativeParams),
}

impl From<proto::agent_intent::Params> for IntentParams {
    fn from(p: proto::agent_intent::Params) -> Self {
        match p {
            proto::agent_intent::Params::Conditional(c) => Self::Conditional(c.into()),
            proto::agent_intent::Params::Twap(t) => Self::Twap(t.into()),
            proto::agent_intent::Params::MultiLeg(m) => Self::MultiLeg(m.into()),
            proto::agent_intent::Params::Declarative(d) => Self::Declarative(d.into()),
        }
    }
}

impl From<IntentParams> for proto::agent_intent::Params {
    fn from(p: IntentParams) -> Self {
        match p {
            IntentParams::Conditional(c) => Self::Conditional(c.into()),
            IntentParams::Twap(t) => Self::Twap(t.into()),
            IntentParams::MultiLeg(m) => Self::MultiLeg(m.into()),
            IntentParams::Declarative(d) => Self::Declarative(d.into()),
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
}

impl AgentIntent {
    /// Returns `true` if the intent is still active (pending or executing).
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
            context_data: Vec::new(),
            blob_merkle_root: Vec::new(),
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
        ] {
            assert_eq!(IntentStatus::from_proto(s.to_proto()), s);
        }
    }

    #[test]
    fn intent_status_helpers() {
        assert!(IntentStatus::Pending.is_active());
        assert!(IntentStatus::Executing.is_active());
        assert!(!IntentStatus::Completed.is_active());
        assert!(IntentStatus::Completed.is_terminal());
        assert!(IntentStatus::Failed.is_terminal());
        assert!(IntentStatus::Cancelled.is_terminal());
        assert!(IntentStatus::Expired.is_terminal());
        assert!(!IntentStatus::Pending.is_terminal());
    }

    #[test]
    fn conditional_params_roundtrip() {
        let params = ConditionalParams {
            condition: "price > 42000".into(),
            action: "market_buy 0.5 BTC".into(),
        };
        let proto: proto::ConditionalParams = params.clone().into();
        let back: ConditionalParams = proto.into();
        assert_eq!(params, back);
    }

    #[test]
    fn twap_params_roundtrip() {
        let params = TwapParams {
            direction: "buy".into(),
            total_size: 100_000,
            duration_ms: 60_000,
            num_slices: 10,
            slice_curve: "linear".into(),
            slippage_tolerance_bps: 50,
            rebalance_trigger: String::new(),
        };
        let proto: proto::TwapParams = params.clone().into();
        let back: TwapParams = proto.into();
        assert_eq!(params, back);
    }

    #[test]
    fn multi_leg_params_roundtrip() {
        let params = MultiLegParams {
            legs: vec![
                Leg { action: "buy".into(), size: 1000, pair: "BTC-USDC".into() },
                Leg { action: "sell".into(), size: 500, pair: "ETH-USDC".into() },
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
                condition: "price > 50000".into(),
                action: "buy 1 BTC".into(),
            })),
            vc_proof_hash: "vc-hash".into(),
            expiry_timestamp: 1_700_003_600,
            priority_boost: 5,
            status: IntentStatus::Pending,
            created_at: 1_700_000_000,
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
        };
        let proto: proto::Params = params.clone().into();
        let back: Params = proto.into();
        assert_eq!(params, back);
    }
}
