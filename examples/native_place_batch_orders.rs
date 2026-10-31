//! Example: Placing a batch of CLOB orders using the Morpheum Native SDK.
//!
//! This example demonstrates the recommended way to place a batch using:
//! - `native()` convenience constructor
//! - `NativeSigner` from seed (for deterministic testing)
//! - Fluent `PlaceOrderBuilder` plus `PlaceBatchOrdersBuilder` for a
//!   `/clob.v1.MsgPlaceBatchOrdersRequest`
//! - Signing via `TxBuilder` (the broadcast call stays a comment)
//!
//! `PlaceBatchOrdersRequest::to_any()` under `std` stamps each order with
//! `SystemTime::now()` and derives `orders_hash` from that timestamp, so the
//! printed tx hash can change between runs.
//!
//! Run with:
//! ```bash
//! cargo run -p morpheum-sdk-examples --example native_place_batch_orders
//! ```

use morpheum_sdk_native::clob::{OrderType, PlaceBatchOrdersBuilder, PlaceOrderBuilder, Side};
use morpheum_sdk_native::core::signing::signer::Signer;
use morpheum_sdk_native::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Morpheum Native SDK - Place Batch Orders Example");

    // 1. Create a deterministic signer (in production, load from secure storage or mnemonic)
    let signer = NativeSigner::from_seed(&[0x42; 32]);

    // 2. Create the main SDK instance
    let sdk = native(signer.clone());

    // 3. Build one limit order, then a batch of that single order.
    //    `PlaceOrderBuilder::address` and `PlaceBatchOrdersBuilder::from_address`
    //    take a string. The signing `AccountId` has no `Display`; the SDK
    //    `AccountId` renders those 32 bytes as hex. TimeInForce stays the
    //    default Gtc. client_order_id, credential_proof, and expiry_timestamp
    //    stay unset. `orders_hash` stays unset so `to_any()` fills it from
    //    the clock (`to_proto_with_canonical_hash`).
    let address = AccountId::from(signer.account_id()).to_string();
    let order = PlaceOrderBuilder::new()
        .address(address.clone())
        .market_index(0)
        .price("100")
        .quantity("1")
        .side(Side::Buy)
        .order_type(OrderType::Limit)
        .build()?;
    let batch = PlaceBatchOrdersBuilder::new()
        .from_address(address)
        .order(order)
        .build()?;

    println!("📋 Place batch orders request built successfully");

    // 4. Create the transaction using TxBuilder and sign it.
    // The target chain's genesis hash, from operator configuration (never from
    // the node you submit to). `sign()` refuses a transaction built without one.
    let genesis_hash = [0u8; 32]; // Replace with the configured genesis hash
    let signed_tx = TxBuilder::new(signer)
        .chain_id(sdk.config().default_chain_id.clone())
        .with_genesis_hash(genesis_hash)
        .memo("Placing batch of 1 limit buy via SDK example")
        .add_message(batch.to_any())
        .sign()
        .await?;

    println!("🔏 Transaction signed successfully");
    println!("   TxHash: {}", signed_tx.txhash_hex());

    // 5. In a real application, you would now broadcast the transaction:
    // let result = sdk.clob().broadcast(signed_tx.raw_bytes()).await?;
    // println!("✅ Batch placed on-chain! TxHash: {}", result.txhash);

    println!("\n✅ Example completed successfully!");
    println!("   Next step: Broadcast the raw_bytes to a Sentry node.");

    Ok(())
}
