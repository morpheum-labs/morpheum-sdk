//! x402 settlement helpers.
//!
//! High-level functions for interacting with the X402Settlement contract on EVM.

use alloy::primitives::{Address, FixedBytes, U256};
use alloy::providers::Provider;
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signer;

use crate::bridge::parse_dispatch_id;
use crate::contracts::IX402Settlement;
use crate::provider::EvmProvider;
use crate::types::{EvmError, PaymentInfo, PaymentResult, X402PayParams};

/// Executes an x402 payment with EIP-712 signature.
///
/// 1. Fetches the EIP-712 digest from the contract
/// 2. Signs the digest with the provided signer key
/// 3. Calls `pay()` with the signature components (v, r, s)
///
/// Does **not** handle ERC-20 approval -- call [`crate::bridge::approve_erc20`] first.
pub async fn pay_x402(
    provider: &EvmProvider,
    settlement: Address,
    signer_key: &PrivateKeySigner,
    params: &X402PayParams,
) -> Result<PaymentResult, EvmError> {
    let contract = IX402Settlement::new(settlement, provider);

    let block = provider
        .get_block_number()
        .await
        .map_err(|e| EvmError::Provider(format!("get_block_number: {e}")))?;
    let deadline = U256::from(block + 1000);

    let digest = contract
        .getPaymentDigest(
            params.payment_id,
            params.target_agent_id,
            params.amount,
            deadline,
        )
        .call()
        .await
        .map_err(|e| EvmError::ContractCall(format!("getPaymentDigest: {e}")))?;

    let digest_hash = alloy::primitives::B256::from(digest);
    let signature = signer_key
        .sign_hash(&digest_hash)
        .await
        .map_err(|e| EvmError::Signing(format!("EIP-712 sign: {e}")))?;

    let v = if signature.v() { 28u8 } else { 27u8 };
    let r = FixedBytes::from(signature.r().to_be_bytes::<32>());
    let s = FixedBytes::from(signature.s().to_be_bytes::<32>());

    let pending = contract
        .pay(
            params.payment_id,
            params.target_agent_id,
            params.amount,
            params.memo.clone(),
            params.reply_channel.clone(),
            deadline,
            v,
            r,
            s,
        )
        .value(params.msg_value)
        .send()
        .await
        .map_err(|e| EvmError::ContractCall(format!("pay() send: {e}")))?;

    let receipt = pending
        .get_receipt()
        .await
        .map_err(|e| EvmError::TransactionFailed(format!("pay() receipt: {e}")))?;

    let tx_hash = receipt.transaction_hash;
    let message_id = parse_dispatch_id(&receipt);

    Ok(PaymentResult {
        payment_id: params.payment_id,
        message_id,
        tx_hash,
        amount: params.amount,
    })
}

/// Queries the on-chain PaymentRecord for a given payment ID.
pub async fn get_payment(
    provider: &EvmProvider,
    settlement: Address,
    payment_id: FixedBytes<32>,
) -> Result<PaymentInfo, EvmError> {
    let contract = IX402Settlement::new(settlement, provider);

    let record = contract
        .getPayment(payment_id)
        .call()
        .await
        .map_err(|e| EvmError::ContractCall(format!("getPayment: {e}")))?;

    Ok(PaymentInfo {
        payer: record.payer,
        target_agent_id: record.targetAgentId,
        amount: record.amount,
        asset: record.asset,
        reply_channel: record.replyChannel,
        created_at: record.createdAt,
        settled: record.settled,
        refunded: record.refunded,
    })
}

/// Quotes the Hyperlane dispatch fee for a payment.
pub async fn quote_fee(
    provider: &EvmProvider,
    settlement: Address,
    payment_id: FixedBytes<32>,
    target_agent_id: FixedBytes<32>,
    amount: U256,
) -> Result<U256, EvmError> {
    let contract = IX402Settlement::new(settlement, provider);

    contract
        .quoteFee(payment_id, target_agent_id, amount)
        .call()
        .await
        .map_err(|e| EvmError::ContractCall(format!("quoteFee: {e}")))
}
