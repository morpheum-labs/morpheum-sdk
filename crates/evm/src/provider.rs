//! EVM provider factory.
//!
//! Builds an alloy `FilledProvider` from an RPC URL and a `PrivateKeySigner`.
//! The returned provider includes automatic gas estimation, nonce management,
//! chain ID detection, and wallet signing — ready for immediate use.

use alloy::network::EthereumWallet;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{Identity, ProviderBuilder, RootProvider};
use alloy::signers::local::PrivateKeySigner;

use crate::types::EvmError;

/// Fully-configured alloy provider with gas/nonce/chain-id fillers and wallet signing.
pub type EvmProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

/// Builds an [`EvmProvider`] from an RPC URL string and a private key signer.
pub fn build_provider(rpc_url: &str, signer: PrivateKeySigner) -> Result<EvmProvider, EvmError> {
    let url = rpc_url
        .parse()
        .map_err(|e| EvmError::Provider(format!("invalid RPC URL '{rpc_url}': {e}")))?;

    let wallet = EthereumWallet::from(signer);

    Ok(ProviderBuilder::new().wallet(wallet).connect_http(url))
}
