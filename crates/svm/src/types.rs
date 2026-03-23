//! Domain types for SVM cross-chain operations.

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use thiserror::Error;

/// Result of a Hyperlane Warp Route `transfer_remote` dispatch on Solana.
#[derive(Clone, Debug)]
pub struct DispatchResult {
    /// Hyperlane message ID (keccak256 of the serialized message).
    pub message_id: [u8; 32],
    /// Destination Hyperlane domain.
    pub destination: u32,
    /// Recipient address on the destination chain (32-byte canonical form).
    pub recipient: [u8; 32],
    /// Amount transferred (in token smallest unit).
    pub amount: u64,
    /// Solana transaction signature.
    pub signature: Signature,
    /// Dispatched message PDA address (for reading the full message bytes).
    pub message_storage_pda: Pubkey,
}

/// Result of an x402 `pay` instruction on Solana.
#[derive(Clone, Debug)]
pub struct PaymentResult {
    /// Unique payment identifier (32 bytes).
    pub payment_id: [u8; 32],
    /// Solana transaction signature.
    pub signature: Signature,
    /// Amount escrowed (in SPL token smallest unit).
    pub amount: u64,
    /// Escrow PDA holding the funds.
    pub escrow: Pubkey,
}

/// On-chain payment record fields.
#[derive(Clone, Debug)]
pub struct PaymentInfo {
    pub payer: Pubkey,
    pub target_agent_id: [u8; 32],
    pub amount: u64,
    pub asset_mint: Pubkey,
    pub reply_channel: String,
    pub created_at: i64,
    pub settled: bool,
    pub refunded: bool,
}

/// Errors from SVM operations.
#[derive(Error, Debug)]
pub enum SvmError {
    #[error("RPC error: {0}")]
    Rpc(String),

    #[error("instruction failed: {0}")]
    InstructionFailed(String),

    #[error("transaction failed: {0}")]
    TransactionFailed(String),

    #[error("signing error: {0}")]
    Signing(String),

    #[error("configuration error: {0}")]
    Config(String),

    #[error("account not found: {0}")]
    AccountNotFound(String),

    #[error("deserialization error: {0}")]
    Deserialization(String),

    #[error("address parse error: {0}")]
    AddressParse(String),
}

impl From<SvmError> for morpheum_sdk_core::SdkError {
    fn from(e: SvmError) -> Self {
        Self::Other(e.to_string())
    }
}
