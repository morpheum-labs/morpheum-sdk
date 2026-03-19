//! GMP query client.

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, string::String, vec::Vec};

use async_trait::async_trait;
use prost::Message;

use morpheum_proto::gmp::v1 as pb;
use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};

use crate::types::{GmpParams, ProtocolInfo};

/// Client for querying the GMP module.
pub struct GmpClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl GmpClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Query GMP module parameters.
    pub async fn query_params(&self) -> Result<GmpParams, SdkError> {
        let req = pb::QueryParamsRequest {};
        let data = self.query("/gmp.v1.QueryParamsRequest", req.encode_to_vec()).await?;
        let resp = pb::QueryParamsResponse::decode(data.as_slice())?;
        let params = resp
            .params
            .map(GmpParams::from)
            .unwrap_or_default();
        Ok(params)
    }

    /// Query registered GMP protocols.
    pub async fn query_protocols(&self) -> Result<Vec<ProtocolInfo>, SdkError> {
        let req = pb::QueryProtocolsRequest {};
        let data = self.query("/gmp.v1.QueryProtocolsRequest", req.encode_to_vec()).await?;
        let resp = pb::QueryProtocolsResponse::decode(data.as_slice())?;
        Ok(resp.protocols.into_iter().map(ProtocolInfo::from).collect())
    }

}

#[async_trait(?Send)]
impl MorpheumClient for GmpClient {
    fn config(&self) -> &SdkConfig {
        &self.config
    }

    fn transport(&self) -> &dyn Transport {
        &*self.transport
    }
}
