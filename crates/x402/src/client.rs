//! X402Client — the main entry point for all x402 payment operations
//! in the Morpheum SDK.
//!
//! Provides high-level, type-safe query methods for receipts, policies,
//! capabilities, and module parameters. Transaction construction is handled
//! via the fluent builders in `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{
    MorpheumClient, SdkConfig, SdkError, Transport,
};

use crate::requests::{
    QueryCapabilitiesRequest,
    QueryParamsRequest,
    QueryPolicyRequest,
    QueryReceiptRequest,
    QueryReceiptsByAgentRequest,
    SettleBridgePaymentRequest,
};
use crate::types::{BridgeSettlementResult, Capabilities, Params, Policy, Receipt};

/// Primary client for all x402 payment queries.
///
/// Transaction construction (register policy, update policy, rotate address,
/// approve outbound) is delegated to the fluent builders in `builder.rs`.
pub struct X402Client {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl X402Client {
    /// Creates a new `X402Client` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a single receipt by its ID.
    pub async fn query_receipt(
        &self,
        receipt_id: impl Into<String>,
    ) -> Result<Receipt, SdkError> {
        let req = QueryReceiptRequest::new(receipt_id);
        let proto_req: morpheum_proto::x402::v1::QueryReceiptRequest = req.into();

        let path = "/x402.v1.Query/QueryReceipt";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::x402::v1::QueryReceiptResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .receipt
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("receipt field missing in response"))
    }

    /// Queries receipts for a specific agent with pagination.
    pub async fn query_receipts_by_agent(
        &self,
        agent_id: impl Into<String>,
        limit: u32,
        pagination_key: Option<String>,
    ) -> Result<(Vec<Receipt>, String), SdkError> {
        let mut req = QueryReceiptsByAgentRequest::new(agent_id, limit);
        if let Some(key) = pagination_key {
            req = req.pagination_key(key);
        }

        let proto_req: morpheum_proto::x402::v1::QueryReceiptsByAgentRequest = req.into();

        let path = "/x402.v1.Query/QueryReceiptsByAgent";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::x402::v1::QueryReceiptsByAgentResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let receipts = proto_res.receipts.into_iter().map(Into::into).collect();
        Ok((receipts, proto_res.next_pagination_key))
    }

    /// Queries an agent's spending policy.
    pub async fn query_policy(
        &self,
        agent_id: impl Into<String>,
        policy_id: impl Into<String>,
    ) -> Result<Option<Policy>, SdkError> {
        let req = QueryPolicyRequest::new(agent_id, policy_id);
        let proto_req: morpheum_proto::x402::v1::QueryPolicyRequest = req.into();

        let path = "/x402.v1.Query/QueryPolicy";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::x402::v1::QueryPolicyResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        if !proto_res.exists {
            return Ok(None);
        }

        Ok(proto_res.policy.map(Into::into))
    }

    /// Queries an agent's x402 capabilities.
    pub async fn query_capabilities(
        &self,
        agent_id: impl Into<String>,
    ) -> Result<Capabilities, SdkError> {
        let req = QueryCapabilitiesRequest::new(agent_id);
        let proto_req: morpheum_proto::x402::v1::QueryCapabilitiesRequest = req.into();

        let path = "/x402.v1.Query/QueryCapabilities";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::x402::v1::QueryCapabilitiesResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .capabilities
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("capabilities field missing in response"))
    }

    /// Settles a cross-chain bridge payment on Morpheum.
    ///
    /// Submits an `X402PaymentPacket` from an external EVM chain, validates it,
    /// processes it through the native inbound path, and returns a receipt with
    /// Merkle proof plus a GMP reply payload for confirmation on the source chain.
    ///
    /// This is the primary SDK entry point for relay services and operators
    /// performing cross-chain settlement.
    pub async fn settle_bridge_payment(
        &self,
        request: SettleBridgePaymentRequest,
    ) -> Result<BridgeSettlementResult, SdkError> {
        let proto_req: morpheum_proto::x402::v1::MsgSettleBridgePayment = request.into();

        let path = "/x402.v1.Msg/SettleBridgePayment";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::x402::v1::SettleBridgePaymentResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.into())
    }

    /// Queries the x402 module parameters.
    pub async fn query_params(&self) -> Result<Params, SdkError> {
        let proto_req: morpheum_proto::x402::v1::QueryParamsRequest = QueryParamsRequest.into();

        let path = "/x402.v1.Query/QueryParams";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::x402::v1::QueryParamsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .params
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("params field missing in response"))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for X402Client {
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
    use morpheum_sdk_core::SdkConfig;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(
            &self,
            _tx_bytes: Vec<u8>,
        ) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!("not needed for x402 query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/x402.v1.Query/QueryReceipt" => {
                    let dummy = morpheum_proto::x402::v1::QueryReceiptResponse {
                        receipt: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/x402.v1.Query/QueryReceiptsByAgent" => {
                    let dummy = morpheum_proto::x402::v1::QueryReceiptsByAgentResponse {
                        receipts: vec![],
                        next_pagination_key: String::new(),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/x402.v1.Query/QueryPolicy" => {
                    let dummy = morpheum_proto::x402::v1::QueryPolicyResponse {
                        policy: Some(Default::default()),
                        exists: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/x402.v1.Query/QueryCapabilities" => {
                    let dummy = morpheum_proto::x402::v1::QueryCapabilitiesResponse {
                        capabilities: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/x402.v1.Query/QueryParams" => {
                    let dummy = morpheum_proto::x402::v1::QueryParamsResponse {
                        params: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/x402.v1.Msg/SettleBridgePayment" => {
                    let dummy = morpheum_proto::x402::v1::SettleBridgePaymentResponse {
                        success: true,
                        receipt: Some(Default::default()),
                        gmp_reply_payload: vec![1, 2, 3],
                        receipt_hash: "hash123".into(),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    fn test_client() -> X402Client {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        X402Client::new(config, Box::new(DummyTransport))
    }

    #[tokio::test]
    async fn query_receipt_works() {
        let client = test_client();
        let result = client.query_receipt("rcpt-1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn query_receipts_by_agent_works() {
        let client = test_client();
        let result = client.query_receipts_by_agent("agent-1", 10, None).await;
        assert!(result.is_ok());
        let (receipts, _next_key) = result.unwrap();
        assert!(receipts.is_empty());
    }

    #[tokio::test]
    async fn query_policy_works() {
        let client = test_client();
        let result = client.query_policy("agent-1", "pol-1").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn query_capabilities_works() {
        let client = test_client();
        let result = client.query_capabilities("agent-1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn query_params_works() {
        let client = test_client();
        let result = client.query_params().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn settle_bridge_payment_works() {
        use crate::requests::SettleBridgePaymentRequest;
        use crate::types::PaymentPacket;

        let client = test_client();
        let packet = PaymentPacket {
            payment_id: "pay-001".into(),
            source_chain: "eip155:8453".into(),
            target_agent_id: "agent-1".into(),
            amount: 5000,
            asset: "USDC".into(),
            memo: String::new(),
            signature_payload: vec![0xAA],
            reply_channel: "gmp-42".into(),
        };

        let req = SettleBridgePaymentRequest::new("relayer-1", packet);
        let result = client.settle_bridge_payment(req).await;
        assert!(result.is_ok());

        let settlement = result.unwrap();
        assert!(settlement.success);
        assert!(settlement.receipt.is_some());
        assert_eq!(settlement.receipt_hash, "hash123");
    }
}
