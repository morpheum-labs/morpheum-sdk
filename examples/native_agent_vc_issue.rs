//! Example: AI Agent issuing a Verifiable Credential (VC) with TradingKey delegation.
//!
//! This example demonstrates the recommended pattern for autonomous agents:
//! - Using `agent()` convenience constructor with `AgentSigner`
//! - Building a `TradingKeyClaim` using `VcClaimBuilder`
//! - Attaching the claim to a transaction (embedded in SignerInfo)
//! - Signing with the delegated Trading Key
//!
//! Run with:
//! ```bash
//! cargo run -p morpheum-sdk-examples --example native_agent_vc_issue
//! ```

use morpheum_sdk_native::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🤖 Morpheum Native SDK - Agent VC Issuance Example");

    // 1. Create the owner (human) and agent signers
    let owner_signer = NativeSigner::from_seed(b"owner-secret-seed-32-bytes-long!!!");
    let agent_signer = AgentSigner::new(
        b"agent-trading-key-seed-32-bytes-long!!!",
        AccountId::new([0xAA; 32]), // Agent's AccountId
        None,                       // Optional claim (will be built below)
    );

    // 2. Build a TradingKeyClaim — this is the core of agent delegation
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let claim = VcClaimBuilder::new()
        .issuer(owner_signer.account_id())           // Owner (human) is the issuer
        .subject(agent_signer.account_id())          // Agent is the subject
        .permissions(0b0001)                         // Example: TRADE permission (bit 0)
        .max_daily_usd(500_000)                      // $500k daily spending limit
        .expiry(now_secs + 86_400)                   // 24 hours from now
        .nonce_sub_range(1000, 2000)                 // Isolated nonce range for parallelism
        .build(now_secs)?;

    println!("✅ TradingKeyClaim built successfully");
    println!("   Issuer : {}", claim.issuer);
    println!("   Subject: {}", claim.subject);
    println!("   Expires: {} seconds from now", claim.expiry_timestamp - now_secs);

    // 3. Create the SDK using the AgentSigner
    let sdk = agent(agent_signer);

    // 4. Build a transaction that includes the TradingKeyClaim
    //    (In this example we issue a VC, but any transaction works)
    let signed_tx = TxBuilder::new(sdk.signer.clone()) // Agent signs with delegated key
        .chain_id("morpheum-test-1")
        .memo("Agent issuing VC with delegated TradingKey")
        .with_trading_key_claim(claim)                    // ← Critical: claim is embedded
        .add_message(prost_types::Any {
            type_url: "/vc.v1.MsgIssue".to_string(),
            value: vec![1, 2, 3], // Placeholder for real VC issuance message
        })
        .sign()
        .await?;

    println!("🔏 Transaction signed successfully by Agent");
    println!("   TxHash : {}", signed_tx.txhash_hex());
    println!("   Size   : {} bytes", signed_tx.raw_bytes().len());

    // 5. In production you would broadcast:
    // let result = sdk.auth().broadcast(signed_tx.raw_bytes()).await?;
    // println!("✅ VC issued on-chain! TxHash: {}", result.txhash);

    println!("\n🎉 Agent VC issuance example completed successfully!");
    println!("   The TradingKeyClaim was embedded in the transaction and covered by the signature.");

    Ok(())
}