/**
 * Morpheum WASM SDK - Browser Taproot (Bitcoin) Market Creation Example
 *
 * This example demonstrates how to create a market on Morpheum using the WASM SDK
 * with a Taproot wallet (Unisat, Leather, Xverse, etc.).
 *
 * Key features shown:
 * - Proper panic hook setup for better error messages
 * - Async Taproot wallet connection via factory method
 * - Fluent TxBuilderWasm API for transaction construction
 * - Adding a generic protobuf message (market creation)
 * - Signing with the connected Taproot wallet using BIP-322
 * - Clean error handling and user-friendly logging
 *
 * Run this example in a modern browser with Unisat (or compatible Taproot wallet) installed.
 */

import {
    TxBuilderWasm,
    setPanicHook,
    type SignedTx,
} from '@morpheum/sdk';

// Enable better panic messages in the browser console
setPanicHook();

async function main(): Promise<void> {
    console.log('🚀 Morpheum WASM SDK - Taproot (Bitcoin) Market Creation Example');
    console.log('Connecting to Taproot wallet (Unisat/Leather/Xverse)...');

    try {
        // 1. Create TxBuilder configured for Taproot wallet
        //    This connects to window.unisat and requests account access
        const builder = await TxBuilderWasm.newTaproot()
            .chainId('morpheum-test-1')
            .memo('Market creation from Taproot wallet via Morpheum WASM SDK');

        console.log('✅ Connected to Taproot wallet successfully');

        // 2. Prepare a market creation message
        //    In a real app, encode a proper protobuf message here
        const marketMsgBytes = new Uint8Array([
            // Placeholder for real MsgCreateMarketRequest
        ]);

        // 3. Build and sign the transaction using the fluent API
        const signedTx: SignedTx = await builder
            .addMessage(
                'type.googleapis.com/market.v1.MsgCreateMarketRequest',
                marketMsgBytes
            )
            .sign();

        console.log('🔏 Transaction signed successfully with Taproot wallet!');
        console.log('   TxHash          :', signedTx.txhash);
        console.log('   Raw bytes length:', signedTx.raw_bytes.length);

        // 4. In a real dApp, broadcast the transaction
        //    Example:
        //    const response = await fetch('/api/broadcast', {
        //        method: 'POST',
        //        body: signedTx.raw_bytes,
        //    });

        console.log('\n✅ Example completed successfully!');
        console.log('   Next step: Broadcast signedTx.raw_bytes to a Sentry node.');

    } catch (error: any) {
        console.error('❌ Failed to create and sign market transaction:');
        console.error('   ', error.message || error);

        if (error.message?.includes('Unisat') || error.message?.includes('Taproot')) {
            console.error('💡 Make sure a Taproot wallet (Unisat, Leather, or Xverse) is installed and unlocked.');
        }
    }
}

// Run the example
main().catch((err) => {
    console.error('💥 Unhandled error in main():', err);
});