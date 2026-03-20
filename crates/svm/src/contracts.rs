//! Program IDs, PDA derivation, and instruction builders for Solana programs
//! used in Morpheum cross-chain operations.
//!
//! Covers the Hyperlane Sealevel Warp Route and the x402 Settlement Anchor program.

use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;

/// Hyperlane Sealevel Warp Route program (collateral variant).
///
/// Deployed from `hyperlane-xyz/hyperlane-monorepo/rust/sealevel/programs/hyperlane-sealevel-token-collateral`.
/// This is a placeholder -- replace with the actual deployed program ID.
pub const HYPERLANE_WARP_ROUTE_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("HypWarpCoLLRt111111111111111111111111111111");

/// x402 Settlement Anchor program (from `contracts/solana/programs/x402-settlement`).
pub const X402_SETTLEMENT_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("95mAeRSnfH8KtP72sNJ5Ks8zSwf91WbhXn7E6HLjKSKZ");

/// SPL Token program ID (canonical).
pub const SPL_TOKEN_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

/// SPL Associated Token Account program ID (canonical).
pub const SPL_ATA_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

/// Derives the x402 settlement state PDA.
pub fn settlement_state_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"state"], &X402_SETTLEMENT_PROGRAM_ID)
}

/// Derives the x402 payment record PDA for a given payment ID.
pub fn payment_pda(payment_id: &[u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"payment", payment_id.as_ref()],
        &X402_SETTLEMENT_PROGRAM_ID,
    )
}

/// Derives the x402 escrow token account PDA for a given payment ID.
pub fn escrow_pda(payment_id: &[u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"escrow", payment_id.as_ref()],
        &X402_SETTLEMENT_PROGRAM_ID,
    )
}

/// Derives the associated token account for a given wallet and mint.
pub fn associated_token_address(wallet: &Pubkey, mint: &Pubkey) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(wallet, mint)
}

/// Builds the Anchor instruction discriminator from a method name.
///
/// Anchor uses `sha256("global:<method_name>")[..8]` as the 8-byte discriminator.
fn anchor_discriminator(method_name: &str) -> [u8; 8] {
    use std::io::Write;
    let input = format!("global:{method_name}");
    let hash = solana_sdk::hash::hashv(&[input.as_bytes()]);
    let mut disc = [0u8; 8];
    (&mut disc[..]).write_all(&hash.as_ref()[..8]).ok();
    disc
}

/// Builds the `pay` instruction for the x402 settlement program.
pub fn build_pay_instruction(
    payer: &Pubkey,
    payer_token_account: &Pubkey,
    mint: &Pubkey,
    payment_id: [u8; 32],
    target_agent_id: [u8; 32],
    amount: u64,
    reply_channel: &str,
) -> Instruction {
    let (payment_pda, _) = payment_pda(&payment_id);
    let (escrow_pda, _) = escrow_pda(&payment_id);
    let (state_pda, _) = settlement_state_pda();

    let mut data = Vec::with_capacity(8 + 32 + 32 + 8 + 4 + reply_channel.len());
    data.extend_from_slice(&anchor_discriminator("pay"));
    data.extend_from_slice(&payment_id);
    data.extend_from_slice(&target_agent_id);
    data.extend_from_slice(&amount.to_le_bytes());
    let reply_len = reply_channel.len() as u32;
    data.extend_from_slice(&reply_len.to_le_bytes());
    data.extend_from_slice(reply_channel.as_bytes());

    Instruction {
        program_id: X402_SETTLEMENT_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(payment_pda, false),
            AccountMeta::new(escrow_pda, false),
            AccountMeta::new(*payer, true),
            AccountMeta::new(*payer_token_account, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new(state_pda, false),
            AccountMeta::new_readonly(SPL_TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(solana_sdk::system_program::ID, false),
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::ID, false),
        ],
        data,
    }
}

/// Builds the `pay_via_hyperlane` instruction for x402 settlement via Hyperlane.
pub fn build_pay_via_hyperlane_instruction(
    payer: &Pubkey,
    payer_token_account: &Pubkey,
    mint: &Pubkey,
    hyperlane_mailbox: &Pubkey,
    payment_id: [u8; 32],
    target_agent_id: [u8; 32],
    amount: u64,
) -> Instruction {
    let (payment_pda, _) = payment_pda(&payment_id);
    let (escrow_pda, _) = escrow_pda(&payment_id);
    let (state_pda, _) = settlement_state_pda();

    let mut data = Vec::with_capacity(8 + 32 + 32 + 8);
    data.extend_from_slice(&anchor_discriminator("pay_via_hyperlane"));
    data.extend_from_slice(&payment_id);
    data.extend_from_slice(&target_agent_id);
    data.extend_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: X402_SETTLEMENT_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(payment_pda, false),
            AccountMeta::new(escrow_pda, false),
            AccountMeta::new(*payer, true),
            AccountMeta::new(*payer_token_account, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new(state_pda, false),
            AccountMeta::new_readonly(SPL_TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(solana_sdk::system_program::ID, false),
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::ID, false),
            AccountMeta::new_readonly(*hyperlane_mailbox, false),
        ],
        data,
    }
}
