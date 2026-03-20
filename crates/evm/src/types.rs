//! Domain types for EVM cross-chain operations.

use alloy::primitives::{Address, FixedBytes, TxHash, U256};
use thiserror::Error;

/// Result of a Warp Route `transferRemote` dispatch.
#[derive(Clone, Debug)]
pub struct DispatchResult {
    /// Hyperlane message ID (keccak256 of the serialized message).
    pub message_id: FixedBytes<32>,
    /// Destination Hyperlane domain.
    pub destination: u32,
    /// Recipient address on the destination chain (left-padded bytes32).
    pub recipient: FixedBytes<32>,
    /// Amount transferred (in token smallest unit).
    pub amount: U256,
    /// EVM transaction hash of the `transferRemote` call.
    pub tx_hash: TxHash,
}

/// Result of an x402 `pay()` call.
#[derive(Clone, Debug)]
pub struct PaymentResult {
    /// Unique payment identifier.
    pub payment_id: FixedBytes<32>,
    /// Hyperlane message ID (if dispatched via Hyperlane).
    pub message_id: Option<FixedBytes<32>>,
    /// EVM transaction hash.
    pub tx_hash: TxHash,
    /// Amount escrowed.
    pub amount: U256,
}

/// Subset of on-chain PaymentRecord fields.
#[derive(Clone, Debug)]
pub struct PaymentInfo {
    pub payer: Address,
    pub target_agent_id: FixedBytes<32>,
    pub amount: U256,
    pub asset: Address,
    pub reply_channel: String,
    pub created_at: u64,
    pub settled: bool,
    pub refunded: bool,
}

/// Parameters for an x402 payment.
#[derive(Clone, Debug)]
pub struct X402PayParams {
    pub payment_id: FixedBytes<32>,
    pub target_agent_id: FixedBytes<32>,
    pub amount: U256,
    pub memo: String,
    pub reply_channel: String,
    /// Hyperlane dispatch fee (sent as `msg.value`).
    pub msg_value: U256,
}

/// Errors from EVM operations.
#[derive(Error, Debug)]
pub enum EvmError {
    #[error("provider error: {0}")]
    Provider(String),

    #[error("contract call failed: {0}")]
    ContractCall(String),

    #[error("transaction failed: {0}")]
    TransactionFailed(String),

    #[error("signing error: {0}")]
    Signing(String),

    #[error("configuration error: {0}")]
    Config(String),

    #[error("address parse error: {0}")]
    AddressParse(String),
}
