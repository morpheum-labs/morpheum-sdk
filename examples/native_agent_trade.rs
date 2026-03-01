//! Example: AI Agent performing a trade with verified TradingKeyClaim.
//!
//! This example demonstrates the full secure agent trading flow:
//! - Create an AgentSigner with a TradingKey
//! - Build and cryptographically verify a TradingKeyClaim
//! - Attach the verified claim to a transaction (embedded in SignerInfo)
//! - Sign and broadcast using the delegated Trading Key
//!
//! Run with:
//! ```bash
//! cargo run -p morpheum-sdk-examples --example native_agent_trade
//! ```

use morpheum_sdk_native::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🤖 Morpheum Native SDK - Agent Trade with Verified Claim Example");

    // 1. Create the AgentSigner (the delegated trading key)
    let agent_signer = AgentSigner::new(
        b"agent-trading-key-seed-32-bytes-long!!!",
        AccountId::new([0xAA; 32]), // Agent's own AccountId
        None,
    );

    // 2. Build a TradingKeyClaim (issued by the owner to this agent)
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let claim = VcClaimBuilder::new()
        .issuer(AccountId::new([0x11; 32]))           // Owner (human) as issuer
        .subject(agent_signer.account_id())           // This agent as subject
        .permissions(0b0001)                          // TRADE permission (bit 0)
        .max_daily_usd(1_000_000)                     // $1M daily limit
        .expiry(now_secs + 86_400)                    // Valid for 24 hours
        .nonce_sub_range(5000, 6000)                  // Isolated nonce range for safe parallelism
        .build(now_secs)?;

    println!("✅ TradingKeyClaim built");

    // 3. Verify the claim before using it (critical security step)
    claim.verify(now_secs, &AccountId::new([0x11; 32]))?;
    println!("✅ TradingKeyClaim verified successfully");

    // 4. Create the SDK using the AgentSigner
    let sdk = agent(agent_signer);

    // 5. Build a trade transaction (example: placing a market order)
    // In a real application this would be a proper MsgCreateOrder or similar
    let signed_tx = TxBuilder::new(sdk.signer.clone())
        .chain_id("morpheum-test-1")
        .memo("Agent executing trade with verified TradingKeyClaim")
        .with_trading_key_claim(claim)                    // ← Claim is embedded and signed
        .add_message(prost_types::Any {
            type_url: "/market.v1.MsgCreateOrder".to_string(),
            value: vec![1, 2, 3, 4], // Placeholder for real order message
        })
        .sign()
        .await?;

    println!("🔏 Trade transaction signed by Agent");
    println!("   TxHash : {}", signed_tx.txhash_hex());
    println!("   Size   : {} bytes", signed_tx.raw_bytes().len());

    // 6. In production you would broadcast the transaction:
    // let result = sdk.market().broadcast(signed_tx.raw_bytes()).await?;
    // println!("✅ Trade executed on-chain! TxHash: {}", result.txhash);

    println!("\n🎉 Agent trade example completed successfully!");
    println!("   The TradingKeyClaim was verified, embedded, and covered by the agent's signature.");

    Ok(())
}