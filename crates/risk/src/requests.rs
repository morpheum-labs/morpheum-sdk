//! Request wrappers for the risk module.

use alloc::string::String;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::risk::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::RiskConfig;

// ====================== TRANSACTION REQUESTS ======================

/// Trigger an epoch-level risk tick for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EpochRiskTickRequest {
    pub epoch_id: u64,
    pub market_index: u64,
    pub logical_timestamp: u64,
}

impl EpochRiskTickRequest {
    pub fn new(epoch_id: u64, market_index: u64, logical_timestamp: u64) -> Self {
        Self { epoch_id, market_index, logical_timestamp }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgEpochRiskTick {
            epoch_id: self.epoch_id, market_index: self.market_index,
            logical_timestamp: self.logical_timestamp,
        };
        ProtoAny { type_url: "/risk.v1.MsgEpochRiskTick".into(), value: msg.encode_to_vec() }
    }
}

/// Trigger a liquidation check for a market at a given mark price.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LiquidationCheckRequest {
    pub market_index: u64,
    pub mark_price: u64,
    pub logical_timestamp: u64,
}

impl LiquidationCheckRequest {
    pub fn new(market_index: u64, mark_price: u64, logical_timestamp: u64) -> Self {
        Self { market_index, mark_price, logical_timestamp }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgLiquidationCheck {
            market_index: self.market_index, mark_price: self.mark_price,
            logical_timestamp: self.logical_timestamp,
        };
        ProtoAny { type_url: "/risk.v1.MsgLiquidationCheck".into(), value: msg.encode_to_vec() }
    }
}

/// Update the risk module configuration (governance).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateRiskConfigRequest {
    pub config: RiskConfig,
}

impl UpdateRiskConfigRequest {
    pub fn new(config: RiskConfig) -> Self { Self { config } }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgUpdateRiskConfig {
            config: Some(self.config.clone().into()),
        };
        ProtoAny { type_url: "/risk.v1.MsgUpdateRiskConfig".into(), value: msg.encode_to_vec() }
    }
}

/// Report a liquidation shortfall.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ShortfallReportRequest {
    pub bucket_id: u64,
    pub liquidation_id: u64,
    pub market_index: u64,
    pub shortfall_amount: u64,
}

impl ShortfallReportRequest {
    pub fn new(bucket_id: u64, liquidation_id: u64, market_index: u64, shortfall_amount: u64) -> Self {
        Self { bucket_id, liquidation_id, market_index, shortfall_amount }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgShortfallReport {
            bucket_id: self.bucket_id, liquidation_id: self.liquidation_id,
            market_index: self.market_index, shortfall_amount: self.shortfall_amount,
        };
        ProtoAny { type_url: "/risk.v1.MsgShortfallReport".into(), value: msg.encode_to_vec() }
    }
}

/// Notify that a bucket liquidation has been executed.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BucketLiquidationExecutedRequest {
    pub bucket_id: u64,
    pub liquidation_id: u64,
    pub market_index: u64,
    pub shortfall_sat: u64,
    pub block_height: u64,
    pub shard_id: u64,
}

impl BucketLiquidationExecutedRequest {
    pub fn new(
        bucket_id: u64, liquidation_id: u64, market_index: u64,
        shortfall_sat: u64, block_height: u64, shard_id: u64,
    ) -> Self {
        Self { bucket_id, liquidation_id, market_index, shortfall_sat, block_height, shard_id }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgBucketLiquidationExecuted {
            bucket_id: self.bucket_id, liquidation_id: self.liquidation_id,
            market_index: self.market_index, shortfall_sat: self.shortfall_sat,
            block_height: self.block_height, shard_id: self.shard_id,
        };
        ProtoAny { type_url: "/risk.v1.MsgBucketLiquidationExecuted".into(), value: msg.encode_to_vec() }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query heatmap data for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetHeatmapRequest {
    pub market_index: u64,
    pub depth: u32,
}

impl GetHeatmapRequest {
    pub fn new(market_index: u64, depth: u32) -> Self { Self { market_index, depth } }
}

impl From<GetHeatmapRequest> for proto::GetHeatmapRequest {
    fn from(r: GetHeatmapRequest) -> Self { Self { market_index: r.market_index, depth: r.depth } }
}

/// Query OI ratio for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetOiRatioRequest {
    pub market_index: u64,
}

impl GetOiRatioRequest {
    pub fn new(market_index: u64) -> Self { Self { market_index } }
}

impl From<GetOiRatioRequest> for proto::GetOiRatioRequest {
    fn from(r: GetOiRatioRequest) -> Self { Self { market_index: r.market_index } }
}

/// Query maintenance margin for a position.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetMaintenanceMarginRequest {
    pub market_index: u64,
    /// u128 as decimal string.
    pub size: String,
    pub entry_price: u64,
    pub is_long: bool,
    pub leverage: u64,
    pub mark_price: u64,
}

impl GetMaintenanceMarginRequest {
    pub fn new(
        market_index: u64, size: impl Into<String>, entry_price: u64,
        is_long: bool, leverage: u64, mark_price: u64,
    ) -> Self {
        Self { market_index, size: size.into(), entry_price, is_long, leverage, mark_price }
    }
}

impl From<GetMaintenanceMarginRequest> for proto::GetMaintenanceMarginRequest {
    fn from(r: GetMaintenanceMarginRequest) -> Self {
        Self {
            market_index: r.market_index, size: r.size, entry_price: r.entry_price,
            is_long: r.is_long, leverage: r.leverage, mark_price: r.mark_price,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_risk_tick_to_any() {
        let any = EpochRiskTickRequest::new(1, 0, 100).to_any();
        assert_eq!(any.type_url, "/risk.v1.MsgEpochRiskTick");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn liquidation_check_to_any() {
        let any = LiquidationCheckRequest::new(0, 50000_00000000, 100).to_any();
        assert_eq!(any.type_url, "/risk.v1.MsgLiquidationCheck");
    }

    #[test]
    fn shortfall_report_to_any() {
        let any = ShortfallReportRequest::new(1, 2, 0, 5000).to_any();
        assert_eq!(any.type_url, "/risk.v1.MsgShortfallReport");
    }

    #[test]
    fn bucket_liquidation_executed_to_any() {
        let any = BucketLiquidationExecutedRequest::new(1, 2, 0, 5000, 100, 0).to_any();
        assert_eq!(any.type_url, "/risk.v1.MsgBucketLiquidationExecuted");
    }

    #[test]
    fn query_conversions() {
        let p: proto::GetHeatmapRequest = GetHeatmapRequest::new(0, 10).into();
        assert_eq!(p.depth, 10);

        let p: proto::GetOiRatioRequest = GetOiRatioRequest::new(0).into();
        assert_eq!(p.market_index, 0);

        let p: proto::GetMaintenanceMarginRequest =
            GetMaintenanceMarginRequest::new(0, "100000", 50000, true, 10, 51000).into();
        assert_eq!(p.size, "100000");
        assert!(p.is_long);
    }
}
