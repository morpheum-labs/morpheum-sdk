//! Compile-time registry of known chain metadata for SDK/CLI convenience.
//!
//! Provides CAIP-2 validation, human-readable chain names, default assets,
//! and signature scheme hints. This registry is purely informational —
//! settlement works for any `source_chain` string regardless of whether
//! it appears in `KNOWN_CHAINS`.

/// Cryptographic signature scheme used by a chain family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignatureScheme {
    /// EIP-712 typed data signatures (EVM chains).
    Eip712,
    /// Ed25519 signatures (Solana, Stellar).
    Ed25519,
    /// secp256k1 ECDSA (Tron, Bitcoin-derived).
    Secp256k1,
    /// Sr25519 Schnorr signatures (Substrate/Polkadot).
    Sr25519,
}

/// Metadata for a known chain, used for validation and display.
#[derive(Clone, Debug)]
pub struct ChainMetadata {
    /// CAIP-2 namespace (e.g., `"eip155"`, `"solana"`).
    pub namespace: &'static str,
    /// CAIP-2 reference / chain ID (e.g., `"8453"` for Base).
    pub reference: &'static str,
    /// Human-readable name for CLI output.
    pub display_name: &'static str,
    /// Common asset identifiers on this chain.
    pub native_assets: &'static [&'static str],
    /// Signature scheme used on this chain.
    pub signature_scheme: SignatureScheme,
}

impl ChainMetadata {
    /// Returns the full CAIP-2 identifier (e.g., `"eip155:8453"`).
    pub fn caip2(&self) -> alloc::string::String {
        alloc::format!("{}:{}", self.namespace, self.reference)
    }
}

/// Static registry of known chains. Extensible at compile time.
pub const KNOWN_CHAINS: &[ChainMetadata] = &[
    // ── EVM chains ──────────────────────────────────────────
    ChainMetadata {
        namespace: "eip155",
        reference: "1",
        display_name: "Ethereum",
        native_assets: &["USDC", "USDT", "DAI"],
        signature_scheme: SignatureScheme::Eip712,
    },
    ChainMetadata {
        namespace: "eip155",
        reference: "8453",
        display_name: "Base",
        native_assets: &["USDC"],
        signature_scheme: SignatureScheme::Eip712,
    },
    ChainMetadata {
        namespace: "eip155",
        reference: "137",
        display_name: "Polygon",
        native_assets: &["USDC", "USDT"],
        signature_scheme: SignatureScheme::Eip712,
    },
    ChainMetadata {
        namespace: "eip155",
        reference: "42161",
        display_name: "Arbitrum",
        native_assets: &["USDC"],
        signature_scheme: SignatureScheme::Eip712,
    },
    ChainMetadata {
        namespace: "eip155",
        reference: "56",
        display_name: "BSC",
        native_assets: &["USDC", "BUSD"],
        signature_scheme: SignatureScheme::Eip712,
    },
    ChainMetadata {
        namespace: "eip155",
        reference: "10",
        display_name: "Optimism",
        native_assets: &["USDC"],
        signature_scheme: SignatureScheme::Eip712,
    },
    ChainMetadata {
        namespace: "eip155",
        reference: "43114",
        display_name: "Avalanche",
        native_assets: &["USDC"],
        signature_scheme: SignatureScheme::Eip712,
    },
    // ── Non-EVM chains ──────────────────────────────────────
    ChainMetadata {
        namespace: "solana",
        reference: "5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
        display_name: "Solana",
        native_assets: &["USDC"],
        signature_scheme: SignatureScheme::Ed25519,
    },
    ChainMetadata {
        namespace: "tron",
        reference: "0x2b6653dc",
        display_name: "Tron",
        native_assets: &["USDT"],
        signature_scheme: SignatureScheme::Secp256k1,
    },
    ChainMetadata {
        namespace: "stellar",
        reference: "pubnet",
        display_name: "Stellar",
        native_assets: &["USDC"],
        signature_scheme: SignatureScheme::Ed25519,
    },
    // ── EVM testnets ────────────────────────────────────────
    ChainMetadata {
        namespace: "eip155",
        reference: "84532",
        display_name: "Base Sepolia",
        native_assets: &["USDC"],
        signature_scheme: SignatureScheme::Eip712,
    },
    ChainMetadata {
        namespace: "eip155",
        reference: "11155111",
        display_name: "Ethereum Sepolia",
        native_assets: &["USDC"],
        signature_scheme: SignatureScheme::Eip712,
    },
    ChainMetadata {
        namespace: "eip155",
        reference: "80002",
        display_name: "Polygon Amoy",
        native_assets: &["USDC"],
        signature_scheme: SignatureScheme::Eip712,
    },
    ChainMetadata {
        namespace: "eip155",
        reference: "421614",
        display_name: "Arbitrum Sepolia",
        native_assets: &["USDC"],
        signature_scheme: SignatureScheme::Eip712,
    },
    // ── Non-EVM testnets ──────────────────────────────────
    ChainMetadata {
        namespace: "solana",
        reference: "EtWTRABZaYq6iMfeYKouRu166VU2xqa1",
        display_name: "Solana Devnet",
        native_assets: &["USDC"],
        signature_scheme: SignatureScheme::Ed25519,
    },
    ChainMetadata {
        namespace: "tron",
        reference: "0xcd8690dc",
        display_name: "Tron Nile",
        native_assets: &["USDT"],
        signature_scheme: SignatureScheme::Secp256k1,
    },
    ChainMetadata {
        namespace: "stellar",
        reference: "testnet",
        display_name: "Stellar Testnet",
        native_assets: &["USDC"],
        signature_scheme: SignatureScheme::Ed25519,
    },
    // ── Local test chains ─────────────────────────────────
    ChainMetadata {
        namespace: "eip155",
        reference: "31337",
        display_name: "Anvil (local)",
        native_assets: &["USDC"],
        signature_scheme: SignatureScheme::Eip712,
    },
];

/// Looks up a chain by human-readable short name (case-insensitive).
///
/// Supports common aliases: `"base"`, `"eth"`, `"ethereum"`, `"polygon"`,
/// `"arb"`, `"arbitrum"`, `"bsc"`, `"solana"`, `"sol"`, `"tron"`, `"stellar"`,
/// `"optimism"`, `"op"`, `"avalanche"`, `"avax"`, `"anvil"`.
pub fn resolve_chain_name(name: &str) -> Option<&'static ChainMetadata> {
    let lower = name.to_ascii_lowercase();
    match lower.as_str() {
        // Mainnets
        "ethereum" | "eth" => find_by_caip2("eip155", "1"),
        "base" => find_by_caip2("eip155", "8453"),
        "polygon" | "matic" => find_by_caip2("eip155", "137"),
        "arbitrum" | "arb" => find_by_caip2("eip155", "42161"),
        "bsc" | "binance" => find_by_caip2("eip155", "56"),
        "optimism" | "op" => find_by_caip2("eip155", "10"),
        "avalanche" | "avax" => find_by_caip2("eip155", "43114"),
        "solana" | "sol" => find_by_caip2("solana", "5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp"),
        "tron" | "trx" => find_by_caip2("tron", "0x2b6653dc"),
        "stellar" | "xlm" => find_by_caip2("stellar", "pubnet"),
        // EVM testnets
        "base-sepolia" | "base_sepolia" => find_by_caip2("eip155", "84532"),
        "sepolia" => find_by_caip2("eip155", "11155111"),
        "polygon-amoy" | "polygon_amoy" | "amoy" => find_by_caip2("eip155", "80002"),
        "arbitrum-sepolia" | "arb-sepolia" | "arb_sepolia" => find_by_caip2("eip155", "421614"),
        // Non-EVM testnets
        "solana-devnet" | "sol-devnet" => find_by_caip2("solana", "EtWTRABZaYq6iMfeYKouRu166VU2xqa1"),
        "tron-nile" | "nile" => find_by_caip2("tron", "0xcd8690dc"),
        "stellar-testnet" => find_by_caip2("stellar", "testnet"),
        // Local
        "anvil" | "local" => find_by_caip2("eip155", "31337"),
        _ => None,
    }
}

/// Looks up a chain by CAIP-2 namespace and reference.
pub fn find_by_caip2(namespace: &str, reference: &str) -> Option<&'static ChainMetadata> {
    KNOWN_CHAINS
        .iter()
        .find(|c| c.namespace == namespace && c.reference == reference)
}

/// Looks up a chain by a full CAIP-2 string (e.g., `"eip155:8453"`).
pub fn find_by_caip2_str(caip2: &str) -> Option<&'static ChainMetadata> {
    let (ns, ref_) = caip2.split_once(':')?;
    find_by_caip2(ns, ref_)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_known_names() {
        assert_eq!(resolve_chain_name("base").unwrap().display_name, "Base");
        assert_eq!(resolve_chain_name("Ethereum").unwrap().display_name, "Ethereum");
        assert_eq!(resolve_chain_name("ETH").unwrap().display_name, "Ethereum");
        assert_eq!(resolve_chain_name("sol").unwrap().display_name, "Solana");
        assert_eq!(resolve_chain_name("BSC").unwrap().display_name, "BSC");
        assert_eq!(resolve_chain_name("tron").unwrap().display_name, "Tron");
        assert_eq!(resolve_chain_name("stellar").unwrap().display_name, "Stellar");
        assert_eq!(resolve_chain_name("anvil").unwrap().display_name, "Anvil (local)");
    }

    #[test]
    fn resolve_unknown_returns_none() {
        assert!(resolve_chain_name("unknown-chain").is_none());
    }

    #[test]
    fn find_by_caip2_str_works() {
        let meta = find_by_caip2_str("eip155:8453").unwrap();
        assert_eq!(meta.display_name, "Base");
    }

    #[test]
    fn caip2_format() {
        let meta = find_by_caip2("eip155", "8453").unwrap();
        assert_eq!(meta.caip2(), "eip155:8453");
    }

    #[test]
    fn all_chains_have_at_least_one_asset() {
        for chain in KNOWN_CHAINS {
            assert!(
                !chain.native_assets.is_empty(),
                "{} has no native assets",
                chain.display_name
            );
        }
    }
}
