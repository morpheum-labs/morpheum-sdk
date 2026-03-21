//! Error types for CCTP SDK operations.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CctpError {
    #[error("query failed: {0}")]
    Query(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("deserialization error: {0}")]
    Deserialization(String),
}

impl From<CctpError> for morpheum_sdk_core::SdkError {
    fn from(e: CctpError) -> Self {
        Self::Other(e.to_string().into())
    }
}
