//! Example: Amending a CLOB order using the Morpheum Native SDK.
//!
//! This example demonstrates the recommended way to modify an order using:
//! - `native()` convenience constructor
//! - `NativeSigner` from seed (for deterministic testing)
//! - Fluent `ModifyOrderBuilder` for a `/clob.v1.MsgModifyOrderRequest`
//! - Signing via `TxBuilder` (the broadcast call stays a comment)
//!
//! Run with:
//! ```bash
//! cargo run -p morpheum-sdk-examples --example native_modify_order
//! ```

use morpheum_sdk_native::clob::ModifyOrderBuilder;
use morpheum_sdk_native::core::signing::signer::Signer;
use morpheum_sdk_native::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Morpheum Native SDK - Modify Order Example");

    // 1. Create a deterministic signer (in production, load from secure storage or mnemonic)
    let signer = NativeSigner::from_seed(&[0x42; 32]);

    // 2. Create the main SDK instance
    let sdk = native(signer.clone());

    // 3. Build the modify request. `ModifyOrderBuilder::address` takes a string.
    //    The signing `AccountId` has no `Display`; the SDK `AccountId` renders
    //    those 32 bytes as hex.
    let address = AccountId::from(signer.account_id()).to_string();
    let modify_request = ModifyOrderBuilder::new()
        .address(address)
        .order_id("order-test-1")
        .symbol("BTC-USDC")
        .new_price("101")
        .new_quantity("2")
        .build()?;

    println!("📋 Modify order request built successfully");

    // 4. Create the transaction using TxBuilder and sign it.
    // The target chain's genesis hash, from operator configuration (never from
    // the node you submit to). `sign()` refuses a transaction built without one.
    let genesis_hash = [0u8; 32]; // Replace with the configured genesis hash
    let signed_tx = TxBuilder::new(signer)
        .chain_id(sdk.config().default_chain_id.clone())
        .with_genesis_hash(genesis_hash)
        .memo("Modifying order-test-1 on BTC-USDC via SDK example")
        .add_message(modify_request.to_any())
        .sign()
        .await?;

    println!("🔏 Transaction signed successfully");
    println!("   TxHash: {}", signed_tx.txhash_hex());

    // 5. In a real application, you would now broadcast the transaction:
    // let result = sdk.clob().broadcast(signed_tx.raw_bytes()).await?;
    // println!("✅ Order modified on-chain! TxHash: {}", result.txhash);

    println!("\n✅ Example completed successfully!");
    println!("   Next step: Broadcast the raw_bytes to a Sentry node.");

    Ok(())
}
