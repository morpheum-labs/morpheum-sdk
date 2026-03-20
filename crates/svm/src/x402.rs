//! x402 settlement helpers for Solana.
//!
//! High-level functions for interacting with the x402 Settlement Anchor program.

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::contracts;
use crate::provider::SvmProvider;
use crate::types::{PaymentInfo, PaymentResult, SvmError};

/// Executes an x402 payment via the self-relay path.
///
/// Transfers SPL tokens into an escrow PDA and emits a `PaymentEscrowed` event.
/// The off-chain relay picks up the event and settles on Morpheum.
pub fn pay_x402(
    provider: &SvmProvider,
    mint: &Pubkey,
    payment_id: [u8; 32],
    target_agent_id: [u8; 32],
    amount: u64,
    reply_channel: &str,
) -> Result<PaymentResult, SvmError> {
    let payer = provider.keypair().pubkey();
    let payer_ata = contracts::associated_token_address(&payer, mint);

    let ix = contracts::build_pay_instruction(
        &payer,
        &payer_ata,
        mint,
        payment_id,
        target_agent_id,
        amount,
        reply_channel,
    );

    let recent_blockhash = provider
        .client()
        .get_latest_blockhash()
        .map_err(|e| SvmError::Rpc(format!("get_latest_blockhash: {e}")))?;

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer),
        &[provider.keypair()],
        recent_blockhash,
    );

    let signature = provider
        .client()
        .send_and_confirm_transaction(&tx)
        .map_err(|e| SvmError::TransactionFailed(format!("pay: {e}")))?;

    let (escrow, _) = contracts::escrow_pda(&payment_id);

    Ok(PaymentResult {
        payment_id,
        signature,
        amount,
        escrow,
    })
}

/// Executes an x402 payment via Hyperlane GMP.
///
/// Same as [`pay_x402`] but dispatches the settlement message through the
/// Hyperlane Sealevel mailbox program instead of relying on off-chain relay.
pub fn pay_x402_via_hyperlane(
    provider: &SvmProvider,
    mint: &Pubkey,
    hyperlane_mailbox: &Pubkey,
    payment_id: [u8; 32],
    target_agent_id: [u8; 32],
    amount: u64,
) -> Result<PaymentResult, SvmError> {
    let payer = provider.keypair().pubkey();
    let payer_ata = contracts::associated_token_address(&payer, mint);

    let ix = contracts::build_pay_via_hyperlane_instruction(
        &payer,
        &payer_ata,
        mint,
        hyperlane_mailbox,
        payment_id,
        target_agent_id,
        amount,
    );

    let recent_blockhash = provider
        .client()
        .get_latest_blockhash()
        .map_err(|e| SvmError::Rpc(format!("get_latest_blockhash: {e}")))?;

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer),
        &[provider.keypair()],
        recent_blockhash,
    );

    let signature = provider
        .client()
        .send_and_confirm_transaction(&tx)
        .map_err(|e| SvmError::TransactionFailed(format!("pay_via_hyperlane: {e}")))?;

    let (escrow, _) = contracts::escrow_pda(&payment_id);

    Ok(PaymentResult {
        payment_id,
        signature,
        amount,
        escrow,
    })
}

/// Queries the on-chain payment record for a given payment ID.
pub fn get_payment(
    provider: &SvmProvider,
    payment_id: &[u8; 32],
) -> Result<PaymentInfo, SvmError> {
    let (pda, _) = contracts::payment_pda(payment_id);

    let account = provider
        .client()
        .get_account(&pda)
        .map_err(|e| SvmError::AccountNotFound(format!("payment {pda}: {e}")))?;

    deserialize_payment_record(&account.data)
}

/// Deserializes an Anchor `PaymentRecord` from account data.
///
/// Anchor accounts have an 8-byte discriminator prefix, then the Borsh-serialized fields.
fn deserialize_payment_record(data: &[u8]) -> Result<PaymentInfo, SvmError> {
    if data.len() < 8 {
        return Err(SvmError::Deserialization("account data too short".into()));
    }

    let payload = &data[8..];
    let mut offset = 0;

    let _initialized = read_bool(payload, &mut offset)?;
    let payment_id_bytes = read_bytes32(payload, &mut offset)?;
    let _ = payment_id_bytes;
    let payer = read_pubkey(payload, &mut offset)?;
    let target_agent_id = read_bytes32(payload, &mut offset)?;
    let amount = read_u64(payload, &mut offset)?;
    let asset_mint = read_pubkey(payload, &mut offset)?;
    let reply_channel = read_string(payload, &mut offset)?;
    let created_at = read_i64(payload, &mut offset)?;
    let settled = read_bool(payload, &mut offset)?;
    let refunded = read_bool(payload, &mut offset)?;

    Ok(PaymentInfo {
        payer,
        target_agent_id,
        amount,
        asset_mint,
        reply_channel,
        created_at,
        settled,
        refunded,
    })
}

fn read_bool(data: &[u8], offset: &mut usize) -> Result<bool, SvmError> {
    if *offset >= data.len() {
        return Err(SvmError::Deserialization("unexpected end of data".into()));
    }
    let val = data[*offset] != 0;
    *offset += 1;
    Ok(val)
}

fn read_u64(data: &[u8], offset: &mut usize) -> Result<u64, SvmError> {
    if *offset + 8 > data.len() {
        return Err(SvmError::Deserialization("unexpected end of data".into()));
    }
    let val = u64::from_le_bytes(data[*offset..*offset + 8].try_into().unwrap());
    *offset += 8;
    Ok(val)
}

fn read_i64(data: &[u8], offset: &mut usize) -> Result<i64, SvmError> {
    if *offset + 8 > data.len() {
        return Err(SvmError::Deserialization("unexpected end of data".into()));
    }
    let val = i64::from_le_bytes(data[*offset..*offset + 8].try_into().unwrap());
    *offset += 8;
    Ok(val)
}

fn read_bytes32(data: &[u8], offset: &mut usize) -> Result<[u8; 32], SvmError> {
    if *offset + 32 > data.len() {
        return Err(SvmError::Deserialization("unexpected end of data".into()));
    }
    let mut buf = [0u8; 32];
    buf.copy_from_slice(&data[*offset..*offset + 32]);
    *offset += 32;
    Ok(buf)
}

fn read_pubkey(data: &[u8], offset: &mut usize) -> Result<Pubkey, SvmError> {
    let bytes = read_bytes32(data, offset)?;
    Ok(Pubkey::from(bytes))
}

fn read_string(data: &[u8], offset: &mut usize) -> Result<String, SvmError> {
    if *offset + 4 > data.len() {
        return Err(SvmError::Deserialization("unexpected end of data".into()));
    }
    let len = u32::from_le_bytes(data[*offset..*offset + 4].try_into().unwrap()) as usize;
    *offset += 4;
    if *offset + len > data.len() {
        return Err(SvmError::Deserialization("string too long".into()));
    }
    let s = String::from_utf8(data[*offset..*offset + len].to_vec())
        .map_err(|e| SvmError::Deserialization(format!("invalid utf8: {e}")))?;
    *offset += len;
    Ok(s)
}
