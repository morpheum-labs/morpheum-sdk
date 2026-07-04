//! Request wrappers for the risk module.

use alloc::string::String;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::risk::v1 as proto;

use crate::types::RiskParams;

// ====================== TRANSACTION REQUESTS ======================

/// Trigger an epoch-level risk tick for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TriggerLiquidationRequest {
    pub market_index: u64,
    pub bucket_id: u64,
}

impl TriggerLiquidationRequest {
    pub fn new(market_index: u64, bucket_id: u64) -> Self {
        Self {
            market_index,
            bucket_id,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgTriggerLiquidation {
            market_index: self.market_index,
            bucket_id: self.bucket_id,
        };
        ProtoAny {
            type_url: "/risk.v1.MsgTriggerLiquidation".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Update the risk module parameters (governance).
///
/// `MsgUpdateParams` is a full-replace write: submitting this clears every
/// sub-config not carried in `params`. Seed `params` from the current
/// on-chain value (see [`crate::client::RiskClient::get_params`]) rather than
/// starting from a fresh/default [`RiskParams`], unless you intend to reset
/// every sub-config.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsRequest {
    pub authority: String,
    pub params: RiskParams,
}

impl UpdateParamsRequest {
    pub fn new(authority: impl Into<String>, params: RiskParams) -> Self {
        Self {
            authority: authority.into(),
            params,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgUpdateParams {
            authority: self.authority.clone(),
            params: Some(self.params.clone().into()),
        };
        ProtoAny {
            type_url: "/risk.v1.MsgUpdateParams".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Query the current risk module governance parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetParamsRequest;

impl From<GetParamsRequest> for proto::QueryParamsRequest {
    fn from(_: GetParamsRequest) -> Self {
        Self {}
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
    pub fn new(market_index: u64, depth: u32) -> Self {
        Self {
            market_index,
            depth,
        }
    }
}

impl From<GetHeatmapRequest> for proto::GetHeatmapRequest {
    fn from(r: GetHeatmapRequest) -> Self {
        Self {
            market_index: r.market_index,
            depth: r.depth,
        }
    }
}

/// Query OI ratio for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetOiRatioRequest {
    pub market_index: u64,
}

impl GetOiRatioRequest {
    pub fn new(market_index: u64) -> Self {
        Self { market_index }
    }
}

impl From<GetOiRatioRequest> for proto::GetOiRatioRequest {
    fn from(r: GetOiRatioRequest) -> Self {
        Self {
            market_index: r.market_index,
        }
    }
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
        market_index: u64,
        size: impl Into<String>,
        entry_price: u64,
        is_long: bool,
        leverage: u64,
        mark_price: u64,
    ) -> Self {
        Self {
            market_index,
            size: size.into(),
            entry_price,
            is_long,
            leverage,
            mark_price,
        }
    }
}

impl From<GetMaintenanceMarginRequest> for proto::GetMaintenanceMarginRequest {
    fn from(r: GetMaintenanceMarginRequest) -> Self {
        Self {
            market_index: r.market_index,
            size: r.size,
            entry_price: r.entry_price,
            is_long: r.is_long,
            leverage: r.leverage,
            mark_price: r.mark_price,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RiskConfig;

    #[test]
    fn trigger_liquidation_to_any() {
        let any = TriggerLiquidationRequest::new(0, 1).to_any();
        assert_eq!(any.type_url, "/risk.v1.MsgTriggerLiquidation");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn update_params_to_any() {
        let any = UpdateParamsRequest::new(
            "morpheum1gov",
            RiskParams {
                config: RiskConfig {
                    band_width_bps: 100,
                    num_bands_above_below: 10,
                    imbalance_threshold_bps: 500,
                    imbalance_hysteresis_bps: 100,
                    max_scan_limit: 100,
                    liquidation_margin_ratio_bps: 500,
                    partial_band_shift_enabled: true,
                },
                auction_params: None,
                spot_risk: None,
                spot_collateral: None,
                tiered_margin: None,
                portfolio_var: None,
                clmm_collateral: None,
            },
        )
        .to_any();
        assert_eq!(any.type_url, "/risk.v1.MsgUpdateParams");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn get_params_request_conversion() {
        let p: proto::QueryParamsRequest = GetParamsRequest.into();
        assert_eq!(p, proto::QueryParamsRequest {});
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
