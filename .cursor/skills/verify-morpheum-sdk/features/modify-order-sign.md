# Modify order sign

Modify order sign builds a CLOB amend request from a fixed seed and signs that request. The message type URL is `/clob.v1.MsgModifyOrderRequest`. The program prints the tx hash and exits. It does not broadcast.

## Sub-features

- `modify-order-build` builds a `ModifyOrderRequest` with `ModifyOrderBuilder`.
- `modify-order-sign` signs that request and prints `TxHash`.
- `modify-order-stable` a second run prints the same hash.
- `modify-order-offline` the example binary exits 0 with that hash inside an empty network namespace.

## How to get to it (user POV)

- Run `cargo run -p morpheum-sdk-examples --example native_modify_order` from the repo root.

## Driving it with verify-morpheum-sdk

Preconditions:

- `doctor` prints `doctor ok` for this run.
- `examples/Cargo.toml` declares example `native_modify_order`.
- The examples package enables the native `clob` feature. `full` does not include `clob`.
- The sibling manifests listed in the index exist.

- **User command.** The drive session runs `cargo run -p morpheum-sdk-examples --example native_modify_order`. Exit code `0`. Stdout contains `Modify Order Example`, `Modify order request built successfully`, `Transaction signed successfully`, and `Example completed successfully`.
- **Hash.** Read `TxHash` from that stdout. It is 64 hex characters. This example does not print `Size`.
- **Empty netns, twice.** Run `unshare --net --map-root-user -- target/debug/examples/native_modify_order` twice. Both exit `0` and print the same hash as the cargo run.
- **Proof.** `evidence` prints `result=PASS` and `feature=modify-order-sign`.

## Gotchas

- The signer seed is `[0x42; 32]`. The chain id comes from `sdk.config().default_chain_id`, which `native()` sets to `morpheum-1`. The genesis hash is 32 zero bytes.
- `ModifyOrderBuilder::address` takes `impl Into<String>`. The signing `AccountId` has no `Display`. The example passes the SDK `AccountId` hex of `signer.account_id()`.
- The order id is `order-test-1`. The symbol is `BTC-USDC`. The new price is `101`. The new quantity is `2`. The proto `timestamp` stays unset. The default nonce is all zeros, so the hash does not depend on the clock.
- A change to those literals changes the hash. Compare three runs of the same binary; do not freeze a hash in this file.
- The examples package depends on `morpheum-sdk-native` with `features = ["full", "clob"]`. Do not expect `full` alone to export `morpheum_sdk_native::clob`.
- This drive signs an amend. It does not build a `CancelOrderBuilder` cancel.
- `cargo run` inside the empty netns is the wrong process to watch. Run the example binary there.
- The broadcast call in the example source is a comment. The netns exit code is the check that the process finished without a route.
