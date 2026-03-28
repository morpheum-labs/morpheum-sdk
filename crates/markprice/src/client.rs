//! MarkPriceClient — queries for canonical mark price and source attribution.

use alloc::boxed::Box;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::markprice::v1 as proto;

use crate::requests;
use crate::types::{MarkPriceWithSource, MarkSource};

/// Primary client for mark price queries.
pub struct MarkPriceClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl MarkPriceClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets the canonical mark price for a market.
    /// Returns `None` if no price is available.
    pub async fn get_mark_price(&self, market_index: u64) -> Result<Option<u64>, SdkError> {
        let req = requests::GetMarkPriceRequest::new(market_index);
        let proto_req: proto::GetMarkPriceRequest = req.into();
        let resp = self.query("/markprice.v1.Query/GetMarkPrice", proto_req.encode_to_vec()).await?;
        let p = proto::GetMarkPriceResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(if p.found { Some(p.mark_price) } else { None })
    }

    /// Gets the mark price with source attribution for a market.
    /// Returns `None` if no price is available.
    pub async fn get_mark_price_with_source(&self, market_index: u64) -> Result<Option<MarkPriceWithSource>, SdkError> {
        let req = requests::GetMarkPriceWithSourceRequest::new(market_index);
        let proto_req: proto::GetMarkPriceWithSourceRequest = req.into();
        let resp = self.query("/markprice.v1.Query/GetMarkPriceWithSource", proto_req.encode_to_vec()).await?;
        let p = proto::GetMarkPriceWithSourceResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(if p.found {
            Some(MarkPriceWithSource { mark_price: p.mark_price, source: MarkSource::from(p.source) })
        } else {
            None
        })
    }
}

#[async_trait(?Send)]
impl MorpheumClient for MarkPriceClient {
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
                "/markprice.v1.Query/GetMarkPrice" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetMarkPriceResponse {
                        found: true, mark_price: 50_000_000_000,
                    }))
                }
                "/markprice.v1.Query/GetMarkPriceWithSource" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetMarkPriceWithSourceResponse {
                        found: true, mark_price: 50_000_000_000, source: 1,
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> MarkPriceClient {
        MarkPriceClient::new(
            SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"),
            Box::new(DummyTransport),
        )
    }

    #[tokio::test]
    async fn get_mark_price_found() {
        let price = make_client().get_mark_price(1).await.unwrap();
        assert_eq!(price, Some(50_000_000_000));
    }

    #[tokio::test]
    async fn get_mark_price_with_source_found() {
        let mps = make_client().get_mark_price_with_source(1).await.unwrap().unwrap();
        assert_eq!(mps.mark_price, 50_000_000_000);
        assert_eq!(mps.source, MarkSource::Twap);
    }
}
