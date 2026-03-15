//! Inference Registry module for the Morpheum SDK.
//!
//! The inference_registry is a tiny, high-leverage MormModule that makes
//! inference a first-class, on-chain, provable primitive. It maintains a
//! governance-controlled registry of quantized open models (Llama-3.1,
//! Qwen2.5, DeepSeek-R1, etc.) with zk commitments, supported-ops bitflags,
//! and versioned metadata.
//!
//! Transactions:
//! - `RegisterModel` — register a new model (governance only)
//!
//! Queries:
//! - `QueryModel` — single model lookup by model_id
//! - `QueryModelsByQuant` — filter by quantization format
//! - `QueryActiveModels` — enumerate all active models
//! - `QueryParams` — current module parameters
//!
//! # Quick Start
//! ```rust,ignore
//! use morpheum_sdk_inference_registry::prelude::*;
//!
//! let request = RegisterModelBuilder::new()
//!     .authority("morpheum1gov")
//!     .display_name("Llama-3.1-8B-Q4")
//!     .quant_format(QuantFormat::Q4KM)
//!     .param_count(8)
//!     .zk_commitment(commitment_hash.to_vec())
//!     .supported_ops(ops::INFER | ops::EMBED)
//!     .version(1)
//!     .build()?;
//!
//! let any = request.to_any();
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod builder;
pub mod client;
pub mod types;
pub mod requests;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::InferenceRegistryClient;

pub use builder::RegisterModelBuilder;

pub use types::{
    ModelCommitment,
    ModelStatus,
    Params,
    QuantFormat,
    ops,
};

pub use requests::*;

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the Inference Registry module.
pub mod prelude {
    pub use super::{
        InferenceRegistryClient,
        RegisterModelBuilder,
        ModelCommitment,
        ModelStatus,
        Params,
        QuantFormat,
        ops,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the Inference Registry module (synchronized with workspace version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn version_is_set() {
        assert!(!super::VERSION.is_empty());
    }

    #[test]
    fn ops_are_distinct() {
        use super::ops::*;
        assert_ne!(INFER, EMBED);
        assert_ne!(EMBED, VECTOR_SEARCH);
        assert_ne!(VECTOR_SEARCH, FINE_TUNE);
        assert_eq!(INFER | EMBED, 0b11);
    }
}
