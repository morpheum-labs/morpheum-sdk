# Cancel order sign

Cancel order sign builds a CLOB cancel request from a fixed seed and signs that request. The message type URL is `/clob.v1.MsgCancelOrderRequest`. The program prints the tx hash and exits. It does not broadcast.

## Sub-features

- `cancel-order-build` builds a `CancelOrderRequest` with `CancelOrderBuilder`.
- `cancel-order-sign` signs that request and prints `TxHash`.
- `cancel-order-stable` a second run prints the same hash.
- `cancel-order-offline` the example binary exits 0 with that hash inside an empty network namespace.

## How to get to it (user POV)

- Run `cargo run -p morpheum-sdk-examples --example native_cancel_order` from the repo root.

## Driving it with verify-morpheum-sdk

Preconditions:

- `doctor` prints `doctor ok` for this run.
- `examples/Cargo.toml` declares example `native_cancel_order`.
- The examples package enables the native `clob` feature. `full` does not include `clob`.
- The sibling manifests listed in the index exist.

- **User command.** The drive session runs `cargo run -p morpheum-sdk-examples --example native_cancel_order`. Exit code `0`. Stdout contains `Cancel Order Example`, `Cancel order request built successfully`, `Transaction signed successfully`, and `Example completed successfully`.
- **Hash.** Read `TxHash` from that stdout. It is 64 hex characters. This example does not print `Size`.
- **Empty netns, twice.** Run `unshare --net --map-root-user -- target/debug/examples/native_cancel_order` twice. Both exit `0` and print the same hash as the cargo run.
- **Proof.** `evidence` prints `result=PASS` and `feature=cancel-order-sign`.

## Gotchas

- The signer seed is `[0x42; 32]`. The chain id comes from `sdk.config().default_chain_id`, which `native()` sets to `morpheum-1`. The genesis hash is 32 zero bytes.
- `CancelOrderBuilder::address` takes `impl Into<String>`. The signing `AccountId` has no `Display`. The example passes the SDK `AccountId` hex of `signer.account_id()`.
- The order id is `order-test-1`. The symbol is `BTC-USDC`. The proto `timestamp` stays unset. The default nonce is all zeros, so the hash does not depend on the clock.
- A change to those literals changes the hash. Compare three runs of the same binary; do not freeze a hash in this file.
- The examples package depends on `morpheum-sdk-native` with `features = ["full", "clob"]`. Do not expect `full` alone to export `morpheum_sdk_native::clob`.
- This drive signs a cancel. It does not build a `ModifyOrderBuilder` amend.
- `cargo run` inside the empty netns is the wrong process to watch. Run the example binary there.
- The broadcast call in the example source is a comment. The netns exit code is the check that the process finished without a route.
