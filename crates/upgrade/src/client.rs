//! UpgradeClient — the main entry point for all upgrade-related queries
//! in the Morpheum SDK.
//!
//! Provides high-level, type-safe methods for querying upgrades, active
//! upgrades, validator readiness, and upgrade status summaries. Transaction
//! operations (signal ready, cancel, execute) are handled via the fluent
//! builders in `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};

use crate::requests::{
    QueryActiveUpgradesRequest, QueryUpgradeRequest, QueryUpgradeStatusRequest,
    QueryUpgradesRequest, QueryValidatorReadinessRequest,
};
use crate::types::{
    Upgrade, UpgradeStatus, UpgradeStatusSummary, UpgradeType, ValidatorReadinessOverview,
};

/// Primary client for all upgrade-related queries.
///
/// Transaction construction (signal ready, cancel, execute) is delegated to
/// the fluent builders in `builder.rs`.
pub struct UpgradeClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl UpgradeClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a single upgrade by ID.
    pub async fn query_upgrade(&self, upgrade_id: u64) -> Result<Upgrade, SdkError> {
        let req = QueryUpgradeRequest::new(upgrade_id);
        let proto_req: morpheum_proto::upgrade::v1::QueryUpgradeRequest = req.into();

        let response_bytes = self
            .query("/upgrade.v1.Query/QueryUpgrade", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::upgrade::v1::QueryUpgradeResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .upgrade
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("upgrade field missing in response"))
    }

    /// Queries upgrades with optional status/type filters and pagination.
    pub async fn query_upgrades(
        &self,
        limit: i32,
        offset: i32,
        status_filter: Option<UpgradeStatus>,
        type_filter: Option<UpgradeType>,
    ) -> Result<Vec<Upgrade>, SdkError> {
        let mut req = QueryUpgradesRequest::new(limit, offset);
        if let Some(status) = status_filter {
            req = req.status_filter(status);
        }
        if let Some(upgrade_type) = type_filter {
            req = req.type_filter(upgrade_type);
        }

        let proto_req: morpheum_proto::upgrade::v1::QueryUpgradesRequest = req.into();

        let response_bytes = self
            .query("/upgrade.v1.Query/QueryUpgrades", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::upgrade::v1::QueryUpgradesResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.upgrades.into_iter().map(Into::into).collect())
    }

    /// Queries currently active upgrades (usually 0 or 1).
    pub async fn query_active_upgrades(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Upgrade>, SdkError> {
        let req = QueryActiveUpgradesRequest::new(limit, offset);
        let proto_req: morpheum_proto::upgrade::v1::QueryActiveUpgradesRequest = req.into();

        let response_bytes = self
            .query(
                "/upgrade.v1.Query/QueryActiveUpgrades",
                proto_req.encode_to_vec(),
            )
            .await?;

        let proto_res = morpheum_proto::upgrade::v1::QueryActiveUpgradesResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.upgrades.into_iter().map(Into::into).collect())
    }

    /// Queries validator readiness for a specific upgrade.
    ///
    /// Optionally filter by a specific validator address.
    pub async fn query_validator_readiness(
        &self,
        upgrade_id: u64,
        validator_address: Option<String>,
    ) -> Result<ValidatorReadinessOverview, SdkError> {
        let mut req = QueryValidatorReadinessRequest::new(upgrade_id);
        if let Some(addr) = validator_address {
            req = req.validator_address(addr);
        }

        let proto_req: morpheum_proto::upgrade::v1::QueryValidatorReadinessRequest = req.into();

        let response_bytes = self
            .query(
                "/upgrade.v1.Query/QueryValidatorReadiness",
                proto_req.encode_to_vec(),
            )
            .await?;

        let proto_res = morpheum_proto::upgrade::v1::QueryValidatorReadinessResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(ValidatorReadinessOverview {
            upgrade_id: proto_res.upgrade_id,
            total_ready_count: proto_res.total_ready_count,
            required_threshold: proto_res.required_threshold,
            readiness_list: proto_res.readiness_list.into_iter().map(Into::into).collect(),
        })
    }

    /// Queries the upgrade status summary (fast path for AI agents and monitoring).
    pub async fn query_upgrade_status(
        &self,
        upgrade_id: u64,
    ) -> Result<UpgradeStatusSummary, SdkError> {
        let req = QueryUpgradeStatusRequest::new(upgrade_id);
        let proto_req: morpheum_proto::upgrade::v1::QueryUpgradeStatusRequest = req.into();

        let response_bytes = self
            .query(
                "/upgrade.v1.Query/QueryUpgradeStatus",
                proto_req.encode_to_vec(),
            )
            .await?;

        let proto_res = morpheum_proto::upgrade::v1::QueryUpgradeStatusResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(UpgradeStatusSummary {
            upgrade_id: proto_res.upgrade_id,
            status: UpgradeStatus::from(proto_res.status),
            activation_staple_id: proto_res.activation_staple_id,
            estimated_activation_time: proto_res
                .estimated_activation_time
                .map(|t| t.seconds as u64)
                .unwrap_or(0),
            ready_validator_count: proto_res.ready_validator_count,
            zero_downtime_guaranteed: proto_res.zero_downtime_guaranteed,
        })
    }
}

#[async_trait(?Send)]
impl MorpheumClient for UpgradeClient {
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
    use morpheum_sdk_core::{BroadcastResult, SdkConfig};

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _tx_bytes: Vec<u8>) -> Result<BroadcastResult, SdkError> {
            unimplemented!("not needed for upgrade query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/upgrade.v1.Query/QueryUpgrade" => {
                    let dummy = morpheum_proto::upgrade::v1::QueryUpgradeResponse {
                        success: true,
                        error_message: "".into(),
                        upgrade: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/upgrade.v1.Query/QueryUpgrades"
                | "/upgrade.v1.Query/QueryActiveUpgrades" => {
                    let dummy = morpheum_proto::upgrade::v1::QueryUpgradesResponse {
                        success: true,
                        error_message: "".into(),
                        upgrades: vec![],
                        total_count: 0,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/upgrade.v1.Query/QueryValidatorReadiness" => {
                    let dummy = morpheum_proto::upgrade::v1::QueryValidatorReadinessResponse {
                        success: true,
                        error_message: "".into(),
                        upgrade_id: 1,
                        total_ready_count: 0,
                        required_threshold: 67,
                        readiness_list: vec![],
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/upgrade.v1.Query/QueryUpgradeStatus" => {
                    let dummy = morpheum_proto::upgrade::v1::QueryUpgradeStatusResponse {
                        success: true,
                        error_message: "".into(),
                        upgrade_id: 1,
                        status: 1,
                        activation_staple_id: 100,
                        estimated_activation_time: None,
                        ready_validator_count: 50,
                        zero_downtime_guaranteed: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn upgrade_client_query_upgrade_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = UpgradeClient::new(config, Box::new(DummyTransport));
        let result = client.query_upgrade(1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn upgrade_client_query_upgrades_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = UpgradeClient::new(config, Box::new(DummyTransport));
        let result = client.query_upgrades(10, 0, None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn upgrade_client_query_active_upgrades_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = UpgradeClient::new(config, Box::new(DummyTransport));
        let result = client.query_active_upgrades(10, 0).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn upgrade_client_query_validator_readiness_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = UpgradeClient::new(config, Box::new(DummyTransport));
        let result = client.query_validator_readiness(1, None).await;
        assert!(result.is_ok());
        let overview = result.unwrap();
        assert_eq!(overview.required_threshold, 67);
    }

    #[tokio::test]
    async fn upgrade_client_query_status_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = UpgradeClient::new(config, Box::new(DummyTransport));
        let result = client.query_upgrade_status(1).await;
        assert!(result.is_ok());
        let summary = result.unwrap();
        assert_eq!(summary.status, UpgradeStatus::Scheduled);
        assert!(summary.zero_downtime_guaranteed);
    }
}
