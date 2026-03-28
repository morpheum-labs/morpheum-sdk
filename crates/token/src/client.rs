//! TokenClient — queries for token info, listings, programmable logic, and hook simulation.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::token::v1 as proto;

use crate::requests;
use crate::types::{ProgrammableLogicConfig, SimulateHookResult, TokenInfo, TokenSummary};

/// Paginated token listing result.
pub struct TokensPage {
    pub tokens: Vec<TokenSummary>,
    pub next_offset: u64,
    pub total_count: u64,
}

/// Programmable logic query result.
pub struct ProgrammableLogicResult {
    pub config: Option<ProgrammableLogicConfig>,
    pub exists: bool,
}

/// Primary client for token module queries.
pub struct TokenClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl TokenClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets token info by asset index.
    pub async fn get_token_info(&self, asset_index: u64) -> Result<TokenInfo, SdkError> {
        let req = requests::GetTokenInfoRequest::new(asset_index);
        let proto_req: proto::GetTokenInfoRequest = req.into();
        let resp = self.query("/token.v1.Query/GetTokenInfo", proto_req.encode_to_vec()).await?;
        let p = proto::GetTokenInfoResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.into())
    }

    /// Lists tokens with pagination.
    pub async fn list_tokens(&self, req: requests::ListTokensRequest) -> Result<TokensPage, SdkError> {
        let proto_req: proto::ListTokensRequest = req.into();
        let resp = self.query("/token.v1.Query/ListTokens", proto_req.encode_to_vec()).await?;
        let p = proto::ListTokensResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(TokensPage {
            tokens: p.tokens.into_iter().map(Into::into).collect(),
            next_offset: p.next_offset, total_count: p.total_count,
        })
    }

    /// Gets the programmable logic configuration for a token.
    pub async fn get_programmable_logic(&self, asset_index: u64) -> Result<ProgrammableLogicResult, SdkError> {
        let req = requests::GetProgrammableLogicRequest::new(asset_index);
        let proto_req: proto::GetProgrammableLogicRequest = req.into();
        let resp = self.query("/token.v1.Query/GetProgrammableLogic", proto_req.encode_to_vec()).await?;
        let p = proto::GetProgrammableLogicResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(ProgrammableLogicResult { config: p.config.map(Into::into), exists: p.exists })
    }

    /// Dry-run simulates a hook execution.
    pub async fn simulate_hook(&self, req: requests::SimulateHookRequest) -> Result<SimulateHookResult, SdkError> {
        let proto_req: proto::SimulateHookRequest = req.into();
        let resp = self.query("/token.v1.Query/SimulateHook", proto_req.encode_to_vec()).await?;
        let p = proto::SimulateHookResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.into())
    }
}

#[async_trait(?Send)]
impl MorpheumClient for TokenClient {
    fn config(&self) -> &SdkConfig { &self.config }
    fn transport(&self) -> &dyn Transport { &*self.transport }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!()
        }
        async fn query(&self, path: &str, _: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/token.v1.Query/GetTokenInfo" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetTokenInfoResponse {
                        name: "MORM".into(), symbol: "MORM".into(), decimals: 18,
                        tradable: true, agent_creator_did: None,
                        metadata: Default::default(), created_at: None, origin: None,
                    }))
                }
                "/token.v1.Query/ListTokens" => {
                    Ok(prost::Message::encode_to_vec(&proto::ListTokensResponse {
                        tokens: alloc::vec![], next_offset: 0, total_count: 0,
                    }))
                }
                "/token.v1.Query/GetProgrammableLogic" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetProgrammableLogicResponse {
                        config: None, exists: false,
                    }))
                }
                "/token.v1.Query/SimulateHook" => {
                    Ok(prost::Message::encode_to_vec(&proto::SimulateHookResponse {
                        success: true, fuel_used: 100, return_data: alloc::vec![],
                        error_message: String::new(), emitted_events: alloc::vec![],
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> TokenClient {
        TokenClient::new(
            SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"),
            Box::new(DummyTransport),
        )
    }

    #[tokio::test]
    async fn get_token_info_works() {
        let info = make_client().get_token_info(1).await.unwrap();
        assert_eq!(info.symbol, "MORM");
        assert!(info.tradable);
    }

    #[tokio::test]
    async fn list_tokens_works() {
        let page = make_client().list_tokens(requests::ListTokensRequest::new(0, 50)).await.unwrap();
        assert!(page.tokens.is_empty());
        assert_eq!(page.total_count, 0);
    }

    #[tokio::test]
    async fn get_programmable_logic_works() {
        let r = make_client().get_programmable_logic(1).await.unwrap();
        assert!(!r.exists);
        assert!(r.config.is_none());
    }

    #[tokio::test]
    async fn simulate_hook_works() {
        let r = make_client().simulate_hook(
            requests::SimulateHookRequest::new(1, crate::types::HookPoint::OnTransfer, "morph1a", "morph1b", "1000"),
        ).await.unwrap();
        assert!(r.success);
        assert_eq!(r.fuel_used, 100);
    }
}
