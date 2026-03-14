//! Fluent builders for the DAO module.
//!
//! Ergonomic, type-safe fluent builders for all DAO transaction operations.
//! Each builder follows the classic Builder pattern and returns the corresponding
//! request type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_sdk_core::{AccountId, SdkError};

use crate::requests::{
    CancelDaoProposalRequest, CreateDaoProposalRequest, CreateDaoRequest,
    DaoDepositRequest, DaoVoteRequest, ExecuteDaoProposalRequest,
    SignDaoProposalRequest, WithdrawDaoDepositRequest,
};
use crate::types::{DaoConfig, DaoType, GovernedAsset, WeightedDaoVoteOption};

// ====================== CREATE DAO ======================

#[derive(Default)]
pub struct CreateDaoBuilder {
    from_address: Option<AccountId>,
    name: Option<String>,
    description: Option<String>,
    community_token_mint: Option<String>,
    council_token_mint: Option<String>,
    dao_type: Option<DaoType>,
    config: Option<DaoConfig>,
    initial_governed_assets: Vec<GovernedAsset>,
    metadata: BTreeMap<String, String>,
}

impl CreateDaoBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn community_token_mint(mut self, mint: impl Into<String>) -> Self {
        self.community_token_mint = Some(mint.into());
        self
    }

    pub fn council_token_mint(mut self, mint: impl Into<String>) -> Self {
        self.council_token_mint = Some(mint.into());
        self
    }

    pub fn dao_type(mut self, dao_type: DaoType) -> Self {
        self.dao_type = Some(dao_type);
        self
    }

    pub fn config(mut self, config: DaoConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn governed_assets(mut self, assets: Vec<GovernedAsset>) -> Self {
        self.initial_governed_assets = assets;
        self
    }

    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Result<CreateDaoRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for DAO creation")
        })?;
        let name = self.name.ok_or_else(|| {
            SdkError::invalid_input("name is required")
        })?;
        let community_token_mint = self.community_token_mint.ok_or_else(|| {
            SdkError::invalid_input("community_token_mint is required")
        })?;
        let dao_type = self.dao_type.ok_or_else(|| {
            SdkError::invalid_input("dao_type is required")
        })?;
        let config = self.config.ok_or_else(|| {
            SdkError::invalid_input("config is required for DAO creation")
        })?;

        let mut req = CreateDaoRequest::new(
            from_address,
            name,
            community_token_mint,
            dao_type,
            config,
        );

        if let Some(desc) = self.description {
            req = req.with_description(desc);
        }
        if let Some(council) = self.council_token_mint {
            req = req.with_council_token_mint(council);
        }
        if !self.initial_governed_assets.is_empty() {
            req = req.with_governed_assets(self.initial_governed_assets);
        }
        if !self.metadata.is_empty() {
            req = req.with_metadata(self.metadata);
        }

        Ok(req)
    }
}

// ====================== CREATE PROPOSAL ======================

#[derive(Default)]
pub struct CreateDaoProposalBuilder {
    from_address: Option<AccountId>,
    dao_id: Option<u64>,
    title: Option<String>,
    description: Option<String>,
    metadata: Option<String>,
    instructions: Vec<ProtoAny>,
    additional_metadata: BTreeMap<String, String>,
}

impl CreateDaoProposalBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn dao_id(mut self, id: u64) -> Self {
        self.dao_id = Some(id);
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    pub fn instructions(mut self, instructions: Vec<ProtoAny>) -> Self {
        self.instructions = instructions;
        self
    }

    pub fn add_instruction(mut self, instruction: ProtoAny) -> Self {
        self.instructions.push(instruction);
        self
    }

    pub fn additional_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional_metadata.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Result<CreateDaoProposalRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for proposal creation")
        })?;
        let dao_id = self.dao_id.ok_or_else(|| {
            SdkError::invalid_input("dao_id is required")
        })?;
        let title = self.title.ok_or_else(|| {
            SdkError::invalid_input("title is required")
        })?;
        let description = self.description.ok_or_else(|| {
            SdkError::invalid_input("description is required")
        })?;

        let mut req = CreateDaoProposalRequest::new(from_address, dao_id, title, description);

        if let Some(meta) = self.metadata {
            req = req.with_metadata(meta);
        }
        if !self.instructions.is_empty() {
            req = req.with_instructions(self.instructions);
        }
        if !self.additional_metadata.is_empty() {
            req = req.with_additional_metadata(self.additional_metadata);
        }

        Ok(req)
    }
}

// ====================== VOTE ======================

#[derive(Default)]
pub struct DaoVoteBuilder {
    from_address: Option<AccountId>,
    dao_id: Option<u64>,
    proposal_id: Option<u64>,
    options: Vec<WeightedDaoVoteOption>,
    conviction_multiplier: u64,
}

impl DaoVoteBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn dao_id(mut self, id: u64) -> Self {
        self.dao_id = Some(id);
        self
    }

    pub fn proposal_id(mut self, id: u64) -> Self {
        self.proposal_id = Some(id);
        self
    }

    pub fn options(mut self, options: Vec<WeightedDaoVoteOption>) -> Self {
        self.options = options;
        self
    }

    pub fn add_option(mut self, option: WeightedDaoVoteOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn conviction_multiplier(mut self, multiplier: u64) -> Self {
        self.conviction_multiplier = multiplier;
        self
    }

    pub fn build(self) -> Result<DaoVoteRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for voting")
        })?;
        let dao_id = self.dao_id.ok_or_else(|| {
            SdkError::invalid_input("dao_id is required")
        })?;
        let proposal_id = self.proposal_id.ok_or_else(|| {
            SdkError::invalid_input("proposal_id is required")
        })?;
        if self.options.is_empty() {
            return Err(SdkError::invalid_input("at least one vote option is required"));
        }

        Ok(DaoVoteRequest {
            from_address,
            dao_id,
            proposal_id,
            options: self.options,
            conviction_multiplier: self.conviction_multiplier,
        })
    }
}

// ====================== SIGN PROPOSAL ======================

#[derive(Default)]
pub struct SignDaoProposalBuilder {
    from_address: Option<AccountId>,
    dao_id: Option<u64>,
    proposal_id: Option<u64>,
}

impl SignDaoProposalBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn dao_id(mut self, id: u64) -> Self {
        self.dao_id = Some(id);
        self
    }

    pub fn proposal_id(mut self, id: u64) -> Self {
        self.proposal_id = Some(id);
        self
    }

    pub fn build(self) -> Result<SignDaoProposalRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for signing a proposal")
        })?;
        let dao_id = self.dao_id.ok_or_else(|| {
            SdkError::invalid_input("dao_id is required")
        })?;
        let proposal_id = self.proposal_id.ok_or_else(|| {
            SdkError::invalid_input("proposal_id is required")
        })?;

        Ok(SignDaoProposalRequest::new(from_address, dao_id, proposal_id))
    }
}

// ====================== DEPOSIT ======================

#[derive(Default)]
pub struct DaoDepositBuilder {
    from_address: Option<AccountId>,
    dao_id: Option<u64>,
    token_mint: Option<String>,
    amount: Option<String>,
    lock_until: u64,
}

impl DaoDepositBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn dao_id(mut self, id: u64) -> Self {
        self.dao_id = Some(id);
        self
    }

    pub fn token_mint(mut self, mint: impl Into<String>) -> Self {
        self.token_mint = Some(mint.into());
        self
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn lock_until(mut self, lock_until: u64) -> Self {
        self.lock_until = lock_until;
        self
    }

    pub fn build(self) -> Result<DaoDepositRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for deposit")
        })?;
        let dao_id = self.dao_id.ok_or_else(|| {
            SdkError::invalid_input("dao_id is required")
        })?;
        let token_mint = self.token_mint.ok_or_else(|| {
            SdkError::invalid_input("token_mint is required")
        })?;
        let amount = self.amount.ok_or_else(|| {
            SdkError::invalid_input("amount is required")
        })?;

        let mut req = DaoDepositRequest::new(from_address, dao_id, token_mint, amount);
        if self.lock_until > 0 {
            req = req.with_lock_until(self.lock_until);
        }
        Ok(req)
    }
}

// ====================== CANCEL PROPOSAL ======================

#[derive(Default)]
pub struct CancelDaoProposalBuilder {
    from_address: Option<AccountId>,
    dao_id: Option<u64>,
    proposal_id: Option<u64>,
    reason: Option<String>,
}

impl CancelDaoProposalBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn dao_id(mut self, id: u64) -> Self {
        self.dao_id = Some(id);
        self
    }

    pub fn proposal_id(mut self, id: u64) -> Self {
        self.proposal_id = Some(id);
        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn build(self) -> Result<CancelDaoProposalRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for cancellation")
        })?;
        let dao_id = self.dao_id.ok_or_else(|| {
            SdkError::invalid_input("dao_id is required")
        })?;
        let proposal_id = self.proposal_id.ok_or_else(|| {
            SdkError::invalid_input("proposal_id is required")
        })?;

        Ok(CancelDaoProposalRequest::new(
            from_address,
            dao_id,
            proposal_id,
            self.reason.unwrap_or_else(|| "No reason provided".into()),
        ))
    }
}

// ====================== EXECUTE PROPOSAL ======================

#[derive(Default)]
pub struct ExecuteDaoProposalBuilder {
    from_address: Option<AccountId>,
    dao_id: Option<u64>,
    proposal_id: Option<u64>,
}

impl ExecuteDaoProposalBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn dao_id(mut self, id: u64) -> Self {
        self.dao_id = Some(id);
        self
    }

    pub fn proposal_id(mut self, id: u64) -> Self {
        self.proposal_id = Some(id);
        self
    }

    pub fn build(self) -> Result<ExecuteDaoProposalRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for execution")
        })?;
        let dao_id = self.dao_id.ok_or_else(|| {
            SdkError::invalid_input("dao_id is required")
        })?;
        let proposal_id = self.proposal_id.ok_or_else(|| {
            SdkError::invalid_input("proposal_id is required")
        })?;

        Ok(ExecuteDaoProposalRequest::new(from_address, dao_id, proposal_id))
    }
}

// ====================== WITHDRAW DEPOSIT ======================

#[derive(Default)]
pub struct WithdrawDaoDepositBuilder {
    from_address: Option<AccountId>,
    dao_id: Option<u64>,
    token_mint: Option<String>,
    amount: Option<String>,
}

impl WithdrawDaoDepositBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn dao_id(mut self, id: u64) -> Self {
        self.dao_id = Some(id);
        self
    }

    pub fn token_mint(mut self, mint: impl Into<String>) -> Self {
        self.token_mint = Some(mint.into());
        self
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn build(self) -> Result<WithdrawDaoDepositRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for withdrawal")
        })?;
        let dao_id = self.dao_id.ok_or_else(|| {
            SdkError::invalid_input("dao_id is required")
        })?;
        let token_mint = self.token_mint.ok_or_else(|| {
            SdkError::invalid_input("token_mint is required")
        })?;
        let amount = self.amount.ok_or_else(|| {
            SdkError::invalid_input("amount is required (use \"0\" for all)")
        })?;

        Ok(WithdrawDaoDepositRequest::new(from_address, dao_id, token_mint, amount))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DaoVoteOption;
    use morpheum_sdk_core::AccountId;

    #[test]
    fn create_dao_builder_full_flow() {
        let from = AccountId::new([1u8; 32]);
        let config = DaoConfig {
            voting_period: "86400s".into(),
            hold_up_time: "3600s".into(),
            min_deposit_for_proposal: "1000".into(),
            quorum: "0.2".into(),
            approval_threshold: "0.5".into(),
            allow_council_override: true,
            use_conviction_voting: false,
            max_active_proposals: 10,
            plugin_configs: BTreeMap::new(),
        };

        let request = CreateDaoBuilder::new()
            .from_address(from.clone())
            .name("Morpheum Grants DAO")
            .description("Manages ecosystem grants")
            .community_token_mint("morpheum1mint")
            .council_token_mint("morpheum1council")
            .dao_type(DaoType::Hybrid)
            .config(config)
            .metadata("website", "https://dao.morpheum.xyz")
            .build()
            .unwrap();

        assert_eq!(request.from_address, from);
        assert_eq!(request.name, "Morpheum Grants DAO");
        assert_eq!(request.dao_type, DaoType::Hybrid);
        assert!(request.council_token_mint.is_some());
    }

    #[test]
    fn create_dao_builder_validation() {
        let result = CreateDaoBuilder::new().build();
        assert!(result.is_err());
    }

    #[test]
    fn vote_builder_requires_options() {
        let from = AccountId::new([2u8; 32]);
        let result = DaoVoteBuilder::new()
            .from_address(from)
            .dao_id(1)
            .proposal_id(1)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn vote_builder_with_split_vote() {
        let from = AccountId::new([3u8; 32]);

        let request = DaoVoteBuilder::new()
            .from_address(from)
            .dao_id(1)
            .proposal_id(42)
            .add_option(WeightedDaoVoteOption::new(DaoVoteOption::Yes, "0.7"))
            .add_option(WeightedDaoVoteOption::new(DaoVoteOption::No, "0.3"))
            .conviction_multiplier(4)
            .build()
            .unwrap();

        assert_eq!(request.dao_id, 1);
        assert_eq!(request.proposal_id, 42);
        assert_eq!(request.options.len(), 2);
        assert_eq!(request.conviction_multiplier, 4);
    }

    #[test]
    fn deposit_builder_works() {
        let request = DaoDepositBuilder::new()
            .from_address(AccountId::new([4u8; 32]))
            .dao_id(1)
            .token_mint("morpheum1mint")
            .amount("5000000")
            .lock_until(1_700_100_000)
            .build()
            .unwrap();

        assert_eq!(request.dao_id, 1);
        assert_eq!(request.amount, "5000000");
        assert_eq!(request.lock_until, 1_700_100_000);
    }

    #[test]
    fn execute_proposal_builder_works() {
        let request = ExecuteDaoProposalBuilder::new()
            .from_address(AccountId::new([5u8; 32]))
            .dao_id(1)
            .proposal_id(99)
            .build()
            .unwrap();

        assert_eq!(request.dao_id, 1);
        assert_eq!(request.proposal_id, 99);
    }

    #[test]
    fn withdraw_deposit_builder_works() {
        let request = WithdrawDaoDepositBuilder::new()
            .from_address(AccountId::new([6u8; 32]))
            .dao_id(1)
            .token_mint("morpheum1mint")
            .amount("0")
            .build()
            .unwrap();

        assert_eq!(request.amount, "0");
    }
}
