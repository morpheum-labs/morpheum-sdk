//! Request wrappers for the OSA module.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::osa::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

// ====================== TRANSACTION REQUESTS ======================

/// Create a per-outcome settlement account.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateAccountRequest {
    pub creator: String,
    pub market_index: u64,
    pub outcome_id: String,
    pub collateral_asset_index: u64,
    pub initial_collateral: u64,
}

impl CreateAccountRequest {
    pub fn new(
        creator: impl Into<String>, market_index: u64,
        outcome_id: impl Into<String>, collateral_asset_index: u64,
        initial_collateral: u64,
    ) -> Self {
        Self {
            creator: creator.into(), market_index, outcome_id: outcome_id.into(),
            collateral_asset_index, initial_collateral,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgCreateOutcomeSettlementAccount {
            creator: self.creator.clone(), market_index: self.market_index,
            outcome_id: self.outcome_id.clone(), collateral_asset_index: self.collateral_asset_index,
            initial_collateral: self.initial_collateral,
        };
        ProtoAny { type_url: "/osa.v1.MsgCreateOutcomeSettlementAccount".into(), value: msg.encode_to_vec() }
    }
}

/// Buy shares — collateral in, shares out (1:1 minus hook fee).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuySharesRequest {
    pub buyer: String,
    pub account_id: String,
    pub collateral_amount: u64,
    pub min_shares_received: u64,
    pub max_fee_bps: u32,
}

impl BuySharesRequest {
    pub fn new(
        buyer: impl Into<String>, account_id: impl Into<String>,
        collateral_amount: u64, min_shares_received: u64, max_fee_bps: u32,
    ) -> Self {
        Self {
            buyer: buyer.into(), account_id: account_id.into(),
            collateral_amount, min_shares_received, max_fee_bps,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgBuyShares {
            buyer: self.buyer.clone(), account_id: self.account_id.clone(),
            collateral_amount: self.collateral_amount, min_shares_received: self.min_shares_received,
            max_fee_bps: self.max_fee_bps,
        };
        ProtoAny { type_url: "/osa.v1.MsgBuyShares".into(), value: msg.encode_to_vec() }
    }
}

/// Sell shares — shares in, collateral out (1:1).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SellSharesRequest {
    pub seller: String,
    pub account_id: String,
    pub shares_amount: u64,
    pub min_collateral_received: u64,
}

impl SellSharesRequest {
    pub fn new(
        seller: impl Into<String>, account_id: impl Into<String>,
        shares_amount: u64, min_collateral_received: u64,
    ) -> Self {
        Self {
            seller: seller.into(), account_id: account_id.into(),
            shares_amount, min_collateral_received,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgSellShares {
            seller: self.seller.clone(), account_id: self.account_id.clone(),
            shares_amount: self.shares_amount, min_collateral_received: self.min_collateral_received,
        };
        ProtoAny { type_url: "/osa.v1.MsgSellShares".into(), value: msg.encode_to_vec() }
    }
}

/// Merge positions — burn shares across accounts, reclaim collateral.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MergePositionsRequest {
    pub merger: String,
    pub account_ids: Vec<String>,
    pub min_collateral_received: u64,
}

impl MergePositionsRequest {
    pub fn new(
        merger: impl Into<String>, account_ids: Vec<String>,
        min_collateral_received: u64,
    ) -> Self {
        Self { merger: merger.into(), account_ids, min_collateral_received }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgMergePositions {
            merger: self.merger.clone(), account_ids: self.account_ids.clone(),
            min_collateral_received: self.min_collateral_received,
        };
        ProtoAny { type_url: "/osa.v1.MsgMergePositions".into(), value: msg.encode_to_vec() }
    }
}

/// Claim payout — winners redeem shares for collateral.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClaimPayoutRequest {
    pub claimer: String,
    pub beneficiary: String,
    pub account_id: String,
}

impl ClaimPayoutRequest {
    pub fn new(
        claimer: impl Into<String>, beneficiary: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Self {
        Self { claimer: claimer.into(), beneficiary: beneficiary.into(), account_id: account_id.into() }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgClaimPayout {
            claimer: self.claimer.clone(), beneficiary: self.beneficiary.clone(),
            account_id: self.account_id.clone(),
        };
        ProtoAny { type_url: "/osa.v1.MsgClaimPayout".into(), value: msg.encode_to_vec() }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query a single outcome settlement account by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetAccountRequest {
    pub account_id: String,
}

impl GetAccountRequest {
    pub fn new(account_id: impl Into<String>) -> Self { Self { account_id: account_id.into() } }
}

impl From<GetAccountRequest> for proto::QueryGetAccountRequest {
    fn from(r: GetAccountRequest) -> Self { Self { account_id: r.account_id } }
}

/// Query user share balance within an outcome settlement account.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetBalanceRequest {
    pub account_id: String,
    pub address: String,
}

impl GetBalanceRequest {
    pub fn new(account_id: impl Into<String>, address: impl Into<String>) -> Self {
        Self { account_id: account_id.into(), address: address.into() }
    }
}

impl From<GetBalanceRequest> for proto::QueryGetBalanceRequest {
    fn from(r: GetBalanceRequest) -> Self { Self { account_id: r.account_id, address: r.address } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_account_to_any() {
        let any = CreateAccountRequest::new("morph1xyz", 1, "yes", 2, 1000).to_any();
        assert_eq!(any.type_url, "/osa.v1.MsgCreateOutcomeSettlementAccount");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn buy_shares_to_any() {
        let any = BuySharesRequest::new("morph1xyz", "acct1", 1000, 990, 100).to_any();
        assert_eq!(any.type_url, "/osa.v1.MsgBuyShares");
    }

    #[test]
    fn sell_shares_to_any() {
        let any = SellSharesRequest::new("morph1xyz", "acct1", 500, 490).to_any();
        assert_eq!(any.type_url, "/osa.v1.MsgSellShares");
    }

    #[test]
    fn claim_payout_to_any() {
        let any = ClaimPayoutRequest::new("morph1xyz", "morph1ben", "acct1").to_any();
        assert_eq!(any.type_url, "/osa.v1.MsgClaimPayout");
    }

    #[test]
    fn query_conversions() {
        let p: proto::QueryGetAccountRequest = GetAccountRequest::new("acct1").into();
        assert_eq!(p.account_id, "acct1");

        let p: proto::QueryGetBalanceRequest = GetBalanceRequest::new("acct1", "morph1xyz").into();
        assert_eq!(p.address, "morph1xyz");
    }
}
