//! `MarketplaceClient` — the main entry point for marketplace-related operations
//! in the Morpheum SDK.
//!
//! This client provides high-level, type-safe methods for querying marketplace
//! listings, bids, active listings, and module parameters. Transaction
//! operations (list, bid, accept, evaluate, params) are handled via the fluent
//! builders in `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_proto::marketplace::v1 as proto;
use morpheum_sdk_core::{
    MorpheumClient, SdkConfig, SdkError, Transport,
};

use crate::{
    requests::{
        QueryActiveListingsRequest,
        QueryBidsByListingRequest,
        QueryListingRequest,
        QueryListingsRequest,
    },
    types::{AgentListing, Bid, Params},
};

/// Primary client for all marketplace-related queries.
///
/// Transaction construction (list, bid, accept, evaluate, params) is delegated
/// to the fluent builders in `builder.rs` for maximum ergonomics and type safety.
pub struct MarketplaceClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl MarketplaceClient {
    /// Creates a new `MarketplaceClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a specific listing by ID.
    ///
    /// Returns `None` if the listing is not found.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError`] if the transport query fails or the response cannot
    /// be decoded.
    pub async fn query_listing(
        &self,
        listing_id: impl Into<alloc::string::String>,
    ) -> Result<Option<AgentListing>, SdkError> {
        let request = QueryListingRequest::new(listing_id);
        let encoded: proto::QueryListingRequest = request.into();

        let raw = self
            .query("/marketplace.v1.Query/QueryListing", encoded.encode_to_vec())
            .await?;

        let decoded = proto::QueryListingResponse::decode(raw.as_slice())
            .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryListingResponse = decoded.into();
        if response.found { Ok(response.listing) } else { Ok(None) }
    }

    /// Queries multiple listings with optional seller/status filters (paginated).
    ///
    /// # Errors
    ///
    /// Returns [`SdkError`] if the transport query fails or the response cannot
    /// be decoded.
    pub async fn query_listings(
        &self,
        request: QueryListingsRequest,
    ) -> Result<(Vec<AgentListing>, u32), SdkError> {
        let encoded: proto::QueryListingsRequest = request.into();

        let raw = self
            .query("/marketplace.v1.Query/QueryListings", encoded.encode_to_vec())
            .await?;

        let decoded = proto::QueryListingsResponse::decode(raw.as_slice())
            .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryListingsResponse = decoded.into();
        Ok((response.listings, response.total_count))
    }

    /// Queries all bids for a specific listing (paginated).
    ///
    /// # Errors
    ///
    /// Returns [`SdkError`] if the transport query fails or the response cannot
    /// be decoded.
    pub async fn query_bids_by_listing(
        &self,
        listing_id: impl Into<alloc::string::String>,
        limit: u32,
        offset: u32,
    ) -> Result<(Vec<Bid>, u32), SdkError> {
        let request = QueryBidsByListingRequest::new(listing_id, limit, offset);
        let encoded: proto::QueryBidsByListingRequest = request.into();

        let raw = self
            .query("/marketplace.v1.Query/QueryBidsByListing", encoded.encode_to_vec())
            .await?;

        let decoded = proto::QueryBidsByListingResponse::decode(raw.as_slice())
            .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryBidsByListingResponse = decoded.into();
        Ok((response.bids, response.total_count))
    }

    /// Queries all currently active listings (paginated).
    ///
    /// # Errors
    ///
    /// Returns [`SdkError`] if the transport query fails or the response cannot
    /// be decoded.
    pub async fn query_active_listings(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<(Vec<AgentListing>, u32), SdkError> {
        let request = QueryActiveListingsRequest::new(limit, offset);
        let encoded: proto::QueryActiveListingsRequest = request.into();

        let raw = self
            .query("/marketplace.v1.Query/QueryActiveListings", encoded.encode_to_vec())
            .await?;

        let decoded = proto::QueryActiveListingsResponse::decode(raw.as_slice())
            .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryActiveListingsResponse = decoded.into();
        Ok((response.listings, response.total_count))
    }

    /// Queries the current module parameters.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError`] if the transport query fails or the response cannot
    /// be decoded.
    pub async fn query_params(&self) -> Result<Option<Params>, SdkError> {
        let request = crate::requests::QueryParamsRequest;
        let encoded: proto::QueryParamsRequest = request.into();

        let raw = self
            .query("/marketplace.v1.Query/QueryParams", encoded.encode_to_vec())
            .await?;

        let decoded = proto::QueryParamsResponse::decode(raw.as_slice())
            .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryParamsResponse = decoded.into();
        Ok(response.params)
    }
}

#[async_trait(?Send)]
impl MorpheumClient for MarketplaceClient {
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
    use crate::types::ListingStatus;
    use morpheum_sdk_core::SdkConfig;

    // Deterministic transport for compile-time and basic runtime testing
    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(
            &self,
            _tx_bytes: Vec<u8>,
        ) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!("not needed for marketplace query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/marketplace.v1.Query/QueryListing" => {
                    let dummy = proto::QueryListingResponse {
                        listing: Some(proto::AgentListing {
                            listing_id: "listing-001".into(),
                            agent_hash: "agent-abc".into(),
                            seller_agent_hash: "seller-xyz".into(),
                            listing_type: 0, // FullOwnership
                            price_usd: 1_000_000,
                            status: 0, // Active
                            ..Default::default()
                        }),
                        found: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/marketplace.v1.Query/QueryListings" => {
                    let dummy = proto::QueryListingsResponse {
                        listings: vec![proto::AgentListing::default()],
                        total_count: 1,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/marketplace.v1.Query/QueryBidsByListing" => {
                    let dummy = proto::QueryBidsByListingResponse {
                        bids: vec![
                            proto::Bid {
                                bid_id: "bid-001".into(),
                                listing_id: "listing-001".into(),
                                amount_usd: 450_000,
                                ..Default::default()
                            },
                        ],
                        total_count: 1,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/marketplace.v1.Query/QueryActiveListings" => {
                    let dummy = proto::QueryActiveListingsResponse {
                        listings: vec![proto::AgentListing::default(), proto::AgentListing::default()],
                        total_count: 2,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/marketplace.v1.Query/QueryParams" => {
                    let dummy = proto::QueryParamsResponse {
                        params: Some(proto::Params {
                            default_platform_cut_bps: 250,
                            default_escrow_timeout_seconds: 86_400,
                            min_reputation_to_list: 0,
                            listings_enabled: true,
                            max_listings_per_agent: 50,
                            default_evaluation_fee_usd: 100,
                        }),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    fn make_client() -> MarketplaceClient {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        MarketplaceClient::new(config, Box::new(DummyTransport))
    }

    #[tokio::test]
    async fn query_listing_works() {
        let client = make_client();

        let listing = client.query_listing("listing-001").await.unwrap();
        let listing = listing.expect("listing should be present");
        assert_eq!(listing.listing_id, "listing-001");
        assert_eq!(listing.agent_hash, "agent-abc");
        assert_eq!(listing.listing_type, crate::types::ListingType::FullOwnership);
        assert!(listing.status.is_active());
    }

    #[tokio::test]
    async fn query_listings_works() {
        let client = make_client();

        let req = QueryListingsRequest::new(10, 0)
            .with_status(ListingStatus::Active);

        let (listings, total) = client.query_listings(req).await.unwrap();
        assert_eq!(total, 1);
        assert_eq!(listings.len(), 1);
    }

    #[tokio::test]
    async fn query_bids_by_listing_works() {
        let client = make_client();

        let (bids, total) = client.query_bids_by_listing("listing-001", 10, 0).await.unwrap();
        assert_eq!(total, 1);
        assert_eq!(bids.len(), 1);
        assert_eq!(bids[0].bid_id, "bid-001");
        assert_eq!(bids[0].amount_usd, 450_000);
    }

    #[tokio::test]
    async fn query_active_listings_works() {
        let client = make_client();

        let (listings, total) = client.query_active_listings(10, 0).await.unwrap();
        assert_eq!(total, 2);
        assert_eq!(listings.len(), 2);
    }

    #[tokio::test]
    async fn query_params_works() {
        let client = make_client();

        let params = client.query_params().await.unwrap().expect("params should be present");
        assert_eq!(params.default_platform_cut_bps, 250);
        assert_eq!(params.default_escrow_timeout_seconds, 86_400);
        assert!(params.listings_enabled);
        assert_eq!(params.max_listings_per_agent, 50);
        assert_eq!(params.default_evaluation_fee_usd, 100);
    }
}
