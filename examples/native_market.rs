//! Example: Creating a new market using the Morpheum Native SDK.
//!
//! This example demonstrates the recommended way to create a market using:
//! - `native()` convenience constructor
//! - `NativeSigner` from seed (for deterministic testing)
//! - Fluent `MarketCreateBuilder` for type-safe market creation
//! - Signing and broadcasting via the SDK
//!
//! Run with:
//! ```bash
//! cargo run -p morpheum-sdk-examples --example native_market
//! ```

use morpheum_sdk_native::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Morpheum Native SDK - Market Creation Example");

    // 1. Create a deterministic signer (in production, load from secure storage or mnemonic)
    let signer = NativeSigner::from_seed(b"morpheum-example-seed-32-bytes-long!!");

    // 2. Create the main SDK instance
    let sdk = native(signer);

    // 3. Prepare market parameters using the fluent builder
    let market_params = MarketParams {
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

    // 4. Build the market creation request using the fluent builder
    let create_request = MarketCreateBuilder::new()
        .from_address(sdk.config().default_chain_id.as_str()) // Using chain ID as placeholder; in real usage this would be the creator's AccountId
        .base_asset_index(1)   // Example: BTC
        .quote_asset_index(2)  // Example: USDC
        .market_type(MarketType::Perp)
        .orderbook_type("clob")
        .params(market_params)
        .governance_proposal_id("gov-2026-001")
        .build()?;

    println!("📋 Market creation request built successfully");

    // 5. Create the transaction using TxBuilder and sign it
    let signed_tx = TxBuilder::new(sdk.signer.clone()) // In real usage, you would pass the signer properly
        .chain_id("morpheum-test-1")
        .memo("Creating BTC-USDC-PERP market via SDK example")
        .add_message(create_request.to_any())
        .sign()
        .await?;

    println!("🔏 Transaction signed successfully");
    println!("   TxHash: {}", signed_tx.txhash_hex());

    // 6. In a real application, you would now broadcast the transaction:
    // let result = sdk.market().broadcast(signed_tx.raw_bytes()).await?;
    // println!("✅ Market created on-chain! TxHash: {}", result.txhash);

    println!("\n✅ Example completed successfully!");
    println!("   Next step: Broadcast the raw_bytes to a Sentry node.");

    Ok(())
}