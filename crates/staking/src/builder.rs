//! Fluent builders for the Staking module.
//!
//! Ergonomic, type-safe builders for all staking transaction operations.
//! Each builder validates required fields and returns the corresponding
//! request type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::*;
use crate::types::MisbehaviorType;

// ============================================================================
// StakeBuilder
// ============================================================================

/// Fluent builder for staking MORM to a validator.
#[derive(Default)]
pub struct StakeBuilder {
    address: Option<String>,
    validator_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
    external_address: Option<String>,
}

impl StakeBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn address(mut self, addr: impl Into<String>) -> Self {
        self.address = Some(addr.into()); self
    }
    pub fn validator_id(mut self, id: impl Into<String>) -> Self {
        self.validator_id = Some(id.into()); self
    }
    pub fn asset_index(mut self, idx: u64) -> Self {
        self.asset_index = Some(idx); self
    }
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into()); self
    }
    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into()); self
    }

    pub fn build(self) -> Result<StakeRequest, SdkError> {
        let address = self.address.ok_or_else(|| SdkError::invalid_input("address is required for staking"))?;
        let validator_id = self.validator_id.ok_or_else(|| SdkError::invalid_input("validator_id is required for staking"))?;
        let asset_index = self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required for staking"))?;
        let amount = self.amount.ok_or_else(|| SdkError::invalid_input("amount is required for staking"))?;

        let mut req = StakeRequest::new(address, validator_id, asset_index, amount);
        req.external_address = self.external_address;
        Ok(req)
    }
}

// ============================================================================
// UnstakeBuilder
// ============================================================================

/// Fluent builder for unstaking MORM.
#[derive(Default)]
pub struct UnstakeBuilder {
    address: Option<String>,
    validator_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
    external_address: Option<String>,
}

impl UnstakeBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn address(mut self, addr: impl Into<String>) -> Self {
        self.address = Some(addr.into()); self
    }
    pub fn validator_id(mut self, id: impl Into<String>) -> Self {
        self.validator_id = Some(id.into()); self
    }
    pub fn asset_index(mut self, idx: u64) -> Self {
        self.asset_index = Some(idx); self
    }
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into()); self
    }
    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into()); self
    }

    pub fn build(self) -> Result<UnstakeRequest, SdkError> {
        let address = self.address.ok_or_else(|| SdkError::invalid_input("address is required for unstaking"))?;
        let validator_id = self.validator_id.ok_or_else(|| SdkError::invalid_input("validator_id is required for unstaking"))?;
        let asset_index = self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required for unstaking"))?;
        let amount = self.amount.ok_or_else(|| SdkError::invalid_input("amount is required for unstaking"))?;

        let mut req = UnstakeRequest::new(address, validator_id, asset_index, amount);
        req.external_address = self.external_address;
        Ok(req)
    }
}

// ============================================================================
// DelegateBuilder
// ============================================================================

/// Fluent builder for delegating MORM to a validator.
#[derive(Default)]
pub struct DelegateBuilder {
    delegator_address: Option<String>,
    validator_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
    delegator_external_address: Option<String>,
}

impl DelegateBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn delegator_address(mut self, addr: impl Into<String>) -> Self {
        self.delegator_address = Some(addr.into()); self
    }
    pub fn validator_id(mut self, id: impl Into<String>) -> Self {
        self.validator_id = Some(id.into()); self
    }
    pub fn asset_index(mut self, idx: u64) -> Self {
        self.asset_index = Some(idx); self
    }
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into()); self
    }
    pub fn delegator_external_address(mut self, addr: impl Into<String>) -> Self {
        self.delegator_external_address = Some(addr.into()); self
    }

    pub fn build(self) -> Result<DelegateRequest, SdkError> {
        let delegator_address = self.delegator_address.ok_or_else(|| SdkError::invalid_input("delegator_address is required for delegation"))?;
        let validator_id = self.validator_id.ok_or_else(|| SdkError::invalid_input("validator_id is required for delegation"))?;
        let asset_index = self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required for delegation"))?;
        let amount = self.amount.ok_or_else(|| SdkError::invalid_input("amount is required for delegation"))?;

        let mut req = DelegateRequest::new(delegator_address, validator_id, asset_index, amount);
        req.delegator_external_address = self.delegator_external_address;
        Ok(req)
    }
}

// ============================================================================
// UndelegateBuilder
// ============================================================================

/// Fluent builder for undelegating MORM from a validator.
#[derive(Default)]
pub struct UndelegateBuilder {
    delegator_address: Option<String>,
    validator_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
    delegator_external_address: Option<String>,
    delegator_chain_type: Option<i32>,
}

impl UndelegateBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn delegator_address(mut self, addr: impl Into<String>) -> Self {
        self.delegator_address = Some(addr.into()); self
    }
    pub fn validator_id(mut self, id: impl Into<String>) -> Self {
        self.validator_id = Some(id.into()); self
    }
    pub fn asset_index(mut self, idx: u64) -> Self {
        self.asset_index = Some(idx); self
    }
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into()); self
    }
    pub fn delegator_external_address(mut self, addr: impl Into<String>) -> Self {
        self.delegator_external_address = Some(addr.into()); self
    }
    pub fn delegator_chain_type(mut self, ct: i32) -> Self {
        self.delegator_chain_type = Some(ct); self
    }

    pub fn build(self) -> Result<UndelegateRequest, SdkError> {
        let delegator_address = self.delegator_address.ok_or_else(|| SdkError::invalid_input("delegator_address is required for undelegation"))?;
        let validator_id = self.validator_id.ok_or_else(|| SdkError::invalid_input("validator_id is required for undelegation"))?;
        let asset_index = self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required for undelegation"))?;
        let amount = self.amount.ok_or_else(|| SdkError::invalid_input("amount is required for undelegation"))?;

        let mut req = UndelegateRequest::new(delegator_address, validator_id, asset_index, amount);
        req.delegator_external_address = self.delegator_external_address;
        req.delegator_chain_type = self.delegator_chain_type;
        Ok(req)
    }
}

// ============================================================================
// RedelegateBuilder
// ============================================================================

/// Fluent builder for redelegating MORM between validators.
#[derive(Default)]
pub struct RedelegateBuilder {
    delegator_address: Option<String>,
    from_validator_id: Option<String>,
    to_validator_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
    delegator_external_address: Option<String>,
    delegator_chain_type: Option<i32>,
}

impl RedelegateBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn delegator_address(mut self, addr: impl Into<String>) -> Self {
        self.delegator_address = Some(addr.into()); self
    }
    pub fn from_validator_id(mut self, id: impl Into<String>) -> Self {
        self.from_validator_id = Some(id.into()); self
    }
    pub fn to_validator_id(mut self, id: impl Into<String>) -> Self {
        self.to_validator_id = Some(id.into()); self
    }
    pub fn asset_index(mut self, idx: u64) -> Self {
        self.asset_index = Some(idx); self
    }
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into()); self
    }
    pub fn delegator_external_address(mut self, addr: impl Into<String>) -> Self {
        self.delegator_external_address = Some(addr.into()); self
    }
    pub fn delegator_chain_type(mut self, ct: i32) -> Self {
        self.delegator_chain_type = Some(ct); self
    }

    pub fn build(self) -> Result<RedelegateRequest, SdkError> {
        let delegator_address = self.delegator_address.ok_or_else(|| SdkError::invalid_input("delegator_address is required for redelegation"))?;
        let from_validator_id = self.from_validator_id.ok_or_else(|| SdkError::invalid_input("from_validator_id is required for redelegation"))?;
        let to_validator_id = self.to_validator_id.ok_or_else(|| SdkError::invalid_input("to_validator_id is required for redelegation"))?;
        let asset_index = self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required for redelegation"))?;
        let amount = self.amount.ok_or_else(|| SdkError::invalid_input("amount is required for redelegation"))?;

        let mut req = RedelegateRequest::new(delegator_address, from_validator_id, to_validator_id, asset_index, amount);
        req.delegator_external_address = self.delegator_external_address;
        req.delegator_chain_type = self.delegator_chain_type;
        Ok(req)
    }
}

// ============================================================================
// ClaimRewardsBuilder
// ============================================================================

/// Fluent builder for claiming staking rewards.
#[derive(Default)]
pub struct ClaimRewardsBuilder {
    address: Option<String>,
    validator_id: Option<String>,
    external_address: Option<String>,
}

impl ClaimRewardsBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn address(mut self, addr: impl Into<String>) -> Self {
        self.address = Some(addr.into()); self
    }
    pub fn validator_id(mut self, id: impl Into<String>) -> Self {
        self.validator_id = Some(id.into()); self
    }
    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into()); self
    }

    pub fn build(self) -> Result<ClaimRewardsRequest, SdkError> {
        let address = self.address.ok_or_else(|| SdkError::invalid_input("address is required for claiming rewards"))?;
        let validator_id = self.validator_id.unwrap_or_default();

        let mut req = ClaimRewardsRequest::new(address, validator_id);
        req.external_address = self.external_address;
        Ok(req)
    }
}

// ============================================================================
// ReportMisbehaviorBuilder
// ============================================================================

/// Fluent builder for reporting validator misbehavior.
#[derive(Default)]
pub struct ReportMisbehaviorBuilder {
    validator_id: Option<String>,
    misbehavior_type: Option<MisbehaviorType>,
    evidence: Option<Vec<u8>>,
    severity: Option<String>,
    height: u64,
    sig: Option<Vec<u8>>,
    reporter_address: Option<String>,
    reporter_external_address: Option<String>,
    reporter_chain_type: Option<i32>,
}

impl ReportMisbehaviorBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn validator_id(mut self, id: impl Into<String>) -> Self {
        self.validator_id = Some(id.into()); self
    }
    pub fn misbehavior_type(mut self, t: MisbehaviorType) -> Self {
        self.misbehavior_type = Some(t); self
    }
    pub fn evidence(mut self, evidence: Vec<u8>) -> Self {
        self.evidence = Some(evidence); self
    }
    pub fn severity(mut self, severity: impl Into<String>) -> Self {
        self.severity = Some(severity.into()); self
    }
    pub fn height(mut self, height: u64) -> Self {
        self.height = height; self
    }
    pub fn sig(mut self, sig: Vec<u8>) -> Self {
        self.sig = Some(sig); self
    }
    pub fn reporter_address(mut self, addr: impl Into<String>) -> Self {
        self.reporter_address = Some(addr.into()); self
    }
    pub fn reporter_external_address(mut self, addr: impl Into<String>) -> Self {
        self.reporter_external_address = Some(addr.into()); self
    }
    pub fn reporter_chain_type(mut self, ct: i32) -> Self {
        self.reporter_chain_type = Some(ct); self
    }

    pub fn build(self) -> Result<ReportMisbehaviorRequest, SdkError> {
        let validator_id = self.validator_id.ok_or_else(|| SdkError::invalid_input("validator_id is required for misbehavior report"))?;
        let misbehavior_type = self.misbehavior_type.ok_or_else(|| SdkError::invalid_input("misbehavior_type is required for misbehavior report"))?;
        let evidence = self.evidence.ok_or_else(|| SdkError::invalid_input("evidence is required for misbehavior report"))?;
        let severity = self.severity.ok_or_else(|| SdkError::invalid_input("severity is required for misbehavior report"))?;
        let sig = self.sig.ok_or_else(|| SdkError::invalid_input("sig is required for misbehavior report"))?;
        let reporter_address = self.reporter_address.ok_or_else(|| SdkError::invalid_input("reporter_address is required for misbehavior report"))?;

        let mut req = ReportMisbehaviorRequest::new(validator_id, misbehavior_type, evidence, severity, reporter_address, sig);
        req.height = self.height;
        req.reporter_external_address = self.reporter_external_address;
        req.reporter_chain_type = self.reporter_chain_type;
        Ok(req)
    }
}

// ============================================================================
// VoteOnSlashingBuilder
// ============================================================================

/// Fluent builder for voting on a slashing proposal.
#[derive(Default)]
pub struct VoteOnSlashingBuilder {
    misbehavior_id: Option<String>,
    vote: Option<bool>,
    voter_address: Option<String>,
    sig: Option<Vec<u8>>,
    voter_external_address: Option<String>,
    voter_chain_type: Option<i32>,
}

impl VoteOnSlashingBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn misbehavior_id(mut self, id: impl Into<String>) -> Self {
        self.misbehavior_id = Some(id.into()); self
    }
    pub fn vote(mut self, vote: bool) -> Self {
        self.vote = Some(vote); self
    }
    pub fn voter_address(mut self, addr: impl Into<String>) -> Self {
        self.voter_address = Some(addr.into()); self
    }
    pub fn sig(mut self, sig: Vec<u8>) -> Self {
        self.sig = Some(sig); self
    }
    pub fn voter_external_address(mut self, addr: impl Into<String>) -> Self {
        self.voter_external_address = Some(addr.into()); self
    }
    pub fn voter_chain_type(mut self, ct: i32) -> Self {
        self.voter_chain_type = Some(ct); self
    }

    pub fn build(self) -> Result<VoteOnSlashingRequest, SdkError> {
        let misbehavior_id = self.misbehavior_id.ok_or_else(|| SdkError::invalid_input("misbehavior_id is required for slashing vote"))?;
        let vote = self.vote.ok_or_else(|| SdkError::invalid_input("vote is required for slashing vote"))?;
        let voter_address = self.voter_address.ok_or_else(|| SdkError::invalid_input("voter_address is required for slashing vote"))?;
        let sig = self.sig.ok_or_else(|| SdkError::invalid_input("sig is required for slashing vote"))?;

        let mut req = VoteOnSlashingRequest::new(misbehavior_id, vote, voter_address, sig);
        req.voter_external_address = self.voter_external_address;
        req.voter_chain_type = self.voter_chain_type;
        Ok(req)
    }
}

// ============================================================================
// ApplySlashingBuilder
// ============================================================================

/// Fluent builder for applying a slashing penalty.
#[derive(Default)]
pub struct ApplySlashingBuilder {
    misbehavior_id: Option<String>,
    validator_id: Option<String>,
    slash_type: Option<String>,
    asset_index: u64,
    balance_penalty: Option<String>,
    reputation_penalty: Option<String>,
    quorum_votes: BTreeMap<String, bool>,
    sig: Option<Vec<u8>>,
}

impl ApplySlashingBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn misbehavior_id(mut self, id: impl Into<String>) -> Self {
        self.misbehavior_id = Some(id.into()); self
    }
    pub fn validator_id(mut self, id: impl Into<String>) -> Self {
        self.validator_id = Some(id.into()); self
    }
    pub fn slash_type(mut self, t: impl Into<String>) -> Self {
        self.slash_type = Some(t.into()); self
    }
    pub fn asset_index(mut self, idx: u64) -> Self {
        self.asset_index = idx; self
    }
    pub fn balance_penalty(mut self, penalty: impl Into<String>) -> Self {
        self.balance_penalty = Some(penalty.into()); self
    }
    pub fn reputation_penalty(mut self, penalty: impl Into<String>) -> Self {
        self.reputation_penalty = Some(penalty.into()); self
    }
    pub fn quorum_vote(mut self, voter: impl Into<String>, vote: bool) -> Self {
        self.quorum_votes.insert(voter.into(), vote); self
    }
    pub fn sig(mut self, sig: Vec<u8>) -> Self {
        self.sig = Some(sig); self
    }

    pub fn build(self) -> Result<ApplySlashingRequest, SdkError> {
        let misbehavior_id = self.misbehavior_id.ok_or_else(|| SdkError::invalid_input("misbehavior_id is required for slashing"))?;
        let validator_id = self.validator_id.ok_or_else(|| SdkError::invalid_input("validator_id is required for slashing"))?;
        let slash_type = self.slash_type.ok_or_else(|| SdkError::invalid_input("slash_type is required for slashing"))?;
        let sig = self.sig.ok_or_else(|| SdkError::invalid_input("sig is required for slashing"))?;

        let mut req = ApplySlashingRequest::new(misbehavior_id, validator_id, slash_type, sig);
        req.asset_index = self.asset_index;
        if let Some(bp) = self.balance_penalty { req.balance_penalty = bp; }
        if let Some(rp) = self.reputation_penalty { req.reputation_penalty = rp; }
        req.quorum_votes = self.quorum_votes;
        Ok(req)
    }
}

// ============================================================================
// UpdateParamsBuilder
// ============================================================================

/// Fluent builder for governance-gated staking parameter updates.
#[derive(Default)]
pub struct UpdateParamsBuilder {
    authority: Option<String>,
    params: Option<morpheum_proto::staking::v1::Params>,
}

impl UpdateParamsBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn authority(mut self, authority: impl Into<String>) -> Self {
        self.authority = Some(authority.into()); self
    }
    pub fn params(mut self, params: morpheum_proto::staking::v1::Params) -> Self {
        self.params = Some(params); self
    }

    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        let authority = self.authority.ok_or_else(|| SdkError::invalid_input("authority is required for update_params"))?;
        let params = self.params.ok_or_else(|| SdkError::invalid_input("params is required for update_params"))?;
        Ok(UpdateParamsRequest::new(authority, params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn stake_builder_works() {
        let req = StakeBuilder::new()
            .address("morm1abc")
            .validator_id("val-1")
            .asset_index(0)
            .amount("1000000")
            .build()
            .unwrap();
        assert_eq!(req.address, "morm1abc");
        assert_eq!(req.validator_id, "val-1");
    }

    #[test]
    fn stake_builder_missing_required() {
        assert!(StakeBuilder::new().build().is_err());
        assert!(StakeBuilder::new().address("a").build().is_err());
    }

    #[test]
    fn delegate_builder_works() {
        let req = DelegateBuilder::new()
            .delegator_address("morm1abc")
            .validator_id("val-1")
            .asset_index(0)
            .amount("500000")
            .build()
            .unwrap();
        assert_eq!(req.delegator_address, "morm1abc");
    }

    #[test]
    fn redelegate_builder_works() {
        let req = RedelegateBuilder::new()
            .delegator_address("morm1abc")
            .from_validator_id("val-1")
            .to_validator_id("val-2")
            .asset_index(0)
            .amount("250000")
            .build()
            .unwrap();
        assert_eq!(req.from_validator_id, "val-1");
        assert_eq!(req.to_validator_id, "val-2");
    }

    #[test]
    fn claim_rewards_builder_works() {
        let req = ClaimRewardsBuilder::new()
            .address("morm1abc")
            .validator_id("val-1")
            .build()
            .unwrap();
        assert_eq!(req.address, "morm1abc");
    }

    #[test]
    fn claim_rewards_builder_no_validator() {
        let req = ClaimRewardsBuilder::new()
            .address("morm1abc")
            .build()
            .unwrap();
        assert!(req.validator_id.is_empty());
    }

    #[test]
    fn report_misbehavior_builder_works() {
        let req = ReportMisbehaviorBuilder::new()
            .validator_id("val-1")
            .misbehavior_type(MisbehaviorType::DoubleVote)
            .evidence(vec![1, 2, 3])
            .severity("critical")
            .height(100)
            .sig(vec![4, 5, 6])
            .reporter_address("morm1reporter")
            .build()
            .unwrap();
        assert_eq!(req.misbehavior_type, MisbehaviorType::DoubleVote);
        assert_eq!(req.height, 100);
    }

    #[test]
    fn vote_on_slashing_builder_works() {
        let req = VoteOnSlashingBuilder::new()
            .misbehavior_id("misb-1")
            .vote(true)
            .voter_address("morm1voter")
            .sig(vec![7, 8])
            .build()
            .unwrap();
        assert!(req.vote);
    }

    #[test]
    fn apply_slashing_builder_works() {
        let req = ApplySlashingBuilder::new()
            .misbehavior_id("misb-1")
            .validator_id("val-1")
            .slash_type("both")
            .balance_penalty("10000")
            .reputation_penalty("50")
            .quorum_vote("voter-1", true)
            .quorum_vote("voter-2", true)
            .sig(vec![9, 10])
            .build()
            .unwrap();
        assert_eq!(req.quorum_votes.len(), 2);
    }

}
