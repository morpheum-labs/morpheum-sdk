//! Fluent builders for the Agent Registry module.
//!
//! Each builder follows the classic Builder pattern with validation and returns
//! the corresponding request type from `requests.rs` for seamless integration
//! with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::TriggerProtocolSyncRequest;

/// Fluent builder for triggering a manual protocol sync.
///
/// # Example
/// ```rust,ignore
/// let request = TriggerProtocolSyncBuilder::new()
///     .authority("morpheum1gov")
///     .agent_hash(hash_bytes.to_vec())
///     .protocol("erc8004")
///     .protocol("a2a")
///     .build()?;
///
/// let any = request.to_any();
/// ```
#[derive(Default)]
pub struct TriggerProtocolSyncBuilder {
    authority: Option<String>,
    agent_hash: Option<Vec<u8>>,
    protocols: Vec<String>,
}

impl TriggerProtocolSyncBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the governance authority address.
    pub fn authority(mut self, authority: impl Into<String>) -> Self {
        self.authority = Some(authority.into());
        self
    }

    /// Sets the agent hash (32-byte SHA-256).
    pub fn agent_hash(mut self, hash: Vec<u8>) -> Self {
        self.agent_hash = Some(hash);
        self
    }

    /// Adds a single protocol to the sync list.
    pub fn protocol(mut self, protocol: impl Into<String>) -> Self {
        self.protocols.push(protocol.into());
        self
    }

    /// Sets all protocols to sync at once (replaces any previously added).
    pub fn protocols(mut self, protocols: Vec<String>) -> Self {
        self.protocols = protocols;
        self
    }

    /// Builds the trigger-protocol-sync request, performing validation.
    pub fn build(self) -> Result<TriggerProtocolSyncRequest, SdkError> {
        let authority = self.authority.ok_or_else(|| {
            SdkError::invalid_input("authority is required for TriggerProtocolSync")
        })?;

        let agent_hash = self.agent_hash.ok_or_else(|| {
            SdkError::invalid_input("agent_hash is required for TriggerProtocolSync")
        })?;

        Ok(TriggerProtocolSyncRequest::new(authority, agent_hash, self.protocols))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn trigger_sync_builder_full_flow() {
        let request = TriggerProtocolSyncBuilder::new()
            .authority("morpheum1gov")
            .agent_hash(vec![0xAA; 32])
            .protocol("erc8004")
            .protocol("a2a")
            .build()
            .unwrap();

        assert_eq!(request.authority, "morpheum1gov");
        assert_eq!(request.agent_hash, vec![0xAA; 32]);
        assert_eq!(request.protocols, vec!["erc8004", "a2a"]);
    }

    #[test]
    fn trigger_sync_builder_protocols_replaces() {
        let request = TriggerProtocolSyncBuilder::new()
            .authority("morpheum1gov")
            .agent_hash(vec![0xBB; 32])
            .protocol("should_be_replaced")
            .protocols(vec!["mcp".into(), "did".into()])
            .build()
            .unwrap();

        assert_eq!(request.protocols, vec!["mcp", "did"]);
    }

    #[test]
    fn trigger_sync_builder_empty_protocols_allowed() {
        let request = TriggerProtocolSyncBuilder::new()
            .authority("morpheum1gov")
            .agent_hash(vec![0xCC; 32])
            .build()
            .unwrap();

        assert!(request.protocols.is_empty());
    }

    #[test]
    fn trigger_sync_builder_validation() {
        let result = TriggerProtocolSyncBuilder::new().build();
        assert!(result.is_err());

        let result = TriggerProtocolSyncBuilder::new()
            .authority("morpheum1gov")
            .build();
        assert!(result.is_err());
    }

}
