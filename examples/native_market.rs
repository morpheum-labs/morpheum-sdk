//! Example: Creating a new market using the Morpheum Native SDK.
//!
//! This example demonstrates the recommended way to create a market using:
//! - `native()` convenience constructor
//! - `NativeSigner` from seed (for deterministic testing)
//! - Fluent `MarketCreateBuilder` for type-safe market creation
//! - Signing and broadcasting via the SDK
//!
//! Market terms are integer strings: `tick_size` and `lot_size` must be
//! positive integers, and a spot, perp, future or option market trades on
//! the `"clob"` orderbook.
//!
//! Run with:
//! ```bash
//! cargo run -p morpheum-sdk-examples --example native_market
//! ```

use morpheum_sdk_native::core::signing::signer::Signer;
use morpheum_sdk_native::market::builder::MarketCreateBuilder;
use morpheum_sdk_native::market::types::{
    ClobMarketConfig, MarketParams, MarketType, MarketTypeConfig,
};
use morpheum_sdk_native::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Morpheum Native SDK - Market Creation Example");

    // 1. Create a deterministic signer (in production, load from secure storage or mnemonic)
    let signer = NativeSigner::from_seed(&[0x42; 32]);

    // 2. Create the main SDK instance
    let sdk = native(signer.clone());

    // 3. Prepare the market terms: integer strings, starting from the CLOB defaults
    let market_params = MarketParams {
        type_config: Some(MarketTypeConfig::Clob(ClobMarketConfig {
            tick_size: "100".to_string(),
            max_leverage: "20".to_string(),
            ..ClobMarketConfig::default()
        })),
        ..MarketParams::default()
    };

    // 4. Build the market creation request using the fluent builder
    let create_request = MarketCreateBuilder::new()
        .from_address(signer.account_id()) // The creator is the signing account
        .base_asset_index(1) // Example: BTC
        .quote_asset_index(2) // Example: USDC
        .market_type(MarketType::Perp)
        .orderbook_type("clob")
        .params(market_params)
        .governance_proposal_id("gov-2026-001")
        .build()?;

    println!("📋 Market creation request built successfully");

    // 5. Create the transaction using TxBuilder and sign it
    let signed_tx = TxBuilder::new(signer)
        .chain_id(sdk.config().default_chain_id.clone())
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
