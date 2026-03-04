//! Fluent builders for the Marketplace module.
//!
//! This module provides ergonomic, type-safe fluent builders for all marketplace
//! transaction operations (list agent, place bid, accept bid, request evaluation,
//! parameter updates). Each builder follows the classic Builder pattern and
//! returns the corresponding request type from `requests.rs` for seamless
//! integration with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    AcceptBidRequest, ListAgentRequest, PlaceBidRequest, RequestEvaluationRequest,
    UpdateParamsRequest,
};
use crate::types::{AgentListing, ListingType, Params, RevenueShareConfig};

/// Fluent builder for listing an agent on the marketplace.
///
/// # Example
/// ```rust,ignore
/// let request = ListAgentBuilder::new()
///     .agent_hash("agent-abc")
///     .seller_agent_hash("seller-xyz")
///     .listing_type(ListingType::FullOwnership)
///     .price_usd(1_000_000)
///     .revenue_share_config(RevenueShareConfig { ... })
///     .metadata_hash("meta-hash")
///     .seller_signature(sig_bytes)
///     .build()?;
///
/// let any = request.to_any();
/// ```
#[derive(Default)]
pub struct ListAgentBuilder {
    agent_hash: Option<String>,
    seller_agent_hash: Option<String>,
    listing_type: Option<ListingType>,
    price_usd: Option<u64>,
    revenue_share_config: Option<RevenueShareConfig>,
    duration_seconds: u64,
    metadata_hash: Option<String>,
    expires_at: u64,
    seller_signature: Option<Vec<u8>>,
}

impl ListAgentBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the agent hash of the agent being listed.
    pub fn agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.agent_hash = Some(hash.into());
        self
    }

    /// Sets the seller's agent hash.
    pub fn seller_agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.seller_agent_hash = Some(hash.into());
        self
    }

    /// Sets the listing type.
    pub fn listing_type(mut self, listing_type: ListingType) -> Self {
        self.listing_type = Some(listing_type);
        self
    }

    /// Sets the price in USD (scaled).
    pub fn price_usd(mut self, price: u64) -> Self {
        self.price_usd = Some(price);
        self
    }

    /// Sets the revenue share configuration.
    pub fn revenue_share_config(mut self, config: RevenueShareConfig) -> Self {
        self.revenue_share_config = Some(config);
        self
    }

    /// Sets the rental duration in seconds (0 = permanent, for non-rental types).
    pub fn duration_seconds(mut self, duration: u64) -> Self {
        self.duration_seconds = duration;
        self
    }

    /// Sets the metadata hash (from Persistent Memory snapshot).
    pub fn metadata_hash(mut self, hash: impl Into<String>) -> Self {
        self.metadata_hash = Some(hash.into());
        self
    }

    /// Sets the expiry timestamp for the listing.
    pub fn expires_at(mut self, ts: u64) -> Self {
        self.expires_at = ts;
        self
    }

    /// Sets the seller's signature.
    pub fn seller_signature(mut self, sig: Vec<u8>) -> Self {
        self.seller_signature = Some(sig);
        self
    }

    /// Builds the list-agent request, performing validation.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError`] if any required field is missing, the price is zero,
    /// the revenue share config is invalid, or a rental listing has zero duration.
    pub fn build(self) -> Result<ListAgentRequest, SdkError> {
        let agent_hash = self.agent_hash.ok_or_else(|| {
            SdkError::invalid_input("agent_hash is required for ListAgent")
        })?;

        let seller_agent_hash = self.seller_agent_hash.ok_or_else(|| {
            SdkError::invalid_input("seller_agent_hash is required for ListAgent")
        })?;

        let listing_type = self.listing_type.ok_or_else(|| {
            SdkError::invalid_input("listing_type is required for ListAgent")
        })?;

        let price_usd = self.price_usd.ok_or_else(|| {
            SdkError::invalid_input("price_usd is required for ListAgent")
        })?;

        if price_usd == 0 {
            return Err(SdkError::invalid_input("price_usd must be greater than zero"));
        }

        // Validate revenue share config if provided
        if let Some(ref config) = self.revenue_share_config {
            if !config.is_valid() {
                return Err(SdkError::invalid_input(alloc::format!(
                    "revenue_share_config total_bps is {} but must be 10000",
                    config.total_bps(),
                )));
            }
        }

        // Rental listings must have a positive duration
        if listing_type == ListingType::Rental && self.duration_seconds == 0 {
            return Err(SdkError::invalid_input(
                "duration_seconds must be > 0 for Rental listings",
            ));
        }

        let metadata_hash = self.metadata_hash.ok_or_else(|| {
            SdkError::invalid_input("metadata_hash is required for ListAgent")
        })?;

        let seller_signature = self.seller_signature.ok_or_else(|| {
            SdkError::invalid_input("seller_signature is required for ListAgent")
        })?;

        let listing = AgentListing {
            listing_id: String::new(), // server-assigned
            agent_hash,
            seller_agent_hash,
            listing_type,
            price_usd,
            revenue_share_config: self.revenue_share_config,
            duration_seconds: self.duration_seconds,
            metadata_hash,
            status: crate::types::ListingStatus::Active,
            created_at: 0, // server-assigned
            expires_at: self.expires_at,
        };

        Ok(ListAgentRequest::new(listing, seller_signature))
    }
}

/// Fluent builder for placing a bid on a marketplace listing.
///
/// # Example
/// ```rust,ignore
/// let request = PlaceBidBuilder::new()
///     .listing_id("listing-001")
///     .amount_usd(450_000)
///     .bidder_signature(sig_bytes)
///     .build()?;
/// ```
#[derive(Default)]
pub struct PlaceBidBuilder {
    listing_id: Option<String>,
    amount_usd: Option<u64>,
    bidder_signature: Option<Vec<u8>>,
}

impl PlaceBidBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the listing ID to bid on.
    pub fn listing_id(mut self, id: impl Into<String>) -> Self {
        self.listing_id = Some(id.into());
        self
    }

    /// Sets the bid amount in USD (scaled).
    pub fn amount_usd(mut self, amount: u64) -> Self {
        self.amount_usd = Some(amount);
        self
    }

    /// Sets the bidder's signature.
    pub fn bidder_signature(mut self, sig: Vec<u8>) -> Self {
        self.bidder_signature = Some(sig);
        self
    }

    /// Builds the place-bid request, performing validation.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError`] if any required field is missing or the amount is zero.
    pub fn build(self) -> Result<PlaceBidRequest, SdkError> {
        let listing_id = self.listing_id.ok_or_else(|| {
            SdkError::invalid_input("listing_id is required for PlaceBid")
        })?;

        let amount_usd = self.amount_usd.ok_or_else(|| {
            SdkError::invalid_input("amount_usd is required for PlaceBid")
        })?;

        if amount_usd == 0 {
            return Err(SdkError::invalid_input("amount_usd must be greater than zero"));
        }

        let bidder_signature = self.bidder_signature.ok_or_else(|| {
            SdkError::invalid_input("bidder_signature is required for PlaceBid")
        })?;

        Ok(PlaceBidRequest::new(listing_id, amount_usd, bidder_signature))
    }
}

/// Fluent builder for accepting a bid (seller action).
///
/// # Example
/// ```rust,ignore
/// let request = AcceptBidBuilder::new()
///     .listing_id("listing-001")
///     .bid_id("bid-001")
///     .seller_signature(sig_bytes)
///     .build()?;
/// ```
#[derive(Default)]
pub struct AcceptBidBuilder {
    listing_id: Option<String>,
    bid_id: Option<String>,
    seller_signature: Option<Vec<u8>>,
}

impl AcceptBidBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the listing ID.
    pub fn listing_id(mut self, id: impl Into<String>) -> Self {
        self.listing_id = Some(id.into());
        self
    }

    /// Sets the bid ID to accept.
    pub fn bid_id(mut self, id: impl Into<String>) -> Self {
        self.bid_id = Some(id.into());
        self
    }

    /// Sets the seller's signature authorising acceptance.
    pub fn seller_signature(mut self, sig: Vec<u8>) -> Self {
        self.seller_signature = Some(sig);
        self
    }

    /// Builds the accept-bid request, performing validation.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError`] if any required field is missing.
    pub fn build(self) -> Result<AcceptBidRequest, SdkError> {
        let listing_id = self.listing_id.ok_or_else(|| {
            SdkError::invalid_input("listing_id is required for AcceptBid")
        })?;

        let bid_id = self.bid_id.ok_or_else(|| {
            SdkError::invalid_input("bid_id is required for AcceptBid")
        })?;

        let seller_signature = self.seller_signature.ok_or_else(|| {
            SdkError::invalid_input("seller_signature is required for AcceptBid")
        })?;

        Ok(AcceptBidRequest::new(listing_id, bid_id, seller_signature))
    }
}

/// Fluent builder for requesting an evaluation of an agent.
///
/// # Example
/// ```rust,ignore
/// let request = RequestEvaluationBuilder::new()
///     .agent_hash("agent-abc")
///     .evaluator_agent_hash("evaluator-xyz")
///     .fee_usd(200)
///     .requester_signature(sig_bytes)
///     .build()?;
/// ```
#[derive(Default)]
pub struct RequestEvaluationBuilder {
    agent_hash: Option<String>,
    evaluator_agent_hash: Option<String>,
    fee_usd: Option<u64>,
    requester_signature: Option<Vec<u8>>,
}

impl RequestEvaluationBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the agent hash of the agent to be evaluated.
    pub fn agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.agent_hash = Some(hash.into());
        self
    }

    /// Sets the evaluator agent hash.
    pub fn evaluator_agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.evaluator_agent_hash = Some(hash.into());
        self
    }

    /// Sets the evaluation fee in USD (scaled).
    pub fn fee_usd(mut self, fee: u64) -> Self {
        self.fee_usd = Some(fee);
        self
    }

    /// Sets the requester's signature.
    pub fn requester_signature(mut self, sig: Vec<u8>) -> Self {
        self.requester_signature = Some(sig);
        self
    }

    /// Builds the request-evaluation request, performing validation.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError`] if any required field is missing.
    pub fn build(self) -> Result<RequestEvaluationRequest, SdkError> {
        let agent_hash = self.agent_hash.ok_or_else(|| {
            SdkError::invalid_input("agent_hash is required for RequestEvaluation")
        })?;

        let evaluator_agent_hash = self.evaluator_agent_hash.ok_or_else(|| {
            SdkError::invalid_input("evaluator_agent_hash is required for RequestEvaluation")
        })?;

        let fee_usd = self.fee_usd.ok_or_else(|| {
            SdkError::invalid_input("fee_usd is required for RequestEvaluation")
        })?;

        let requester_signature = self.requester_signature.ok_or_else(|| {
            SdkError::invalid_input("requester_signature is required for RequestEvaluation")
        })?;

        Ok(RequestEvaluationRequest::new(
            agent_hash,
            evaluator_agent_hash,
            fee_usd,
            requester_signature,
        ))
    }
}

/// Fluent builder for updating marketplace module parameters (governance only).
///
/// # Example
/// ```rust,ignore
/// let request = UpdateParamsBuilder::new()
///     .params(Params {
///         default_platform_cut_bps: 300,
///         ..Default::default()
///     })
///     .gov_signature(sig_bytes)
///     .build()?;
/// ```
#[derive(Default)]
pub struct UpdateParamsBuilder {
    params: Option<Params>,
    gov_signature: Option<Vec<u8>>,
}

impl UpdateParamsBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the new module parameters.
    pub fn params(mut self, params: Params) -> Self {
        self.params = Some(params);
        self
    }

    /// Sets the governance signature authorising this update.
    pub fn gov_signature(mut self, sig: Vec<u8>) -> Self {
        self.gov_signature = Some(sig);
        self
    }

    /// Builds the update-params request, performing validation.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError`] if params or governance signature is missing.
    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        let params = self.params.ok_or_else(|| {
            SdkError::invalid_input("params are required for UpdateParams")
        })?;

        let gov_signature = self.gov_signature.ok_or_else(|| {
            SdkError::invalid_input("gov_signature is required for UpdateParams")
        })?;

        Ok(UpdateParamsRequest::new(params, gov_signature))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    fn sample_revenue_share() -> RevenueShareConfig {
        RevenueShareConfig {
            creator_cut_bps: 2000,
            seller_cut_bps: 6000,
            evaluator_cut_bps: 750,
            platform_cut_bps: 1250,
        }
    }

    #[test]
    fn list_agent_builder_full_flow() {
        let req = ListAgentBuilder::new()
            .agent_hash("agent-abc")
            .seller_agent_hash("seller-xyz")
            .listing_type(ListingType::FullOwnership)
            .price_usd(1_000_000)
            .revenue_share_config(sample_revenue_share())
            .metadata_hash("meta-hash")
            .expires_at(1_702_592_000)
            .seller_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(req.listing.agent_hash, "agent-abc");
        assert_eq!(req.listing.seller_agent_hash, "seller-xyz");
        assert_eq!(req.listing.listing_type, ListingType::FullOwnership);
        assert_eq!(req.listing.price_usd, 1_000_000);
    }

    #[test]
    fn list_agent_builder_missing_fields() {
        assert!(ListAgentBuilder::new().build().is_err());

        // Missing seller_agent_hash
        assert!(ListAgentBuilder::new()
            .agent_hash("agent-abc")
            .listing_type(ListingType::FullOwnership)
            .price_usd(100)
            .metadata_hash("hash")
            .seller_signature(vec![0u8; 64])
            .build()
            .is_err());
    }

    #[test]
    fn list_agent_builder_zero_price_rejected() {
        let result = ListAgentBuilder::new()
            .agent_hash("agent-abc")
            .seller_agent_hash("seller-xyz")
            .listing_type(ListingType::FullOwnership)
            .price_usd(0)
            .metadata_hash("hash")
            .seller_signature(vec![0u8; 64])
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn list_agent_builder_invalid_revenue_share_rejected() {
        let bad_config = RevenueShareConfig {
            creator_cut_bps: 5000,
            seller_cut_bps: 5000,
            evaluator_cut_bps: 1000,
            platform_cut_bps: 0,
        };
        let result = ListAgentBuilder::new()
            .agent_hash("agent-abc")
            .seller_agent_hash("seller-xyz")
            .listing_type(ListingType::FullOwnership)
            .price_usd(100)
            .revenue_share_config(bad_config)
            .metadata_hash("hash")
            .seller_signature(vec![0u8; 64])
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn list_agent_builder_rental_requires_duration() {
        let result = ListAgentBuilder::new()
            .agent_hash("agent-abc")
            .seller_agent_hash("seller-xyz")
            .listing_type(ListingType::Rental)
            .price_usd(100)
            .metadata_hash("hash")
            .seller_signature(vec![0u8; 64])
            .build();
        assert!(result.is_err());

        // With duration — valid
        let result = ListAgentBuilder::new()
            .agent_hash("agent-abc")
            .seller_agent_hash("seller-xyz")
            .listing_type(ListingType::Rental)
            .price_usd(100)
            .duration_seconds(2_592_000)
            .metadata_hash("hash")
            .seller_signature(vec![0u8; 64])
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn place_bid_builder_full_flow() {
        let req = PlaceBidBuilder::new()
            .listing_id("listing-001")
            .amount_usd(450_000)
            .bidder_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(req.listing_id, "listing-001");
        assert_eq!(req.amount_usd, 450_000);
    }

    #[test]
    fn place_bid_builder_zero_amount_rejected() {
        let result = PlaceBidBuilder::new()
            .listing_id("listing-001")
            .amount_usd(0)
            .bidder_signature(vec![0u8; 64])
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn place_bid_builder_validation() {
        assert!(PlaceBidBuilder::new().build().is_err());

        // Missing bidder_signature
        assert!(PlaceBidBuilder::new()
            .listing_id("listing-001")
            .amount_usd(100)
            .build()
            .is_err());
    }

    #[test]
    fn accept_bid_builder_full_flow() {
        let req = AcceptBidBuilder::new()
            .listing_id("listing-001")
            .bid_id("bid-001")
            .seller_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(req.listing_id, "listing-001");
        assert_eq!(req.bid_id, "bid-001");
    }

    #[test]
    fn accept_bid_builder_validation() {
        assert!(AcceptBidBuilder::new().build().is_err());

        assert!(AcceptBidBuilder::new()
            .listing_id("listing-001")
            .build()
            .is_err());

        assert!(AcceptBidBuilder::new()
            .listing_id("listing-001")
            .bid_id("bid-001")
            .build()
            .is_err()); // missing seller_signature
    }

    #[test]
    fn request_evaluation_builder_full_flow() {
        let req = RequestEvaluationBuilder::new()
            .agent_hash("agent-abc")
            .evaluator_agent_hash("evaluator-xyz")
            .fee_usd(200)
            .requester_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(req.agent_hash, "agent-abc");
        assert_eq!(req.evaluator_agent_hash, "evaluator-xyz");
        assert_eq!(req.fee_usd, 200);
    }

    #[test]
    fn request_evaluation_builder_validation() {
        assert!(RequestEvaluationBuilder::new().build().is_err());

        assert!(RequestEvaluationBuilder::new()
            .agent_hash("agent-abc")
            .evaluator_agent_hash("evaluator-xyz")
            .fee_usd(200)
            .build()
            .is_err()); // missing signature
    }

    #[test]
    fn update_params_builder_works() {
        let req = UpdateParamsBuilder::new()
            .params(Params {
                default_platform_cut_bps: 300,
                max_listings_per_agent: 100,
                ..Default::default()
            })
            .gov_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(req.params.default_platform_cut_bps, 300);
        assert_eq!(req.params.max_listings_per_agent, 100);
        assert!(req.params.listings_enabled); // default
    }

    #[test]
    fn update_params_builder_validation() {
        assert!(UpdateParamsBuilder::new().build().is_err());

        assert!(UpdateParamsBuilder::new()
            .params(Params::default())
            .build()
            .is_err()); // missing gov_signature
    }
}
