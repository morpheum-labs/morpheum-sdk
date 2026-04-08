//! BondingCurveClient — main entry point for all bonding-curve queries.

use alloc::boxed::Box;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::bonding_curve::v1 as proto;

use crate::requests;
use crate::types::{BondingCurveParams, BondingCurveState};

/// Token price and effective market-cap snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PriceSnapshot {
    pub current_price: alloc::string::String,
    pub effective_mcap: alloc::string::String,
}

/// Primary client for all bonding-curve queries.
pub struct BondingCurveClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl BondingCurveClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Fetches the bonding-curve state for a token.
    /// Returns `None` if the curve is not found.
    pub async fn get_curve_state(&self, token_index: u64) -> Result<Option<BondingCurveState>, SdkError> {
        let req = requests::GetCurveStateRequest::new(token_index);
        let proto_req: proto::GetCurveStateRequest = req.into();
        let resp = self.query("/bondingcurve.v1.Query/GetCurveState", proto_req.encode_to_vec()).await?;
        let p = proto::GetCurveStateResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        if !p.found {
            return Ok(None);
        }
        Ok(p.state.map(Into::into))
    }

    /// Queries the current price and effective market cap for a token.
    pub async fn get_price(&self, token_index: u64) -> Result<PriceSnapshot, SdkError> {
        let req = requests::GetPriceRequest::new(token_index);
        let proto_req: proto::GetPriceRequest = req.into();
        let resp = self.query("/bondingcurve.v1.Query/GetPrice", proto_req.encode_to_vec()).await?;
        let p = proto::GetPriceResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(PriceSnapshot { current_price: p.current_price, effective_mcap: p.effective_mcap })
    }

    /// Queries module-level governance parameters.
    pub async fn get_params(&self) -> Result<BondingCurveParams, SdkError> {
        let proto_req: proto::QueryParamsRequest = requests::QueryParamsRequest::new().into();
        let resp = self.query("/bondingcurve.v1.Query/GetParams", proto_req.encode_to_vec()).await?;
        let p = proto::QueryParamsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.params
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("params field missing in response"))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for BondingCurveClient {
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
                "/bondingcurve.v1.Query/GetCurveState" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetCurveStateResponse {
                        state: None, found: false,
                    }))
                }
                "/bondingcurve.v1.Query/GetPrice" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetPriceResponse {
                        current_price: "500000".into(), effective_mcap: "42000000".into(),
                    }))
                }
                "/bondingcurve.v1.Query/GetParams" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryParamsResponse {
                        params: Some(proto::Params::default()),
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> BondingCurveClient {
        BondingCurveClient::new(SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"), Box::new(DummyTransport))
    }

    #[tokio::test]
    async fn get_curve_state_not_found() {
        let result = make_client().get_curve_state(999).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn get_price_works() {
        let snap = make_client().get_price(42).await.unwrap();
        assert_eq!(snap.current_price, "500000");
        assert_eq!(snap.effective_mcap, "42000000");
    }

    #[tokio::test]
    async fn get_params_works() {
        let params = make_client().get_params().await;
        assert!(params.is_ok());
    }
}
