//! InsuranceClient — queries for vault balances, LP stakes, bad debt, IL metrics, and thresholds.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::insurance::v1 as proto;

use crate::requests;
use crate::types::{BadDebtRecord, IlMetrics, LpStake, PageInfo, ThresholdStatus, VaultBalance};

macro_rules! svc_path {
    ($method:expr) => {
        concat!("/insurance.v1.InsuranceService/", $method)
    };
}

/// Primary client for insurance vault queries.
pub struct InsuranceClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl InsuranceClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets the vault balance for an asset (or the default if none specified).
    pub async fn get_vault_balance(&self, asset_index: Option<u64>) -> Result<VaultBalance, SdkError> {
        let req = requests::GetVaultBalanceRequest::new(asset_index);
        let proto_req: proto::GetVaultBalanceRequest = req.into();
        let resp = self.query(svc_path!("GetVaultBalance"), proto_req.encode_to_vec()).await?;
        let p = proto::GetVaultBalanceResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.balance.map(Into::into).ok_or_else(|| SdkError::transport("balance field missing"))
    }

    /// Gets a specific LP stake by address.
    pub async fn get_lp_stake(&self, req: requests::GetLpStakeRequest) -> Result<LpStake, SdkError> {
        let proto_req: proto::GetLpStakeRequest = req.into();
        let resp = self.query(svc_path!("GetLPStake"), proto_req.encode_to_vec()).await?;
        let p = proto::GetLpStakeResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.stake.map(Into::into).ok_or_else(|| SdkError::transport("stake field missing"))
    }

    /// Lists all LP stakes with pagination.
    pub async fn list_lp_stakes(&self, req: requests::ListLpStakesRequest) -> Result<(Vec<LpStake>, PageInfo), SdkError> {
        let proto_req: proto::ListLpStakesRequest = req.into();
        let resp = self.query(svc_path!("ListLPStakes"), proto_req.encode_to_vec()).await?;
        let p = proto::ListLpStakesResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        let stakes = p.stakes.into_iter().map(Into::into).collect();
        let page = p.pagination.map_or(PageInfo { next_key: Vec::new(), total: 0 }, |pg| {
            PageInfo { next_key: pg.next_key, total: pg.total }
        });
        Ok((stakes, page))
    }

    /// Gets bad debt history with pagination and optional time bounds.
    pub async fn get_bad_debt_history(&self, req: requests::GetBadDebtHistoryRequest) -> Result<(Vec<BadDebtRecord>, PageInfo), SdkError> {
        let proto_req: proto::GetBadDebtHistoryRequest = req.into();
        let resp = self.query(svc_path!("GetBadDebtHistory"), proto_req.encode_to_vec()).await?;
        let p = proto::GetBadDebtHistoryResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        let records = p.records.into_iter().map(Into::into).collect();
        let page = p.pagination.map_or(PageInfo { next_key: Vec::new(), total: 0 }, |pg| {
            PageInfo { next_key: pg.next_key, total: pg.total }
        });
        Ok((records, page))
    }

    /// Gets impermanent loss metrics for an asset.
    pub async fn get_il_metrics(&self, asset_index: Option<u64>) -> Result<IlMetrics, SdkError> {
        let req = requests::GetIlMetricsRequest::new(asset_index);
        let proto_req: proto::GetIlMetricsRequest = req.into();
        let resp = self.query(svc_path!("GetILMetrics"), proto_req.encode_to_vec()).await?;
        let p = proto::GetIlMetricsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.metrics.map(Into::into).ok_or_else(|| SdkError::transport("metrics field missing"))
    }

    /// Gets the vault threshold status for an asset.
    pub async fn get_threshold_status(&self, asset_index: Option<u64>) -> Result<ThresholdStatus, SdkError> {
        let req = requests::GetThresholdStatusRequest::new(asset_index);
        let proto_req: proto::GetThresholdStatusRequest = req.into();
        let resp = self.query(svc_path!("GetThresholdStatus"), proto_req.encode_to_vec()).await?;
        let p = proto::GetThresholdStatusResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.status.map(Into::into).ok_or_else(|| SdkError::transport("status field missing"))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for InsuranceClient {
    fn config(&self) -> &SdkConfig { &self.config }
    fn transport(&self) -> &dyn Transport { &*self.transport }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!()
        }
        async fn query(&self, path: &str, _: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/insurance.v1.InsuranceService/GetVaultBalance" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetVaultBalanceResponse {
                        success: true,
                        balance: Some(proto::VaultBalance {
                            asset: Some(morpheum_proto::primitives::v1::Asset {
                                asset_index: 1, symbol: "USDC".into(), ..Default::default()
                            }),
                            total_balance: "1000000".into(),
                            available_balance: "800000".into(),
                            reserved_balance: "200000".into(),
                            min_threshold: "100000".into(),
                            updated_at: None,
                        }),
                    }))
                }
                "/insurance.v1.InsuranceService/GetLPStake" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetLpStakeResponse {
                        success: true,
                        stake: Some(proto::LpStake {
                            stake_id: "s1".into(), address: "morph1xyz".into(),
                            asset: Some(morpheum_proto::primitives::v1::Asset {
                                asset_index: 1, symbol: "USDC".into(), ..Default::default()
                            }),
                            amount: "500".into(), shares: "500".into(), pending_yield: "10".into(),
                            stake_time: None, last_claim_time: None,
                            external_address: None, chain_type: None,
                        }),
                    }))
                }
                "/insurance.v1.InsuranceService/ListLPStakes" => {
                    Ok(prost::Message::encode_to_vec(&proto::ListLpStakesResponse {
                        success: true, stakes: vec![], pagination: None,
                    }))
                }
                "/insurance.v1.InsuranceService/GetThresholdStatus" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetThresholdStatusResponse {
                        success: true,
                        status: Some(proto::ThresholdStatus {
                            asset: None, near_depletion: false, paused: false,
                            depletion_percentage: "5000".into(), last_checked: None,
                        }),
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> InsuranceClient {
        InsuranceClient::new(
            SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"),
            Box::new(DummyTransport),
        )
    }

    #[tokio::test]
    async fn get_vault_balance_works() {
        let bal = make_client().get_vault_balance(Some(1)).await.unwrap();
        assert_eq!(bal.asset_index, 1);
        assert_eq!(bal.total_balance, "1000000");
    }

    #[tokio::test]
    async fn get_lp_stake_works() {
        let stake = make_client()
            .get_lp_stake(requests::GetLpStakeRequest::new("morph1xyz"))
            .await.unwrap();
        assert_eq!(stake.stake_id, "s1");
        assert_eq!(stake.amount, "500");
    }

    #[tokio::test]
    async fn list_lp_stakes_works() {
        let (stakes, page) = make_client()
            .list_lp_stakes(requests::ListLpStakesRequest::new(0, 50))
            .await.unwrap();
        assert!(stakes.is_empty());
        assert_eq!(page.total, 0);
    }

    #[tokio::test]
    async fn get_threshold_status_works() {
        let status = make_client().get_threshold_status(None).await.unwrap();
        assert!(!status.near_depletion);
        assert_eq!(status.depletion_percentage, "5000");
    }
}
