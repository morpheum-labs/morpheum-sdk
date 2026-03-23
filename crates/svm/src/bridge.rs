//! Warp Route bridge helpers for Solana.
//!
//! High-level functions for SVM <-> Morpheum token bridging via Hyperlane
//! Sealevel Warp Route programs (both SPL collateral and native variants).

use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::contracts;
use crate::provider::SvmProvider;
use crate::types::{DispatchResult, SvmError};

/// SPL Noop program used by Hyperlane Sealevel programs for logging dispatched messages.
/// This is the Hyperlane-specific noop, not the standard SPL noop.
const SPL_NOOP_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV");

/// 8-byte discriminator used by all Hyperlane Sealevel Token program instructions.
/// See `account_utils::PROGRAM_INSTRUCTION_DISCRIMINATOR` in the hyperlane-monorepo.
const PROGRAM_INSTRUCTION_DISCRIMINATOR: [u8; 8] = [1, 1, 1, 1, 1, 1, 1, 1];

/// Borsh enum variant index for `Instruction::TransferRemote`.
const TRANSFER_REMOTE_VARIANT: u8 = 1;

/// Discriminator stored at the start of dispatched message PDA accounts.
const DISPATCHED_MESSAGE_DISCRIMINATOR: &[u8; 8] = b"DISPATCH";

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

/// Calls `transfer_remote` on the Hyperlane Sealevel Warp Route collateral program.
///
/// Locks SPL tokens into the escrow PDA and dispatches a Hyperlane message to the
/// destination domain. The caller must ensure the payer's token account has
/// sufficient balance.
///
/// Returns the dispatch result containing the real Hyperlane message ID (keccak256
/// of the dispatched message), the Solana transaction signature, and the dispatched
/// message PDA address.
pub fn transfer_remote(
    provider: &SvmProvider,
    warp_route_program: &Pubkey,
    mailbox_program: &Pubkey,
    mint: &Pubkey,
    destination: u32,
    recipient: [u8; 32],
    amount: u64,
) -> Result<DispatchResult, SvmError> {
    let payer = provider.keypair().pubkey();
    let payer_ata = contracts::associated_token_address(&payer, mint);

    let unique_message_keypair = Keypair::new();
    let (message_storage_pda, _) = contracts::mailbox_dispatched_message_pda(
        mailbox_program,
        &unique_message_keypair.pubkey(),
    );

    let ix = build_transfer_remote_instruction(
        warp_route_program,
        mailbox_program,
        &payer,
        &payer_ata,
        mint,
        &unique_message_keypair.pubkey(),
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
        &[provider.keypair(), &unique_message_keypair],
        recent_blockhash,
    );

    let signature = provider
        .client()
        .send_and_confirm_transaction(&tx)
        .map_err(|e| SvmError::TransactionFailed(format!("transfer_remote: {e}")))?;

    let message_id = extract_message_id_from_logs(provider.client(), &signature)
        .unwrap_or_else(|e| {
            tracing::warn!("failed to extract message ID from logs: {e}");
            [0u8; 32]
        });

    Ok(DispatchResult {
        message_id,
        destination,
        recipient,
        amount,
        signature,
        message_storage_pda,
    })
}

/// Returns the SPL token balance for `owner` and `mint`.
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

/// Returns the native SOL (lamport) balance for a given public key.
pub fn native_balance(client: &RpcClient, pubkey: &Pubkey) -> Result<u64, SvmError> {
    client
        .get_balance(pubkey)
        .map_err(|e| SvmError::Rpc(format!("get_balance: {e}")))
}

/// Calls `transfer_remote` on the Hyperlane Sealevel Warp Route **native** program.
///
/// Transfers lamports from the payer into the native collateral PDA and dispatches
/// a Hyperlane message to the destination domain. Uses the same wire format as the
/// SPL collateral variant but with a different account layout (no mint, ATA, or
/// SPL token program — only system transfers).
pub fn transfer_remote_native(
    provider: &SvmProvider,
    warp_route_program: &Pubkey,
    mailbox_program: &Pubkey,
    destination: u32,
    recipient: [u8; 32],
    amount: u64,
) -> Result<DispatchResult, SvmError> {
    let payer = provider.keypair().pubkey();

    let unique_message_keypair = Keypair::new();
    let (message_storage_pda, _) = contracts::mailbox_dispatched_message_pda(
        mailbox_program,
        &unique_message_keypair.pubkey(),
    );

    let ix = build_transfer_remote_native_instruction(
        warp_route_program,
        mailbox_program,
        &payer,
        &unique_message_keypair.pubkey(),
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
        &[provider.keypair(), &unique_message_keypair],
        recent_blockhash,
    );

    let signature = provider
        .client()
        .send_and_confirm_transaction(&tx)
        .map_err(|e| SvmError::TransactionFailed(format!("transfer_remote_native: {e}")))?;

    let message_id = extract_message_id_from_logs(provider.client(), &signature)
        .unwrap_or_else(|e| {
            tracing::warn!("failed to extract message ID from logs: {e}");
            [0u8; 32]
        });

    Ok(DispatchResult {
        message_id,
        destination,
        recipient,
        amount,
        signature,
        message_storage_pda,
    })
}

/// Reads the full Hyperlane message bytes from a dispatched message PDA.
///
/// Account layout (AccountData wrapper adds 1-byte initialized flag):
///   1  byte:  initialized flag
///   8  bytes: discriminator ("DISPATCH")
///   4  bytes: nonce (u32 LE)
///   8  bytes: slot (u64 LE)
///   32 bytes: unique_message_pubkey
///   rest:     encoded Hyperlane message
pub fn read_dispatched_message(
    client: &RpcClient,
    message_pda: &Pubkey,
) -> Result<Vec<u8>, SvmError> {
    let account = client
        .get_account(message_pda)
        .map_err(|e| SvmError::AccountNotFound(format!("dispatched message PDA: {e}")))?;

    let data = &account.data;
    // 1 (init flag) + 8 (DISPATCH) + 4 (nonce) + 8 (slot) + 32 (pubkey)
    const HEADER_LEN: usize = 1 + 8 + 4 + 8 + 32;
    if data.len() < HEADER_LEN + 1 {
        return Err(SvmError::Deserialization(
            "dispatched message account too small".into(),
        ));
    }

    if &data[1..9] != DISPATCHED_MESSAGE_DISCRIMINATOR {
        return Err(SvmError::Deserialization(format!(
            "dispatched message discriminator mismatch: got {:?}",
            &data[1..9]
        )));
    }

    Ok(data[HEADER_LEN..].to_vec())
}

// ── Instruction builder ─────────────────────────────────────────────

/// Builds the `TransferRemote` instruction for the Hyperlane Sealevel
/// Warp Route collateral program.
///
/// Account layout (no IGP):
///   0.  system_program        (executable)
///   1.  spl_noop              (executable)
///   2.  token PDA             (readonly)
///   3.  mailbox program       (executable)
///   4.  mailbox outbox        (writable)
///   5.  dispatch authority     (readonly)
///   6.  payer / token sender  (signer)
///   7.  unique message keypair(signer)
///   8.  message storage PDA   (writable)
///   9.  SPL token program     (executable)
///   10. mint                  (writable)
///   11. payer ATA             (writable)
///   12. escrow PDA            (writable)
#[allow(clippy::too_many_arguments)]
fn build_transfer_remote_instruction(
    warp_route_program: &Pubkey,
    mailbox_program: &Pubkey,
    payer: &Pubkey,
    payer_ata: &Pubkey,
    mint: &Pubkey,
    unique_message_pubkey: &Pubkey,
    destination: u32,
    recipient: [u8; 32],
    amount: u64,
) -> Instruction {
    let (token_pda, _) = contracts::hyperlane_token_pda(warp_route_program);
    let (outbox_pda, _) = contracts::mailbox_outbox_pda(mailbox_program);
    let (dispatch_authority, _) =
        contracts::mailbox_dispatch_authority_pda(warp_route_program);
    let (message_storage_pda, _) =
        contracts::mailbox_dispatched_message_pda(mailbox_program, unique_message_pubkey);
    let (escrow_pda, _) = contracts::warp_route_escrow_pda(warp_route_program);

    let data = encode_transfer_remote(destination, recipient, amount);

    Instruction {
        program_id: *warp_route_program,
        accounts: vec![
            AccountMeta::new_readonly(solana_sdk::system_program::ID, false),
            AccountMeta::new_readonly(SPL_NOOP_PROGRAM_ID, false),
            AccountMeta::new_readonly(token_pda, false),
            AccountMeta::new_readonly(*mailbox_program, false),
            AccountMeta::new(outbox_pda, false),
            AccountMeta::new_readonly(dispatch_authority, false),
            AccountMeta::new_readonly(*payer, true),
            AccountMeta::new_readonly(*unique_message_pubkey, true),
            AccountMeta::new(message_storage_pda, false),
            AccountMeta::new_readonly(contracts::SPL_TOKEN_PROGRAM_ID, false),
            AccountMeta::new(*mint, false),
            AccountMeta::new(*payer_ata, false),
            AccountMeta::new(escrow_pda, false),
        ],
        data,
    }
}

/// Builds the `TransferRemote` instruction for the Hyperlane Sealevel
/// Warp Route **native** program.
///
/// Account layout (no IGP):
///   0.  system_program           (executable)
///   1.  spl_noop                 (executable)
///   2.  token PDA                (readonly)
///   3.  mailbox program          (executable)
///   4.  mailbox outbox           (writable)
///   5.  dispatch authority        (readonly)
///   6.  payer / lamport sender   (signer, writable)
///   7.  unique message keypair   (signer)
///   8.  message storage PDA      (writable)
///   9.  system_program           (plugin: transfer_in)
///   10. native collateral PDA    (writable, plugin: transfer_in)
fn build_transfer_remote_native_instruction(
    warp_route_program: &Pubkey,
    mailbox_program: &Pubkey,
    payer: &Pubkey,
    unique_message_pubkey: &Pubkey,
    destination: u32,
    recipient: [u8; 32],
    amount: u64,
) -> Instruction {
    let (token_pda, _) = contracts::hyperlane_token_pda(warp_route_program);
    let (outbox_pda, _) = contracts::mailbox_outbox_pda(mailbox_program);
    let (dispatch_authority, _) =
        contracts::mailbox_dispatch_authority_pda(warp_route_program);
    let (message_storage_pda, _) =
        contracts::mailbox_dispatched_message_pda(mailbox_program, unique_message_pubkey);
    let (native_collateral_pda, _) =
        contracts::warp_route_native_collateral_pda(warp_route_program);

    let data = encode_transfer_remote(destination, recipient, amount);

    Instruction {
        program_id: *warp_route_program,
        accounts: vec![
            AccountMeta::new_readonly(solana_sdk::system_program::ID, false),
            AccountMeta::new_readonly(SPL_NOOP_PROGRAM_ID, false),
            AccountMeta::new_readonly(token_pda, false),
            AccountMeta::new_readonly(*mailbox_program, false),
            AccountMeta::new(outbox_pda, false),
            AccountMeta::new_readonly(dispatch_authority, false),
            AccountMeta::new(*payer, true),
            AccountMeta::new_readonly(*unique_message_pubkey, true),
            AccountMeta::new(message_storage_pda, false),
            AccountMeta::new_readonly(solana_sdk::system_program::ID, false),
            AccountMeta::new(native_collateral_pda, false),
        ],
        data,
    }
}

/// Encodes the `Instruction::TransferRemote` data with the discriminator prefix.
///
/// Wire format:
///   [1,1,1,1,1,1,1,1]   8 bytes  PROGRAM_INSTRUCTION_DISCRIMINATOR
///   [1]                  1 byte   Borsh enum variant (TransferRemote = 1)
///   destination_domain   4 bytes  u32 LE
///   recipient            32 bytes H256 (raw bytes)
///   amount_or_id         32 bytes U256 (4 × u64 LE, least significant first)
fn encode_transfer_remote(destination: u32, recipient: [u8; 32], amount: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(8 + 1 + 4 + 32 + 32);
    data.extend_from_slice(&PROGRAM_INSTRUCTION_DISCRIMINATOR);
    data.push(TRANSFER_REMOTE_VARIANT);
    data.extend_from_slice(&destination.to_le_bytes());
    data.extend_from_slice(&recipient);
    // U256: 4 × u64 LE limbs, least-significant first
    data.extend_from_slice(&amount.to_le_bytes());
    data.extend_from_slice(&[0u8; 24]);
    data
}

// ── Message extraction ──────────────────────────────────────────────

/// Extracts the Hyperlane message ID from the confirmed transaction log line:
///   "Dispatched message to <domain>, ID 0x<hex>"
fn extract_message_id_from_logs(
    client: &RpcClient,
    signature: &solana_sdk::signature::Signature,
) -> Result<[u8; 32], SvmError> {
    use solana_client::rpc_config::RpcTransactionConfig;
    use solana_transaction_status::UiTransactionEncoding;

    let tx = client
        .get_transaction_with_config(
            signature,
            RpcTransactionConfig {
                encoding: Some(UiTransactionEncoding::Base64),
                commitment: Some(CommitmentConfig::confirmed()),
                ..Default::default()
            },
        )
        .map_err(|e| SvmError::Rpc(format!("get_transaction: {e}")))?;

    let meta = tx.transaction.meta.as_ref().ok_or_else(|| {
        SvmError::Deserialization("no tx metadata".into())
    })?;

    let logs: Vec<String> = match &meta.log_messages {
        solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => logs.clone(),
        _ => Vec::new(),
    };

    for log in &logs {
        if let Some(id_hex) = extract_message_id_from_log_line(log) {
            let bytes = hex::decode(id_hex).map_err(|e| {
                SvmError::Deserialization(format!("bad message ID hex: {e}"))
            })?;
            if bytes.len() == 32 {
                let mut id = [0u8; 32];
                id.copy_from_slice(&bytes);
                return Ok(id);
            }
        }
    }

    Err(SvmError::Deserialization(
        "no Hyperlane message ID found in transaction logs".into(),
    ))
}

/// Parses "Dispatched message to <domain>, ID 0x<hex>" from a log line.
fn extract_message_id_from_log_line(log: &str) -> Option<&str> {
    let marker = "ID 0x";
    let idx = log.find(marker)?;
    let hex_start = idx + marker.len();
    let hex_str = &log[hex_start..];
    if hex_str.len() >= 64 {
        Some(&hex_str[..64])
    } else {
        None
    }
}
