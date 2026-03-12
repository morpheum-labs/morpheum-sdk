//! Fluent builders for the Bank module.
//!
//! Ergonomic, type-safe builders for all bank transaction operations.
//! Each builder validates required fields and returns the corresponding
//! request type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    BridgeAssetRequest, CrossChainTransferRequest, DepositRequest, MintRequest,
    OnboardAssetRequest, TransferRequest, TransferToBucketRequest, WithdrawRequest,
};
use crate::types::{AssetIdentifier, ChainType};

// ============================================================================
// TransferBuilder
// ============================================================================

/// Fluent builder for native asset transfers.
#[derive(Default)]
pub struct TransferBuilder {
    from_address: Option<String>,
    to_address: Option<String>,
    amount: Option<String>,
    asset_index: Option<u64>,
    memo: Option<String>,
    from_external_address: Option<String>,
    to_external_address: Option<String>,
}

impl TransferBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, addr: impl Into<String>) -> Self {
        self.from_address = Some(addr.into());
        self
    }

    pub fn to_address(mut self, addr: impl Into<String>) -> Self {
        self.to_address = Some(addr.into());
        self
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn asset_index(mut self, idx: u64) -> Self {
        self.asset_index = Some(idx);
        self
    }

    pub fn memo(mut self, memo: impl Into<String>) -> Self {
        self.memo = Some(memo.into());
        self
    }

    pub fn from_external_address(mut self, addr: impl Into<String>) -> Self {
        self.from_external_address = Some(addr.into());
        self
    }

    pub fn to_external_address(mut self, addr: impl Into<String>) -> Self {
        self.to_external_address = Some(addr.into());
        self
    }

    pub fn build(self) -> Result<TransferRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for transfer")
        })?;
        let to_address = self.to_address.ok_or_else(|| {
            SdkError::invalid_input("to_address is required for transfer")
        })?;
        let amount = self.amount.ok_or_else(|| {
            SdkError::invalid_input("amount is required for transfer")
        })?;
        let asset_index = self.asset_index.ok_or_else(|| {
            SdkError::invalid_input("asset_index is required for transfer")
        })?;

        let mut req = TransferRequest::new(from_address, to_address, amount, asset_index);
        if let Some(m) = self.memo {
            req.memo = m;
        }
        req.from_external_address = self.from_external_address;
        req.to_external_address = self.to_external_address;
        Ok(req)
    }
}

// ============================================================================
// CrossChainTransferBuilder
// ============================================================================

/// Fluent builder for cross-chain asset transfers.
#[derive(Default)]
pub struct CrossChainTransferBuilder {
    from_address: Option<String>,
    target_chain: Option<ChainType>,
    to_address: Option<String>,
    amount: Option<String>,
    asset: Option<AssetIdentifier>,
    memo: Option<String>,
    target_bridge_id: Option<String>,
    from_external_address: Option<String>,
    to_external_address: Option<String>,
}

impl CrossChainTransferBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, addr: impl Into<String>) -> Self {
        self.from_address = Some(addr.into());
        self
    }

    pub fn target_chain(mut self, chain: ChainType) -> Self {
        self.target_chain = Some(chain);
        self
    }

    pub fn to_address(mut self, addr: impl Into<String>) -> Self {
        self.to_address = Some(addr.into());
        self
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn asset(mut self, asset: AssetIdentifier) -> Self {
        self.asset = Some(asset);
        self
    }

    pub fn memo(mut self, memo: impl Into<String>) -> Self {
        self.memo = Some(memo.into());
        self
    }

    pub fn target_bridge_id(mut self, id: impl Into<String>) -> Self {
        self.target_bridge_id = Some(id.into());
        self
    }

    pub fn from_external_address(mut self, addr: impl Into<String>) -> Self {
        self.from_external_address = Some(addr.into());
        self
    }

    pub fn to_external_address(mut self, addr: impl Into<String>) -> Self {
        self.to_external_address = Some(addr.into());
        self
    }

    pub fn build(self) -> Result<CrossChainTransferRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for cross-chain transfer")
        })?;
        let target_chain = self.target_chain.ok_or_else(|| {
            SdkError::invalid_input("target_chain is required for cross-chain transfer")
        })?;
        let to_address = self.to_address.ok_or_else(|| {
            SdkError::invalid_input("to_address is required for cross-chain transfer")
        })?;
        let amount = self.amount.ok_or_else(|| {
            SdkError::invalid_input("amount is required for cross-chain transfer")
        })?;
        let asset = self.asset.ok_or_else(|| {
            SdkError::invalid_input("asset identifier is required for cross-chain transfer")
        })?;

        let mut req = CrossChainTransferRequest::new(
            from_address, target_chain, to_address, amount, asset,
        );
        if let Some(m) = self.memo {
            req.memo = m;
        }
        if let Some(b) = self.target_bridge_id {
            req.target_bridge_id = b;
        }
        req.from_external_address = self.from_external_address;
        req.to_external_address = self.to_external_address;
        Ok(req)
    }
}

// ============================================================================
// TransferToBucketBuilder
// ============================================================================

/// Fluent builder for spot-to-bucket transfers (perpetuals margin).
#[derive(Default)]
pub struct TransferToBucketBuilder {
    address: Option<String>,
    bucket_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
}

impl TransferToBucketBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn address(mut self, addr: impl Into<String>) -> Self {
        self.address = Some(addr.into());
        self
    }

    pub fn bucket_id(mut self, id: impl Into<String>) -> Self {
        self.bucket_id = Some(id.into());
        self
    }

    pub fn asset_index(mut self, idx: u64) -> Self {
        self.asset_index = Some(idx);
        self
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn build(self) -> Result<TransferToBucketRequest, SdkError> {
        let address = self.address.ok_or_else(|| {
            SdkError::invalid_input("address is required for transfer to bucket")
        })?;
        let bucket_id = self.bucket_id.ok_or_else(|| {
            SdkError::invalid_input("bucket_id is required for transfer to bucket")
        })?;
        let asset_index = self.asset_index.ok_or_else(|| {
            SdkError::invalid_input("asset_index is required for transfer to bucket")
        })?;
        let amount = self.amount.ok_or_else(|| {
            SdkError::invalid_input("amount is required for transfer to bucket")
        })?;

        Ok(TransferToBucketRequest::new(address, bucket_id, asset_index, amount))
    }
}

// ============================================================================
// MintBuilder
// ============================================================================

/// Fluent builder for minting new assets (restricted operation).
#[derive(Default)]
pub struct MintBuilder {
    recipient_address: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
    permissions: Vec<String>,
    module_account: Option<String>,
}

impl MintBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn recipient_address(mut self, addr: impl Into<String>) -> Self {
        self.recipient_address = Some(addr.into());
        self
    }

    pub fn asset_index(mut self, idx: u64) -> Self {
        self.asset_index = Some(idx);
        self
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn permission(mut self, perm: impl Into<String>) -> Self {
        self.permissions.push(perm.into());
        self
    }

    pub fn permissions(mut self, perms: Vec<String>) -> Self {
        self.permissions = perms;
        self
    }

    pub fn module_account(mut self, account: impl Into<String>) -> Self {
        self.module_account = Some(account.into());
        self
    }

    pub fn build(self) -> Result<MintRequest, SdkError> {
        let recipient_address = self.recipient_address.ok_or_else(|| {
            SdkError::invalid_input("recipient_address is required for minting")
        })?;
        let asset_index = self.asset_index.ok_or_else(|| {
            SdkError::invalid_input("asset_index is required for minting")
        })?;
        let amount = self.amount.ok_or_else(|| {
            SdkError::invalid_input("amount is required for minting")
        })?;

        let mut req = MintRequest::new(recipient_address, asset_index, amount);
        req.permissions = self.permissions;
        if let Some(m) = self.module_account {
            req.module_account = m;
        }
        Ok(req)
    }
}

// ============================================================================
// OnboardAssetBuilder
// ============================================================================

/// Fluent builder for onboarding a new asset.
#[derive(Default)]
pub struct OnboardAssetBuilder {
    from_address: Option<String>,
    name: Option<String>,
    asset_symbol: Option<String>,
    salt: Option<Vec<u8>>,
    asset_type: Option<i32>,
    initial_supply: Option<String>,
}

impl OnboardAssetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, addr: impl Into<String>) -> Self {
        self.from_address = Some(addr.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn asset_symbol(mut self, sym: impl Into<String>) -> Self {
        self.asset_symbol = Some(sym.into());
        self
    }

    pub fn salt(mut self, salt: Vec<u8>) -> Self {
        self.salt = Some(salt);
        self
    }

    pub fn asset_type(mut self, t: i32) -> Self {
        self.asset_type = Some(t);
        self
    }

    pub fn initial_supply(mut self, supply: impl Into<String>) -> Self {
        self.initial_supply = Some(supply.into());
        self
    }

    pub fn build(self) -> Result<OnboardAssetRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for asset onboarding")
        })?;
        let name = self.name.ok_or_else(|| {
            SdkError::invalid_input("name is required for asset onboarding")
        })?;
        let asset_symbol = self.asset_symbol.ok_or_else(|| {
            SdkError::invalid_input("asset_symbol is required for asset onboarding")
        })?;
        let asset_type = self.asset_type.ok_or_else(|| {
            SdkError::invalid_input("asset_type is required for asset onboarding")
        })?;

        let mut req = OnboardAssetRequest::new(from_address, name, asset_symbol, asset_type);
        if let Some(s) = self.salt {
            req.salt = s;
        }
        if let Some(supply) = self.initial_supply {
            req.initial_supply = supply;
        }
        Ok(req)
    }
}

// ============================================================================
// BridgeAssetBuilder
// ============================================================================

/// Fluent builder for bridging assets to a VM.
#[derive(Default)]
pub struct BridgeAssetBuilder {
    from_address: Option<String>,
    to_vm_address: Option<String>,
    asset: Option<AssetIdentifier>,
    amount: Option<String>,
    from_external_address: Option<String>,
    chain_type: Option<ChainType>,
    unify: bool,
}

impl BridgeAssetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, addr: impl Into<String>) -> Self {
        self.from_address = Some(addr.into());
        self
    }

    pub fn to_vm_address(mut self, addr: impl Into<String>) -> Self {
        self.to_vm_address = Some(addr.into());
        self
    }

    pub fn asset(mut self, asset: AssetIdentifier) -> Self {
        self.asset = Some(asset);
        self
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn from_external_address(mut self, addr: impl Into<String>) -> Self {
        self.from_external_address = Some(addr.into());
        self
    }

    pub fn chain_type(mut self, ct: ChainType) -> Self {
        self.chain_type = Some(ct);
        self
    }

    pub fn unify(mut self, unify: bool) -> Self {
        self.unify = unify;
        self
    }

    pub fn build(self) -> Result<BridgeAssetRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for bridge")
        })?;
        let to_vm_address = self.to_vm_address.ok_or_else(|| {
            SdkError::invalid_input("to_vm_address is required for bridge")
        })?;
        let asset = self.asset.ok_or_else(|| {
            SdkError::invalid_input("asset identifier is required for bridge")
        })?;
        let amount = self.amount.ok_or_else(|| {
            SdkError::invalid_input("amount is required for bridge")
        })?;

        let mut req = BridgeAssetRequest::new(from_address, to_vm_address, asset, amount);
        req.from_external_address = self.from_external_address;
        req.chain_type = self.chain_type;
        req.unify = self.unify;
        Ok(req)
    }
}

// ============================================================================
// DepositBuilder
// ============================================================================

/// Fluent builder for processing inbound deposits.
#[derive(Default)]
pub struct DepositBuilder {
    from_address: Option<String>,
    asset: Option<AssetIdentifier>,
    amount: Option<String>,
    source_chain: Option<ChainType>,
    is_genesis_mint: bool,
    external_address: Option<String>,
}

impl DepositBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, addr: impl Into<String>) -> Self {
        self.from_address = Some(addr.into());
        self
    }

    pub fn asset(mut self, asset: AssetIdentifier) -> Self {
        self.asset = Some(asset);
        self
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn source_chain(mut self, chain: ChainType) -> Self {
        self.source_chain = Some(chain);
        self
    }

    pub fn genesis_mint(mut self, is_genesis: bool) -> Self {
        self.is_genesis_mint = is_genesis;
        self
    }

    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into());
        self
    }

    pub fn build(self) -> Result<DepositRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for deposit")
        })?;
        let asset = self.asset.ok_or_else(|| {
            SdkError::invalid_input("asset identifier is required for deposit")
        })?;
        let amount = self.amount.ok_or_else(|| {
            SdkError::invalid_input("amount is required for deposit")
        })?;
        let source_chain = self.source_chain.ok_or_else(|| {
            SdkError::invalid_input("source_chain is required for deposit")
        })?;

        let mut req = DepositRequest::new(from_address, asset, amount, source_chain);
        req.is_genesis_mint = self.is_genesis_mint;
        req.external_address = self.external_address;
        Ok(req)
    }
}

// ============================================================================
// WithdrawBuilder
// ============================================================================

/// Fluent builder for processing outbound withdrawals.
#[derive(Default)]
pub struct WithdrawBuilder {
    from_address: Option<String>,
    asset: Option<AssetIdentifier>,
    amount: Option<String>,
    destination_chain: Option<ChainType>,
    destination_address: Option<String>,
    fast_withdrawal: bool,
    external_address: Option<String>,
}

impl WithdrawBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, addr: impl Into<String>) -> Self {
        self.from_address = Some(addr.into());
        self
    }

    pub fn asset(mut self, asset: AssetIdentifier) -> Self {
        self.asset = Some(asset);
        self
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn destination_chain(mut self, chain: ChainType) -> Self {
        self.destination_chain = Some(chain);
        self
    }

    pub fn destination_address(mut self, addr: impl Into<String>) -> Self {
        self.destination_address = Some(addr.into());
        self
    }

    pub fn fast_withdrawal(mut self, fast: bool) -> Self {
        self.fast_withdrawal = fast;
        self
    }

    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into());
        self
    }

    pub fn build(self) -> Result<WithdrawRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for withdrawal")
        })?;
        let asset = self.asset.ok_or_else(|| {
            SdkError::invalid_input("asset identifier is required for withdrawal")
        })?;
        let amount = self.amount.ok_or_else(|| {
            SdkError::invalid_input("amount is required for withdrawal")
        })?;
        let destination_chain = self.destination_chain.ok_or_else(|| {
            SdkError::invalid_input("destination_chain is required for withdrawal")
        })?;
        let destination_address = self.destination_address.ok_or_else(|| {
            SdkError::invalid_input("destination_address is required for withdrawal")
        })?;

        let mut req = WithdrawRequest::new(
            from_address, asset, amount, destination_chain, destination_address,
        );
        req.fast_withdrawal = self.fast_withdrawal;
        req.external_address = self.external_address;
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn transfer_builder_full_flow() {
        let req = TransferBuilder::new()
            .from_address("morm1sender")
            .to_address("morm1recipient")
            .amount("1000")
            .asset_index(0)
            .memo("test transfer")
            .build()
            .unwrap();

        assert_eq!(req.from_address, "morm1sender");
        assert_eq!(req.to_address, "morm1recipient");
        assert_eq!(req.amount, "1000");
        assert_eq!(req.asset_index, 0);
        assert_eq!(req.memo, "test transfer");
    }

    #[test]
    fn transfer_builder_missing_required() {
        assert!(TransferBuilder::new().build().is_err());
        assert!(TransferBuilder::new().from_address("a").build().is_err());
        assert!(TransferBuilder::new().from_address("a").to_address("b").build().is_err());
    }

    #[test]
    fn cross_chain_builder_works() {
        let req = CrossChainTransferBuilder::new()
            .from_address("morm1abc")
            .target_chain(ChainType::Ethereum)
            .to_address("0xdef")
            .amount("500")
            .asset(AssetIdentifier::index(0))
            .build()
            .unwrap();

        assert_eq!(req.target_chain, ChainType::Ethereum);
        assert_eq!(req.asset, AssetIdentifier::ByIndex(0));
    }

    #[test]
    fn mint_builder_works() {
        let req = MintBuilder::new()
            .recipient_address("morm1abc")
            .asset_index(0)
            .amount("1000000")
            .permission("mint")
            .module_account("bank")
            .build()
            .unwrap();

        assert_eq!(req.recipient_address, "morm1abc");
        assert_eq!(req.permissions, vec!["mint"]);
    }

    #[test]
    fn deposit_builder_works() {
        let req = DepositBuilder::new()
            .from_address("morm1abc")
            .asset(AssetIdentifier::symbol("MORM"))
            .amount("10000")
            .source_chain(ChainType::Ethereum)
            .build()
            .unwrap();

        assert_eq!(req.source_chain, ChainType::Ethereum);
    }

    #[test]
    fn withdraw_builder_works() {
        let req = WithdrawBuilder::new()
            .from_address("morm1abc")
            .asset(AssetIdentifier::index(0))
            .amount("5000")
            .destination_chain(ChainType::Solana)
            .destination_address("SolAddr")
            .fast_withdrawal(true)
            .build()
            .unwrap();

        assert!(req.fast_withdrawal);
        assert_eq!(req.destination_chain, ChainType::Solana);
    }

    #[test]
    fn onboard_asset_builder_works() {
        let req = OnboardAssetBuilder::new()
            .from_address("morm1abc")
            .name("TestToken")
            .asset_symbol("TST")
            .asset_type(2)
            .initial_supply("1000000")
            .salt(vec![1, 2, 3])
            .build()
            .unwrap();

        assert_eq!(req.asset_symbol, "TST");
        assert_eq!(req.initial_supply, "1000000");
    }

    #[test]
    fn bridge_asset_builder_works() {
        let req = BridgeAssetBuilder::new()
            .from_address("morm1abc")
            .to_vm_address("0xVmAddr")
            .asset(AssetIdentifier::index(1))
            .amount("250")
            .unify(true)
            .build()
            .unwrap();

        assert!(req.unify);
        assert_eq!(req.asset, AssetIdentifier::ByIndex(1));
    }

    #[test]
    fn transfer_to_bucket_builder_works() {
        let req = TransferToBucketBuilder::new()
            .address("morm1abc")
            .bucket_id("bucket-1")
            .asset_index(0)
            .amount("1000")
            .build()
            .unwrap();

        assert_eq!(req.bucket_id, "bucket-1");
    }
}
