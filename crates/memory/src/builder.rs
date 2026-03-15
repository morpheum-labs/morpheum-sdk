//! Fluent builders for the Memory module.
//!
//! This module provides ergonomic, type-safe fluent builders for all memory
//! transaction operations (store, update, delete, parameter updates). Each
//! builder follows the classic Builder pattern and returns the corresponding
//! request type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    DeleteEntryRequest, StoreEntryRequest, UpdateEntryRequest,
};
use crate::types::MemoryEntryType;

/// Fluent builder for storing a new memory entry.
///
/// # Example
/// ```rust,ignore
/// let request = StoreEntryBuilder::new()
///     .agent_hash("agent-abc")
///     .key("strategy/v1")
///     .value(serialized_bytes)
///     .entry_type(MemoryEntryType::Semantic)
///     .expires_at(1_700_003_600)
///     .owner_signature(sig_bytes)
///     .build()?;
///
/// let any = request.to_any();
/// ```
#[derive(Default)]
pub struct StoreEntryBuilder {
    agent_hash: Option<String>,
    key: Option<String>,
    value: Option<Vec<u8>>,
    entry_type: Option<MemoryEntryType>,
    expires_at: Option<u64>,
    owner_signature: Option<Vec<u8>>,
}

impl StoreEntryBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the agent hash (SHA-256 of the agent's DID).
    pub fn agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.agent_hash = Some(hash.into());
        self
    }

    /// Sets the entry key (unique within the agent's namespace).
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// Sets the raw value bytes.
    pub fn value(mut self, value: Vec<u8>) -> Self {
        self.value = Some(value);
        self
    }

    /// Sets the entry type classification.
    pub fn entry_type(mut self, entry_type: MemoryEntryType) -> Self {
        self.entry_type = Some(entry_type);
        self
    }

    /// Sets the expiry timestamp (0 = never expires).
    pub fn expires_at(mut self, ts: u64) -> Self {
        self.expires_at = Some(ts);
        self
    }

    /// Sets the owner signature authorising this store.
    pub fn owner_signature(mut self, sig: Vec<u8>) -> Self {
        self.owner_signature = Some(sig);
        self
    }

    /// Builds the store request, performing validation.
    pub fn build(self) -> Result<StoreEntryRequest, SdkError> {
        let agent_hash = self.agent_hash.ok_or_else(|| {
            SdkError::invalid_input("agent_hash is required for StoreEntry")
        })?;

        let key = self.key.ok_or_else(|| {
            SdkError::invalid_input("key is required for StoreEntry")
        })?;

        let value = self.value.ok_or_else(|| {
            SdkError::invalid_input("value is required for StoreEntry")
        })?;

        let entry_type = self.entry_type.ok_or_else(|| {
            SdkError::invalid_input("entry_type is required for StoreEntry")
        })?;

        let owner_signature = self.owner_signature.ok_or_else(|| {
            SdkError::invalid_input("owner_signature is required for StoreEntry")
        })?;

        let mut req = StoreEntryRequest::new(agent_hash, key, value, entry_type, owner_signature);

        if let Some(expires_at) = self.expires_at {
            req = req.with_expires_at(expires_at);
        }

        Ok(req)
    }
}

/// Fluent builder for updating an existing memory entry.
///
/// # Example
/// ```rust,ignore
/// let request = UpdateEntryBuilder::new()
///     .agent_hash("agent-abc")
///     .key("strategy/v1")
///     .new_value(updated_bytes)
///     .new_expires_at(1_700_010_000)
///     .owner_signature(sig_bytes)
///     .build()?;
/// ```
#[derive(Default)]
pub struct UpdateEntryBuilder {
    agent_hash: Option<String>,
    key: Option<String>,
    new_value: Option<Vec<u8>>,
    new_expires_at: Option<u64>,
    owner_signature: Option<Vec<u8>>,
}

impl UpdateEntryBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the agent hash.
    pub fn agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.agent_hash = Some(hash.into());
        self
    }

    /// Sets the key of the entry to update.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// Sets the new value bytes.
    pub fn new_value(mut self, value: Vec<u8>) -> Self {
        self.new_value = Some(value);
        self
    }

    /// Sets the new expiry timestamp (0 = never expires).
    pub fn new_expires_at(mut self, ts: u64) -> Self {
        self.new_expires_at = Some(ts);
        self
    }

    /// Sets the owner signature authorising this update.
    pub fn owner_signature(mut self, sig: Vec<u8>) -> Self {
        self.owner_signature = Some(sig);
        self
    }

    /// Builds the update request, performing validation.
    pub fn build(self) -> Result<UpdateEntryRequest, SdkError> {
        let agent_hash = self.agent_hash.ok_or_else(|| {
            SdkError::invalid_input("agent_hash is required for UpdateEntry")
        })?;

        let key = self.key.ok_or_else(|| {
            SdkError::invalid_input("key is required for UpdateEntry")
        })?;

        let new_value = self.new_value.ok_or_else(|| {
            SdkError::invalid_input("new_value is required for UpdateEntry")
        })?;

        let owner_signature = self.owner_signature.ok_or_else(|| {
            SdkError::invalid_input("owner_signature is required for UpdateEntry")
        })?;

        let mut req = UpdateEntryRequest::new(agent_hash, key, new_value, owner_signature);

        if let Some(expires_at) = self.new_expires_at {
            req = req.with_new_expires_at(expires_at);
        }

        Ok(req)
    }
}

/// Fluent builder for deleting a memory entry.
///
/// # Example
/// ```rust,ignore
/// let request = DeleteEntryBuilder::new()
///     .agent_hash("agent-abc")
///     .key("strategy/v1")
///     .owner_signature(sig_bytes)
///     .build()?;
/// ```
#[derive(Default)]
pub struct DeleteEntryBuilder {
    agent_hash: Option<String>,
    key: Option<String>,
    owner_signature: Option<Vec<u8>>,
}

impl DeleteEntryBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the agent hash.
    pub fn agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.agent_hash = Some(hash.into());
        self
    }

    /// Sets the key of the entry to delete.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// Sets the owner signature authorising this deletion.
    pub fn owner_signature(mut self, sig: Vec<u8>) -> Self {
        self.owner_signature = Some(sig);
        self
    }

    /// Builds the delete request, performing validation.
    pub fn build(self) -> Result<DeleteEntryRequest, SdkError> {
        let agent_hash = self.agent_hash.ok_or_else(|| {
            SdkError::invalid_input("agent_hash is required for DeleteEntry")
        })?;

        let key = self.key.ok_or_else(|| {
            SdkError::invalid_input("key is required for DeleteEntry")
        })?;

        let owner_signature = self.owner_signature.ok_or_else(|| {
            SdkError::invalid_input("owner_signature is required for DeleteEntry")
        })?;

        Ok(DeleteEntryRequest::new(agent_hash, key, owner_signature))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn store_entry_builder_full_flow() {
        let request = StoreEntryBuilder::new()
            .agent_hash("agent-abc")
            .key("strategy/v1")
            .value(vec![1, 2, 3])
            .entry_type(MemoryEntryType::Semantic)
            .expires_at(1_700_003_600)
            .owner_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.agent_hash, "agent-abc");
        assert_eq!(request.key, "strategy/v1");
        assert_eq!(request.entry_type, MemoryEntryType::Semantic);
        assert_eq!(request.expires_at, 1_700_003_600);
    }

    #[test]
    fn store_entry_builder_defaults() {
        let request = StoreEntryBuilder::new()
            .agent_hash("agent-abc")
            .key("test")
            .value(vec![1])
            .entry_type(MemoryEntryType::Episodic)
            .owner_signature(vec![0u8; 64])
            .build()
            .unwrap();

        // expires_at defaults to 0 when not set
        assert_eq!(request.expires_at, 0);
    }

    #[test]
    fn store_entry_builder_validation() {
        // Missing all fields
        let result = StoreEntryBuilder::new().build();
        assert!(result.is_err());

        // Missing value
        let result = StoreEntryBuilder::new()
            .agent_hash("agent-abc")
            .key("test")
            .entry_type(MemoryEntryType::Episodic)
            .owner_signature(vec![0u8; 64])
            .build();
        assert!(result.is_err());

        // Missing entry_type
        let result = StoreEntryBuilder::new()
            .agent_hash("agent-abc")
            .key("test")
            .value(vec![1])
            .owner_signature(vec![0u8; 64])
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn update_entry_builder_full_flow() {
        let request = UpdateEntryBuilder::new()
            .agent_hash("agent-abc")
            .key("strategy/v1")
            .new_value(vec![4, 5, 6])
            .new_expires_at(1_700_010_000)
            .owner_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.agent_hash, "agent-abc");
        assert_eq!(request.key, "strategy/v1");
        assert_eq!(request.new_value, vec![4, 5, 6]);
        assert_eq!(request.new_expires_at, 1_700_010_000);
    }

    #[test]
    fn update_entry_builder_defaults() {
        let request = UpdateEntryBuilder::new()
            .agent_hash("agent-abc")
            .key("test")
            .new_value(vec![1])
            .owner_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.new_expires_at, 0);
    }

    #[test]
    fn update_entry_builder_validation() {
        let result = UpdateEntryBuilder::new().build();
        assert!(result.is_err());

        let result = UpdateEntryBuilder::new()
            .agent_hash("agent-abc")
            .key("test")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn delete_entry_builder_full_flow() {
        let request = DeleteEntryBuilder::new()
            .agent_hash("agent-abc")
            .key("strategy/v1")
            .owner_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.agent_hash, "agent-abc");
        assert_eq!(request.key, "strategy/v1");
    }

    #[test]
    fn delete_entry_builder_validation() {
        let result = DeleteEntryBuilder::new().build();
        assert!(result.is_err());

        let result = DeleteEntryBuilder::new()
            .agent_hash("agent-abc")
            .build();
        assert!(result.is_err());

        let result = DeleteEntryBuilder::new()
            .agent_hash("agent-abc")
            .key("test")
            .build();
        assert!(result.is_err());
    }

}
