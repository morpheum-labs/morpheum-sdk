//! Domain types for the Outcome Settlement Account (OSA) module.
//!
//! Covers account lifecycle statuses, per-outcome settlement accounts,
//! user share balances, and streaming settlement/payout events.

use alloc::string::String;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::osa::v1 as proto;

// ====================== ENUMS ======================

/// Account lifecycle: Open -> Resolving -> Settled -> Archived.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AccountStatus {
    Unspecified,
    Open,
    Resolving,
    Settled,
    Archived,
}

impl From<i32> for AccountStatus {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Open, 2 => Self::Resolving,
            3 => Self::Settled, 4 => Self::Archived,
            _ => Self::Unspecified,
        }
    }
}

impl From<AccountStatus> for i32 {
    fn from(s: AccountStatus) -> Self {
        match s {
            AccountStatus::Unspecified => 0, AccountStatus::Open => 1,
            AccountStatus::Resolving => 2, AccountStatus::Settled => 3,
            AccountStatus::Archived => 4,
        }
    }
}

// ====================== DOMAIN TYPES ======================

/// Per-outcome collateral container for prediction markets.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OutcomeSettlementAccount {
    pub market_index: u64,
    pub outcome_id: String,
    pub collateral_asset_index: u64,
    pub total_locked_collateral: u64,
    pub total_shares_outstanding: u64,
    pub status: AccountStatus,
    /// Scaled redemption rate: 10000 = 1.0; 0 = loser outcome.
    pub redemption_rate: u64,
    pub created_at: u64,
    pub settled_at: Option<u64>,
}

impl From<proto::OutcomeSettlementAccount> for OutcomeSettlementAccount {
    fn from(p: proto::OutcomeSettlementAccount) -> Self {
        Self {
            market_index: p.market_index,
            outcome_id: p.outcome_id,
            collateral_asset_index: p.collateral_asset_index,
            total_locked_collateral: p.total_locked_collateral,
            total_shares_outstanding: p.total_shares_outstanding,
            status: AccountStatus::from(p.status),
            redemption_rate: p.redemption_rate,
            created_at: p.created_at,
            settled_at: p.settled_at,
        }
    }
}

/// User share balance within a single outcome account.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Balance {
    pub shares: u64,
    pub last_claimed: Option<u64>,
}

impl From<proto::Balance> for Balance {
    fn from(p: proto::Balance) -> Self {
        Self { shares: p.shares, last_claimed: p.last_claimed }
    }
}

// ====================== EVENTS ======================

/// Emitted when an outcome is settled.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SettlementEvent {
    pub account_id: String,
    pub market_index: u64,
    pub outcome_id: String,
    pub redemption_rate: u64,
}

impl From<proto::SettlementEvent> for SettlementEvent {
    fn from(p: proto::SettlementEvent) -> Self {
        Self {
            account_id: p.account_id, market_index: p.market_index,
            outcome_id: p.outcome_id, redemption_rate: p.redemption_rate,
        }
    }
}

/// Emitted when a payout is claimed.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PayoutEvent {
    pub account_id: String,
    pub beneficiary: String,
    pub shares: u64,
    pub collateral_out: u64,
}

impl From<proto::PayoutEvent> for PayoutEvent {
    fn from(p: proto::PayoutEvent) -> Self {
        Self {
            account_id: p.account_id, beneficiary: p.beneficiary,
            shares: p.shares, collateral_out: p.collateral_out,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_status_roundtrip() {
        for s in [AccountStatus::Open, AccountStatus::Resolving, AccountStatus::Settled, AccountStatus::Archived] {
            let v: i32 = s.into();
            assert_eq!(s, AccountStatus::from(v));
        }
        assert_eq!(AccountStatus::Unspecified, AccountStatus::from(99));
    }

    #[test]
    fn outcome_settlement_account_from_proto() {
        let p = proto::OutcomeSettlementAccount {
            market_index: 1, outcome_id: "yes".into(),
            collateral_asset_index: 2, total_locked_collateral: 1000,
            total_shares_outstanding: 1000, status: 1,
            redemption_rate: 0, created_at: 100, settled_at: None,
        };
        let a: OutcomeSettlementAccount = p.into();
        assert_eq!(a.status, AccountStatus::Open);
        assert_eq!(a.settled_at, None);
    }

    #[test]
    fn balance_from_proto() {
        let p = proto::Balance { shares: 500, last_claimed: Some(200) };
        let b: Balance = p.into();
        assert_eq!(b.shares, 500);
        assert_eq!(b.last_claimed, Some(200));
    }
}
