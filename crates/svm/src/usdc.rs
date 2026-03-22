//! USDC native program client helpers for Morpheum's internal SVM engine.
//!
//! Provides instruction encoding, program ID derivation, and `MsgExecute`
//! builders that mirror the on-chain encoding in
//! `mormcore::modules::svm::programs::usdc`.
//!
//! ## Instruction encoding
//!
//! | Byte 0 (discriminator) | Bytes 1..17 (LE u128) | Accounts           |
//! |------------------------|-----------------------|--------------------|
//! | 0 = Transfer           | amount                | \[from, to\]       |
//! | 1 = Approve            | amount                | \[owner, spender\] |
//! | 2 = `TransferFrom`     | amount                | \[from, to\]       |
//! | 3 = `BalanceOf`        | --                    | \[owner\]          |
//! | 4 = Allowance          | --                    | \[owner, spender\] |

use crate::types::SvmError;

/// Bank asset index for canonical USDC (`FiatTokenPrimitive`).
pub const USDC_ASSET_INDEX: u64 = 1;

/// Default compute limit for USDC native program instructions.
pub const DEFAULT_COMPUTE_LIMIT: u64 = 10_000;

/// Morpheum `MsgExecute` type URL used by the SVM actor.
pub const MSG_EXECUTE_TYPE_URL: &str = "/morpheum.svm.v1.MsgExecute";

// Instruction discriminators (must match mormcore).
const DISC_TRANSFER: u8 = 0;
const DISC_APPROVE: u8 = 1;
const DISC_TRANSFER_FROM: u8 = 2;
const DISC_BALANCE_OF: u8 = 3;
const DISC_ALLOWANCE: u8 = 4;

/// Derives the deterministic USDC native program ID.
///
/// Uses `blake3::hash(b"morpheum_usdc_native_program")` truncated to
/// 20 bytes and hex-encoded, matching the address format used by
/// `SvmExecutor::execute_deploy` in mormcore.
#[must_use]
pub fn usdc_program_id() -> String {
    let hash = blake3::hash(b"morpheum_usdc_native_program");
    hex::encode(&hash.as_bytes()[..20])
}

/// Account metadata for an SVM instruction (client-side mirror of
/// `mormcore::modules::svm::types::msgs::AccountMeta`).
#[derive(Clone, Debug, serde::Serialize)]
pub struct AccountMeta {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl AccountMeta {
    pub fn new(pubkey: impl Into<String>, is_signer: bool, is_writable: bool) -> Self {
        Self {
            pubkey: pubkey.into(),
            is_signer,
            is_writable,
        }
    }

    pub fn writable(pubkey: impl Into<String>) -> Self {
        Self::new(pubkey, false, true)
    }

    pub fn readonly(pubkey: impl Into<String>) -> Self {
        Self::new(pubkey, false, false)
    }
}

// ── Instruction encoding ────────────────────────────────────────────────

fn encode_write(disc: u8, amount: u128) -> Vec<u8> {
    let mut data = Vec::with_capacity(17);
    data.push(disc);
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

/// Encode a `Transfer` instruction: move `amount` USDC from `accounts[0]` to `accounts[1]`.
#[must_use]
pub fn encode_transfer(amount: u128) -> Vec<u8> {
    encode_write(DISC_TRANSFER, amount)
}

/// Encode an `Approve` instruction: let `accounts[1]` spend `amount` of `accounts[0]`'s USDC.
#[must_use]
pub fn encode_approve(amount: u128) -> Vec<u8> {
    encode_write(DISC_APPROVE, amount)
}

/// Encode a `TransferFrom` instruction: spender (tx sender) moves `amount` from
/// `accounts[0]` to `accounts[1]`.
#[must_use]
pub fn encode_transfer_from(amount: u128) -> Vec<u8> {
    encode_write(DISC_TRANSFER_FROM, amount)
}

/// Encode a `BalanceOf` instruction: returns LE u128 balance of `accounts[0]`.
#[must_use]
pub fn encode_balance_of() -> Vec<u8> {
    vec![DISC_BALANCE_OF]
}

/// Encode an `Allowance` instruction: returns LE u128 allowance of
/// `accounts[0]` (owner) granted to `accounts[1]` (spender).
#[must_use]
pub fn encode_allowance() -> Vec<u8> {
    vec![DISC_ALLOWANCE]
}

// ── Return data decoding ────────────────────────────────────────────────

/// Decode a LE-encoded u128 from native program return data.
///
/// # Errors
///
/// Returns `SvmError::Deserialization` if `data` is shorter than 16 bytes.
pub fn decode_u128(data: &[u8]) -> Result<u128, SvmError> {
    if data.len() < 16 {
        return Err(SvmError::Deserialization(format!(
            "expected at least 16 bytes for u128, got {}",
            data.len()
        )));
    }
    let bytes: [u8; 16] = data[..16]
        .try_into()
        .map_err(|_| SvmError::Deserialization("u128 decode failed".into()))?;
    Ok(u128::from_le_bytes(bytes))
}

// ── MsgExecute builder ──────────────────────────────────────────────────

/// Serializable `MsgExecute` matching the JSON format expected by
/// `mormcore::modules::svm::actor::SvmActor`.
#[derive(Clone, Debug, serde::Serialize)]
struct MsgExecute {
    sender: String,
    program_id: String,
    instruction_data: Vec<u8>,
    accounts: Vec<AccountMeta>,
    compute_limit: u64,
}

/// Builds a `google.protobuf.Any` wrapping a JSON-serialized `MsgExecute`
/// targeting the USDC native program.
///
/// The returned `Any` can be submitted via `MorpheumHarness::submit_signed_tx`
/// or the CLI's Ingress gRPC endpoint.
///
/// # Errors
///
/// Returns `SvmError::InstructionFailed` if JSON serialization fails.
pub fn build_usdc_execute(
    sender: &str,
    instruction_data: Vec<u8>,
    accounts: Vec<AccountMeta>,
    compute_limit: u64,
) -> Result<morpheum_proto::google::protobuf::Any, SvmError> {
    let msg = MsgExecute {
        sender: sender.to_string(),
        program_id: usdc_program_id(),
        instruction_data,
        accounts,
        compute_limit,
    };

    let value = serde_json::to_vec(&msg)
        .map_err(|e| SvmError::InstructionFailed(format!("MsgExecute serialization: {e}")))?;

    Ok(morpheum_proto::google::protobuf::Any {
        type_url: MSG_EXECUTE_TYPE_URL.to_string(),
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_id_is_40_hex_chars() {
        let id = usdc_program_id();
        assert_eq!(id.len(), 40, "20 bytes = 40 hex chars");
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn encode_transfer_produces_17_bytes() {
        let data = encode_transfer(1_000_000);
        assert_eq!(data.len(), 17);
        assert_eq!(data[0], 0);
        assert_eq!(
            u128::from_le_bytes(data[1..17].try_into().unwrap()),
            1_000_000
        );
    }

    #[test]
    fn encode_balance_of_produces_1_byte() {
        let data = encode_balance_of();
        assert_eq!(data, vec![3]);
    }

    #[test]
    fn decode_u128_roundtrip() {
        let val: u128 = 42_000_000;
        let encoded = val.to_le_bytes().to_vec();
        assert_eq!(decode_u128(&encoded).unwrap(), val);
    }

    #[test]
    fn decode_u128_too_short() {
        assert!(decode_u128(&[0; 8]).is_err());
    }

    #[test]
    fn build_usdc_execute_produces_any() {
        let any = build_usdc_execute(
            "test_sender",
            encode_balance_of(),
            vec![AccountMeta::readonly("owner_addr")],
            DEFAULT_COMPUTE_LIMIT,
        )
        .unwrap();

        assert_eq!(any.type_url, MSG_EXECUTE_TYPE_URL);
        assert!(!any.value.is_empty());

        let parsed: serde_json::Value = serde_json::from_slice(&any.value).unwrap();
        assert_eq!(parsed["program_id"].as_str().unwrap(), usdc_program_id());
        assert_eq!(parsed["sender"].as_str().unwrap(), "test_sender");
    }
}
