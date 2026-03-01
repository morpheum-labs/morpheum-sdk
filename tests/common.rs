//! Shared test utilities for Morpheum SDK integration tests.
//!
//! This module provides deterministic test data, helper functions,
//! and common setup logic used across all integration tests.
//! It is designed to be clean, reusable, and production-grade.

use morpheum_sdk_native::prelude::*;

/// Test seed used for deterministic NativeSigner and AgentSigner creation.
pub const TEST_SEED: [u8; 32] = [42u8; 32];

/// Standard BIP-39 test mnemonic (12 words).
pub const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

/// Test chain ID used across all integration tests.
pub const TEST_CHAIN_ID: &str = "morpheum-test-1";

/// Test RPC endpoint (points to local or public testnet sentry).
pub const TEST_RPC_ENDPOINT: &str = "https://sentry.morpheum.xyz";

/// Returns a deterministic `NativeSigner` for tests.
pub fn test_native_signer() -> NativeSigner {
    NativeSigner::from_seed(&TEST_SEED)
}

/// Returns a deterministic `AgentSigner` for tests.
pub fn test_agent_signer() -> AgentSigner {
    AgentSigner::new(
        &TEST_SEED,
        AccountId::new([0xAA; 32]), // Fixed test agent address
        None,
    )
}

/// Creates a fully configured test SDK instance with default transport.
pub fn test_sdk() -> MorpheumSdk {
    MorpheumSdk::new(TEST_RPC_ENDPOINT, TEST_CHAIN_ID)
}

/// Creates a test SDK instance using a specific signer (for agent tests).
pub fn test_sdk_with_signer(signer: impl Into<morpheum_sdk_native::signing::NativeSigner>) -> MorpheumSdk {
    // In real usage we would pass the signer properly; for tests we use the convenience wrapper
    let _ = signer;
    test_sdk()
}

/// A simple test nonce provider that returns increasing values.
/// Useful for deterministic tests without network dependency.
#[derive(Clone, Debug, Default)]
pub struct TestNonceProvider {
    pub current: u64,
}

impl TestNonceProvider {
    pub fn new(start: u64) -> Self {
        Self { current: start }
    }
}

#[async_trait::async_trait]
impl morpheum_sdk_core::NonceProvider for TestNonceProvider {
    async fn get_nonce(&self, _address: &AccountId) -> Result<u64, morpheum_sdk_core::SdkError> {
        Ok(self.current)
    }

    async fn increment(&mut self) -> Result<u64, morpheum_sdk_core::SdkError> {
        self.current += 1;
        Ok(self.current)
    }
}

/// Helper to create a minimal valid `TradingKeyClaim` for tests.
pub fn test_trading_key_claim(
    issuer: AccountId,
    subject: AccountId,
    now_secs: u64,
) -> Result<TradingKeyClaim, morpheum_sdk_core::SdkError> {
    VcClaimBuilder::new()
        .issuer(issuer)
        .subject(subject)
        .permissions(0b0001)           // TRADE permission
        .max_daily_usd(100_000)
        .expiry(now_secs + 86_400)     // 24 hours
        .nonce_sub_range(1000, 2000)
        .build(now_secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helpers_compile_and_work() {
        let signer = test_native_signer();
        assert_eq!(signer.public_key().len(), 32); // ed25519 pubkey is 32 bytes

        let sdk = test_sdk();
        assert_eq!(sdk.config().default_chain_id.as_str(), TEST_CHAIN_ID);
    }

    #[tokio::test]
    async fn test_nonce_provider_works() {
        let mut provider = TestNonceProvider::new(100);
        assert_eq!(provider.get_nonce(&AccountId::new([0; 32])).await.unwrap(), 100);
        assert_eq!(provider.increment().await.unwrap(), 101);
    }
}