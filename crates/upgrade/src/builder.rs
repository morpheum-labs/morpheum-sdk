//! Fluent builders for the Upgrade module.
//!
//! Ergonomic, type-safe fluent builders for all upgrade transaction operations.
//! Each builder follows the classic Builder pattern and returns the corresponding
//! request type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::{AccountId, SdkError};

use crate::requests::{
    CancelUpgradeRequest, ExecuteUpgradeRequest, SignalUpgradeReadyRequest,
};

// ====================== SIGNAL UPGRADE READY ======================

#[derive(Default)]
pub struct SignalUpgradeReadyBuilder {
    from_address: Option<AccountId>,
    upgrade_id: Option<u64>,
    validator_pubkey: Option<Vec<u8>>,
    signature: Option<Vec<u8>>,
}

impl SignalUpgradeReadyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn upgrade_id(mut self, id: u64) -> Self {
        self.upgrade_id = Some(id);
        self
    }

    pub fn validator_pubkey(mut self, pubkey: Vec<u8>) -> Self {
        self.validator_pubkey = Some(pubkey);
        self
    }

    pub fn signature(mut self, sig: Vec<u8>) -> Self {
        self.signature = Some(sig);
        self
    }

    pub fn build(self) -> Result<SignalUpgradeReadyRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for signaling upgrade readiness")
        })?;
        let upgrade_id = self.upgrade_id.ok_or_else(|| {
            SdkError::invalid_input("upgrade_id is required")
        })?;
        let validator_pubkey = self.validator_pubkey.ok_or_else(|| {
            SdkError::invalid_input("validator_pubkey is required")
        })?;
        let signature = self.signature.ok_or_else(|| {
            SdkError::invalid_input("signature is required (BLS over upgrade_id + binary_hash)")
        })?;

        Ok(SignalUpgradeReadyRequest::new(
            from_address,
            upgrade_id,
            validator_pubkey,
            signature,
        ))
    }
}

// ====================== CANCEL UPGRADE ======================

#[derive(Default)]
pub struct CancelUpgradeBuilder {
    from_address: Option<AccountId>,
    upgrade_id: Option<u64>,
    reason: Option<String>,
}

impl CancelUpgradeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn upgrade_id(mut self, id: u64) -> Self {
        self.upgrade_id = Some(id);
        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn build(self) -> Result<CancelUpgradeRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for upgrade cancellation")
        })?;
        let upgrade_id = self.upgrade_id.ok_or_else(|| {
            SdkError::invalid_input("upgrade_id is required")
        })?;
        let reason = self.reason.ok_or_else(|| {
            SdkError::invalid_input("reason is required for audit purposes")
        })?;

        Ok(CancelUpgradeRequest::new(from_address, upgrade_id, reason))
    }
}

// ====================== EXECUTE UPGRADE ======================

#[derive(Default)]
pub struct ExecuteUpgradeBuilder {
    from_address: Option<AccountId>,
    upgrade_id: Option<u64>,
}

impl ExecuteUpgradeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_address(mut self, address: impl Into<AccountId>) -> Self {
        self.from_address = Some(address.into());
        self
    }

    pub fn upgrade_id(mut self, id: u64) -> Self {
        self.upgrade_id = Some(id);
        self
    }

    pub fn build(self) -> Result<ExecuteUpgradeRequest, SdkError> {
        let from_address = self.from_address.ok_or_else(|| {
            SdkError::invalid_input("from_address is required for upgrade execution")
        })?;
        let upgrade_id = self.upgrade_id.ok_or_else(|| {
            SdkError::invalid_input("upgrade_id is required")
        })?;

        Ok(ExecuteUpgradeRequest::new(from_address, upgrade_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use morpheum_sdk_core::AccountId;

    #[test]
    fn signal_ready_builder_full_flow() {
        let from = AccountId::new([1u8; 32]);

        let request = SignalUpgradeReadyBuilder::new()
            .from_address(from.clone())
            .upgrade_id(42)
            .validator_pubkey(alloc::vec![0xaa, 0xbb, 0xcc])
            .signature(alloc::vec![0x01, 0x02, 0x03])
            .build()
            .unwrap();

        assert_eq!(request.from_address, from);
        assert_eq!(request.upgrade_id, 42);
        assert!(!request.validator_pubkey.is_empty());
        assert!(!request.signature.is_empty());
    }

    #[test]
    fn signal_ready_builder_validation() {
        let result = SignalUpgradeReadyBuilder::new().build();
        assert!(result.is_err());

        let result = SignalUpgradeReadyBuilder::new()
            .from_address(AccountId::new([1u8; 32]))
            .upgrade_id(1)
            .validator_pubkey(alloc::vec![0xaa])
            .build();
        assert!(result.is_err(), "should require signature");
    }

    #[test]
    fn cancel_upgrade_builder_requires_reason() {
        let result = CancelUpgradeBuilder::new()
            .from_address(AccountId::new([2u8; 32]))
            .upgrade_id(5)
            .build();
        assert!(result.is_err(), "reason is required for audit");
    }

    #[test]
    fn cancel_upgrade_builder_works() {
        let request = CancelUpgradeBuilder::new()
            .from_address(AccountId::new([2u8; 32]))
            .upgrade_id(5)
            .reason("critical vulnerability discovered")
            .build()
            .unwrap();

        assert_eq!(request.upgrade_id, 5);
        assert_eq!(request.reason, "critical vulnerability discovered");
    }

    #[test]
    fn execute_upgrade_builder_works() {
        let request = ExecuteUpgradeBuilder::new()
            .from_address(AccountId::new([3u8; 32]))
            .upgrade_id(99)
            .build()
            .unwrap();

        assert_eq!(request.upgrade_id, 99);
    }

    #[test]
    fn execute_upgrade_builder_validation() {
        let result = ExecuteUpgradeBuilder::new().build();
        assert!(result.is_err());
    }
}
