//! Example: Placing a CLOB order using the Morpheum Native SDK.
//!
//! This example demonstrates the recommended way to place an order using:
//! - `native()` convenience constructor
//! - `NativeSigner` from seed (for deterministic testing)
//! - Fluent `PlaceOrderBuilder` for a `/clob.v1.MsgPlaceOrderRequest`
//! - Signing via `TxBuilder` (the broadcast call stays a comment)
//!
//! Run with:
//! ```bash
//! cargo run -p morpheum-sdk-examples --example native_place_order
//! ```

use morpheum_sdk_native::clob::{OrderType, PlaceOrderBuilder, Side};
use morpheum_sdk_native::core::signing::signer::Signer;
use morpheum_sdk_native::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Morpheum Native SDK - Place Order Example");

    // 1. Create a deterministic signer (in production, load from secure storage or mnemonic)
    let signer = NativeSigner::from_seed(&[0x42; 32]);

    // 2. Create the main SDK instance
    let sdk = native(signer.clone());

    // 3. Build the place request. `PlaceOrderBuilder::address` takes a string.
    //    The signing `AccountId` has no `Display`; the SDK `AccountId` renders
    //    those 32 bytes as hex. TimeInForce stays the default Gtc. client_order_id,
    //    credential_proof, and expiry_timestamp stay unset so the proto timestamp
    //    stays unset and the hash does not depend on the clock.
    let address = AccountId::from(signer.account_id()).to_string();
    let place_request = PlaceOrderBuilder::new()
        .address(address)
        .market_index(0)
        .price("100")
        .quantity("1")
        .side(Side::Buy)
        .order_type(OrderType::Limit)
        .build()?;

    println!("📋 Place order request built successfully");

    // 4. Create the transaction using TxBuilder and sign it.
    // The target chain's genesis hash, from operator configuration (never from
    // the node you submit to). `sign()` refuses a transaction built without one.
    let genesis_hash = [0u8; 32]; // Replace with the configured genesis hash
    let signed_tx = TxBuilder::new(signer)
        .chain_id(sdk.config().default_chain_id.clone())
        .with_genesis_hash(genesis_hash)
        .memo("Placing limit buy on market 0 via SDK example")
        .add_message(place_request.to_any())
        .sign()
        .await?;

    println!("🔏 Transaction signed successfully");
    println!("   TxHash: {}", signed_tx.txhash_hex());

    // 5. In a real application, you would now broadcast the transaction:
    // let result = sdk.clob().broadcast(signed_tx.raw_bytes()).await?;
    // println!("✅ Order placed on-chain! TxHash: {}", result.txhash);

    println!("\n✅ Example completed successfully!");
    println!("   Next step: Broadcast the raw_bytes to a Sentry node.");

    Ok(())
}
