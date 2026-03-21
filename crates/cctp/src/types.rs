//! Canonical CCTP types re-exported from `hpl-cctp-handler`.
//!
//! This module provides the single source of truth for all types used in
//! CCTP handler contract interactions. Consumers (the CCTP fulfiller service,
//! E2E tests, CLI tools) import from here instead of redefining their own.

pub use hpl_cctp_handler::state::PendingTransfer;

pub use hpl_cctp_handler::msg::{
    ConfigResponse, ExecuteMsg, HandleMsg, InstantiateMsg, PendingTransferResponse,
    PendingTransfersResponse, PostMintHookMsg, QueryMsg, RouteResponse, RoutesResponse,
};

pub use cosmwasm_std::HexBinary;
