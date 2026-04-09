//! Request and response wrappers for the Bank module.
//!
//! Clean, type-safe Rust APIs around the raw protobuf messages.
//! Includes `to_any()` methods for seamless integration with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::bank::v1 as proto;

use crate::types::{AssetIdentifier, ChainType};

// ====================== TRANSACTION REQUESTS ======================

/// Request to transfer assets between native addresses.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransferRequest {
    pub from_address: String,
    pub to_address: String,
    pub amount: String,
    pub asset_index: u64,
    pub memo: String,
    pub from_external_address: Option<String>,
    pub to_external_address: Option<String>,
}

impl TransferRequest {
    pub fn new(
        from_address: impl Into<String>,
        to_address: impl Into<String>,
        amount: impl Into<String>,
        asset_index: u64,
    ) -> Self {
        Self {
            from_address: from_address.into(),
            to_address: to_address.into(),
            amount: amount.into(),
            asset_index,
            memo: String::new(),
            from_external_address: None,
            to_external_address: None,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgTransferRequest = self.clone().into();
        ProtoAny {
            type_url: "/bank.v1.MsgTransferRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<TransferRequest> for proto::MsgTransferRequest {
    fn from(req: TransferRequest) -> Self {
        Self {
            from_address: req.from_address,
            to_address: req.to_address,
            amount: req.amount,
            asset_index: req.asset_index,
            memo: req.memo,
            timestamp: None,
            from_external_address: req.from_external_address,
            to_external_address: req.to_external_address,
        }
    }
}

/// Request for cross-chain asset transfer.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CrossChainTransferRequest {
    pub from_address: String,
    pub target_chain: ChainType,
    pub to_address: String,
    pub amount: String,
    pub asset: AssetIdentifier,
    pub memo: String,
    pub target_bridge_id: String,
    pub from_external_address: Option<String>,
    pub to_external_address: Option<String>,
}

impl CrossChainTransferRequest {
    pub fn new(
        from_address: impl Into<String>,
        target_chain: ChainType,
        to_address: impl Into<String>,
        amount: impl Into<String>,
        asset: AssetIdentifier,
    ) -> Self {
        Self {
            from_address: from_address.into(),
            target_chain,
            to_address: to_address.into(),
            amount: amount.into(),
            asset,
            memo: String::new(),
            target_bridge_id: String::new(),
            from_external_address: None,
            to_external_address: None,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCrossChainTransferRequest = self.clone().into();
        ProtoAny {
            type_url: "/bank.v1.MsgCrossChainTransferRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<CrossChainTransferRequest> for proto::MsgCrossChainTransferRequest {
    fn from(req: CrossChainTransferRequest) -> Self {
        let asset_identifier = match req.asset {
            AssetIdentifier::ByIndex(idx) => {
                Some(proto::msg_cross_chain_transfer_request::AssetIdentifier::AssetIndex(idx))
            }
            AssetIdentifier::BySymbol(sym) => {
                Some(proto::msg_cross_chain_transfer_request::AssetIdentifier::AssetSymbol(sym))
            }
        };
        Self {
            from_address: req.from_address,
            target_chain: i32::from(req.target_chain),
            to_address: req.to_address,
            amount: req.amount,
            memo: req.memo,
            timestamp: None,
            target_bridge_id: req.target_bridge_id,
            from_external_address: req.from_external_address,
            to_external_address: req.to_external_address,
            asset_identifier,
        }
    }
}

/// Request to transfer assets from spot balance to a perpetuals bucket.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransferToBucketRequest {
    pub address: String,
    pub bucket_id: String,
    pub asset_index: u64,
    pub amount: String,
}

impl TransferToBucketRequest {
    pub fn new(
        address: impl Into<String>,
        bucket_id: impl Into<String>,
        asset_index: u64,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            bucket_id: bucket_id.into(),
            asset_index,
            amount: amount.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgTransferToBucketRequest = self.clone().into();
        ProtoAny {
            type_url: "/bank.v1.MsgTransferToBucketRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<TransferToBucketRequest> for proto::MsgTransferToBucketRequest {
    fn from(req: TransferToBucketRequest) -> Self {
        Self {
            address: req.address,
            bucket_id: req.bucket_id,
            asset_index: req.asset_index,
            amount: req.amount,
            timestamp: None,
        }
    }
}

/// Request to mint new assets (requires permissions).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MintRequest {
    pub recipient_address: String,
    pub asset_index: u64,
    pub amount: String,
    pub permissions: Vec<String>,
    pub module_account: String,
    /// Caller identity (contract address) for mint authority validation.
    pub authority: String,
}

impl MintRequest {
    pub fn new(
        recipient_address: impl Into<String>,
        asset_index: u64,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            recipient_address: recipient_address.into(),
            asset_index,
            amount: amount.into(),
            permissions: Vec::new(),
            module_account: String::new(),
            authority: String::new(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgMintRequest = self.clone().into();
        ProtoAny {
            type_url: "/bank.v1.MsgMintRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<MintRequest> for proto::MsgMintRequest {
    fn from(req: MintRequest) -> Self {
        Self {
            recipient_address: req.recipient_address,
            asset_index: req.asset_index,
            amount: req.amount,
            timestamp: None,
            permissions: req.permissions,
            module_account: req.module_account,
            authority: req.authority,
        }
    }
}

/// Request to onboard a new asset into the registry.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OnboardAssetRequest {
    pub from_address: String,
    pub name: String,
    pub asset_symbol: String,
    pub salt: Vec<u8>,
    pub asset_type: i32,
    pub initial_supply: String,
}

impl OnboardAssetRequest {
    pub fn new(
        from_address: impl Into<String>,
        name: impl Into<String>,
        asset_symbol: impl Into<String>,
        asset_type: i32,
    ) -> Self {
        Self {
            from_address: from_address.into(),
            name: name.into(),
            asset_symbol: asset_symbol.into(),
            salt: Vec::new(),
            asset_type,
            initial_supply: String::new(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgOnboardAssetRequest = self.clone().into();
        ProtoAny {
            type_url: "/bank.v1.MsgOnboardAssetRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<OnboardAssetRequest> for proto::MsgOnboardAssetRequest {
    fn from(req: OnboardAssetRequest) -> Self {
        Self {
            from_address: req.from_address,
            name: req.name,
            asset_symbol: req.asset_symbol,
            salt: req.salt,
            asset_type: req.asset_type,
            initial_supply: req.initial_supply,
            timestamp: None,
        }
    }
}

/// Request to bridge an asset to/from a VM.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BridgeAssetRequest {
    pub from_address: String,
    pub to_vm_address: String,
    pub asset: AssetIdentifier,
    pub amount: String,
    pub from_external_address: Option<String>,
    pub chain_type: Option<ChainType>,
    pub unify: bool,
}

impl BridgeAssetRequest {
    pub fn new(
        from_address: impl Into<String>,
        to_vm_address: impl Into<String>,
        asset: AssetIdentifier,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            from_address: from_address.into(),
            to_vm_address: to_vm_address.into(),
            asset,
            amount: amount.into(),
            from_external_address: None,
            chain_type: None,
            unify: false,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgBridgeAssetRequest = self.clone().into();
        ProtoAny {
            type_url: "/bank.v1.MsgBridgeAssetRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<BridgeAssetRequest> for proto::MsgBridgeAssetRequest {
    fn from(req: BridgeAssetRequest) -> Self {
        let asset_identifier = match req.asset {
            AssetIdentifier::ByIndex(idx) => {
                Some(proto::msg_bridge_asset_request::AssetIdentifier::AssetIndex(idx))
            }
            AssetIdentifier::BySymbol(sym) => {
                Some(proto::msg_bridge_asset_request::AssetIdentifier::AssetSymbol(sym))
            }
        };
        Self {
            from_address: req.from_address,
            to_vm_address: req.to_vm_address,
            amount: req.amount,
            timestamp: None,
            from_external_address: req.from_external_address,
            chain_type: req.chain_type.map(i32::from),
            unify: req.unify,
            asset_identifier,
        }
    }
}

/// Request to process an inbound deposit (credit assets).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DepositRequest {
    pub from_address: String,
    pub asset: AssetIdentifier,
    pub amount: String,
    pub source_chain: ChainType,
    pub is_genesis_mint: bool,
    pub external_address: Option<String>,
}

impl DepositRequest {
    pub fn new(
        from_address: impl Into<String>,
        asset: AssetIdentifier,
        amount: impl Into<String>,
        source_chain: ChainType,
    ) -> Self {
        Self {
            from_address: from_address.into(),
            asset,
            amount: amount.into(),
            source_chain,
            is_genesis_mint: false,
            external_address: None,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgDepositRequest = self.clone().into();
        ProtoAny {
            type_url: "/bank.v1.MsgDepositRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<DepositRequest> for proto::MsgDepositRequest {
    fn from(req: DepositRequest) -> Self {
        let asset_identifier = match req.asset {
            AssetIdentifier::ByIndex(idx) => {
                Some(proto::msg_deposit_request::AssetIdentifier::AssetIndex(idx))
            }
            AssetIdentifier::BySymbol(sym) => {
                Some(proto::msg_deposit_request::AssetIdentifier::AssetSymbol(sym))
            }
        };
        Self {
            from_address: req.from_address,
            amount: req.amount,
            source_chain: i32::from(req.source_chain),
            timestamp: None,
            metadata: Default::default(),
            is_genesis_mint: req.is_genesis_mint,
            external_address: req.external_address,
            asset_identifier,
        }
    }
}

/// Request to process an outbound withdrawal (debit assets).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WithdrawRequest {
    pub from_address: String,
    pub asset: AssetIdentifier,
    pub amount: String,
    pub destination_chain: ChainType,
    pub destination_address: String,
    pub fast_withdrawal: bool,
    pub external_address: Option<String>,
}

impl WithdrawRequest {
    pub fn new(
        from_address: impl Into<String>,
        asset: AssetIdentifier,
        amount: impl Into<String>,
        destination_chain: ChainType,
        destination_address: impl Into<String>,
    ) -> Self {
        Self {
            from_address: from_address.into(),
            asset,
            amount: amount.into(),
            destination_chain,
            destination_address: destination_address.into(),
            fast_withdrawal: false,
            external_address: None,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgWithdrawRequest = self.clone().into();
        ProtoAny {
            type_url: "/bank.v1.MsgWithdrawRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<WithdrawRequest> for proto::MsgWithdrawRequest {
    fn from(req: WithdrawRequest) -> Self {
        let asset_identifier = match req.asset {
            AssetIdentifier::ByIndex(idx) => {
                Some(proto::msg_withdraw_request::AssetIdentifier::AssetIndex(idx))
            }
            AssetIdentifier::BySymbol(sym) => {
                Some(proto::msg_withdraw_request::AssetIdentifier::AssetSymbol(sym))
            }
        };
        Self {
            from_address: req.from_address,
            amount: req.amount,
            destination_chain: i32::from(req.destination_chain),
            destination_address: req.destination_address,
            timestamp: None,
            fast_withdrawal: req.fast_withdrawal,
            external_address: req.external_address,
            asset_identifier,
        }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query balance for a single asset.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryBalanceRequest {
    pub address: String,
    pub asset_index: u64,
    pub chain_type: Option<ChainType>,
}

impl QueryBalanceRequest {
    pub fn new(address: impl Into<String>, asset_index: u64) -> Self {
        Self {
            address: address.into(),
            asset_index,
            chain_type: None,
        }
    }

    pub fn with_chain_type(mut self, chain_type: ChainType) -> Self {
        self.chain_type = Some(chain_type);
        self
    }
}

impl From<QueryBalanceRequest> for proto::QueryBalanceRequest {
    fn from(req: QueryBalanceRequest) -> Self {
        Self {
            address: req.address,
            asset_index: req.asset_index,
            chain_type: req.chain_type.map(i32::from),
        }
    }
}

/// Query all balances for an address.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryBalancesRequest {
    pub address: String,
    pub chain_type: Option<ChainType>,
    pub asset_type_filter: Option<i32>,
}

impl QueryBalancesRequest {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            chain_type: None,
            asset_type_filter: None,
        }
    }
}

impl From<QueryBalancesRequest> for proto::QueryBalancesRequest {
    fn from(req: QueryBalancesRequest) -> Self {
        Self {
            address: req.address,
            chain_type: req.chain_type.map(i32::from),
            asset_type_filter: req.asset_type_filter,
        }
    }
}

/// Query all registered assets in the bank's asset registry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryAssetsRequest {
    pub type_filter: Option<i32>,
}

impl QueryAssetsRequest {
    pub fn new(type_filter: Option<i32>) -> Self {
        Self { type_filter }
    }
}

impl From<QueryAssetsRequest> for proto::QueryAssetsRequest {
    fn from(req: QueryAssetsRequest) -> Self {
        Self {
            type_filter: req.type_filter,
        }
    }
}

/// Query aggregated bank fee statistics.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryBankFeeStatsRequest;

impl From<QueryBankFeeStatsRequest> for proto::QueryBankFeeStatsRequest {
    fn from(_req: QueryBankFeeStatsRequest) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transfer_request_to_any() {
        let req = TransferRequest::new("morm1abc", "morm1def", "1000", 0);
        let any = req.to_any();
        assert_eq!(any.type_url, "/bank.v1.MsgTransferRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn cross_chain_transfer_request_to_any() {
        let req = CrossChainTransferRequest::new(
            "morm1abc",
            ChainType::Ethereum,
            "0xdef",
            "500",
            AssetIdentifier::index(0),
        );
        let any = req.to_any();
        assert_eq!(any.type_url, "/bank.v1.MsgCrossChainTransferRequest");
    }

    #[test]
    fn mint_request_to_any() {
        let req = MintRequest::new("morm1abc", 0, "1000000");
        let any = req.to_any();
        assert_eq!(any.type_url, "/bank.v1.MsgMintRequest");
    }

    #[test]
    fn deposit_request_to_any() {
        let req = DepositRequest::new(
            "morm1abc",
            AssetIdentifier::symbol("MORM"),
            "10000",
            ChainType::Ethereum,
        );
        let any = req.to_any();
        assert_eq!(any.type_url, "/bank.v1.MsgDepositRequest");
    }

    #[test]
    fn withdraw_request_to_any() {
        let req = WithdrawRequest::new(
            "morm1abc",
            AssetIdentifier::index(0),
            "5000",
            ChainType::Solana,
            "SolAddr123",
        );
        let any = req.to_any();
        assert_eq!(any.type_url, "/bank.v1.MsgWithdrawRequest");
    }

    #[test]
    fn onboard_asset_request_to_any() {
        let req = OnboardAssetRequest::new("morm1abc", "TestToken", "TST", 2);
        let any = req.to_any();
        assert_eq!(any.type_url, "/bank.v1.MsgOnboardAssetRequest");
    }

    #[test]
    fn bridge_asset_request_to_any() {
        let req = BridgeAssetRequest::new(
            "morm1abc",
            "0xVmAddr",
            AssetIdentifier::index(1),
            "250",
        );
        let any = req.to_any();
        assert_eq!(any.type_url, "/bank.v1.MsgBridgeAssetRequest");
    }

    #[test]
    fn transfer_to_bucket_request_to_any() {
        let req = TransferToBucketRequest::new("morm1abc", "bucket-1", 0, "1000");
        let any = req.to_any();
        assert_eq!(any.type_url, "/bank.v1.MsgTransferToBucketRequest");
    }
}
