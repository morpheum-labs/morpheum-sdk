//! Fluent builders for the Market module.
//!
//! This module provides ergonomic, type-safe fluent builders for all major
//! market operations (creation, activation, suspension, updates, and margin changes).
//! Each builder follows the classic Builder pattern and returns the corresponding
//! request type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::string::String;

use morpheum_sdk_core::{AccountId, SdkError};

use crate::requests::{
    ActivateMarketRequest,
    ChangeMarketMarginRatioRequest,
    CreateMarketRequest,
    SuspendMarketRequest,
    UpdateMarketRequest,
};
use crate::types::{MarketParams, MarketType};

/// Fluent builder for creating a new market.
///
/// This is the most feature-rich builder in the market module, supporting
/// full market parameters, market type, and governance references.
#[derive(Default)]
pub struct MarketCreateBuilder {
    from_address: Option<AccountId>,
    base_asset_index: Option<u64>,
    quote_asset_index: Option<u64>,
    market_type: Option<MarketType>,
    orderbook_type: Option<String>,
    params: Option<MarketParams>,
    governance_proposal_id: Option<String>,
}

impl MarketCreateBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the creator address (signer).
    ///
    /// Accepts any type that converts into `AccountId`, including
    /// `morpheum_signing_core::types::AccountId` from a `Signer`.
    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    /// Sets the base asset index.
    pub fn base_asset_index(mut self, index: u64) -> Self {
        self.base_asset_index = Some(index);
        self
    }

    /// Sets the quote asset index.
    pub fn quote_asset_index(mut self, index: u64) -> Self {
        self.quote_asset_index = Some(index);
        self
    }

    /// Sets the market type (Spot, Perp, Future, etc.).
    pub fn market_type(mut self, market_type: MarketType) -> Self {
        self.market_type = Some(market_type);
        self
    }

    /// Sets the orderbook type (e.g., "clob", "amm").
    pub fn orderbook_type(mut self, orderbook_type: impl Into<String>) -> Self {
        self.orderbook_type = Some(orderbook_type.into());
        self
    }

    /// Sets the full market parameters.
    pub fn params(mut self, params: MarketParams) -> Self {
        self.params = Some(params);
        self
    }

    /// Sets an optional governance proposal ID that authorized this market creation.
    pub fn governance_proposal_id(mut self, id: impl Into<String>) -> Self {
        self.governance_proposal_id = Some(id.into());
        self
    }

    /// Builds and validates the create request.
    pub fn build(self) -> Result<CreateMarketRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for market creation")
        })?;

        let base_asset_index = self.base_asset_index.ok_or_else(|| {
            SdkError::invalid_input("base_asset_index is required")
        })?;

        let quote_asset_index = self.quote_asset_index.ok_or_else(|| {
            SdkError::invalid_input("quote_asset_index is required")
        })?;

        let market_type = self.market_type.ok_or_else(|| {
            SdkError::invalid_input("market_type is required")
        })?;

        let orderbook_type = self.orderbook_type.ok_or_else(|| {
            SdkError::invalid_input("orderbook_type is required")
        })?;

        let params = self.params.ok_or_else(|| {
            SdkError::invalid_input("params are required for market creation")
        })?;

        Ok(CreateMarketRequest {
            from_address,
            base_asset_index,
            quote_asset_index,
            market_type,
            orderbook_type,
            params,
            governance_proposal_id: self.governance_proposal_id,
        })
    }
}

/// Fluent builder for activating a pending market.
#[derive(Default)]
pub struct ActivateMarketBuilder {
    market_index: Option<u64>,
    activator: Option<AccountId>,
}

impl ActivateMarketBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn market_index(mut self, index: u64) -> Self {
        self.market_index = Some(index);
        self
    }

    /// Sets the activator address.
    ///
    /// Accepts any type that converts into `AccountId`.
    pub fn activator(mut self, activator: impl Into<AccountId>) -> Self {
        self.activator = Some(activator.into());
        self
    }

    pub fn build(self) -> Result<ActivateMarketRequest, SdkError> {
        let market_index = self.market_index.ok_or_else(|| {
            SdkError::invalid_input("market_index is required for activation")
        })?;

        let activator = self.activator.ok_or_else(|| {
            SdkError::invalid_input("activator is required")
        })?;

        Ok(ActivateMarketRequest::new(market_index, activator))
    }
}

/// Fluent builder for suspending a market.
#[derive(Default)]
pub struct SuspendMarketBuilder {
    market_index: Option<u64>,
    reason: Option<String>,
    suspender: Option<AccountId>,
}

impl SuspendMarketBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn market_index(mut self, index: u64) -> Self {
        self.market_index = Some(index);
        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Sets the suspender address.
    ///
    /// Accepts any type that converts into `AccountId`.
    pub fn suspender(mut self, suspender: impl Into<AccountId>) -> Self {
        self.suspender = Some(suspender.into());
        self
    }

    pub fn build(self) -> Result<SuspendMarketRequest, SdkError> {
        let market_index = self.market_index.ok_or_else(|| {
            SdkError::invalid_input("market_index is required for suspension")
        })?;

        let suspender = self.suspender.ok_or_else(|| {
            SdkError::invalid_input("suspender is required")
        })?;

        Ok(SuspendMarketRequest::new(
            market_index,
            self.reason.unwrap_or_else(|| "No reason provided".into()),
            suspender,
        ))
    }
}

/// Fluent builder for updating market parameters.
#[derive(Default)]
pub struct UpdateMarketBuilder {
    market_index: Option<u64>,
    params: Option<MarketParams>,
    from_address: Option<AccountId>,
    governance_proposal_id: Option<String>,
}

impl UpdateMarketBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn market_index(mut self, index: u64) -> Self {
        self.market_index = Some(index);
        self
    }

    pub fn params(mut self, params: MarketParams) -> Self {
        self.params = Some(params);
        self
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn governance_proposal_id(mut self, id: impl Into<String>) -> Self {
        self.governance_proposal_id = Some(id.into());
        self
    }

    pub fn build(self) -> Result<UpdateMarketRequest, SdkError> {
        let market_index = self.market_index.ok_or_else(|| {
            SdkError::invalid_input("market_index is required for update")
        })?;

        let params = self.params.ok_or_else(|| {
            SdkError::invalid_input("params are required for market update")
        })?;

        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required")
        })?;

        Ok(UpdateMarketRequest::new(market_index, params, from_address)
            .with_governance_proposal_id_opt(self.governance_proposal_id))
    }
}

impl UpdateMarketRequest {
    // Helper to support optional governance ID in builder flow
    fn with_governance_proposal_id_opt(mut self, id: Option<String>) -> Self {
        self.governance_proposal_id = id;
        self
    }
}

/// Fluent builder for changing market margin ratios.
#[derive(Default)]
pub struct ChangeMarketMarginRatioBuilder {
    from_address: Option<AccountId>,
    market_index: Option<u64>,
    new_initial_margin_ratio: Option<String>,
    new_maintenance_margin_ratio: Option<String>,
    reason: Option<String>,
}

impl ChangeMarketMarginRatioBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn market_index(mut self, index: u64) -> Self {
        self.market_index = Some(index);
        self
    }

    pub fn new_initial_margin_ratio(mut self, ratio: impl Into<String>) -> Self {
        self.new_initial_margin_ratio = Some(ratio.into());
        self
    }

    pub fn new_maintenance_margin_ratio(mut self, ratio: impl Into<String>) -> Self {
        self.new_maintenance_margin_ratio = Some(ratio.into());
        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn build(self) -> Result<ChangeMarketMarginRatioRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required")
        })?;

        let market_index = self.market_index.ok_or_else(|| {
            SdkError::invalid_input("market_index is required")
        })?;

        Ok(ChangeMarketMarginRatioRequest::new(from_address, market_index)
            .new_initial_margin_ratio_opt(self.new_initial_margin_ratio)
            .new_maintenance_margin_ratio_opt(self.new_maintenance_margin_ratio)
            .reason_opt(self.reason))
    }
}

impl ChangeMarketMarginRatioRequest {
    fn new_initial_margin_ratio_opt(mut self, ratio: Option<String>) -> Self {
        self.new_initial_margin_ratio = ratio;
        self
    }

    fn new_maintenance_margin_ratio_opt(mut self, ratio: Option<String>) -> Self {
        self.new_maintenance_margin_ratio = ratio;
        self
    }

    fn reason_opt(mut self, reason: Option<String>) -> Self {
        self.reason = reason;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use morpheum_sdk_core::AccountId;

    #[test]
    fn market_create_builder_full_flow() {
        let from = AccountId::new([1u8; 32]);
        let params = crate::types::MarketParams {
            min_order_size: "0.001".into(),
            tick_size: "0.01".into(),
            lot_size: "1".into(),
            max_leverage: "100".into(),
            initial_margin_ratio: "0.1".into(),
            maintenance_margin_ratio: "0.05".into(),
            taker_fee_rate: "0.0005".into(),
            maker_fee_rate: "0.0002".into(),
            allow_market_orders: true,
            allow_stop_orders: true,
            perp_config: None,
            additional_params: alloc::collections::BTreeMap::new(),
        };

        let request = MarketCreateBuilder::new()
            .from_address(from.clone())
            .base_asset_index(1)
            .quote_asset_index(2)
            .market_type(MarketType::Perp)
            .orderbook_type("clob")
            .params(params)
            .governance_proposal_id("gov-456")
            .build()
            .unwrap();

        assert_eq!(request.from_address, from);
        assert_eq!(request.market_type, MarketType::Perp);
        assert_eq!(request.governance_proposal_id, Some("gov-456".into()));
    }

    #[test]
    fn market_create_builder_validation() {
        let result = MarketCreateBuilder::new().build();
        assert!(result.is_err());
    }

    #[test]
    fn activate_builder_works() {
        let request = ActivateMarketBuilder::new()
            .market_index(42)
            .activator(AccountId::new([5u8; 32]))
            .build()
            .unwrap();

        assert_eq!(request.market_index, 42);
    }
}