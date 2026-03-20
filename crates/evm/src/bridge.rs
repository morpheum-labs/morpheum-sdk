//! Warp Route bridge helpers.
//!
//! High-level functions for EVM -> Morpheum token bridging via HypERC20Collateral.

use alloy::primitives::{Address, FixedBytes, TxHash, U256};
use alloy::providers::WalletProvider;
use alloy::rpc::types::TransactionReceipt;
use alloy::sol_types::SolEvent;

use crate::contracts::{IERC20, IHypERC20Collateral, IMailbox};
use crate::provider::EvmProvider;
use crate::types::{DispatchResult, EvmError};

/// Approves `spender` to spend `amount` of `token` on behalf of the wallet.
///
/// Waits for on-chain confirmation before returning.
pub async fn approve_erc20(
    provider: &EvmProvider,
    token: Address,
    spender: Address,
    amount: U256,
) -> Result<TxHash, EvmError> {
    let erc20 = IERC20::new(token, provider);
    let pending = erc20
        .approve(spender, amount)
        .send()
        .await
        .map_err(|e| EvmError::ContractCall(format!("approve send: {e}")))?;

    let receipt = pending
        .get_receipt()
        .await
        .map_err(|e| EvmError::TransactionFailed(format!("approve receipt: {e}")))?;

    Ok(receipt.transaction_hash)
}

/// Calls `transferRemote` on the HypERC20Collateral contract.
///
/// Does **not** handle ERC-20 approval -- call [`approve_erc20`] first.
/// The `msg_value` parameter covers the Hyperlane IGP fee (pass `U256::ZERO`
/// if the route has no required fee).
pub async fn transfer_remote(
    provider: &EvmProvider,
    collateral: Address,
    destination: u32,
    recipient: FixedBytes<32>,
    amount: U256,
    msg_value: U256,
) -> Result<DispatchResult, EvmError> {
    let contract = IHypERC20Collateral::new(collateral, provider);

    let pending = contract
        .transferRemote(destination, recipient, amount)
        .value(msg_value)
        .send()
        .await
        .map_err(|e| EvmError::ContractCall(format!("transferRemote send: {e}")))?;

    let receipt = pending
        .get_receipt()
        .await
        .map_err(|e| EvmError::TransactionFailed(format!("transferRemote receipt: {e}")))?;

    let tx_hash = receipt.transaction_hash;
    let message_id = parse_dispatch_id(&receipt).unwrap_or(FixedBytes::from(tx_hash));

    Ok(DispatchResult {
        message_id,
        destination,
        recipient,
        amount,
        tx_hash,
    })
}

/// Returns the ERC-20 balance of `account` for the given `token`.
pub async fn balance_of(
    provider: &EvmProvider,
    token: Address,
    account: Address,
) -> Result<U256, EvmError> {
    let erc20 = IERC20::new(token, provider);
    erc20
        .balanceOf(account)
        .call()
        .await
        .map_err(|e| EvmError::ContractCall(format!("balanceOf: {e}")))
}

/// Returns the wallet's own ERC-20 balance.
pub async fn my_balance(provider: &EvmProvider, token: Address) -> Result<U256, EvmError> {
    balance_of(provider, token, provider.default_signer_address()).await
}

/// Extracts the Hyperlane message ID from a `DispatchId(bytes32 indexed messageId)` log.
///
/// The Mailbox emits this event on every `dispatch()` call. The `messageId` is
/// `keccak256(message)` -- the canonical identifier used by validators, relayers,
/// and the destination chain for delivery tracking.
pub fn parse_dispatch_id(receipt: &TransactionReceipt) -> Option<FixedBytes<32>> {
    let dispatch_id_topic = IMailbox::DispatchId::SIGNATURE_HASH;

    for log in receipt.inner.logs() {
        if log.topic0() == Some(&dispatch_id_topic) {
            if let Some(message_id) = log.topics().get(1) {
                return Some(*message_id);
            }
        }
    }
    None
}
