//! CCTP bridge helpers for `CctpHyperlaneWrapper`.
//!
//! High-level functions for EVM -> Morpheum canonical USDC bridging via
//! Circle CCTP + Hyperlane messaging.

use alloy::primitives::{Address, Bytes, FixedBytes, TxHash, U256};
use alloy::sol;
use alloy::sol_types::SolEvent;

use crate::provider::EvmProvider;
use crate::types::EvmError;

sol! {
    #[sol(rpc)]
    interface ICctpHyperlaneWrapper {
        function bridgeUsdc(
            uint256 amount,
            bytes32 recipient,
            bytes calldata hookCalldata
        ) external payable returns (bytes32 messageId);

        function quoteDispatch(
            uint256 amount,
            bytes32 recipient,
            bytes calldata hookCalldata
        ) external view returns (uint256 fee);

        event BridgeInitiated(
            address indexed sender,
            bytes32 indexed recipient,
            uint256 amount,
            uint64 cctpNonce,
            bytes32 hyperlaneMessageId
        );
    }
}

/// Result of a CCTP bridge operation.
#[derive(Clone, Debug)]
pub struct CctpBridgeResult {
    /// EVM transaction hash.
    pub tx_hash: TxHash,
    /// Hyperlane message ID returned by `bridgeUsdc`.
    pub message_id: FixedBytes<32>,
    /// CCTP nonce from the `BridgeInitiated` event (if parsed).
    pub cctp_nonce: Option<u64>,
}

/// Quotes the Hyperlane dispatch fee for a CCTP bridge operation.
pub async fn quote_cctp_dispatch(
    provider: &EvmProvider,
    wrapper: Address,
    amount: U256,
    recipient: FixedBytes<32>,
    calldata: Bytes,
) -> Result<U256, EvmError> {
    let contract = ICctpHyperlaneWrapper::new(wrapper, provider);
    contract
        .quoteDispatch(amount, recipient, calldata)
        .call()
        .await
        .map_err(|e| EvmError::ContractCall(format!("quoteDispatch: {e}")))
}

/// Burns USDC via Circle CCTP and dispatches a Hyperlane message to Morpheum.
///
/// The caller must have already approved the wrapper contract to spend `amount`
/// of USDC via [`crate::approve_erc20`].
pub async fn bridge_usdc(
    provider: &EvmProvider,
    wrapper: Address,
    amount: U256,
    recipient: FixedBytes<32>,
    calldata: Bytes,
    fee: U256,
) -> Result<CctpBridgeResult, EvmError> {
    let contract = ICctpHyperlaneWrapper::new(wrapper, provider);

    let pending = contract
        .bridgeUsdc(amount, recipient, calldata)
        .value(fee)
        .send()
        .await
        .map_err(|e| EvmError::ContractCall(format!("bridgeUsdc send: {e}")))?;

    let receipt = pending
        .get_receipt()
        .await
        .map_err(|e| EvmError::TransactionFailed(format!("bridgeUsdc receipt: {e}")))?;

    let tx_hash = receipt.transaction_hash;

    let bridge_topic = ICctpHyperlaneWrapper::BridgeInitiated::SIGNATURE_HASH;
    let mut message_id = FixedBytes::from(tx_hash);
    let mut cctp_nonce = None;

    for log in receipt.inner.logs() {
        if log.topic0() == Some(&bridge_topic) {
            if let Ok(event) =
                ICctpHyperlaneWrapper::BridgeInitiated::decode_log(&log.inner)
            {
                message_id = event.data.hyperlaneMessageId;
                cctp_nonce = Some(event.data.cctpNonce);
                break;
            }
        }
    }

    Ok(CctpBridgeResult {
        tx_hash,
        message_id,
        cctp_nonce,
    })
}
