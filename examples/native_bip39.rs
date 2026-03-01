//! Example: Using BIP-39 mnemonic with NativeSigner in the Morpheum SDK.
//!
//! This example demonstrates the recommended way for humans to use the SDK:
//! - Load a wallet from a BIP-39 mnemonic phrase (standard 12 or 24 words)
//! - Optional passphrase support (for extra security)
//! - Create a NativeSigner and use it with the SDK
//! - Sign a transaction using the derived key
//!
//! Run with:
//! ```bash
//! cargo run -p morpheum-sdk-examples --example native_bip39
//! ```

use morpheum_sdk_native::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🔑 Morpheum Native SDK - BIP-39 Mnemonic Example");

    // =============================================
    // 1. Standard test mnemonic (12 words)
    // =============================================
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    println!("📝 Using mnemonic:");
    println!("   {}", mnemonic);

    // =============================================
    // 2. Create NativeSigner from mnemonic
    // =============================================
    // No passphrase (most common case)
    let signer = NativeSigner::from_mnemonic(mnemonic, "")?;

    println!("✅ NativeSigner created successfully from mnemonic");

    // Optional: Show how to use a passphrase (for extra security)
    // let signer_with_passphrase = NativeSigner::from_mnemonic(mnemonic, "my-super-secret-passphrase")?;

    // =============================================
    // 3. Create the SDK using the mnemonic-derived signer
    // =============================================
    let sdk = native(signer);

    // =============================================
    // 4. Build and sign a simple transaction
    // =============================================
    let signed_tx = TxBuilder::new(sdk.signer.clone())
        .chain_id("morpheum-test-1")
        .memo("Transaction signed using BIP-39 mnemonic via Morpheum SDK")
        .add_message(prost_types::Any {
            type_url: "/market.v1.MsgCreateMarketRequest".to_string(),
            value: vec![1, 2, 3, 4], // Placeholder for a real market creation message
        })
        .sign()
        .await?;

    println!("🔏 Transaction signed successfully using mnemonic-derived key");
    println!("   TxHash : {}", signed_tx.txhash_hex());
    println!("   Size   : {} bytes", signed_tx.raw_bytes().len());

    // =============================================
    // 5. In production you would broadcast the transaction:
    // =============================================
    // let result = sdk.market().broadcast(signed_tx.raw_bytes()).await?;
    // println!("✅ Transaction broadcasted! TxHash: {}", result.txhash);

    println!("\n🎉 BIP-39 mnemonic example completed successfully!");
    println!("   The private key was derived deterministically from the mnemonic phrase.");
    println!("   In real applications, never hardcode mnemonics — load them securely.");

    Ok(())
}