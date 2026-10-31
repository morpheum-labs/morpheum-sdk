//! Example: Providing a CLOB market-maker quote using the Morpheum Native SDK.
//!
//! This example demonstrates the recommended way to provide a quote using:
//! - `native()` convenience constructor
//! - `NativeSigner` from seed (for deterministic testing)
//! - Fluent `ProvideMarketMakerQuoteBuilder` for a
//!   `/clob.v1.MsgProvideMarketMakerQuoteRequest`
//! - Signing via `TxBuilder` (the broadcast call stays a comment)
//!
//! `From<ProvideMarketMakerQuoteRequest>` sets `timestamp`, `duration`, and
//! `provider_external_address` to `None`, so the signed bytes do not depend
//! on the clock.
//!
//! Run with:
//! ```bash
//! cargo run -p morpheum-sdk-examples --example native_provide_mm_quote
//! ```

use morpheum_sdk_native::clob::{ProvideMarketMakerQuoteBuilder, Side};
use morpheum_sdk_native::core::signing::signer::Signer;
use morpheum_sdk_native::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Morpheum Native SDK - Provide MM Quote Example");

    // 1. Create a deterministic signer (in production, load from secure storage or mnemonic)
    let signer = NativeSigner::from_seed(&[0x42; 32]);

    // 2. Create the main SDK instance
    let sdk = native(signer.clone());

    // 3. Build the quote. `ProvideMarketMakerQuoteBuilder::provider` takes a
    //    string. The signing `AccountId` has no `Display`; the SDK `AccountId`
    //    renders those 32 bytes as hex. The builder does not set a timestamp
    //    or duration, and `to_any()` leaves both unset.
    let provider = AccountId::from(signer.account_id()).to_string();
    let quote = ProvideMarketMakerQuoteBuilder::new()
        .provider(provider)
        .pool_id("pool-1")
        .market_index(0)
        .side(Side::Buy)
        .price("100")
        .amount("1")
        .build()?;

    println!("📋 Provide market maker quote request built successfully");

    // 4. Create the transaction using TxBuilder and sign it.
    // The target chain's genesis hash, from operator configuration (never from
    // the node you submit to). `sign()` refuses a transaction built without one.
    let genesis_hash = [0u8; 32]; // Replace with the configured genesis hash
    let signed_tx = TxBuilder::new(signer)
        .chain_id(sdk.config().default_chain_id.clone())
        .with_genesis_hash(genesis_hash)
        .memo("Providing market maker quote on market 0 via SDK example")
        .add_message(quote.to_any())
        .sign()
        .await?;

    println!("🔏 Transaction signed successfully");
    println!("   TxHash: {}", signed_tx.txhash_hex());

    // 5. In a real application, you would now broadcast the transaction:
    // let result = sdk.clob().broadcast(signed_tx.raw_bytes()).await?;
    // println!("✅ Quote provided on-chain! TxHash: {}", result.txhash);

    println!("\n✅ Example completed successfully!");
    println!("   Next step: Broadcast the raw_bytes to a Sentry node.");

    Ok(())
}
