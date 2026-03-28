//! TwapClient — query TWAP values for market/window pairs.

use alloc::boxed::Box;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::twap::v1 as proto;

use crate::requests;

/// TWAP query result.
pub struct TwapResult {
    pub found: bool,
    pub value: u64,
}

/// Primary client for TWAP module queries.
pub struct TwapClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl TwapClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets the TWAP value for a given market and window size.
    pub async fn get_twap(&self, req: requests::GetTwapRequest) -> Result<TwapResult, SdkError> {
        let proto_req: proto::GetTwapRequest = req.into();
        let resp = self.query("/twap.v1.Query/GetTwap", proto_req.encode_to_vec()).await?;
        let p = proto::GetTwapResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(TwapResult { found: p.found, value: p.value })
    }
}

#[async_trait(?Send)]
impl MorpheumClient for TwapClient {
    fn config(&self) -> &SdkConfig { &self.config }
    fn transport(&self) -> &dyn Transport { &*self.transport }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!()
        }
        async fn query(&self, path: &str, _: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/twap.v1.Query/GetTwap" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetTwapResponse {
                        found: true, value: 50_000_000_000,
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> TwapClient {
        TwapClient::new(
            SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"),
            Box::new(DummyTransport),
        )
    }

    #[tokio::test]
    async fn get_twap_works() {
        let r = make_client()
            .get_twap(requests::GetTwapRequest::new(1, 300))
            .await.unwrap();
        assert!(r.found);
        assert_eq!(r.value, 50_000_000_000);
    }

    #[tokio::test]
    async fn get_twap_with_staleness_check() {
        let r = make_client()
            .get_twap(requests::GetTwapRequest::new(1, 300).current_block(10000))
            .await.unwrap();
        assert!(r.found);
    }
}
