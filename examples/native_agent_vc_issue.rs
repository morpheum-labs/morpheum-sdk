//! Example: AI Agent issuing a Verifiable Credential (VC) with TradingKey delegation.
//!
//! This example demonstrates the recommended pattern for autonomous agents:
//! - Using `agent()` convenience constructor with `AgentSigner`
//! - Building a `TradingKeyClaim` using `VcClaimBuilder`
//! - Attaching the claim to a transaction (embedded in SignerInfo)
//! - Signing the VC issue message with the delegated Trading Key
//!
//! Run with:
//! ```bash
//! cargo run -p morpheum-sdk-examples --example native_agent_vc_issue
//! ```

use morpheum_sdk_native::core::signing::signer::Signer;
use morpheum_sdk_native::prelude::*;
use morpheum_sdk_native::signing::types::{AccountId as SigningAccountId, Signature};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🤖 Morpheum Native SDK - Agent VC Issuance Example");

    let owner = NativeSigner::from_seed(&[0x11; 32]);
    let agent_id: SigningAccountId = NativeSigner::from_seed(&[0xAA; 32]).account_id();
    let agent_signer = AgentSigner::new(&[0xAA; 32], agent_id, None);

    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    // Non-zero placeholder. `build` checks that a signature is present.
    // It does not check the curve. `verify` checks the issuer key.
    let claim = VcClaimBuilder::new()
        .issuer(owner.account_id())
        .subject(agent_signer.account_id())
        .permissions(0b0001)
        .max_daily_usd(500_000)
        .expiry(now_secs + 86_400)
        .nonce_sub_range(1000, 2000)
        .signature(Signature::Ed25519([1u8; 64]))
        .build(now_secs)?;

    println!("✅ TradingKeyClaim built successfully");
    println!("   Issuer : {:?}", claim.issuer);
    println!("   Subject: {:?}", claim.subject);
    println!(
        "   Expires: {} seconds from now",
        claim.expiry_timestamp - now_secs
    );

    claim.verify(now_secs, &owner.public_key())?;
    println!("✅ TradingKeyClaim verified successfully");

    // `agent()` does not keep the signer. Pass that same signer to `TxBuilder`.
    let _sdk = agent(agent_signer.clone());

    // The target chain's genesis hash, from operator configuration (never from
    // the node you submit to). `sign()` refuses a transaction built without one.
    let genesis_hash = [0u8; 32];
    let signed_tx = TxBuilder::new(agent_signer)
        .chain_id("morpheum-test-1")
        .with_genesis_hash(genesis_hash)
        .memo("Agent issuing VC with delegated TradingKey")
        .with_trading_key_claim(claim)
        .add_message(Any {
            type_url: "/vc.v1.MsgIssue".to_string(),
            value: vec![1, 2, 3],
        })
        .sign()
        .await?;

    println!("🔏 Transaction signed successfully by Agent");
    println!("   TxHash : {}", signed_tx.txhash_hex());
    println!("   Size   : {} bytes", signed_tx.raw_bytes().len());

    // In production you would broadcast:
    // let result = sdk.auth().broadcast(signed_tx.raw_bytes()).await?;
    // println!("✅ VC issued on-chain! TxHash: {}", result.txhash);

    println!("\n🎉 Agent VC issuance example completed successfully!");
    println!(
        "   The TradingKeyClaim was embedded in the transaction and covered by the signature."
    );

    Ok(())
}
