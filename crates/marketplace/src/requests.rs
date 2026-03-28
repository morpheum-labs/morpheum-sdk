//! Request and response wrappers for the Marketplace module.
//!
//! These are clean, ergonomic Rust types that wrap the raw protobuf messages.
//! They provide type safety, validation, helper methods, and seamless conversion
//! to/from protobuf for use with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::types::Params;
use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::marketplace::v1 as proto;

use crate::types::{
    AgentListing, Bid, ListingStatus,
};

// ====================== TRANSACTION REQUESTS ======================

/// Request to list an agent on the marketplace.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListAgentRequest {
    /// The listing details.
    pub listing: AgentListing,
    /// Seller's signature authorising this listing.
    pub seller_signature: Vec<u8>,
}

impl ListAgentRequest {
    /// Creates a new list-agent request.
    pub fn new(listing: AgentListing, seller_signature: Vec<u8>) -> Self {
        Self { listing, seller_signature }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgListAgent = self.clone().into();
        ProtoAny {
            type_url: "/marketplace.v1.MsgListAgent".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ListAgentRequest> for proto::MsgListAgent {
    fn from(req: ListAgentRequest) -> Self {
        Self {
            listing: Some(req.listing.into()),
            seller_signature: req.seller_signature,
        }
    }
}

/// Response from listing an agent.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListAgentResponse {
    /// Server-assigned listing identifier.
    pub listing_id: String,
    /// Timestamp when the listing was created.
    pub listed_at: u64,
    /// Whether the operation succeeded.
    pub success: bool,
}

impl From<proto::ListAgentResponse> for ListAgentResponse {
    fn from(p: proto::ListAgentResponse) -> Self {
        Self {
            listing_id: p.listing_id,
            listed_at: p.listed_at,
            success: p.success,
        }
    }
}

/// Request to place a bid on a listing.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlaceBidRequest {
    /// Listing identifier.
    pub listing_id: String,
    /// Bid amount in USD (scaled).
    pub amount_usd: u64,
    /// Bidder's signature.
    pub bidder_signature: Vec<u8>,
}

impl PlaceBidRequest {
    /// Creates a new place-bid request.
    pub fn new(
        listing_id: impl Into<String>,
        amount_usd: u64,
        bidder_signature: Vec<u8>,
    ) -> Self {
        Self {
            listing_id: listing_id.into(),
            amount_usd,
            bidder_signature,
        }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgPlaceBid = self.clone().into();
        ProtoAny {
            type_url: "/marketplace.v1.MsgPlaceBid".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<PlaceBidRequest> for proto::MsgPlaceBid {
    fn from(req: PlaceBidRequest) -> Self {
        Self {
            listing_id: req.listing_id,
            amount_usd: req.amount_usd,
            bidder_signature: req.bidder_signature,
        }
    }
}

/// Response from placing a bid.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlaceBidResponse {
    /// Server-assigned bid identifier.
    pub bid_id: String,
    /// Timestamp when the bid was placed.
    pub timestamp: u64,
    /// Whether the operation succeeded.
    pub success: bool,
}

impl From<proto::PlaceBidResponse> for PlaceBidResponse {
    fn from(p: proto::PlaceBidResponse) -> Self {
        Self {
            bid_id: p.bid_id,
            timestamp: p.timestamp,
            success: p.success,
        }
    }
}

/// Request to accept a bid (seller action).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AcceptBidRequest {
    /// Listing identifier.
    pub listing_id: String,
    /// Bid identifier to accept.
    pub bid_id: String,
    /// Seller's signature authorising acceptance.
    pub seller_signature: Vec<u8>,
}

impl AcceptBidRequest {
    /// Creates a new accept-bid request.
    pub fn new(
        listing_id: impl Into<String>,
        bid_id: impl Into<String>,
        seller_signature: Vec<u8>,
    ) -> Self {
        Self {
            listing_id: listing_id.into(),
            bid_id: bid_id.into(),
            seller_signature,
        }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgAcceptBid = self.clone().into();
        ProtoAny {
            type_url: "/marketplace.v1.MsgAcceptBid".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<AcceptBidRequest> for proto::MsgAcceptBid {
    fn from(req: AcceptBidRequest) -> Self {
        Self {
            listing_id: req.listing_id,
            bid_id: req.bid_id,
            seller_signature: req.seller_signature,
        }
    }
}

/// Response from accepting a bid.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AcceptBidResponse {
    /// Whether the operation succeeded.
    pub success: bool,
    /// Server-assigned escrow identifier.
    pub escrow_id: String,
    /// Timestamp when the bid was accepted.
    pub accepted_at: u64,
}

impl From<proto::AcceptBidResponse> for AcceptBidResponse {
    fn from(p: proto::AcceptBidResponse) -> Self {
        Self {
            success: p.success,
            escrow_id: p.escrow_id,
            accepted_at: p.accepted_at,
        }
    }
}

/// Request to request an evaluation of an agent.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RequestEvaluationRequest {
    /// Agent hash of the agent to be evaluated.
    pub agent_hash: String,
    /// Evaluator agent hash.
    pub evaluator_agent_hash: String,
    /// Evaluation fee in USD (scaled).
    pub fee_usd: u64,
    /// Requester's signature.
    pub requester_signature: Vec<u8>,
}

impl RequestEvaluationRequest {
    /// Creates a new request-evaluation request.
    pub fn new(
        agent_hash: impl Into<String>,
        evaluator_agent_hash: impl Into<String>,
        fee_usd: u64,
        requester_signature: Vec<u8>,
    ) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            evaluator_agent_hash: evaluator_agent_hash.into(),
            fee_usd,
            requester_signature,
        }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgRequestEvaluation = self.clone().into();
        ProtoAny {
            type_url: "/marketplace.v1.MsgRequestEvaluation".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<RequestEvaluationRequest> for proto::MsgRequestEvaluation {
    fn from(req: RequestEvaluationRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            evaluator_agent_hash: req.evaluator_agent_hash,
            fee_usd: req.fee_usd,
            requester_signature: req.requester_signature,
        }
    }
}

/// Response from requesting an evaluation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RequestEvaluationResponse {
    /// Server-assigned evaluation identifier.
    pub evaluation_id: String,
    /// Timestamp when the evaluation was requested.
    pub requested_at: u64,
    /// Whether the operation succeeded.
    pub success: bool,
}

impl From<proto::RequestEvaluationResponse> for RequestEvaluationResponse {
    fn from(p: proto::RequestEvaluationResponse) -> Self {
        Self {
            evaluation_id: p.evaluation_id,
            requested_at: p.requested_at,
            success: p.success,
        }
    }
}

// ====================== QUERY REQUESTS & RESPONSES ======================

/// Query a specific listing by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryListingRequest {
    /// Listing identifier to query.
    pub listing_id: String,
}

impl QueryListingRequest {
    /// Creates a new query for a specific listing.
    pub fn new(listing_id: impl Into<String>) -> Self {
        Self { listing_id: listing_id.into() }
    }
}

impl From<QueryListingRequest> for proto::QueryListingRequest {
    fn from(req: QueryListingRequest) -> Self {
        Self { listing_id: req.listing_id }
    }
}

/// Response containing a listing (or indicating not found).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryListingResponse {
    /// The listing, if found.
    pub listing: Option<AgentListing>,
    /// Whether the listing was found.
    pub found: bool,
}

impl From<proto::QueryListingResponse> for QueryListingResponse {
    fn from(p: proto::QueryListingResponse) -> Self {
        Self {
            listing: p.listing.map(Into::into),
            found: p.found,
        }
    }
}

/// Query multiple listings with optional seller and status filters (paginated).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryListingsRequest {
    /// Optional filter by seller agent hash (empty = no filter).
    pub seller_agent_hash: String,
    /// Optional status filter (default Active).
    pub status: ListingStatus,
    /// Maximum number of results.
    pub limit: u32,
    /// Pagination offset.
    pub offset: u32,
}

impl QueryListingsRequest {
    /// Creates a new query with pagination.
    pub fn new(limit: u32, offset: u32) -> Self {
        Self { limit, offset, ..Default::default() }
    }

    /// Filters by seller agent hash.
    pub fn with_seller(mut self, seller: impl Into<String>) -> Self {
        self.seller_agent_hash = seller.into();
        self
    }

    /// Filters by listing status.
    pub fn with_status(mut self, status: ListingStatus) -> Self {
        self.status = status;
        self
    }
}

impl From<QueryListingsRequest> for proto::QueryListingsRequest {
    fn from(req: QueryListingsRequest) -> Self {
        Self {
            seller_agent_hash: req.seller_agent_hash,
            status: req.status.to_proto(),
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Response containing paginated listings.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryListingsResponse {
    /// The matching listings.
    pub listings: Vec<AgentListing>,
    /// Total number of matches (may exceed page size).
    pub total_count: u32,
}

impl From<proto::QueryListingsResponse> for QueryListingsResponse {
    fn from(p: proto::QueryListingsResponse) -> Self {
        Self {
            listings: p.listings.into_iter().map(Into::into).collect(),
            total_count: p.total_count,
        }
    }
}

/// Query all bids for a specific listing (paginated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryBidsByListingRequest {
    /// Listing identifier to query bids for.
    pub listing_id: String,
    /// Maximum number of results.
    pub limit: u32,
    /// Pagination offset.
    pub offset: u32,
}

impl QueryBidsByListingRequest {
    /// Creates a new paginated bids query for a listing.
    pub fn new(listing_id: impl Into<String>, limit: u32, offset: u32) -> Self {
        Self {
            listing_id: listing_id.into(),
            limit,
            offset,
        }
    }
}

impl From<QueryBidsByListingRequest> for proto::QueryBidsByListingRequest {
    fn from(req: QueryBidsByListingRequest) -> Self {
        Self {
            listing_id: req.listing_id,
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Response containing paginated bids for a listing.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryBidsByListingResponse {
    /// The matching bids.
    pub bids: Vec<Bid>,
    /// Total number of bids for the listing.
    pub total_count: u32,
}

impl From<proto::QueryBidsByListingResponse> for QueryBidsByListingResponse {
    fn from(p: proto::QueryBidsByListingResponse) -> Self {
        Self {
            bids: p.bids.into_iter().map(Into::into).collect(),
            total_count: p.total_count,
        }
    }
}

/// Query all currently active listings (paginated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryActiveListingsRequest {
    /// Maximum number of results.
    pub limit: u32,
    /// Pagination offset.
    pub offset: u32,
}

impl QueryActiveListingsRequest {
    /// Creates a new paginated active-listings query.
    pub fn new(limit: u32, offset: u32) -> Self {
        Self { limit, offset }
    }
}

impl From<QueryActiveListingsRequest> for proto::QueryActiveListingsRequest {
    fn from(req: QueryActiveListingsRequest) -> Self {
        Self {
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Response containing active listings.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryActiveListingsResponse {
    /// The active listings.
    pub listings: Vec<AgentListing>,
    /// Total number of active listings.
    pub total_count: u32,
}

impl From<proto::QueryActiveListingsResponse> for QueryActiveListingsResponse {
    fn from(p: proto::QueryActiveListingsResponse) -> Self {
        Self {
            listings: p.listings.into_iter().map(Into::into).collect(),
            total_count: p.total_count,
        }
    }
}

/// Query the current module parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsRequest;

impl From<QueryParamsRequest> for proto::QueryParamsRequest {
    fn from(_: QueryParamsRequest) -> Self {
        Self {}
    }
}

/// Response containing the current module parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsResponse {
    /// The current module parameters, if set.
    pub params: Option<Params>,
}

impl From<proto::QueryParamsResponse> for QueryParamsResponse {
    fn from(p: proto::QueryParamsResponse) -> Self {
        Self {
            params: p.params.map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RevenueShareConfig;
    use alloc::vec;

    fn sample_listing() -> AgentListing {
        AgentListing {
            listing_id: "listing-001".into(),
            agent_hash: "agent-abc".into(),
            seller_agent_hash: "seller-xyz".into(),
            listing_type: crate::types::ListingType::FullOwnership,
            price_usd: 1_000_000,
            revenue_share_config: Some(RevenueShareConfig {
                creator_cut_bps: 2000,
                seller_cut_bps: 6000,
                evaluator_cut_bps: 750,
                platform_cut_bps: 1250,
            }),
            duration_seconds: 0,
            metadata_hash: "meta-hash".into(),
            status: ListingStatus::Active,
            created_at: 1_700_000_000,
            expires_at: 1_702_592_000,
        }
    }

    #[test]
    fn list_agent_request_to_any() {
        let req = ListAgentRequest::new(sample_listing(), vec![0u8; 64]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/marketplace.v1.MsgListAgent");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn place_bid_request_to_any() {
        let req = PlaceBidRequest::new("listing-001", 450_000, vec![0u8; 64]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/marketplace.v1.MsgPlaceBid");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn accept_bid_request_to_any() {
        let req = AcceptBidRequest::new("listing-001", "bid-001", vec![0u8; 64]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/marketplace.v1.MsgAcceptBid");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn request_evaluation_to_any() {
        let req = RequestEvaluationRequest::new(
            "agent-abc", "evaluator-xyz", 200, vec![0u8; 64],
        );
        let any = req.to_any();
        assert_eq!(any.type_url, "/marketplace.v1.MsgRequestEvaluation");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn list_agent_response_conversion() {
        let proto_res = proto::ListAgentResponse {
            listing_id: "listing-001".into(),
            listed_at: 1_700_000_000,
            success: true,
        };
        let res: ListAgentResponse = proto_res.into();
        assert!(res.success);
        assert_eq!(res.listing_id, "listing-001");
    }

    #[test]
    fn place_bid_response_conversion() {
        let proto_res = proto::PlaceBidResponse {
            bid_id: "bid-001".into(),
            timestamp: 1_700_001_000,
            success: true,
        };
        let res: PlaceBidResponse = proto_res.into();
        assert!(res.success);
        assert_eq!(res.bid_id, "bid-001");
    }

    #[test]
    fn accept_bid_response_conversion() {
        let proto_res = proto::AcceptBidResponse {
            success: true,
            escrow_id: "escrow-001".into(),
            accepted_at: 1_700_002_000,
        };
        let res: AcceptBidResponse = proto_res.into();
        assert!(res.success);
        assert_eq!(res.escrow_id, "escrow-001");
    }

    #[test]
    fn request_evaluation_response_conversion() {
        let proto_res = proto::RequestEvaluationResponse {
            evaluation_id: "eval-001".into(),
            requested_at: 1_700_003_000,
            success: true,
        };
        let res: RequestEvaluationResponse = proto_res.into();
        assert!(res.success);
        assert_eq!(res.evaluation_id, "eval-001");
    }

    #[test]
    fn query_listing_response_conversion() {
        let proto_res = proto::QueryListingResponse {
            listing: Some(proto::AgentListing {
                listing_id: "listing-001".into(),
                agent_hash: "agent-abc".into(),
                status: 0, // Active
                ..Default::default()
            }),
            found: true,
        };
        let res: QueryListingResponse = proto_res.into();
        assert!(res.found);
        let listing = res.listing.unwrap();
        assert_eq!(listing.listing_id, "listing-001");
        assert_eq!(listing.status, ListingStatus::Active);
    }

    #[test]
    fn query_listings_request_with_filters() {
        let req = QueryListingsRequest::new(20, 0)
            .with_seller("seller-xyz")
            .with_status(ListingStatus::Sold);

        let proto_req: proto::QueryListingsRequest = req.into();
        assert_eq!(proto_req.seller_agent_hash, "seller-xyz");
        assert_eq!(proto_req.status, 1); // Sold
        assert_eq!(proto_req.limit, 20);
    }

    #[test]
    fn query_bids_by_listing_response_conversion() {
        let proto_res = proto::QueryBidsByListingResponse {
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
        let res: QueryBidsByListingResponse = proto_res.into();
        assert_eq!(res.total_count, 1);
        assert_eq!(res.bids[0].bid_id, "bid-001");
    }

    #[test]
    fn query_active_listings_response_conversion() {
        let proto_res = proto::QueryActiveListingsResponse {
            listings: vec![proto::AgentListing::default(), proto::AgentListing::default()],
            total_count: 2,
        };
        let res: QueryActiveListingsResponse = proto_res.into();
        assert_eq!(res.total_count, 2);
        assert_eq!(res.listings.len(), 2);
    }

    #[test]
    fn query_params_response_conversion() {
        let proto_res = proto::QueryParamsResponse {
            params: Some(proto::Params {
                default_platform_cut_bps: 250,
                default_escrow_timeout_seconds: 86_400,
                min_reputation_to_list: 0,
                listings_enabled: true,
                max_listings_per_agent: 50,
                default_evaluation_fee_usd: 100,
            }),
        };
        let res: QueryParamsResponse = proto_res.into();
        let p = res.params.unwrap();
        assert_eq!(p.default_platform_cut_bps, 250);
        assert!(p.listings_enabled);
    }
}
