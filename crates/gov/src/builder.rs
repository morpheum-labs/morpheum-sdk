//! Fluent builders for the Governance module.
//!
//! Ergonomic, type-safe fluent builders for all governance transaction operations.
//! Each builder follows the classic Builder pattern and returns the corresponding
//! request type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_sdk_core::{AccountId, SdkError};

use crate::requests::{
    CancelProposalRequest, ProposalDepositRequest, ProposalVoteRequest,
    ScheduleUpgradeRequest, SubmitProposalRequest,
};
use crate::types::{ProposalClass, UpgradePlan, WeightedVoteOption};

// ====================== SUBMIT PROPOSAL ======================

#[derive(Default)]
pub struct SubmitProposalBuilder {
    from_address: Option<AccountId>,
    proposal_class: Option<ProposalClass>,
    title: Option<String>,
    description: Option<String>,
    metadata: Option<String>,
    messages: Vec<ProtoAny>,
    initial_deposit: Option<String>,
    additional_metadata: BTreeMap<String, String>,
}

impl SubmitProposalBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn proposal_class(mut self, class: ProposalClass) -> Self {
        self.proposal_class = Some(class);
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

    pub fn messages(mut self, messages: Vec<ProtoAny>) -> Self {
        self.messages = messages;
        self
    }

    pub fn add_message(mut self, message: ProtoAny) -> Self {
        self.messages.push(message);
        self
    }

    pub fn initial_deposit(mut self, deposit: impl Into<String>) -> Self {
        self.initial_deposit = Some(deposit.into());
        self
    }

    pub fn additional_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional_metadata.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Result<SubmitProposalRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for proposal submission")
        })?;
        let proposal_class = self.proposal_class.ok_or_else(|| {
            SdkError::invalid_input("proposal_class is required")
        })?;
        let title = self.title.ok_or_else(|| {
            SdkError::invalid_input("title is required")
        })?;
        let description = self.description.ok_or_else(|| {
            SdkError::invalid_input("description is required")
        })?;
        let initial_deposit = self.initial_deposit.ok_or_else(|| {
            SdkError::invalid_input("initial_deposit is required")
        })?;

        Ok(SubmitProposalRequest {
            from_address,
            proposal_class,
            title,
            description,
            metadata: self.metadata.unwrap_or_default(),
            messages: self.messages,
            initial_deposit,
            additional_metadata: self.additional_metadata,
        })
    }
}

// ====================== SCHEDULE UPGRADE ======================

#[derive(Default)]
pub struct ScheduleUpgradeBuilder {
    from_address: Option<AccountId>,
    proposal_class: Option<ProposalClass>,
    upgrade_plan: Option<UpgradePlan>,
    title: Option<String>,
    description: Option<String>,
    metadata: Option<String>,
    initial_deposit: Option<String>,
    additional_metadata: BTreeMap<String, String>,
}

impl ScheduleUpgradeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn proposal_class(mut self, class: ProposalClass) -> Self {
        self.proposal_class = Some(class);
        self
    }

    pub fn upgrade_plan(mut self, plan: UpgradePlan) -> Self {
        self.upgrade_plan = Some(plan);
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

    pub fn initial_deposit(mut self, deposit: impl Into<String>) -> Self {
        self.initial_deposit = Some(deposit.into());
        self
    }

    pub fn additional_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional_metadata.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Result<ScheduleUpgradeRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for upgrade scheduling")
        })?;
        let proposal_class = self.proposal_class.ok_or_else(|| {
            SdkError::invalid_input("proposal_class is required (ROOT or EMERGENCY)")
        })?;
        let upgrade_plan = self.upgrade_plan.ok_or_else(|| {
            SdkError::invalid_input("upgrade_plan is required")
        })?;
        let title = self.title.ok_or_else(|| {
            SdkError::invalid_input("title is required")
        })?;
        let description = self.description.ok_or_else(|| {
            SdkError::invalid_input("description is required")
        })?;
        let initial_deposit = self.initial_deposit.ok_or_else(|| {
            SdkError::invalid_input("initial_deposit is required")
        })?;

        Ok(ScheduleUpgradeRequest {
            from_address,
            proposal_class,
            upgrade_plan,
            title,
            description,
            metadata: self.metadata.unwrap_or_default(),
            initial_deposit,
            additional_metadata: self.additional_metadata,
        })
    }
}

// ====================== DEPOSIT ======================

#[derive(Default)]
pub struct ProposalDepositBuilder {
    from_address: Option<AccountId>,
    proposal_id: Option<u64>,
    amount: Option<String>,
}

impl ProposalDepositBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn proposal_id(mut self, id: u64) -> Self {
        self.proposal_id = Some(id);
        self
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn build(self) -> Result<ProposalDepositRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for deposit")
        })?;
        let proposal_id = self.proposal_id.ok_or_else(|| {
            SdkError::invalid_input("proposal_id is required")
        })?;
        let amount = self.amount.ok_or_else(|| {
            SdkError::invalid_input("amount is required")
        })?;

        Ok(ProposalDepositRequest::new(from_address, proposal_id, amount))
    }
}

// ====================== VOTE ======================

#[derive(Default)]
pub struct ProposalVoteBuilder {
    from_address: Option<AccountId>,
    proposal_id: Option<u64>,
    options: Vec<WeightedVoteOption>,
    conviction_multiplier: u64,
}

impl ProposalVoteBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn proposal_id(mut self, id: u64) -> Self {
        self.proposal_id = Some(id);
        self
    }

    pub fn options(mut self, options: Vec<WeightedVoteOption>) -> Self {
        self.options = options;
        self
    }

    pub fn add_option(mut self, option: WeightedVoteOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn conviction_multiplier(mut self, multiplier: u64) -> Self {
        self.conviction_multiplier = multiplier;
        self
    }

    pub fn build(self) -> Result<ProposalVoteRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for voting")
        })?;
        let proposal_id = self.proposal_id.ok_or_else(|| {
            SdkError::invalid_input("proposal_id is required")
        })?;
        if self.options.is_empty() {
            return Err(SdkError::invalid_input("at least one vote option is required"));
        }

        Ok(ProposalVoteRequest {
            from_address,
            proposal_id,
            options: self.options,
            conviction_multiplier: self.conviction_multiplier,
        })
    }
}

// ====================== CANCEL PROPOSAL ======================

#[derive(Default)]
pub struct CancelProposalBuilder {
    from_address: Option<AccountId>,
    proposal_id: Option<u64>,
    reason: Option<String>,
}

impl CancelProposalBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
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

    pub fn build(self) -> Result<CancelProposalRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for cancellation")
        })?;
        let proposal_id = self.proposal_id.ok_or_else(|| {
            SdkError::invalid_input("proposal_id is required")
        })?;

        Ok(CancelProposalRequest::new(
            from_address,
            proposal_id,
            self.reason.unwrap_or_else(|| "No reason provided".into()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::VoteOption;
    use alloc::collections::BTreeMap;
    use morpheum_sdk_core::AccountId;

    #[test]
    fn submit_proposal_builder_full_flow() {
        let from = AccountId::new([1u8; 32]);

        let request = SubmitProposalBuilder::new()
            .from_address(from.clone())
            .proposal_class(ProposalClass::Standard)
            .title("Upgrade market params")
            .description("Change BTC-USDC tick size to 0.001")
            .metadata("ipfs://QmTest123")
            .initial_deposit("1000000")
            .additional_metadata("affected_market", "BTC-USDC")
            .build()
            .unwrap();

        assert_eq!(request.from_address, from);
        assert_eq!(request.proposal_class, ProposalClass::Standard);
        assert_eq!(request.title, "Upgrade market params");
        assert_eq!(request.additional_metadata.get("affected_market").unwrap(), "BTC-USDC");
    }

    #[test]
    fn submit_proposal_builder_validation() {
        let result = SubmitProposalBuilder::new().build();
        assert!(result.is_err());
    }

    #[test]
    fn vote_builder_requires_options() {
        let from = AccountId::new([2u8; 32]);
        let result = ProposalVoteBuilder::new()
            .from_address(from)
            .proposal_id(1)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn vote_builder_with_split_vote() {
        let from = AccountId::new([3u8; 32]);

        let request = ProposalVoteBuilder::new()
            .from_address(from)
            .proposal_id(42)
            .add_option(WeightedVoteOption::new(VoteOption::Yes, "0.6"))
            .add_option(WeightedVoteOption::new(VoteOption::Abstain, "0.4"))
            .conviction_multiplier(2)
            .build()
            .unwrap();

        assert_eq!(request.proposal_id, 42);
        assert_eq!(request.options.len(), 2);
        assert_eq!(request.conviction_multiplier, 2);
    }

    #[test]
    fn deposit_builder_works() {
        let request = ProposalDepositBuilder::new()
            .from_address(AccountId::new([4u8; 32]))
            .proposal_id(7)
            .amount("500000")
            .build()
            .unwrap();

        assert_eq!(request.proposal_id, 7);
        assert_eq!(request.amount, "500000");
    }

    #[test]
    fn schedule_upgrade_builder_works() {
        let plan = UpgradePlan {
            name: "v2.1.0-morpheum".into(),
            info: "ipfs://QmUpgrade".into(),
            activation_staple_id: 0,
            activation_time: 0,
            binary_hash: alloc::vec![0xde, 0xad, 0xbe, 0xef],
            grace_period_seconds: 3600,
            additional_metadata: BTreeMap::new(),
        };

        let request = ScheduleUpgradeBuilder::new()
            .from_address(AccountId::new([6u8; 32]))
            .proposal_class(ProposalClass::Root)
            .upgrade_plan(plan)
            .title("v2.1.0 Upgrade")
            .description("Zero-downtime binary upgrade")
            .initial_deposit("5000000")
            .additional_metadata("shadow_mode", "true")
            .build()
            .unwrap();

        assert_eq!(request.proposal_class, ProposalClass::Root);
        assert_eq!(request.upgrade_plan.name, "v2.1.0-morpheum");
    }

}
