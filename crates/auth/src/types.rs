//! Domain types for the Auth module.
//!
//! These are clean, idiomatic Rust representations of the core auth protobuf
//! messages. They provide ergonomic APIs while maintaining full fidelity
//! with the on-chain data model.

use alloc::{string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;

use morpheum_sdk_core::{AccountId, SdkError};
use morpheum_proto::{
    auth::v1 as proto,
    tx::v1::Nonce as ProtoNonce,
};

/// NonceState — the single source of truth for replay protection and
/// parallel execution on Morpheum.
///
/// Contains the last monotonic nonce, a sliding ring buffer for recent
/// nonces, and a Merkle root for historical verification.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NonceState {
    /// Last used monotonic nonce.
    pub last_monotonic: u64,
    /// Sliding ring buffer of recent nonces (for replay protection).
    pub ring: Vec<ProtoNonce>,
    /// Incremental Merkle root covering nonce history.
    pub merkle_root: Vec<u8>,
}

impl NonceState {
    /// Creates an empty `NonceState` (used for new accounts).
    pub fn new() -> Self {
        Self {
            last_monotonic: 0,
            ring: Vec::new(),
            merkle_root: Vec::new(),
        }
    }

    /// Returns the number of nonces currently in the ring buffer.
    pub fn ring_size(&self) -> usize {
        self.ring.len()
    }
}

impl Default for NonceState {
    fn default() -> Self {
        Self::new()
    }
}

impl From<proto::NonceState> for NonceState {
    fn from(p: proto::NonceState) -> Self {
        Self {
            last_monotonic: p.last_monotonic,
            ring: p.ring,
            merkle_root: p.merkle_root,
        }
    }
}

impl From<NonceState> for proto::NonceState {
    fn from(s: NonceState) -> Self {
        Self {
            last_monotonic: s.last_monotonic,
            ring: s.ring,
            merkle_root: s.merkle_root,
        }
    }
}

/// Converts `Option<proto::NonceState>` to `NonceState`, using default for `None`.
impl From<Option<proto::NonceState>> for NonceState {
    fn from(opt: Option<proto::NonceState>) -> Self {
        opt.map(NonceState::from).unwrap_or_default()
    }
}

/// BaseAccount — the universal account record for both humans and agents.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BaseAccount {
    /// Canonical account address (32-byte AccountId as hex or bech32).
    pub address: String,
    /// Public key (protobuf Any).
    pub pub_key: Option<ProtoAny>,
    /// Immutable account number used in SignDoc.
    pub account_number: u64,
    /// Full nonce state (monotonic + ring + merkle root).
    pub nonce_state: NonceState,
    /// Mana score (Morpheum-specific gasless / reputation metric).
    pub mana_score: u64,
}

impl BaseAccount {
    /// Returns the `AccountId` parsed from the address field.
    pub fn account_id(&self) -> Result<AccountId, SdkError> {
        // The address is typically the hex representation of the 32-byte AccountId
        let bytes = hex::decode(&self.address)
            .map_err(|e| SdkError::invalid_input(alloc::format!("invalid account address: {e}")))?;
        let arr: [u8; 32] = bytes.try_into()
            .map_err(|_| SdkError::invalid_input("account address must be 32 bytes"))?;
        Ok(AccountId::new(arr))
    }
}

impl From<proto::BaseAccount> for BaseAccount {
    fn from(p: proto::BaseAccount) -> Self {
        Self {
            address: p.address,
            pub_key: p.pub_key,
            account_number: p.account_number,
            nonce_state: NonceState::from(p.nonce_state),
            mana_score: p.mana_score,
        }
    }
}

impl From<BaseAccount> for proto::BaseAccount {
    fn from(a: BaseAccount) -> Self {
        Self {
            address: a.address,
            pub_key: a.pub_key,
            account_number: a.account_number,
            nonce_state: Some(a.nonce_state.into()),
            mana_score: a.mana_score,
            storage_deposit: 0,
            bytes_reserved: 0,
        }
    }
}

/// Converts `Option<proto::BaseAccount>` to `BaseAccount`, using defaults for `None`.
impl From<Option<proto::BaseAccount>> for BaseAccount {
    fn from(opt: Option<proto::BaseAccount>) -> Self {
        opt.map(BaseAccount::from).unwrap_or_default()
    }
}

/// ModuleAccount — used by system modules (e.g. bank, staking) that hold funds
/// without a human signer.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModuleAccount {
    pub base_account: BaseAccount,
    pub name: String,
    pub permissions: Vec<String>,
    /// Optional shard identifier for sharded pools.
    pub shard_id: Option<String>,
}

impl From<proto::ModuleAccount> for ModuleAccount {
    fn from(p: proto::ModuleAccount) -> Self {
        Self {
            base_account: BaseAccount::from(p.base_account),
            name: p.name,
            permissions: p.permissions,
            shard_id: p.shard_id,
        }
    }
}

impl From<ModuleAccount> for proto::ModuleAccount {
    fn from(m: ModuleAccount) -> Self {
        Self {
            base_account: Some(m.base_account.into()),
            name: m.name,
            permissions: m.permissions,
            shard_id: m.shard_id,
        }
    }
}

/// ModuleCredential — used for deriving module addresses without exposing keys.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModuleCredential {
    pub module_name: String,
    pub derivation_keys: Vec<Vec<u8>>,
}

impl From<proto::ModuleCredential> for ModuleCredential {
    fn from(p: proto::ModuleCredential) -> Self {
        Self {
            module_name: p.module_name,
            derivation_keys: p.derivation_keys,
        }
    }
}

impl From<ModuleCredential> for proto::ModuleCredential {
    fn from(m: ModuleCredential) -> Self {
        Self {
            module_name: m.module_name,
            derivation_keys: m.derivation_keys,
        }
    }
}

/// Module parameters (governance-controlled).
///
/// Provides sensible defaults for common use cases:
/// - `max_memo_characters`: 256
/// - `tx_sig_limit`: 7
/// - `mana_threshold`: 0
///
/// Override only the fields you need:
/// ```rust,ignore
/// let params = Params {
///     mana_threshold: 100,
///     ..Default::default()
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params {
    pub max_memo_characters: u64,
    pub tx_sig_limit: u64,
    pub mana_threshold: u64,
    pub congestion_threshold_pct: u32,
    pub sponsored_threshold_pct: u32,
    pub max_mana_multiplier_bps: u32,
    pub max_reputation_multiplier_bps: u32,
}

impl Default for Params {
    fn default() -> Self {
        use morpheum_primitives::priority_fee::{
            DEFAULT_CONGESTION_THRESHOLD_PCT, DEFAULT_MAX_MANA_MULTIPLIER_BPS,
            DEFAULT_MAX_REPUTATION_MULTIPLIER_BPS, DEFAULT_SPONSORED_THRESHOLD_PCT,
        };
        Self {
            max_memo_characters: 256,
            tx_sig_limit: 7,
            mana_threshold: 0,
            congestion_threshold_pct: DEFAULT_CONGESTION_THRESHOLD_PCT,
            sponsored_threshold_pct: DEFAULT_SPONSORED_THRESHOLD_PCT,
            max_mana_multiplier_bps: DEFAULT_MAX_MANA_MULTIPLIER_BPS,
            max_reputation_multiplier_bps: DEFAULT_MAX_REPUTATION_MULTIPLIER_BPS,
        }
    }
}

impl From<proto::Params> for Params {
    fn from(p: proto::Params) -> Self {
        Self {
            max_memo_characters: p.max_memo_characters,
            tx_sig_limit: p.tx_sig_limit,
            mana_threshold: p.mana_threshold,
            congestion_threshold_pct: p.congestion_threshold_pct,
            sponsored_threshold_pct: p.sponsored_threshold_pct,
            max_mana_multiplier_bps: p.max_mana_multiplier_bps,
            max_reputation_multiplier_bps: p.max_reputation_multiplier_bps,
        }
    }
}

impl From<Params> for proto::Params {
    fn from(p: Params) -> Self {
        Self {
            max_memo_characters: p.max_memo_characters,
            tx_sig_limit: p.tx_sig_limit,
            mana_threshold: p.mana_threshold,
            congestion_threshold_pct: p.congestion_threshold_pct,
            sponsored_threshold_pct: p.sponsored_threshold_pct,
            max_mana_multiplier_bps: p.max_mana_multiplier_bps,
            max_reputation_multiplier_bps: p.max_reputation_multiplier_bps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions_roundtrip() {
        let nonce_state = NonceState::new();
        let proto_ns: proto::NonceState = nonce_state.clone().into();
        let back: NonceState = proto_ns.into();
        assert_eq!(nonce_state, back);
    }

    #[test]
    fn base_account_account_id() {
        let acc = BaseAccount {
            address: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into(),
            pub_key: None,
            account_number: 42,
            nonce_state: NonceState::new(),
            mana_score: 100,
        };
        let id = acc.account_id().unwrap();
        assert_eq!(
            id.as_bytes(),
            &[
                0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
                0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
                0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
                0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
            ]
        );
    }
}
