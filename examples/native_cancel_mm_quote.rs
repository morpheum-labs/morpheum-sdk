//! Example: Cancelling a CLOB market-maker quote using the Morpheum Native SDK.
//!
//! This example demonstrates the recommended way to cancel a quote using:
//! - `native()` convenience constructor
//! - `NativeSigner` from seed (for deterministic testing)
//! - Fluent `CancelMarketMakerQuoteBuilder` for a
//!   `/clob.v1.MsgCancelMarketMakerQuoteRequest`
//! - Signing via `TxBuilder` (the broadcast call stays a comment)
//!
//! `From<CancelMarketMakerQuoteRequest>` sets `timestamp` to `None`, so the
//! signed bytes do not depend on the clock.
//!
//! Run with:
//! ```bash
//! cargo run -p morpheum-sdk-examples --example native_cancel_mm_quote
//! ```

use morpheum_sdk_native::clob::CancelMarketMakerQuoteBuilder;
use morpheum_sdk_native::core::signing::signer::Signer;
use morpheum_sdk_native::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Morpheum Native SDK - Cancel MM Quote Example");

    let signer = NativeSigner::from_seed(&[0x42; 32]);
    let sdk = native(signer.clone());

    // `provider` takes a string. The signing `AccountId` has no `Display`.
    // The SDK `AccountId` renders those 32 bytes as hex. `to_any()` leaves
    // `timestamp` unset, so the signed bytes do not depend on the clock.
    let provider = AccountId::from(signer.account_id()).to_string();
    let cancel = CancelMarketMakerQuoteBuilder::new()
        .quote_id("quote-1")
        .provider(provider)
        .build()?;

    println!("📋 Cancel market maker quote request built successfully");

    // Genesis hash comes from operator configuration. `sign()` refuses a
    // transaction built without one.
    let genesis_hash = [0u8; 32];
    let signed_tx = TxBuilder::new(signer)
        .chain_id(sdk.config().default_chain_id.clone())
        .with_genesis_hash(genesis_hash)
        .memo("Cancelling market maker quote quote-1 via SDK example")
        .add_message(cancel.to_any())
        .sign()
        .await?;

    println!("🔏 Transaction signed successfully");
    println!("   TxHash: {}", signed_tx.txhash_hex());

    // let result = sdk.clob().broadcast(signed_tx.raw_bytes()).await?;
    // println!("✅ Quote cancelled on-chain! TxHash: {}", result.txhash);

    println!("\n✅ Example completed successfully!");
    println!("   Next step: Broadcast the raw_bytes to a Sentry node.");

    Ok(())
}
