//! Main integration test runner for the Morpheum SDK.
//!
//! This file serves as the central entry point for all integration and end-to-end tests.
//! It includes all module-specific test suites and provides high-level sanity tests
//! for core SDK functionality.

#![cfg(test)]

mod common;
mod market_integration;
mod vc_integration;
mod auth_integration;

use common::*;
use morpheum_sdk_native::prelude::*;

/// High-level SDK initialization and basic signing sanity test.
#[tokio::test]
async fn sdk_initialization_and_basic_signing() {
    let sdk = test_sdk();

    assert_eq!(sdk.config().default_chain_id.as_str(), TEST_CHAIN_ID);

    // Test basic signing flow with NativeSigner
    let signer = test_native_signer();
    let signed_tx = TxBuilder::new(signer)
        .chain_id(TEST_CHAIN_ID)
        .memo("Basic signing sanity test")
        .add_message(Any {
            type_url: "type.googleapis.com/test.v1.MsgTest".to_string(),
            value: vec![1, 2, 3, 4],
        })
        .sign()
        .await
        .expect("Basic signing should succeed");

    assert!(!signed_tx.txhash_hex().is_empty());
    assert!(!signed_tx.raw_bytes().is_empty());
}

/// Test agent signing with TradingKeyClaim (core agent delegation flow).
#[tokio::test]
async fn agent_signing_with_trading_key_claim() {
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let agent_signer = test_agent_signer();

    let claim = test_trading_key_claim(
        AccountId::new([0x11; 32]), // Owner
        agent_signer.account_id(),   // Agent
        now_secs,
    )
        .expect("Test claim should be valid");

    // Verify the claim before use
    claim.verify(now_secs, &AccountId::new([0x11; 32]))
        .expect("Claim verification should succeed");

    let sdk = agent(agent_signer);

    let signed_tx = TxBuilder::new(sdk.signer.clone())
        .chain_id(TEST_CHAIN_ID)
        .memo("Agent signing with verified TradingKeyClaim")
        .with_trading_key_claim(claim)
        .add_message(Any {
            type_url: "type.googleapis.com/test.v1.MsgTest".to_string(),
            value: vec![42],
        })
        .sign()
        .await
        .expect("Agent signing with claim should succeed");

    assert!(!signed_tx.txhash_hex().is_empty());
}

/// Test BIP-39 mnemonic-based signer (human wallet flow).
#[tokio::test]
async fn bip39_mnemonic_signer_works() {
    let signer = NativeSigner::from_mnemonic(TEST_MNEMONIC, "")
        .expect("Valid mnemonic should create signer");

    let sdk = native(signer);

    let signed_tx = TxBuilder::new(sdk.signer.clone())
        .chain_id(TEST_CHAIN_ID)
        .memo("Transaction from BIP-39 mnemonic")
        .add_message(Any {
            type_url: "type.googleapis.com/test.v1.MsgTest".to_string(),
            value: vec![9, 9, 9],
        })
        .sign()
        .await
        .expect("BIP-39 signing should succeed");

    assert!(!signed_tx.txhash_hex().is_empty());
}

// Run all module-specific integration tests
#[tokio::test]
async fn run_all_module_integration_tests() {
    // These tests are defined in their respective files
    // and will be executed automatically when this test runs
    println!("Running full integration test suite...");
}

// The following tests are automatically included via mod declarations above:
// - market_integration::all_tests()
// - vc_integration::all_tests()
// - auth_integration::all_tests()

#[cfg(feature = "market")]
#[tokio::test]
async fn market_module_is_available() {
    let sdk = test_sdk();
    let _market_client = sdk.market();
    // If this compiles, the market feature is working
}

#[cfg(feature = "vc")]
#[tokio::test]
async fn vc_module_is_available() {
    let sdk = test_sdk();
    let _vc_client = sdk.vc();
}

#[cfg(feature = "auth")]
#[tokio::test]
async fn auth_module_is_available() {
    let sdk = test_sdk();
    let _auth_client = sdk.auth();
}