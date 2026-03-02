//! Fluent builders for the Auth module.
//!
//! This module provides ergonomic, type-safe fluent builders for all major
//! auth operations (TradingKey approval/revocation, parameter updates).
//! Each builder follows the classic Builder pattern and returns the
//! corresponding request type from `requests.rs` for seamless integration
//! with `TxBuilder`.

use alloc::{string::String, vec::Vec};

use morpheum_sdk_core::{AccountId, SdkError};

use crate::requests::{
    ApproveTradingKeyRequest,
    RevokeTradingKeyRequest,
    UpdateParamsRequest,
};
use crate::types::Params;

/// Fluent builder for approving a Trading Key (delegated session key).
///
/// This is the primary mechanism for secure, high-frequency agent trading
/// with isolated nonce sub-ranges.
///
/// # Example
/// ```rust,ignore
/// let request = ApproveTradingKeyBuilder::new()
///     .owner(signer.account_id())
///     .trading_key(agent_account_id)
///     .expiry(1_800_000_000)
///     .owner_signature(sig_bytes)
///     .reason("Delegate for high-freq trading")
///     .build()?;
///
/// let any = request.to_any();
/// ```
#[derive(Default)]
pub struct ApproveTradingKeyBuilder {
    owner: Option<AccountId>,
    trading_key: Option<AccountId>,
    expiry_timestamp: Option<u64>,
    owner_signature: Option<Vec<u8>>,
    reason: Option<String>,
}

impl ApproveTradingKeyBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the owner address (the principal delegating authority).
    ///
    /// Accepts any type that converts into `AccountId`, including
    /// `morpheum_signing_core::types::AccountId` from a `Signer`.
    pub fn owner(mut self, owner: impl Into<AccountId>) -> Self {
        self.owner = Some(owner.into());
        self
    }

    /// Sets the trading key address (the agent receiving delegated authority).
    ///
    /// Accepts any type that converts into `AccountId`.
    pub fn trading_key(mut self, trading_key: impl Into<AccountId>) -> Self {
        self.trading_key = Some(trading_key.into());
        self
    }

    /// Sets the expiry timestamp for the trading key approval.
    pub fn expiry(mut self, timestamp: u64) -> Self {
        self.expiry_timestamp = Some(timestamp);
        self
    }

    /// Sets the owner's signature authorizing this delegation.
    pub fn owner_signature(mut self, signature: Vec<u8>) -> Self {
        self.owner_signature = Some(signature);
        self
    }

    /// Sets an optional reason for the approval (for auditability).
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Builds the approve request, performing validation.
    pub fn build(self) -> Result<ApproveTradingKeyRequest, SdkError> {
        let owner = self.owner.ok_or_else(|| {
            SdkError::invalid_input("owner is required for TradingKey approval")
        })?;

        let trading_key = self.trading_key.ok_or_else(|| {
            SdkError::invalid_input("trading_key is required for TradingKey approval")
        })?;

        let expiry_timestamp = self.expiry_timestamp.ok_or_else(|| {
            SdkError::invalid_input("expiry_timestamp is required for TradingKey approval")
        })?;

        let owner_signature = self.owner_signature.ok_or_else(|| {
            SdkError::invalid_input("owner_signature is required for TradingKey approval")
        })?;

        let mut req = ApproveTradingKeyRequest::new(
            owner,
            trading_key,
            expiry_timestamp,
            owner_signature,
        );

        if let Some(reason) = self.reason {
            req = req.with_reason(reason);
        }

        Ok(req)
    }
}

/// Fluent builder for revoking a previously approved Trading Key.
///
/// # Example
/// ```rust,ignore
/// let request = RevokeTradingKeyBuilder::new()
///     .owner(signer.account_id())
///     .trading_key(agent_account_id)
///     .owner_signature(sig_bytes)
///     .reason("Agent compromised")
///     .build()?;
/// ```
#[derive(Default)]
pub struct RevokeTradingKeyBuilder {
    owner: Option<AccountId>,
    trading_key: Option<AccountId>,
    owner_signature: Option<Vec<u8>>,
    reason: Option<String>,
}

impl RevokeTradingKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the owner address.
    ///
    /// Accepts any type that converts into `AccountId`.
    pub fn owner(mut self, owner: impl Into<AccountId>) -> Self {
        self.owner = Some(owner.into());
        self
    }

    /// Sets the trading key address to revoke.
    ///
    /// Accepts any type that converts into `AccountId`.
    pub fn trading_key(mut self, trading_key: impl Into<AccountId>) -> Self {
        self.trading_key = Some(trading_key.into());
        self
    }

    /// Sets the owner's signature authorizing this revocation.
    pub fn owner_signature(mut self, signature: Vec<u8>) -> Self {
        self.owner_signature = Some(signature);
        self
    }

    /// Sets an optional reason for the revocation.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Builds the revoke request, performing validation.
    pub fn build(self) -> Result<RevokeTradingKeyRequest, SdkError> {
        let owner = self.owner.ok_or_else(|| {
            SdkError::invalid_input("owner is required for TradingKey revocation")
        })?;

        let trading_key = self.trading_key.ok_or_else(|| {
            SdkError::invalid_input("trading_key is required for TradingKey revocation")
        })?;

        let owner_signature = self.owner_signature.ok_or_else(|| {
            SdkError::invalid_input("owner_signature is required for TradingKey revocation")
        })?;

        let mut req = RevokeTradingKeyRequest::new(owner, trading_key, owner_signature);

        if let Some(reason) = self.reason {
            req = req.with_reason(reason);
        }

        Ok(req)
    }
}

/// Fluent builder for updating auth module parameters via governance.
///
/// Uses [`Params::default()`] if no explicit params are provided, so callers
/// can override only the fields they care about.
///
/// # Example
/// ```rust,ignore
/// let request = UpdateParamsBuilder::new()
///     .authority(governance_account_id)
///     .params(Params {
///         mana_threshold: 100,
///         ..Default::default()
///     })
///     .build()?;
/// ```
#[derive(Default)]
pub struct UpdateParamsBuilder {
    authority: Option<AccountId>,
    params: Option<Params>,
}

impl UpdateParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the governance authority address.
    ///
    /// Accepts any type that converts into `AccountId`.
    pub fn authority(mut self, authority: impl Into<AccountId>) -> Self {
        self.authority = Some(authority.into());
        self
    }

    /// Sets the new module parameters.
    pub fn params(mut self, params: Params) -> Self {
        self.params = Some(params);
        self
    }

    /// Builds the update params request, performing validation.
    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        let authority = self.authority.ok_or_else(|| {
            SdkError::invalid_input("authority is required for UpdateParams")
        })?;

        let params = self.params.ok_or_else(|| {
            SdkError::invalid_input("params are required for UpdateParams")
        })?;

        Ok(UpdateParamsRequest::new(authority, params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use morpheum_sdk_core::AccountId;

    #[test]
    fn approve_builder_full_flow() {
        let owner = AccountId::new([1u8; 32]);
        let trading_key = AccountId::new([2u8; 32]);

        let request = ApproveTradingKeyBuilder::new()
            .owner(owner.clone())
            .trading_key(trading_key.clone())
            .expiry(1_800_000_000)
            .owner_signature(vec![0u8; 64])
            .reason("Delegate for high-freq trading")
            .build()
            .unwrap();

        assert_eq!(request.owner_address, owner);
        assert_eq!(request.trading_key_address, trading_key);
        assert_eq!(request.expiry_timestamp, 1_800_000_000);
        assert_eq!(request.reason, Some("Delegate for high-freq trading".into()));
    }

    #[test]
    fn approve_builder_validation() {
        let result = ApproveTradingKeyBuilder::new().build();
        assert!(result.is_err());
    }

    #[test]
    fn revoke_builder_works() {
        let owner = AccountId::new([1u8; 32]);

        let request = RevokeTradingKeyBuilder::new()
            .owner(owner.clone())
            .trading_key(AccountId::new([2u8; 32]))
            .owner_signature(vec![0u8; 64])
            .reason("Agent compromised")
            .build()
            .unwrap();

        assert_eq!(request.owner_address, owner);
        assert_eq!(request.reason, Some("Agent compromised".into()));
    }

    #[test]
    fn update_params_builder_works() {
        let authority = AccountId::new([3u8; 32]);

        let request = UpdateParamsBuilder::new()
            .authority(authority.clone())
            .params(Params {
                mana_threshold: 100,
                ..Default::default()
            })
            .build()
            .unwrap();

        assert_eq!(request.authority, authority);
        assert_eq!(request.params.mana_threshold, 100);
        assert_eq!(request.params.max_memo_characters, 256); // default
    }
}
