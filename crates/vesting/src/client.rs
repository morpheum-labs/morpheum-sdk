//! VestingClient — queries for vesting summaries, entries, and parameters.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::vesting::v1 as proto;

use crate::requests;
use crate::types::{VestingEntry, VestingParams, VestingSummary};

/// Vesting entry query result with currently releasable amount.
pub struct VestingEntryResult {
    pub entry: VestingEntry,
    pub currently_releasable: alloc::string::String,
}

/// Paginated vesting entries result.
pub struct VestingEntriesPage {
    pub entries: Vec<VestingEntry>,
    pub total_count: u32,
}

/// Primary client for vesting module queries.
pub struct VestingClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl VestingClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets aggregated vesting summary for a beneficiary.
    pub async fn get_summary(&self, req: requests::QueryVestingSummaryRequest) -> Result<VestingSummary, SdkError> {
        let proto_req: proto::QueryVestingSummaryRequest = req.into();
        let resp = self.query("/vesting.v1.Query/QueryVestingSummary", proto_req.encode_to_vec()).await?;
        let p = proto::QueryVestingSummaryResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.summary.map(Into::into).ok_or_else(|| SdkError::transport("summary field missing"))
    }

    /// Gets a specific vesting entry by ID.
    pub async fn get_entry(&self, req: requests::QueryVestingEntryRequest) -> Result<VestingEntryResult, SdkError> {
        let proto_req: proto::QueryVestingEntryRequest = req.into();
        let resp = self.query("/vesting.v1.Query/QueryVestingEntry", proto_req.encode_to_vec()).await?;
        let p = proto::QueryVestingEntryResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(VestingEntryResult {
            entry: p.entry.map(Into::into).ok_or_else(|| SdkError::transport("entry field missing"))?,
            currently_releasable: p.currently_releasable,
        })
    }

    /// Lists all vesting entries for a beneficiary.
    pub async fn list_entries(&self, req: requests::QueryVestingEntriesRequest) -> Result<VestingEntriesPage, SdkError> {
        let proto_req: proto::QueryVestingEntriesRequest = req.into();
        let resp = self.query("/vesting.v1.Query/QueryVestingEntries", proto_req.encode_to_vec()).await?;
        let p = proto::QueryVestingEntriesResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(VestingEntriesPage {
            entries: p.entries.into_iter().map(Into::into).collect(),
            total_count: p.total_count,
        })
    }

    /// Gets current governance parameters.
    pub async fn get_params(&self) -> Result<VestingParams, SdkError> {
        let proto_req: proto::QueryParamsRequest = requests::QueryParamsRequest.into();
        let resp = self.query("/vesting.v1.Query/QueryParams", proto_req.encode_to_vec()).await?;
        let p = proto::QueryParamsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.params.map(Into::into).ok_or_else(|| SdkError::transport("params field missing"))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for VestingClient {
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
                "/vesting.v1.Query/QueryVestingSummary" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryVestingSummaryResponse {
                        summary: Some(proto::VestingSummary {
                            beneficiary: "morph1user".into(), total_vested: "1000000".into(),
                            total_released: "250000".into(), currently_releasable: "50000".into(),
                            total_locked: "700000".into(), next_unlock_timestamp: 1700100000,
                            entry_count: 3,
                        }),
                        current_timestamp: 1700000000,
                    }))
                }
                "/vesting.v1.Query/QueryVestingEntry" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryVestingEntryResponse {
                        entry: Some(proto::VestingEntry {
                            id: 1, beneficiary: "morph1user".into(),
                            total_amount: "500000".into(), released: "100000".into(),
                            start_timestamp: 1690000000, cliff_duration: 31536000,
                            vesting_duration: 63072000, schedule_type: 2,
                            revocable: true, category: 2,
                            step_timestamps: vec![], step_amounts: vec![],
                        }),
                        currently_releasable: "25000".into(),
                    }))
                }
                "/vesting.v1.Query/QueryVestingEntries" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryVestingEntriesResponse {
                        entries: vec![], total_count: 0,
                    }))
                }
                "/vesting.v1.Query/QueryParams" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryParamsResponse {
                        params: Some(proto::Params {
                            max_entries_per_account: 32, max_cliff_duration: 157680000,
                            max_vesting_duration: 315360000, min_vesting_duration: 2592000,
                            min_vesting_amount: "100000".into(),
                            allow_governance_revocation: true,
                            default_cliff_duration: 31536000,
                        }),
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> VestingClient {
        VestingClient::new(
            SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"),
            Box::new(DummyTransport),
        )
    }

    #[tokio::test]
    async fn get_summary_works() {
        let s = make_client()
            .get_summary(requests::QueryVestingSummaryRequest::new("morph1user"))
            .await.unwrap();
        assert_eq!(s.entry_count, 3);
        assert_eq!(s.currently_releasable, "50000");
    }

    #[tokio::test]
    async fn get_entry_works() {
        let r = make_client()
            .get_entry(requests::QueryVestingEntryRequest::new("morph1user", 1))
            .await.unwrap();
        assert_eq!(r.entry.id, 1);
        assert_eq!(r.currently_releasable, "25000");
    }

    #[tokio::test]
    async fn list_entries_works() {
        let page = make_client()
            .list_entries(requests::QueryVestingEntriesRequest::new("morph1user"))
            .await.unwrap();
        assert!(page.entries.is_empty());
        assert_eq!(page.total_count, 0);
    }

    #[tokio::test]
    async fn get_params_works() {
        let p = make_client().get_params().await.unwrap();
        assert_eq!(p.max_entries_per_account, 32);
        assert!(p.allow_governance_revocation);
    }
}
