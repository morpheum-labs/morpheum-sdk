//! BankClient — the main entry point for all bank-related query operations
//! in the Morpheum SDK.
//!
//! Provides high-level, type-safe methods for querying balances, transactions,
//! and transaction history. Transaction operations (transfer, mint, deposit,
//! withdraw, etc.) are handled via the fluent builders in `builder.rs` +
//! `TxBuilder`.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};

use crate::requests::{QueryBalanceRequest, QueryBalancesRequest};
use crate::types::Balance;

/// Primary client for all bank-related queries.
///
/// Transaction construction (transfer, mint, deposit, withdraw, bridge, etc.)
/// is delegated to the fluent builders in `builder.rs` for maximum ergonomics
/// and type safety.
pub struct BankClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl BankClient {
    /// Creates a new `BankClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries balance for a single asset by index.
    pub async fn query_balance(
        &self,
        address: impl Into<String>,
        asset_index: u64,
    ) -> Result<BalanceResponse, SdkError> {
        let req = QueryBalanceRequest::new(address, asset_index);
        let proto_req: morpheum_proto::bank::v1::QueryBalanceRequest = req.into();

        let path = "/bank.v1.Query/QueryBalance";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::bank::v1::QueryBalanceResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(BalanceResponse {
            address: proto_res.address,
            balance: proto_res.balance,
            available_balance: proto_res.available_balance,
            locked_balance: proto_res.locked_balance,
            external_address: proto_res.external_address,
        })
    }

    /// Queries all balances for an address.
    pub async fn query_balances(
        &self,
        address: impl Into<String>,
    ) -> Result<Vec<Balance>, SdkError> {
        let req = QueryBalancesRequest::new(address);
        let proto_req: morpheum_proto::bank::v1::QueryBalancesRequest = req.into();

        let path = "/bank.v1.Query/QueryBalances";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::bank::v1::QueryBalancesResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res
            .balances
            .into_iter()
            .map(|b| Balance {
                asset_index: b.asset.as_ref().map_or(0, |a| a.asset_index),
                asset_symbol: b.asset.as_ref().map_or_else(String::new, |a| a.symbol.clone()),
                balance: b.balance,
                available_balance: b.available_balance,
                locked_balance: b.locked_balance,
            })
            .collect())
    }

}

/// Response from a single-asset balance query.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BalanceResponse {
    pub address: String,
    pub balance: String,
    pub available_balance: String,
    pub locked_balance: String,
    pub external_address: Option<String>,
}

#[async_trait(?Send)]
impl MorpheumClient for BankClient {
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
            unimplemented!("not needed for bank query tests")
        }

        async fn query(
            &self,
            path: &str,
            _data: Vec<u8>,
        ) -> Result<Vec<u8>, SdkError> {
            match path {
                "/bank.v1.Query/QueryBalance" => {
                    let dummy = morpheum_proto::bank::v1::QueryBalanceResponse {
                        address: "morm1test".into(),
                        balance: "1000".into(),
                        available_balance: "900".into(),
                        locked_balance: "100".into(),
                        ..Default::default()
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/bank.v1.Query/QueryBalances" => {
                    let dummy = morpheum_proto::bank::v1::QueryBalancesResponse {
                        address: "morm1test".into(),
                        balances: vec![],
                        external_address: None,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn bank_client_query_balance_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = BankClient::new(config, Box::new(DummyTransport));

        let result = client.query_balance("morm1test", 0).await;
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert_eq!(resp.balance, "1000");
        assert_eq!(resp.available_balance, "900");
    }

    #[tokio::test]
    async fn bank_client_query_balances_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = BankClient::new(config, Box::new(DummyTransport));

        let result = client.query_balances("morm1test").await;
        assert!(result.is_ok());
    }

}
