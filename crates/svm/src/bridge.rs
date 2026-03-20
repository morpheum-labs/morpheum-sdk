//! Warp Route bridge helpers for Solana.
//!
//! High-level functions for SVM -> Morpheum token bridging via the
//! Hyperlane Sealevel Warp Route program.

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::contracts;
use crate::provider::SvmProvider;
use crate::types::{DispatchResult, SvmError};

/// Creates an associated token account for `wallet` and `mint` if it does not
/// already exist. Returns the ATA address.
pub fn create_ata_if_needed(
    provider: &SvmProvider,
    wallet: &Pubkey,
    mint: &Pubkey,
) -> Result<Pubkey, SvmError> {
    let ata = contracts::associated_token_address(wallet, mint);

    if provider.client().get_account(&ata).is_err() {
        let ix = spl_associated_token_account::instruction::create_associated_token_account(
            &provider.keypair().pubkey(),
            wallet,
            mint,
            &contracts::SPL_TOKEN_PROGRAM_ID,
        );

        let recent_blockhash = provider
            .client()
            .get_latest_blockhash()
            .map_err(|e| SvmError::Rpc(format!("get_latest_blockhash: {e}")))?;

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&provider.keypair().pubkey()),
            &[provider.keypair()],
            recent_blockhash,
        );

        provider
            .client()
            .send_and_confirm_transaction(&tx)
            .map_err(|e| SvmError::TransactionFailed(format!("create ATA: {e}")))?;
    }

    Ok(ata)
}

/// Calls `transfer_remote` on the Hyperlane Sealevel Warp Route program.
///
/// Locks SPL tokens into the collateral program and dispatches a Hyperlane
/// message to the destination domain. The caller must ensure the payer's
/// token account has sufficient balance.
///
/// Returns the dispatch result with the Hyperlane message ID and transaction signature.
pub fn transfer_remote(
    provider: &SvmProvider,
    warp_route_program: &Pubkey,
    mint: &Pubkey,
    destination: u32,
    recipient: [u8; 32],
    amount: u64,
) -> Result<DispatchResult, SvmError> {
    let payer = provider.keypair().pubkey();
    let payer_ata = contracts::associated_token_address(&payer, mint);

    let ix = build_warp_route_transfer_instruction(
        warp_route_program,
        &payer,
        &payer_ata,
        mint,
        destination,
        recipient,
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
        .map_err(|e| SvmError::TransactionFailed(format!("transfer_remote: {e}")))?;

    let message_id = derive_message_id(&signature, destination, &recipient, amount);

    Ok(DispatchResult {
        message_id,
        destination,
        recipient,
        amount,
        signature,
    })
}

/// Returns the SPL token balance for `account` and `mint`.
pub fn balance_of(client: &RpcClient, owner: &Pubkey, mint: &Pubkey) -> Result<u64, SvmError> {
    let ata = contracts::associated_token_address(owner, mint);
    let account = client
        .get_token_account_balance(&ata)
        .map_err(|e| SvmError::AccountNotFound(format!("token account {ata}: {e}")))?;

    account
        .amount
        .parse::<u64>()
        .map_err(|e| SvmError::Deserialization(format!("balance parse: {e}")))
}

/// Returns the signing keypair's SPL token balance.
pub fn my_balance(provider: &SvmProvider, mint: &Pubkey) -> Result<u64, SvmError> {
    balance_of(provider.client(), &provider.address(), mint)
}

fn build_warp_route_transfer_instruction(
    program_id: &Pubkey,
    payer: &Pubkey,
    payer_ata: &Pubkey,
    mint: &Pubkey,
    destination: u32,
    recipient: [u8; 32],
    amount: u64,
) -> solana_sdk::instruction::Instruction {
    use solana_sdk::instruction::AccountMeta;

    let mut data = Vec::with_capacity(4 + 32 + 8);
    data.extend_from_slice(&destination.to_le_bytes());
    data.extend_from_slice(&recipient);
    data.extend_from_slice(&amount.to_le_bytes());

    solana_sdk::instruction::Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(*payer_ata, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new_readonly(contracts::SPL_TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(solana_sdk::system_program::ID, false),
        ],
        data,
    }
}

fn derive_message_id(
    signature: &Signature,
    destination: u32,
    recipient: &[u8; 32],
    amount: u64,
) -> [u8; 32] {
    let mut hasher_input = Vec::with_capacity(64 + 4 + 32 + 8);
    hasher_input.extend_from_slice(signature.as_ref());
    hasher_input.extend_from_slice(&destination.to_le_bytes());
    hasher_input.extend_from_slice(recipient);
    hasher_input.extend_from_slice(&amount.to_le_bytes());
    solana_sdk::hash::hashv(&[&hasher_input]).to_bytes()
}
