//! Request wrappers for the TWAP module.

use alloc::string::String;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::twap::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::MarketTwapConfig;

// ====================== TRANSACTION REQUESTS ======================

/// Update per-market TWAP configuration (governance).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateTwapConfigRequest {
    pub authority: String,
    pub market_index: u64,
    pub config: MarketTwapConfig,
}

impl UpdateTwapConfigRequest {
    pub fn new(
        authority: impl Into<String>,
        market_index: u64,
        config: MarketTwapConfig,
    ) -> Self {
        Self {
            authority: authority.into(),
            market_index,
            config,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgUpdateTwapConfig {
            authority: self.authority.clone(),
            market_index: self.market_index,
            config: Some(self.config.clone().into()),
        };
        ProtoAny { type_url: "/twap.v1.MsgUpdateTwapConfig".into(), value: msg.encode_to_vec() }
    }
}

// ====================== QUERY REQUESTS ======================

/// Get TWAP value for a market and window.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetTwapRequest {
    pub market_index: u64,
    pub window_blocks: u32,
    pub current_block: u64,
}

impl GetTwapRequest {
    pub fn new(market_index: u64, window_blocks: u32) -> Self {
        Self { market_index, window_blocks, current_block: 0 }
    }

    /// Set the current block for staleness checking.
    pub fn current_block(mut self, v: u64) -> Self { self.current_block = v; self }
}

impl From<GetTwapRequest> for proto::GetTwapRequest {
    fn from(r: GetTwapRequest) -> Self {
        Self {
            market_index: r.market_index,
            window_blocks: r.window_blocks,
            current_block: r.current_block,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn update_config_to_any() {
        let cfg = MarketTwapConfig { windows: vec![60, 300], staleness_blocks: 10 };
        let any = UpdateTwapConfigRequest::new("morpheum1gov", 1, cfg).to_any();
        assert_eq!(any.type_url, "/twap.v1.MsgUpdateTwapConfig");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn get_twap_conversion() {
        let p: proto::GetTwapRequest = GetTwapRequest::new(1, 300).current_block(5000).into();
        assert_eq!(p.market_index, 1);
        assert_eq!(p.window_blocks, 300);
        assert_eq!(p.current_block, 5000);
    }

    #[test]
    fn get_twap_default_current_block() {
        let p: proto::GetTwapRequest = GetTwapRequest::new(2, 60).into();
        assert_eq!(p.current_block, 0);
    }
}
