//! VaultClient — queries for vaults, stakes, strategy history, health, and metrics.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::vault::v1 as proto;

use crate::requests;
use crate::types::{
    IlMetrics, Stake, StrategyExecution, Vault, VaultHealth, VaultParams,
};

// ====================== RESULT TYPES ======================

/// Paginated vault list result.
pub struct VaultListPage {
    pub vaults: Vec<Vault>,
}

/// Paginated stakes result.
pub struct StakeListPage {
    pub stakes: Vec<Stake>,
}

/// Paginated strategy execution history.
pub struct StrategyHistoryPage {
    pub executions: Vec<StrategyExecution>,
}

// ====================== CLIENT ======================

macro_rules! svc_path {
    ($method:literal) => {
        concat!("/vault.v1.Query/", $method)
    };
}

/// Primary client for vault module queries.
pub struct VaultClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl VaultClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets a specific vault by ID.
    pub async fn get_vault(&self, req: requests::GetVaultRequest) -> Result<Vault, SdkError> {
        let proto_req: proto::GetVaultRequest = req.into();
        let resp = self.query(svc_path!("GetVault"), proto_req.encode_to_vec()).await?;
        let p = proto::GetVaultResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.vault.map(Into::into).ok_or_else(|| SdkError::transport("vault field missing"))
    }

    /// Lists vaults with optional filters.
    pub async fn list_vaults(&self, req: requests::ListVaultsRequest) -> Result<VaultListPage, SdkError> {
        let proto_req: proto::ListVaultsRequest = req.into();
        let resp = self.query(svc_path!("ListVaults"), proto_req.encode_to_vec()).await?;
        let p = proto::ListVaultsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(VaultListPage { vaults: p.vaults.into_iter().map(Into::into).collect() })
    }

    /// Gets all vaults by agent.
    pub async fn get_vaults_by_agent(&self, req: requests::GetVaultsByAgentRequest) -> Result<VaultListPage, SdkError> {
        let proto_req: proto::GetVaultsByAgentRequest = req.into();
        let resp = self.query(svc_path!("GetVaultsByAgent"), proto_req.encode_to_vec()).await?;
        let p = proto::GetVaultsByAgentResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(VaultListPage { vaults: p.vaults.into_iter().map(Into::into).collect() })
    }

    /// Gets vaults by type.
    pub async fn get_vaults_by_type(&self, req: requests::GetVaultsByTypeRequest) -> Result<VaultListPage, SdkError> {
        let proto_req: proto::GetVaultsByTypeRequest = req.into();
        let resp = self.query(svc_path!("GetVaultsByType"), proto_req.encode_to_vec()).await?;
        let p = proto::GetVaultsByTypeResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(VaultListPage { vaults: p.vaults.into_iter().map(Into::into).collect() })
    }

    /// Gets a user's stake in a specific vault.
    pub async fn get_user_stake(&self, req: requests::GetUserStakeRequest) -> Result<Stake, SdkError> {
        let proto_req: proto::GetUserStakeRequest = req.into();
        let resp = self.query(svc_path!("GetUserStake"), proto_req.encode_to_vec()).await?;
        let p = proto::GetUserStakeResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.stake.map(Into::into).ok_or_else(|| SdkError::transport("stake field missing"))
    }

    /// Lists all stakes for a user across vaults.
    pub async fn list_user_stakes(&self, req: requests::ListUserStakesRequest) -> Result<StakeListPage, SdkError> {
        let proto_req: proto::ListUserStakesRequest = req.into();
        let resp = self.query(svc_path!("ListUserStakes"), proto_req.encode_to_vec()).await?;
        let p = proto::ListUserStakesResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(StakeListPage { stakes: p.stakes.into_iter().map(Into::into).collect() })
    }

    /// Gets strategy execution history for a vault.
    pub async fn get_strategy_history(&self, req: requests::GetStrategyHistoryRequest) -> Result<StrategyHistoryPage, SdkError> {
        let proto_req: proto::GetStrategyHistoryRequest = req.into();
        let resp = self.query(svc_path!("GetStrategyHistory"), proto_req.encode_to_vec()).await?;
        let p = proto::GetStrategyHistoryResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(StrategyHistoryPage { executions: p.executions.into_iter().map(Into::into).collect() })
    }

    /// Gets IL metrics for a vault.
    pub async fn get_il_metrics(&self, req: requests::GetIlMetricsRequest) -> Result<IlMetrics, SdkError> {
        let proto_req: proto::GetIlMetricsRequest = req.into();
        let resp = self.query(svc_path!("GetILMetrics"), proto_req.encode_to_vec()).await?;
        let p = proto::GetIlMetricsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.metrics.map(Into::into).ok_or_else(|| SdkError::transport("metrics field missing"))
    }

    /// Gets real-time vault health.
    pub async fn get_vault_health(&self, req: requests::GetVaultHealthRequest) -> Result<VaultHealth, SdkError> {
        let proto_req: proto::GetVaultHealthRequest = req.into();
        let resp = self.query(svc_path!("GetVaultHealth"), proto_req.encode_to_vec()).await?;
        let p = proto::GetVaultHealthResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.health.map(Into::into).ok_or_else(|| SdkError::transport("health field missing"))
    }

    /// Gets top vaults ranked by metric.
    pub async fn get_top_vaults(&self, req: requests::GetTopVaultsRequest) -> Result<VaultListPage, SdkError> {
        let proto_req: proto::GetTopVaultsRequest = req.into();
        let resp = self.query(svc_path!("GetTopVaults"), proto_req.encode_to_vec()).await?;
        let p = proto::GetTopVaultsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(VaultListPage { vaults: p.vaults.into_iter().map(Into::into).collect() })
    }

    /// Gets current governance parameters.
    pub async fn get_params(&self) -> Result<VaultParams, SdkError> {
        Err(SdkError::transport("vault params query not exposed in proto Query service — use governance queries"))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for VaultClient {
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
                "/vault.v1.Query/GetVault" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetVaultResponse {
                        success: true,
                        vault: Some(proto::Vault {
                            vault_id: "v1".into(), agent_id: "a1".into(), r#type: 1,
                            name: "Test".into(), description: "".into(), asset: None,
                            total_assets: "1000".into(), available_assets: "800".into(),
                            reserved_assets: "200".into(), status: 1,
                            created_at: None, updated_at: None, strategy_hash: "".into(),
                            health_score: "9500".into(), pnl_30d_usd: "100".into(),
                            apy_bps: "1200".into(), vc_claim_hash: vec![], copy_count: "5".into(),
                        }),
                        error_message: "".into(),
                    }))
                }
                "/vault.v1.Query/ListVaults" => {
                    Ok(prost::Message::encode_to_vec(&proto::ListVaultsResponse {
                        success: true, vaults: vec![], pagination: None,
                    }))
                }
                "/vault.v1.Query/GetUserStake" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetUserStakeResponse {
                        success: true,
                        stake: Some(proto::Stake {
                            stake_id: "s1".into(), address: "morph1user".into(),
                            vault_id: "v1".into(), asset: None, amount: "500".into(),
                            shares: "500".into(), pending_yield: "10".into(),
                            stake_time: None, last_claim_time: None,
                        }),
                        error_message: "".into(),
                    }))
                }
                "/vault.v1.Query/GetVaultHealth" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetVaultHealthResponse {
                        success: true,
                        health: Some(proto::VaultHealth {
                            vault_id: "v1".into(), health_score: "9500".into(),
                            apy_bps: "1200".into(), pnl_24h: "50".into(),
                            risk_score: "300".into(), timestamp: None,
                        }),
                    }))
                }
                "/vault.v1.Query/GetTopVaults" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetTopVaultsResponse {
                        success: true, vaults: vec![], pagination: None,
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> VaultClient {
        VaultClient::new(
            SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"),
            Box::new(DummyTransport),
        )
    }

    #[tokio::test]
    async fn get_vault_works() {
        let v = make_client().get_vault(requests::GetVaultRequest::new("v1")).await.unwrap();
        assert_eq!(v.vault_id, "v1");
        assert_eq!(v.total_assets, "1000");
    }

    #[tokio::test]
    async fn list_vaults_works() {
        let page = make_client().list_vaults(requests::ListVaultsRequest::new()).await.unwrap();
        assert!(page.vaults.is_empty());
    }

    #[tokio::test]
    async fn get_user_stake_works() {
        let s = make_client()
            .get_user_stake(requests::GetUserStakeRequest::new("morph1user", "v1"))
            .await.unwrap();
        assert_eq!(s.amount, "500");
    }

    #[tokio::test]
    async fn get_vault_health_works() {
        let h = make_client()
            .get_vault_health(requests::GetVaultHealthRequest::new("v1"))
            .await.unwrap();
        assert_eq!(h.health_score, "9500");
    }

    #[tokio::test]
    async fn get_top_vaults_works() {
        let page = make_client()
            .get_top_vaults(requests::GetTopVaultsRequest::new("apy"))
            .await.unwrap();
        assert!(page.vaults.is_empty());
    }
}
