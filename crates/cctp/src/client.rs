//! Typed query helpers for the `hpl-cctp-handler` contract.
//!
//! Each function constructs the correct `QuerySmartRequest`, calls
//! `CosmWasmClient::query_smart`, and deserializes the response into
//! the canonical contract types — eliminating manual JSON construction
//! and base64 encoding from every consumer.

use morpheum_sdk_cosmwasm::client::CosmWasmClient;
use morpheum_sdk_cosmwasm::requests::QuerySmartRequest;

use crate::error::CctpError;
use crate::types::*;

/// Queries all pending CCTP transfers from the handler contract.
pub async fn query_pending_transfers(
    client: &CosmWasmClient,
    handler: &str,
) -> Result<Vec<PendingTransfer>, CctpError> {
    let resp: PendingTransfersResponse = smart_query(
        client,
        handler,
        &QueryMsg::PendingTransfers {},
    )
    .await?;
    Ok(resp.transfers)
}

/// Queries the handler contract configuration.
pub async fn query_config(
    client: &CosmWasmClient,
    handler: &str,
) -> Result<ConfigResponse, CctpError> {
    smart_query(client, handler, &QueryMsg::Config {}).await
}

/// Queries a specific pending transfer by (source_domain, nonce).
///
/// Returns `None` if no transfer exists for the given key.
pub async fn query_pending_by_nonce(
    client: &CosmWasmClient,
    handler: &str,
    source_domain: u32,
    nonce: u64,
) -> Result<Option<PendingTransfer>, CctpError> {
    let resp: PendingTransferResponse = smart_query(
        client,
        handler,
        &QueryMsg::PendingByNonce {
            source_domain,
            nonce,
        },
    )
    .await?;
    Ok(resp.transfer)
}

/// Queries all enrolled remote routes.
pub async fn query_routes(
    client: &CosmWasmClient,
    handler: &str,
) -> Result<Vec<RouteResponse>, CctpError> {
    let resp: RoutesResponse =
        smart_query(client, handler, &QueryMsg::Routes {}).await?;
    Ok(resp.routes)
}

/// Internal helper: serialize query, call `query_smart`, deserialize response.
async fn smart_query<R: serde::de::DeserializeOwned>(
    client: &CosmWasmClient,
    handler: &str,
    msg: &QueryMsg,
) -> Result<R, CctpError> {
    let query_data = serde_json::to_vec(msg)
        .map_err(|e| CctpError::Serialization(e.to_string()))?;

    let req = QuerySmartRequest {
        contract: handler.to_string(),
        query_data,
    };

    let resp_bytes = client
        .query_smart(&req)
        .await
        .map_err(|e| CctpError::Query(e.to_string()))?;

    serde_json::from_slice(&resp_bytes)
        .map_err(|e| CctpError::Deserialization(e.to_string()))
}
