//! Example: AI Agent performing a trade with verified TradingKeyClaim.
//!
//! This example demonstrates the full secure agent trading flow:
//! - Create an AgentSigner with a TradingKey
//! - Build a TradingKeyClaim and check it against the issuer key
//! - Attach the claim to a transaction (embedded in SignerInfo)
//! - Sign using the delegated Trading Key
//!
//! Run with:
//! ```bash
//! cargo run -p morpheum-sdk-examples --example native_agent_trade
//! ```

use morpheum_sdk_native::core::signing::signer::Signer;
use morpheum_sdk_native::prelude::*;
use morpheum_sdk_native::signing::types::{AccountId as SigningAccountId, Signature};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🤖 Morpheum Native SDK - Agent Trade with Verified Claim Example");

    let owner = NativeSigner::from_seed(&[0x11; 32]);
    let agent_id: SigningAccountId = NativeSigner::from_seed(&[0xAA; 32]).account_id();
    let agent_signer = AgentSigner::new(&[0xAA; 32], agent_id, None);

    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    // Non-zero placeholder. `verify` checks the issuer key, not this signature.
    let claim = VcClaimBuilder::new()
        .issuer(owner.account_id())
        .subject(agent_signer.account_id())
        .permissions(0b0001)
        .max_daily_usd(1_000_000)
        .expiry(now_secs + 86_400)
        .nonce_sub_range(5000, 6000)
        .signature(Signature::Ed25519([1u8; 64]))
        .build(now_secs)?;

    println!("✅ TradingKeyClaim built");

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
        .memo("Agent executing trade with verified TradingKeyClaim")
        .with_trading_key_claim(claim)
        .add_message(Any {
            type_url: "/market.v1.MsgCreateOrder".to_string(),
            value: vec![1, 2, 3, 4],
        })
        .sign()
        .await?;

    println!("🔏 Trade transaction signed by Agent");
    println!("   TxHash : {}", signed_tx.txhash_hex());
    println!("   Size   : {} bytes", signed_tx.raw_bytes().len());

    // In production you would broadcast the transaction:
    // let result = sdk.market().broadcast(signed_tx.raw_bytes()).await?;
    // println!("✅ Trade executed on-chain! TxHash: {}", result.txhash);

    println!("\n🎉 Agent trade example completed successfully!");
    println!(
        "   The TradingKeyClaim was verified, embedded, and covered by the agent's signature."
    );

    Ok(())
}
