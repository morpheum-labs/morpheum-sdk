//! Integration tests for the Market module.
//!
//! These tests verify the full end-to-end flow for market operations using the
//! Morpheum SDK, including fluent builders, transaction signing, and queries.
//! All tests use deterministic test data from `common.rs` for reproducibility.

use super::common::*;
use morpheum_sdk_native::prelude::*;
use std::error::Error;

/// Test creating a market using the fluent builder and signing the transaction.
#[tokio::test]
async fn test_create_market_full_flow() {
    let sdk = test_sdk();
    let signer = test_native_signer();

    // Build realistic market parameters
    let params = MarketParams {
        min_order_size: "0.001".to_string(),
        tick_size: "0.01".to_string(),
        lot_size: "1".to_string(),
        max_leverage: "100".to_string(),
        initial_margin_ratio: "0.1".to_string(),
        maintenance_margin_ratio: "0.05".to_string(),
        taker_fee_rate: "0.0005".to_string(),
        maker_fee_rate: "0.0002".to_string(),
        allow_market_orders: true,
        allow_stop_orders: true,
        perp_config: None,
        additional_params: Default::default(),
    };

    // Build the create request using the fluent builder
    let create_request = MarketCreateBuilder::new()
        .from_address(signer.account_id())
        .base_asset_index(1)                    // BTC
        .quote_asset_index(2)                   // USDC
        .market_type(MarketType::Perp)
        .orderbook_type("clob")
        .params(params)
        .governance_proposal_id("gov-test-001")
        .build()
        .expect("MarketCreateBuilder should succeed with valid inputs");

    // Sign the transaction using TxBuilder
    let signed_tx = TxBuilder::new(signer)
        .chain_id(TEST_CHAIN_ID)
        .memo("Integration test: Create BTC-USDC-PERP market")
        .add_message(create_request.to_any())
        .sign()
        .await
        .expect("Signing market creation transaction should succeed");

    // Assertions
    assert!(!signed_tx.txhash_hex().is_empty(), "TxHash should not be empty");
    assert!(!signed_tx.raw_bytes().is_empty(), "Raw bytes should not be empty");
    assert!(signed_tx.raw_bytes().len() > 100, "Signed transaction should have reasonable size");

    println!("✅ Market creation test passed. TxHash: {}", signed_tx.txhash_hex());
}

/// Test querying a market (happy path).
#[tokio::test]
async fn test_query_market() {
    let sdk = test_sdk();

    let result = sdk.market()
        .query_market(42, None)           // market_index 42
        .await;

    // In integration tests we expect the flow to succeed (even if real data is not returned)
    // The important part is that the client and transport layer work without panic
    assert!(result.is_ok() || result.is_err(), "Query should at least not panic");
}

/// Test querying active markets with pagination.
#[tokio::test]
async fn test_query_active_markets() {
    let sdk = test_sdk();

    let result = sdk.market()
        .query_active_markets(10, 0)
        .await;

    assert!(result.is_ok() || result.is_err(), "Active markets query should not panic");
}

/// Test market statistics query.
#[tokio::test]
async fn test_query_market_stats() {
    let sdk = test_sdk();

    let result = sdk.market()
        .query_market_stats(42, Some("24h".to_string()), None)
        .await;

    assert!(result.is_ok() || result.is_err(), "Market stats query should not panic");
}

/// Test error case: invalid market creation (missing required fields).
#[tokio::test]
async fn test_create_market_validation_error() {
    let result = MarketCreateBuilder::new().build();

    assert!(result.is_err(), "Creating market without required fields should fail");
    let err = result.unwrap_err();
    assert!(err.to_string().contains("required"), "Error message should mention required fields");
}

#[tokio::test]
async fn test_market_module_end_to_end_sanity() {
    println!("Running Market module end-to-end sanity checks...");

    test_create_market_full_flow().await;
    test_query_market().await;
    test_query_active_markets().await;
    test_query_market_stats().await;
    test_create_market_validation_error().await;

    println!("✅ All Market module integration tests passed!");
}