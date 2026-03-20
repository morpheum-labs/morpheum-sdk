//! SVM provider factory.
//!
//! Builds a Solana RPC client paired with a signing keypair, ready for
//! transaction construction and submission.

use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use crate::types::SvmError;

/// A Solana provider combining an RPC client with a signing keypair.
pub struct SvmProvider {
    client: RpcClient,
    keypair: Keypair,
}

impl SvmProvider {
    /// Returns a reference to the underlying RPC client.
    pub fn client(&self) -> &RpcClient {
        &self.client
    }

    /// Returns a reference to the signing keypair.
    pub fn keypair(&self) -> &Keypair {
        &self.keypair
    }

    /// Returns the public key (address) of the signer.
    pub fn address(&self) -> solana_sdk::pubkey::Pubkey {
        self.keypair.pubkey()
    }
}

/// Builds an [`SvmProvider`] from an RPC URL and a keypair.
///
/// Uses `confirmed` commitment for balance queries and `finalized` for
/// transaction confirmation (configurable via the returned `RpcClient`).
pub fn build_provider(rpc_url: &str, keypair: Keypair) -> Result<SvmProvider, SvmError> {
    let client = RpcClient::new_with_commitment(
        rpc_url.to_string(),
        CommitmentConfig::confirmed(),
    );

    Ok(SvmProvider { client, keypair })
}
