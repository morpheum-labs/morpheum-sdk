//! AuthClient — the main entry point for authentication and nonce management
//! in the Morpheum SDK.
//!
//! This client provides high-level, type-safe access to nonce state queries
//! (the single source of truth for replay protection and parallel execution)
//! and will be extended for TradingKey management.

use alloc::boxed::Box;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{
    AccountId, MorpheumClient, SdkConfig, SdkError, Transport,
};

use crate::{
    requests::QueryNonceStateRequest,
    types::NonceState,
};

/// Primary client for all auth-related operations.
///
/// Focused on nonce and account queries. TradingKey approval/revocation
/// is handled via `TxBuilder` using the request wrappers in `requests.rs`.
pub struct AuthClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl AuthClient {
    /// Creates a new `AuthClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries the full `NonceState` for an account.
    ///
    /// This is the canonical source of truth for nonce management on Morpheum.
    /// Use this before building any transaction to ensure correct nonce usage.
    pub async fn query_nonce_state(
        &self,
        address: impl Into<AccountId>,
    ) -> Result<NonceState, SdkError> {
        let req = QueryNonceStateRequest::new(address.into());
        let proto_req: morpheum_proto::auth::v1::QueryNonceStateRequest = req.into();

        let path = "/auth.v1.Query/QueryNonceState";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::auth::v1::QueryNonceStateResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryNonceStateResponse = proto_res.into();
        Ok(response.state)
    }
}

#[async_trait(?Send)]
impl MorpheumClient for AuthClient {
    fn config(&self) -> &SdkConfig {
        &self.config
    }

    fn transport(&self) -> &dyn Transport {
        &*self.transport
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use morpheum_sdk_core::SdkConfig;

    // Dummy transport for compile-time and basic runtime testing
    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _tx_bytes: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!("not needed for auth query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            if path == "/auth.v1.Query/QueryNonceState" {
                // Return a minimal valid response for testing
                    let dummy_res = morpheum_proto::auth::v1::QueryNonceStateResponse {
                    state: Some(morpheum_proto::auth::v1::NonceState {
                        last_monotonic: 42,
                        ring: vec![],
                        merkle_root: vec![],
                    }),
                };
                Ok(prost::Message::encode_to_vec(&dummy_res))
            } else {
                Err(SdkError::transport("unexpected query path"))
            }
        }
    }

    #[tokio::test]
    async fn query_nonce_state_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = AuthClient::new(config, Box::new(DummyTransport));

        let address = AccountId::new([1u8; 32]);
        let result = client.query_nonce_state(address).await;

        assert!(result.is_ok());
        let nonce_state = result.unwrap();
        assert_eq!(nonce_state.last_monotonic, 42);
    }
}