//! Fluent builders for the CLMM module.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    AddLiquidityRequest, ClaimBoostedYieldRequest, ClaimYieldRequest,
    CollectFeesRequest, ForceGlideRequest, RemoveLiquidityRequest,
};

// ====================== ADD LIQUIDITY ======================

/// Fluent builder for adding concentrated liquidity to a pool.
#[derive(Default)]
pub struct AddLiquidityBuilder {
    pool_id: Option<String>,
    owner: Option<String>,
    tick_lower: Option<i32>,
    tick_upper: Option<i32>,
    amount_desired_a: Option<String>,
    amount_desired_b: Option<String>,
    external_address: Option<String>,
}

impl AddLiquidityBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn owner(mut self, o: impl Into<String>) -> Self { self.owner = Some(o.into()); self }
    pub fn tick_lower(mut self, t: i32) -> Self { self.tick_lower = Some(t); self }
    pub fn tick_upper(mut self, t: i32) -> Self { self.tick_upper = Some(t); self }
    pub fn tick_range(mut self, lower: i32, upper: i32) -> Self {
        self.tick_lower = Some(lower); self.tick_upper = Some(upper); self
    }
    pub fn amount_desired_a(mut self, a: impl Into<String>) -> Self { self.amount_desired_a = Some(a.into()); self }
    pub fn amount_desired_b(mut self, b: impl Into<String>) -> Self { self.amount_desired_b = Some(b.into()); self }
    pub fn external_address(mut self, addr: impl Into<String>) -> Self { self.external_address = Some(addr.into()); self }

    pub fn build(self) -> Result<AddLiquidityRequest, SdkError> {
        let pool_id = self.pool_id.ok_or_else(|| SdkError::invalid_input("pool_id is required"))?;
        let owner = self.owner.ok_or_else(|| SdkError::invalid_input("owner is required"))?;
        let tick_lower = self.tick_lower.ok_or_else(|| SdkError::invalid_input("tick_lower is required"))?;
        let tick_upper = self.tick_upper.ok_or_else(|| SdkError::invalid_input("tick_upper is required"))?;
        let amount_a = self.amount_desired_a.ok_or_else(|| SdkError::invalid_input("amount_desired_a is required"))?;
        let amount_b = self.amount_desired_b.ok_or_else(|| SdkError::invalid_input("amount_desired_b is required"))?;

        if tick_lower >= tick_upper {
            return Err(SdkError::invalid_input("tick_lower must be less than tick_upper"));
        }

        let mut req = AddLiquidityRequest::new(pool_id, owner, tick_lower, tick_upper, amount_a, amount_b);
        if let Some(v) = self.external_address { req = req.external_address(v); }
        Ok(req)
    }
}

// ====================== REMOVE LIQUIDITY ======================

/// Fluent builder for removing liquidity from a position.
#[derive(Default)]
pub struct RemoveLiquidityBuilder {
    position_id: Option<String>,
    liquidity_amount: Option<String>,
    min_amount_a: Option<String>,
    min_amount_b: Option<String>,
}

impl RemoveLiquidityBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn position_id(mut self, id: impl Into<String>) -> Self { self.position_id = Some(id.into()); self }
    pub fn liquidity_amount(mut self, a: impl Into<String>) -> Self { self.liquidity_amount = Some(a.into()); self }
    pub fn min_amounts(mut self, a: impl Into<String>, b: impl Into<String>) -> Self {
        self.min_amount_a = Some(a.into()); self.min_amount_b = Some(b.into()); self
    }

    pub fn build(self) -> Result<RemoveLiquidityRequest, SdkError> {
        let position_id = self.position_id.ok_or_else(|| SdkError::invalid_input("position_id is required"))?;
        let liquidity_amount = self.liquidity_amount.ok_or_else(|| SdkError::invalid_input("liquidity_amount is required"))?;
        let mut req = RemoveLiquidityRequest::new(position_id, liquidity_amount);
        if let (Some(a), Some(b)) = (self.min_amount_a, self.min_amount_b) {
            req = req.min_amounts(a, b);
        }
        Ok(req)
    }
}

// ====================== COLLECT FEES ======================

/// Fluent builder for collecting fees from a position.
#[derive(Default)]
pub struct CollectFeesBuilder {
    position_id: Option<String>,
}

impl CollectFeesBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn position_id(mut self, id: impl Into<String>) -> Self { self.position_id = Some(id.into()); self }

    pub fn build(self) -> Result<CollectFeesRequest, SdkError> {
        let position_id = self.position_id.ok_or_else(|| SdkError::invalid_input("position_id is required"))?;
        Ok(CollectFeesRequest::new(position_id))
    }
}

// ====================== CLAIM YIELD ======================

/// Fluent builder for claiming yield from a pool.
#[derive(Default)]
pub struct ClaimYieldBuilder {
    address: Option<String>,
    pool_id: Option<String>,
    external_address: Option<String>,
}

impl ClaimYieldBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn address(mut self, a: impl Into<String>) -> Self { self.address = Some(a.into()); self }
    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn external_address(mut self, addr: impl Into<String>) -> Self { self.external_address = Some(addr.into()); self }

    pub fn build(self) -> Result<ClaimYieldRequest, SdkError> {
        let address = self.address.ok_or_else(|| SdkError::invalid_input("address is required"))?;
        let pool_id = self.pool_id.ok_or_else(|| SdkError::invalid_input("pool_id is required"))?;
        let mut req = ClaimYieldRequest::new(address, pool_id);
        if let Some(v) = self.external_address { req = req.external_address(v); }
        Ok(req)
    }
}

// ====================== CLAIM BOOSTED YIELD ======================

/// Fluent builder for claiming boosted yield.
#[derive(Default)]
pub struct ClaimBoostedYieldBuilder {
    address: Option<String>,
    pool_id: Option<String>,
    external_address: Option<String>,
}

impl ClaimBoostedYieldBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn address(mut self, a: impl Into<String>) -> Self { self.address = Some(a.into()); self }
    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn external_address(mut self, addr: impl Into<String>) -> Self { self.external_address = Some(addr.into()); self }

    pub fn build(self) -> Result<ClaimBoostedYieldRequest, SdkError> {
        let address = self.address.ok_or_else(|| SdkError::invalid_input("address is required"))?;
        let pool_id = self.pool_id.ok_or_else(|| SdkError::invalid_input("pool_id is required"))?;
        let mut req = ClaimBoostedYieldRequest::new(address, pool_id);
        if let Some(v) = self.external_address { req = req.external_address(v); }
        Ok(req)
    }
}

// ====================== FORCE GLIDE ======================

/// Fluent builder for a governance-only ReClmm force-glide.
#[derive(Default)]
pub struct ForceGlideBuilder {
    pool_id: Option<String>,
    target_price: Option<String>,
    authority: Option<String>,
}

impl ForceGlideBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn target_price(mut self, p: impl Into<String>) -> Self { self.target_price = Some(p.into()); self }
    pub fn authority(mut self, a: impl Into<String>) -> Self { self.authority = Some(a.into()); self }

    pub fn build(self) -> Result<ForceGlideRequest, SdkError> {
        let pool_id = self.pool_id.ok_or_else(|| SdkError::invalid_input("pool_id is required"))?;
        let target_price = self.target_price.ok_or_else(|| SdkError::invalid_input("target_price is required"))?;
        let mut req = ForceGlideRequest::new(pool_id, target_price);
        if let Some(v) = self.authority { req = req.authority(v); }
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_liquidity_builder_full() {
        let req = AddLiquidityBuilder::new()
            .pool_id("0x1234").owner("morpheum1abc")
            .tick_range(-100, 100)
            .amount_desired_a("500").amount_desired_b("500")
            .build().unwrap();
        assert_eq!(req.pool_id, "0x1234");
        assert_eq!(req.tick_lower, -100);
    }

    #[test]
    fn add_liquidity_builder_tick_validation() {
        let err = AddLiquidityBuilder::new()
            .pool_id("0x1234").owner("morpheum1abc")
            .tick_range(100, -100)
            .amount_desired_a("500").amount_desired_b("500")
            .build();
        assert!(err.is_err());
    }

    #[test]
    fn add_liquidity_builder_missing_fields() {
        assert!(AddLiquidityBuilder::new().build().is_err());
    }

    #[test]
    fn remove_liquidity_builder_works() {
        let req = RemoveLiquidityBuilder::new()
            .position_id("pos-1").liquidity_amount("250")
            .min_amounts("100", "100")
            .build().unwrap();
        assert_eq!(req.position_id, "pos-1");
    }

    #[test]
    fn collect_fees_builder_works() {
        let req = CollectFeesBuilder::new().position_id("pos-1").build().unwrap();
        assert_eq!(req.position_id, "pos-1");
    }

    #[test]
    fn claim_yield_builder_works() {
        let req = ClaimYieldBuilder::new().address("morpheum1abc").pool_id("0x1234").build().unwrap();
        assert_eq!(req.address, "morpheum1abc");
    }

    #[test]
    fn claim_boosted_yield_builder_works() {
        let req = ClaimBoostedYieldBuilder::new().address("morpheum1abc").pool_id("0x1234").build().unwrap();
        assert_eq!(req.pool_id, "0x1234");
    }

    #[test]
    fn force_glide_builder_works() {
        let req = ForceGlideBuilder::new().pool_id("0x1234").target_price("50000").build().unwrap();
        assert_eq!(req.target_price, "50000");
    }

    #[test]
    fn force_glide_builder_validation() {
        assert!(ForceGlideBuilder::new().build().is_err());
    }
}
