/**
 * Morpheum WASM SDK - Browser MetaMask Market Creation Example
 *
 * This example demonstrates the recommended way to create a market on Morpheum
 * using the WASM SDK in the browser with MetaMask (or any compatible EVM wallet).
 *
 * Key features shown:
 * - Proper panic hook setup for better error messages
 * - Async MetaMask wallet connection via factory method
 * - Fluent TxBuilderWasm API for transaction construction
 * - Adding a generic protobuf message (market creation)
 * - Signing with the connected wallet
 * - Clean error handling and logging
 *
 * Run this example in a modern browser with MetaMask installed.
 */

import {
    TxBuilderWasm,
    setPanicHook,
    type SignedTx,
} from '@morpheum/sdk';

// Enable better panic messages in the browser console
setPanicHook();

async function main(): Promise<void> {
    console.log('🚀 Morpheum WASM SDK - MetaMask Market Creation Example');
    console.log('Connecting to MetaMask...');

    try {
        // 1. Create TxBuilder configured for MetaMask
        //    This connects to window.ethereum and requests account access
        const builder = await TxBuilderWasm.newMetamask()
            .chainId('morpheum-test-1')
            .memo('Market creation from MetaMask via Morpheum WASM SDK');

        console.log('✅ Connected to MetaMask successfully');

        // 2. Prepare a market creation message
        //    In a real app, you would encode a proper protobuf message here
        const marketMsgBytes = new Uint8Array([
            // Placeholder for a real MsgCreateMarketRequest
            // In production: use protobuf encoding from your generated types
        ]);

        // 3. Build and sign the transaction using the fluent API
        const signedTx: SignedTx = await builder
            .addMessage(
                'type.googleapis.com/market.v1.MsgCreateMarketRequest',
                marketMsgBytes
            )
            .sign();

        console.log('🔏 Transaction signed successfully!');
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

        if (error.message?.includes('MetaMask')) {
            console.error('💡 Make sure MetaMask is installed and unlocked.');
        }
    }
}

// Run the example
main().catch((err) => {
    console.error('💥 Unhandled error in main():', err);
});