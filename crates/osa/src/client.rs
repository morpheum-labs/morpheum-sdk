//! OsaClient — queries for outcome settlement accounts and user balances.

use alloc::boxed::Box;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::osa::v1 as proto;

use crate::requests;
use crate::types::{Balance, OutcomeSettlementAccount};

/// Primary client for outcome settlement account queries.
pub struct OsaClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl OsaClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets a single outcome settlement account by ID.
    pub async fn get_account(&self, account_id: &str) -> Result<OutcomeSettlementAccount, SdkError> {
        let req = requests::GetAccountRequest::new(account_id);
        let proto_req: proto::QueryGetAccountRequest = req.into();
        let resp = self.query("/osa.v1.Query/GetAccount", proto_req.encode_to_vec()).await?;
        let p = proto::QueryGetAccountResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.account.map(Into::into).ok_or_else(|| SdkError::transport("account field missing"))
    }

    /// Gets user share balance within an outcome settlement account.
    pub async fn get_balance(&self, account_id: &str, address: &str) -> Result<Balance, SdkError> {
        let req = requests::GetBalanceRequest::new(account_id, address);
        let proto_req: proto::QueryGetBalanceRequest = req.into();
        let resp = self.query("/osa.v1.Query/GetBalance", proto_req.encode_to_vec()).await?;
        let p = proto::QueryGetBalanceResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.balance.map(Into::into).ok_or_else(|| SdkError::transport("balance field missing"))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for OsaClient {
    fn config(&self) -> &SdkConfig { &self.config }
    fn transport(&self) -> &dyn Transport { &*self.transport }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use alloc::vec::Vec;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!()
        }
        async fn query(&self, path: &str, _: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/osa.v1.Query/GetAccount" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryGetAccountResponse {
                        success: true, error_message: String::new(),
                        account: Some(proto::OutcomeSettlementAccount {
                            market_index: 1, outcome_id: "yes".into(),
                            collateral_asset_index: 2, total_locked_collateral: 1000,
                            total_shares_outstanding: 1000, status: 1,
                            redemption_rate: 0, created_at: 100, settled_at: None,
                        }),
                    }))
                }
                "/osa.v1.Query/GetBalance" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryGetBalanceResponse {
                        success: true, error_message: String::new(),
                        balance: Some(proto::Balance { shares: 500, last_claimed: Some(200) }),
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> OsaClient {
        OsaClient::new(
            SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"),
            Box::new(DummyTransport),
        )
    }

    #[tokio::test]
    async fn get_account_works() {
        let acct = make_client().get_account("acct1").await.unwrap();
        assert_eq!(acct.outcome_id, "yes");
        assert_eq!(acct.status, crate::types::AccountStatus::Open);
    }

    #[tokio::test]
    async fn get_balance_works() {
        let bal = make_client().get_balance("acct1", "morph1xyz").await.unwrap();
        assert_eq!(bal.shares, 500);
        assert_eq!(bal.last_claimed, Some(200));
    }
}
