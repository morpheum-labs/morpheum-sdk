//! Request wrappers for the insurance vault module.
//!
//! Transaction requests include `to_any()` for `TxBuilder` integration.
//! Query requests convert to proto via `From` impls.

use alloc::string::String;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::insurance::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::ChainType;

fn make_asset(asset_index: u64) -> morpheum_proto::primitives::v1::Asset {
    morpheum_proto::primitives::v1::Asset { asset_index, ..Default::default() }
}

// ====================== TRANSACTION REQUESTS ======================

/// Absorb a liquidation deficit into the insurance vault.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AbsorbDeficitRequest {
    pub position_id: String,
    pub market_index: u64,
    pub asset_index: u64,
    pub deficit_amount: String,
    pub recovered_amount: String,
    pub absorber_address: String,
    pub absorber_external_address: Option<String>,
    pub absorber_chain_type: Option<ChainType>,
}

impl AbsorbDeficitRequest {
    pub fn new(
        position_id: impl Into<String>,
        market_index: u64,
        asset_index: u64,
        deficit_amount: impl Into<String>,
        recovered_amount: impl Into<String>,
        absorber_address: impl Into<String>,
    ) -> Self {
        Self {
            position_id: position_id.into(),
            market_index,
            asset_index,
            deficit_amount: deficit_amount.into(),
            recovered_amount: recovered_amount.into(),
            absorber_address: absorber_address.into(),
            absorber_external_address: None,
            absorber_chain_type: None,
        }
    }

    pub fn absorber_external(mut self, addr: impl Into<String>, chain: ChainType) -> Self {
        self.absorber_external_address = Some(addr.into());
        self.absorber_chain_type = Some(chain);
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::AbsorbDeficitRequest {
            position_id: self.position_id.clone(),
            market_index: self.market_index,
            asset: Some(make_asset(self.asset_index)),
            deficit_amount: self.deficit_amount.clone(),
            recovered_amount: self.recovered_amount.clone(),
            absorber_address: self.absorber_address.clone(),
            timestamp: None,
            absorber_external_address: self.absorber_external_address.clone(),
            absorber_chain_type: self.absorber_chain_type.map(i32::from),
        };
        ProtoAny { type_url: "/insurance.v1.AbsorbDeficitRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Replenish the vault from fees or penalties.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReplenishVaultRequest {
    pub asset_index: u64,
    pub amount: String,
    pub source: String,
    pub replenisher_address: String,
    pub replenisher_external_address: Option<String>,
    pub replenisher_chain_type: Option<ChainType>,
}

impl ReplenishVaultRequest {
    pub fn new(
        asset_index: u64,
        amount: impl Into<String>,
        source: impl Into<String>,
        replenisher_address: impl Into<String>,
    ) -> Self {
        Self {
            asset_index,
            amount: amount.into(),
            source: source.into(),
            replenisher_address: replenisher_address.into(),
            replenisher_external_address: None,
            replenisher_chain_type: None,
        }
    }

    pub fn replenisher_external(mut self, addr: impl Into<String>, chain: ChainType) -> Self {
        self.replenisher_external_address = Some(addr.into());
        self.replenisher_chain_type = Some(chain);
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::ReplenishVaultRequest {
            asset: Some(make_asset(self.asset_index)),
            amount: self.amount.clone(),
            source: self.source.clone(),
            replenisher_address: self.replenisher_address.clone(),
            timestamp: None,
            replenisher_external_address: self.replenisher_external_address.clone(),
            replenisher_chain_type: self.replenisher_chain_type.map(i32::from),
        };
        ProtoAny { type_url: "/insurance.v1.ReplenishVaultRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Stake assets into the insurance vault.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StakeToVaultRequest {
    pub address: String,
    pub asset_index: u64,
    pub amount: String,
    pub external_address: Option<String>,
}

impl StakeToVaultRequest {
    pub fn new(address: impl Into<String>, asset_index: u64, amount: impl Into<String>) -> Self {
        Self { address: address.into(), asset_index, amount: amount.into(), external_address: None }
    }

    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::StakeToVaultRequest {
            address: self.address.clone(),
            asset: Some(make_asset(self.asset_index)),
            amount: self.amount.clone(),
            timestamp: None,
            external_address: self.external_address.clone(),
        };
        ProtoAny { type_url: "/insurance.v1.StakeToVaultRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Withdraw staked assets from the insurance vault.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WithdrawStakeRequest {
    pub address: String,
    pub asset_index: u64,
    pub shares: String,
    pub external_address: Option<String>,
}

impl WithdrawStakeRequest {
    pub fn new(address: impl Into<String>, asset_index: u64, shares: impl Into<String>) -> Self {
        Self { address: address.into(), asset_index, shares: shares.into(), external_address: None }
    }

    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::WithdrawStakeRequest {
            address: self.address.clone(),
            asset: Some(make_asset(self.asset_index)),
            shares: self.shares.clone(),
            timestamp: None,
            external_address: self.external_address.clone(),
        };
        ProtoAny { type_url: "/insurance.v1.WithdrawStakeRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Claim accumulated yield from an insurance vault stake.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClaimYieldRequest {
    pub address: String,
    pub asset_index: u64,
    pub external_address: Option<String>,
}

impl ClaimYieldRequest {
    pub fn new(address: impl Into<String>, asset_index: u64) -> Self {
        Self { address: address.into(), asset_index, external_address: None }
    }

    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::ClaimYieldRequest {
            address: self.address.clone(),
            asset_index: self.asset_index,
            timestamp: None,
            external_address: self.external_address.clone(),
        };
        ProtoAny { type_url: "/insurance.v1.ClaimYieldRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Claim a bounty for absorption/liquidation work.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClaimBountyRequest {
    pub address: String,
    pub asset_index: u64,
    pub liquidation_id: String,
    pub external_address: Option<String>,
}

impl ClaimBountyRequest {
    pub fn new(
        address: impl Into<String>,
        asset_index: u64,
        liquidation_id: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            asset_index,
            liquidation_id: liquidation_id.into(),
            external_address: None,
        }
    }

    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::ClaimBountyRequest {
            address: self.address.clone(),
            asset: Some(make_asset(self.asset_index)),
            liquidation_id: self.liquidation_id.clone(),
            timestamp: None,
            external_address: self.external_address.clone(),
        };
        ProtoAny { type_url: "/insurance.v1.ClaimBountyRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Hedge impermanent loss.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HedgeIlRequest {
    pub address: String,
    pub asset_index: u64,
    pub amount: String,
    pub external_address: Option<String>,
}

impl HedgeIlRequest {
    pub fn new(address: impl Into<String>, asset_index: u64, amount: impl Into<String>) -> Self {
        Self { address: address.into(), asset_index, amount: amount.into(), external_address: None }
    }

    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::HedgeIlRequest {
            address: self.address.clone(),
            asset: Some(make_asset(self.asset_index)),
            amount: self.amount.clone(),
            timestamp: None,
            external_address: self.external_address.clone(),
        };
        ProtoAny { type_url: "/insurance.v1.HedgeILRequest".into(), value: msg.encode_to_vec() }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query vault balance, optionally filtered by asset.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetVaultBalanceRequest {
    pub asset_index: Option<u64>,
}

impl GetVaultBalanceRequest {
    pub fn new(asset_index: Option<u64>) -> Self { Self { asset_index } }
}

impl From<GetVaultBalanceRequest> for proto::GetVaultBalanceRequest {
    fn from(r: GetVaultBalanceRequest) -> Self {
        Self { asset: r.asset_index.map(make_asset) }
    }
}

/// Query a specific LP stake by address and optional asset.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetLpStakeRequest {
    pub address: String,
    pub asset_index: Option<u64>,
    pub external_address: Option<String>,
    pub chain_type: Option<ChainType>,
}

impl GetLpStakeRequest {
    pub fn new(address: impl Into<String>) -> Self {
        Self { address: address.into(), asset_index: None, external_address: None, chain_type: None }
    }

    pub fn asset_index(mut self, idx: u64) -> Self { self.asset_index = Some(idx); self }

    pub fn external(mut self, addr: impl Into<String>, chain: ChainType) -> Self {
        self.external_address = Some(addr.into());
        self.chain_type = Some(chain);
        self
    }
}

impl From<GetLpStakeRequest> for proto::GetLpStakeRequest {
    fn from(r: GetLpStakeRequest) -> Self {
        Self {
            address: r.address,
            asset: r.asset_index.map(make_asset),
            external_address: r.external_address,
            chain_type: r.chain_type.map(i32::from),
        }
    }
}

/// List all LP stakes with pagination.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListLpStakesRequest {
    pub offset: u64,
    pub limit: u64,
    pub count_total: bool,
}

impl ListLpStakesRequest {
    pub fn new(offset: u64, limit: u64) -> Self { Self { offset, limit, count_total: false } }
    pub fn count_total(mut self) -> Self { self.count_total = true; self }
}

impl From<ListLpStakesRequest> for proto::ListLpStakesRequest {
    fn from(r: ListLpStakesRequest) -> Self {
        Self {
            pagination: Some(morpheum_proto::primitives::v1::PageRequest {
                offset: r.offset, limit: r.limit, count_total: r.count_total,
            }),
        }
    }
}

/// Query bad debt history with pagination and optional time bounds.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetBadDebtHistoryRequest {
    pub offset: u64,
    pub limit: u64,
    pub count_total: bool,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
}

impl GetBadDebtHistoryRequest {
    pub fn new(offset: u64, limit: u64) -> Self {
        Self { offset, limit, count_total: false, start_time: None, end_time: None }
    }

    pub fn count_total(mut self) -> Self { self.count_total = true; self }
    pub fn start_time(mut self, ts: u64) -> Self { self.start_time = Some(ts); self }
    pub fn end_time(mut self, ts: u64) -> Self { self.end_time = Some(ts); self }
}

impl From<GetBadDebtHistoryRequest> for proto::GetBadDebtHistoryRequest {
    fn from(r: GetBadDebtHistoryRequest) -> Self {
        Self {
            pagination: Some(morpheum_proto::primitives::v1::PageRequest {
                offset: r.offset, limit: r.limit, count_total: r.count_total,
            }),
            start_time: r.start_time.map(|s| morpheum_proto::google::protobuf::Timestamp { seconds: s as i64, nanos: 0 }),
            end_time: r.end_time.map(|s| morpheum_proto::google::protobuf::Timestamp { seconds: s as i64, nanos: 0 }),
        }
    }
}

/// Query IL metrics, optionally filtered by asset.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetIlMetricsRequest {
    pub asset_index: Option<u64>,
}

impl GetIlMetricsRequest {
    pub fn new(asset_index: Option<u64>) -> Self { Self { asset_index } }
}

impl From<GetIlMetricsRequest> for proto::GetIlMetricsRequest {
    fn from(r: GetIlMetricsRequest) -> Self {
        Self { asset: r.asset_index.map(make_asset) }
    }
}

/// Query threshold status, optionally filtered by asset.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetThresholdStatusRequest {
    pub asset_index: Option<u64>,
}

impl GetThresholdStatusRequest {
    pub fn new(asset_index: Option<u64>) -> Self { Self { asset_index } }
}

impl From<GetThresholdStatusRequest> for proto::GetThresholdStatusRequest {
    fn from(r: GetThresholdStatusRequest) -> Self {
        Self { asset: r.asset_index.map(make_asset) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absorb_deficit_to_any() {
        let req = AbsorbDeficitRequest::new("pos1", 42, 1, "1000", "500", "morph1xyz");
        let any = req.to_any();
        assert_eq!(any.type_url, "/insurance.v1.AbsorbDeficitRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn absorb_deficit_with_external() {
        let req = AbsorbDeficitRequest::new("pos1", 42, 1, "1000", "500", "morph1xyz")
            .absorber_external("0xdead", ChainType::Ethereum);
        assert_eq!(req.absorber_chain_type, Some(ChainType::Ethereum));
    }

    #[test]
    fn stake_to_vault_to_any() {
        let any = StakeToVaultRequest::new("morph1xyz", 1, "1000").to_any();
        assert_eq!(any.type_url, "/insurance.v1.StakeToVaultRequest");
    }

    #[test]
    fn query_vault_balance_conversion() {
        let p: proto::GetVaultBalanceRequest = GetVaultBalanceRequest::new(Some(1)).into();
        assert!(p.asset.is_some());
        assert_eq!(p.asset.unwrap().asset_index, 1);
    }

    #[test]
    fn query_vault_balance_none() {
        let p: proto::GetVaultBalanceRequest = GetVaultBalanceRequest::new(None).into();
        assert!(p.asset.is_none());
    }

    #[test]
    fn list_lp_stakes_conversion() {
        let p: proto::ListLpStakesRequest = ListLpStakesRequest::new(0, 50).count_total().into();
        let pag = p.pagination.unwrap();
        assert_eq!(pag.limit, 50);
        assert!(pag.count_total);
    }

    #[test]
    fn bad_debt_history_with_time_bounds() {
        let p: proto::GetBadDebtHistoryRequest =
            GetBadDebtHistoryRequest::new(0, 20).start_time(1_700_000_000).end_time(1_700_100_000).into();
        assert!(p.start_time.is_some());
        assert_eq!(p.start_time.unwrap().seconds, 1_700_000_000);
    }
}
