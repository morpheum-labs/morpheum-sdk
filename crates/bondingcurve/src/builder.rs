//! Fluent builders for the bonding-curve module.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{BuyRequest, CreateAgentTokenRequest, ExecuteGraduationRequest, SellRequest};

// ====================== CREATE AGENT TOKEN ======================

/// Fluent builder for creating a new agent token with a bonding curve.
#[derive(Default)]
pub struct CreateAgentTokenBuilder {
    sender: Option<String>,
    agent_creator_did: Option<String>,
    name: Option<String>,
    symbol: Option<String>,
    decimals: Option<u32>,
    icon_url: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    mwvm_hooks: Vec<String>,
    initial_k: Option<String>,
    initial_graduation_mcap: Option<String>,
}

impl CreateAgentTokenBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn sender(mut self, s: impl Into<String>) -> Self { self.sender = Some(s.into()); self }
    pub fn agent_creator_did(mut self, d: impl Into<String>) -> Self { self.agent_creator_did = Some(d.into()); self }
    pub fn name(mut self, n: impl Into<String>) -> Self { self.name = Some(n.into()); self }
    pub fn symbol(mut self, s: impl Into<String>) -> Self { self.symbol = Some(s.into()); self }
    pub fn decimals(mut self, d: u32) -> Self { self.decimals = Some(d); self }
    pub fn icon_url(mut self, u: impl Into<String>) -> Self { self.icon_url = Some(u.into()); self }
    pub fn description(mut self, d: impl Into<String>) -> Self { self.description = Some(d.into()); self }
    pub fn tags(mut self, t: Vec<String>) -> Self { self.tags = t; self }
    pub fn mwvm_hooks(mut self, h: Vec<String>) -> Self { self.mwvm_hooks = h; self }
    pub fn initial_k(mut self, k: impl Into<String>) -> Self { self.initial_k = Some(k.into()); self }
    pub fn initial_graduation_mcap(mut self, m: impl Into<String>) -> Self {
        self.initial_graduation_mcap = Some(m.into()); self
    }

    pub fn build(self) -> Result<CreateAgentTokenRequest, SdkError> {
        let sender = self.sender.ok_or_else(|| SdkError::invalid_input("sender is required"))?;
        let agent_creator_did = self.agent_creator_did.ok_or_else(|| SdkError::invalid_input("agent_creator_did is required"))?;
        let name = self.name.ok_or_else(|| SdkError::invalid_input("name is required"))?;
        let symbol = self.symbol.ok_or_else(|| SdkError::invalid_input("symbol is required"))?;
        let decimals = self.decimals.ok_or_else(|| SdkError::invalid_input("decimals is required"))?;

        let mut req = CreateAgentTokenRequest::new(sender, agent_creator_did, name, symbol, decimals);
        if let Some(v) = self.icon_url { req = req.icon_url(v); }
        if let Some(v) = self.description { req = req.description(v); }
        if !self.tags.is_empty() { req = req.tags(self.tags); }
        if !self.mwvm_hooks.is_empty() { req = req.mwvm_hooks(self.mwvm_hooks); }
        if let Some(v) = self.initial_k { req = req.initial_k(v); }
        if let Some(v) = self.initial_graduation_mcap { req = req.initial_graduation_mcap(v); }
        Ok(req)
    }
}

// ====================== BUY ======================

/// Fluent builder for buying tokens on the bonding curve.
#[derive(Default)]
pub struct BuyBuilder {
    token_index: Option<u64>,
    morm_amount: Option<String>,
}

impl BuyBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn token_index(mut self, idx: u64) -> Self { self.token_index = Some(idx); self }
    pub fn morm_amount(mut self, a: impl Into<String>) -> Self { self.morm_amount = Some(a.into()); self }

    pub fn build(self) -> Result<BuyRequest, SdkError> {
        let token_index = self.token_index.ok_or_else(|| SdkError::invalid_input("token_index is required"))?;
        let morm_amount = self.morm_amount.ok_or_else(|| SdkError::invalid_input("morm_amount is required"))?;
        Ok(BuyRequest::new(token_index, morm_amount))
    }
}

// ====================== SELL ======================

/// Fluent builder for selling tokens on the bonding curve.
#[derive(Default)]
pub struct SellBuilder {
    token_index: Option<u64>,
    token_amount: Option<String>,
}

impl SellBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn token_index(mut self, idx: u64) -> Self { self.token_index = Some(idx); self }
    pub fn token_amount(mut self, a: impl Into<String>) -> Self { self.token_amount = Some(a.into()); self }

    pub fn build(self) -> Result<SellRequest, SdkError> {
        let token_index = self.token_index.ok_or_else(|| SdkError::invalid_input("token_index is required"))?;
        let token_amount = self.token_amount.ok_or_else(|| SdkError::invalid_input("token_amount is required"))?;
        Ok(SellRequest::new(token_index, token_amount))
    }
}

// ====================== EXECUTE GRADUATION ======================

/// Fluent builder for executing graduation.
#[derive(Default)]
pub struct ExecuteGraduationBuilder {
    token_index: Option<u64>,
}

impl ExecuteGraduationBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn token_index(mut self, idx: u64) -> Self { self.token_index = Some(idx); self }

    pub fn build(self) -> Result<ExecuteGraduationRequest, SdkError> {
        let token_index = self.token_index.ok_or_else(|| SdkError::invalid_input("token_index is required"))?;
        Ok(ExecuteGraduationRequest::new(token_index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_agent_token_builder_full() {
        let req = CreateAgentTokenBuilder::new()
            .sender("morpheum1abc").agent_creator_did("did:morpheum:agent1")
            .name("TestCoin").symbol("TST").decimals(8)
            .initial_k("1000000")
            .build().unwrap();
        assert_eq!(req.sender, "morpheum1abc");
        assert_eq!(req.symbol, "TST");
    }

    #[test]
    fn create_agent_token_builder_validation() {
        assert!(CreateAgentTokenBuilder::new().build().is_err());
        assert!(CreateAgentTokenBuilder::new().sender("x").build().is_err());
    }

    #[test]
    fn buy_builder_works() {
        let req = BuyBuilder::new().token_index(42).morm_amount("100000000").build().unwrap();
        assert_eq!(req.token_index, 42);
    }

    #[test]
    fn buy_builder_validation() {
        assert!(BuyBuilder::new().build().is_err());
    }

    #[test]
    fn sell_builder_works() {
        let req = SellBuilder::new().token_index(42).token_amount("50000000").build().unwrap();
        assert_eq!(req.token_index, 42);
    }

    #[test]
    fn execute_graduation_builder_works() {
        let req = ExecuteGraduationBuilder::new().token_index(42).build().unwrap();
        assert_eq!(req.token_index, 42);
    }

    #[test]
    fn execute_graduation_builder_validation() {
        assert!(ExecuteGraduationBuilder::new().build().is_err());
    }
}
