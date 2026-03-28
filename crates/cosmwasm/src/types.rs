//! Domain types for CosmWasm SDK operations.

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::string::String;

use thiserror::Error;

/// Metadata about a deployed CosmWasm contract.
#[derive(Clone, Debug)]
pub struct ContractInfo {
    /// The contract's Morpheum address (bech32 `morm1...`).
    pub address: String,
    /// The code ID of the WASM module backing this contract.
    pub code_id: u64,
    /// The contract's admin address (if set).
    pub admin: Option<String>,
    /// The contract's label (human-readable name set at instantiation).
    pub label: String,
}

/// Raw key-value entry from contract storage.
#[derive(Clone, Debug)]
pub struct RawStateEntry {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

/// Errors from CosmWasm SDK operations.
#[derive(Error, Debug)]
pub enum CosmWasmError {
    #[error("contract query failed: {0}")]
    QueryFailed(String),

    #[error("contract execution failed: {0}")]
    ExecutionFailed(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("deserialization error: {0}")]
    Deserialization(String),

    #[error("transport error: {0}")]
    Transport(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),
}

impl From<CosmWasmError> for morpheum_sdk_core::SdkError {
    fn from(e: CosmWasmError) -> Self {
        Self::Other(e.to_string())
    }
}
