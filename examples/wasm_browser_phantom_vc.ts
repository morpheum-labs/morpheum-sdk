/**
 * Morpheum WASM SDK - Browser Phantom VC Issuance Example
 *
 * This example demonstrates how an AI agent or dApp can issue a Verifiable Credential (VC)
 * using the Morpheum WASM SDK with Phantom wallet (Solana).
 *
 * Key features shown:
 * - Proper panic hook setup for better error messages
 * - Async Phantom wallet connection via factory method
 * - Building a TradingKeyClaim using VcClaimBuilder (for agent delegation)
 * - Attaching the claim to a transaction (embedded in SignerInfo)
 * - Signing with the connected Phantom wallet
 * - Clean error handling and user-friendly logging
 *
 * Run this example in a modern browser with Phantom wallet installed and unlocked.
 */

import {
    TxBuilderWasm,
    VcClaimBuilder,
    setPanicHook,
    type SignedTx,
} from '@morpheum/sdk';

// Enable better panic messages in the browser console
setPanicHook();

async function main(): Promise<void> {
    console.log('🤖 Morpheum WASM SDK - Phantom VC Issuance Example');
    console.log('Connecting to Phantom wallet...');

    try {
        // 1. Create TxBuilder configured for Phantom (Solana)
        //    This connects to window.phantom.solana and requests wallet access
        const builder = await TxBuilderWasm.newPhantom()
            .chainId('morpheum-test-1')
            .memo('Agent issuing VC with delegated TradingKey via Phantom');

        console.log('✅ Connected to Phantom wallet successfully');

        // 2. Build a TradingKeyClaim for agent delegation
        //    In a real app, this would be built by the owner and signed by them
        const nowSecs = Math.floor(Date.now() / 1000);

        const claim = new VcClaimBuilder()
            .issuer(new Uint8Array(32).fill(1))     // Owner (issuer)
            .subject(new Uint8Array(32).fill(2))    // Agent (subject)
            .permissions(0x01)                      // TRADE permission
            .maxDailyUsd(250_000)                   // $250k daily limit
            .expiry(nowSecs + 86_400)               // Valid for 24 hours
            .nonceSubRange(1000, 2000)              // Isolated nonce range for parallelism
            .signature(new Uint8Array(64).fill(42), "ed25519") // Placeholder owner signature
            .build(nowSecs);

        console.log('✅ TradingKeyClaim built successfully');

        // 3. Build and sign the transaction with the attached claim
        const signedTx: SignedTx = await builder
            .withClaim(claim)                       // ← Critical: claim is embedded
            .addMessage(
                'type.googleapis.com/vc.v1.MsgIssue',
                new Uint8Array([1, 2, 3, 4])        // Placeholder for real VC issuance message
            )
            .sign();

        console.log('🔏 Transaction signed successfully with Phantom!');
        console.log('   TxHash          :', signedTx.txhash);
        console.log('   Raw bytes length:', signedTx.raw_bytes.length);

        // 4. In a real dApp, broadcast the transaction
        //    Example:
        //    const response = await fetch('/api/broadcast', {
        //        method: 'POST',
        //        body: signedTx.raw_bytes,
        //    });

        console.log('\n✅ Example completed successfully!');
        console.log('   The TradingKeyClaim was embedded and covered by the Phantom signature.');

    } catch (error: any) {
        console.error('❌ Failed to create and sign VC issuance transaction:');
        console.error('   ', error.message || error);

        if (error.message?.includes('Phantom')) {
            console.error('💡 Make sure Phantom wallet is installed and unlocked.');
        }
    }
}

// Run the example
main().catch((err) => {
    console.error('💥 Unhandled error in main():', err);
});