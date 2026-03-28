//! Request wrappers for the liquidity pool module.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::liquidity::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::PoolStatus;

fn make_asset(asset_index: u64) -> morpheum_proto::primitives::v1::Asset {
    morpheum_proto::primitives::v1::Asset { asset_index, ..Default::default() }
}

// ====================== TRANSACTION REQUESTS ======================

/// Create a new liquidity pool.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreatePoolRequest {
    pub market_index: u64,
    pub asset_index: u64,
    pub initial_liquidity: String,
    pub provider_type: u32,
    pub provider_config: Vec<u8>,
    pub creator_external_address: Option<String>,
    pub creator_chain_type: Option<i32>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub logo_uri: Option<String>,
}

impl CreatePoolRequest {
    pub fn new(
        market_index: u64, asset_index: u64,
        initial_liquidity: impl Into<String>, provider_type: u32,
    ) -> Self {
        Self {
            market_index, asset_index,
            initial_liquidity: initial_liquidity.into(),
            provider_type, provider_config: Vec::new(),
            creator_external_address: None, creator_chain_type: None,
            display_name: None, description: None,
            tags: Vec::new(), logo_uri: None,
        }
    }

    pub fn provider_config(mut self, config: Vec<u8>) -> Self { self.provider_config = config; self }
    pub fn display_name(mut self, v: impl Into<String>) -> Self { self.display_name = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn tags(mut self, v: Vec<String>) -> Self { self.tags = v; self }
    pub fn logo_uri(mut self, v: impl Into<String>) -> Self { self.logo_uri = Some(v.into()); self }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::CreatePoolRequest {
            market_index: self.market_index,
            asset: Some(make_asset(self.asset_index)),
            initial_liquidity: self.initial_liquidity.clone(),
            timestamp: None,
            creator_external_address: self.creator_external_address.clone(),
            creator_chain_type: self.creator_chain_type,
            provider_type: self.provider_type,
            provider_config: self.provider_config.clone(),
            display_name: self.display_name.clone(),
            description: self.description.clone(),
            tags: self.tags.clone(),
            logo_uri: self.logo_uri.clone(),
        };
        ProtoAny { type_url: "/liquidity.v1.CreatePoolRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Update pool parameters (governance).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdatePoolParamsRequest {
    pub pool_id: String,
    pub min_deposit: String,
    pub max_deposit: String,
    pub updater_external_address: Option<String>,
    pub updater_chain_type: Option<i32>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub logo_uri: Option<String>,
}

impl UpdatePoolParamsRequest {
    pub fn new(
        pool_id: impl Into<String>,
        min_deposit: impl Into<String>,
        max_deposit: impl Into<String>,
    ) -> Self {
        Self {
            pool_id: pool_id.into(),
            min_deposit: min_deposit.into(),
            max_deposit: max_deposit.into(),
            updater_external_address: None, updater_chain_type: None,
            display_name: None, description: None,
            tags: Vec::new(), logo_uri: None,
        }
    }

    pub fn display_name(mut self, v: impl Into<String>) -> Self { self.display_name = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn tags(mut self, v: Vec<String>) -> Self { self.tags = v; self }
    pub fn logo_uri(mut self, v: impl Into<String>) -> Self { self.logo_uri = Some(v.into()); self }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::UpdatePoolParamsRequest {
            pool_id: self.pool_id.clone(),
            min_deposit: self.min_deposit.clone(),
            max_deposit: self.max_deposit.clone(),
            timestamp: None,
            updater_external_address: self.updater_external_address.clone(),
            updater_chain_type: self.updater_chain_type,
            display_name: self.display_name.clone(),
            description: self.description.clone(),
            tags: self.tags.clone(),
            logo_uri: self.logo_uri.clone(),
        };
        ProtoAny { type_url: "/liquidity.v1.UpdatePoolParamsRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Rebalance pool depth.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RebalancePoolRequest {
    pub pool_id: String,
    pub target_liquidity: String,
    pub rebalancer_external_address: Option<String>,
    pub rebalancer_chain_type: Option<i32>,
}

impl RebalancePoolRequest {
    pub fn new(pool_id: impl Into<String>, target_liquidity: impl Into<String>) -> Self {
        Self {
            pool_id: pool_id.into(), target_liquidity: target_liquidity.into(),
            rebalancer_external_address: None, rebalancer_chain_type: None,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::RebalancePoolRequest {
            pool_id: self.pool_id.clone(),
            target_liquidity: self.target_liquidity.clone(),
            timestamp: None,
            rebalancer_external_address: self.rebalancer_external_address.clone(),
            rebalancer_chain_type: self.rebalancer_chain_type,
        };
        ProtoAny { type_url: "/liquidity.v1.RebalancePoolRequest".into(), value: msg.encode_to_vec() }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query a specific pool by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetPoolRequest {
    pub pool_id: String,
}

impl GetPoolRequest {
    pub fn new(pool_id: impl Into<String>) -> Self { Self { pool_id: pool_id.into() } }
}

impl From<GetPoolRequest> for proto::GetPoolRequest {
    fn from(r: GetPoolRequest) -> Self { Self { pool_id: r.pool_id } }
}

/// List all pools with pagination and optional status filter.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListPoolsRequest {
    pub offset: u64,
    pub limit: u64,
    pub count_total: bool,
    pub status_filter: Option<PoolStatus>,
}

impl ListPoolsRequest {
    pub fn new(offset: u64, limit: u64) -> Self {
        Self { offset, limit, count_total: false, status_filter: None }
    }
    pub fn count_total(mut self) -> Self { self.count_total = true; self }
    pub fn status_filter(mut self, s: PoolStatus) -> Self { self.status_filter = Some(s); self }
}

impl From<ListPoolsRequest> for proto::ListPoolsRequest {
    fn from(r: ListPoolsRequest) -> Self {
        Self {
            pagination: Some(morpheum_proto::primitives::v1::PageRequest {
                offset: r.offset, limit: r.limit, count_total: r.count_total,
            }),
            status_filter: r.status_filter.map(i32::from),
        }
    }
}

/// Query pools for a specific market with pagination.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetPoolsByMarketRequest {
    pub market_index: u64,
    pub offset: u64,
    pub limit: u64,
    pub count_total: bool,
}

impl GetPoolsByMarketRequest {
    pub fn new(market_index: u64, offset: u64, limit: u64) -> Self {
        Self { market_index, offset, limit, count_total: false }
    }
    pub fn count_total(mut self) -> Self { self.count_total = true; self }
}

impl From<GetPoolsByMarketRequest> for proto::GetPoolsByMarketRequest {
    fn from(r: GetPoolsByMarketRequest) -> Self {
        Self {
            market_index: r.market_index,
            pagination: Some(morpheum_proto::primitives::v1::PageRequest {
                offset: r.offset, limit: r.limit, count_total: r.count_total,
            }),
        }
    }
}

/// Query depth metrics for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetDepthMetricsRequest {
    pub market_index: u64,
}

impl GetDepthMetricsRequest {
    pub fn new(market_index: u64) -> Self { Self { market_index } }
}

impl From<GetDepthMetricsRequest> for proto::GetDepthMetricsRequest {
    fn from(r: GetDepthMetricsRequest) -> Self { Self { market_index: r.market_index } }
}

/// Query pool health by pool ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetPoolHealthRequest {
    pub pool_id: String,
}

impl GetPoolHealthRequest {
    pub fn new(pool_id: impl Into<String>) -> Self { Self { pool_id: pool_id.into() } }
}

impl From<GetPoolHealthRequest> for proto::GetPoolHealthRequest {
    fn from(r: GetPoolHealthRequest) -> Self { Self { pool_id: r.pool_id } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_pool_to_any() {
        let any = CreatePoolRequest::new(1, 2, "1000", 1).to_any();
        assert_eq!(any.type_url, "/liquidity.v1.CreatePoolRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn rebalance_pool_to_any() {
        let any = RebalancePoolRequest::new("pool1", "2000").to_any();
        assert_eq!(any.type_url, "/liquidity.v1.RebalancePoolRequest");
    }

    #[test]
    fn list_pools_with_filter() {
        let p: proto::ListPoolsRequest = ListPoolsRequest::new(0, 50)
            .count_total()
            .status_filter(PoolStatus::Active)
            .into();
        assert!(p.pagination.unwrap().count_total);
        assert_eq!(p.status_filter, Some(1));
    }

    #[test]
    fn get_pools_by_market_conversion() {
        let p: proto::GetPoolsByMarketRequest = GetPoolsByMarketRequest::new(42, 0, 20).into();
        assert_eq!(p.market_index, 42);
    }

    #[test]
    fn get_depth_metrics_conversion() {
        let p: proto::GetDepthMetricsRequest = GetDepthMetricsRequest::new(42).into();
        assert_eq!(p.market_index, 42);
    }
}
