# Place order sign

Place order sign builds a CLOB place request from a fixed seed and signs that request. The message type URL is `/clob.v1.MsgPlaceOrderRequest`. The program prints the tx hash and exits. It does not broadcast.

## Sub-features

- `place-order-build` builds a `PlaceOrderRequest` with `PlaceOrderBuilder`.
- `place-order-sign` signs that request and prints `TxHash`.
- `place-order-stable` a second run prints the same hash.
- `place-order-offline` the example binary exits 0 with that hash inside an empty network namespace.

## How to get to it (user POV)

- Run `cargo run -p morpheum-sdk-examples --example native_place_order` from the repo root.

## Driving it with verify-morpheum-sdk

Preconditions:

- `doctor` prints `doctor ok` for this run.
- `examples/Cargo.toml` declares example `native_place_order`.
- The examples package enables the native `clob` feature. `full` does not include `clob`.
- The sibling manifests listed in the index exist.

- **User command.** The drive session runs `cargo run -p morpheum-sdk-examples --example native_place_order`. Exit code `0`. Stdout contains `Place Order Example`, `Place order request built successfully`, `Transaction signed successfully`, and `Example completed successfully`.
- **Hash.** Read `TxHash` from that stdout. It is 64 hex characters. This example does not print `Size`.
- **Empty netns, twice.** Run `unshare --net --map-root-user -- target/debug/examples/native_place_order` twice. Both exit `0` and print the same hash as the cargo run.
- **Proof.** `evidence` prints `result=PASS` and `feature=place-order-sign`.

## Gotchas

- The signer seed is `[0x42; 32]`. The chain id comes from `sdk.config().default_chain_id`, which `native()` sets to `morpheum-1`. The genesis hash is 32 zero bytes.
- `PlaceOrderBuilder::address` takes `impl Into<String>`. The signing `AccountId` has no `Display`. The example passes the SDK `AccountId` hex of `signer.account_id()`.
- The market index is `0`. The price is `100`. The quantity is `1`. The side is `Side::Buy`. The order type is `OrderType::Limit`. TimeInForce stays the default `Gtc`. `client_order_id`, `credential_proof`, and `expiry_timestamp` stay unset. The proto `timestamp` stays unset. The default nonce is all zeros, so the hash does not depend on the clock.
- A change to those literals changes the hash. Compare three runs of the same binary; do not freeze a hash in this file.
- The examples package depends on `morpheum-sdk-native` with `features = ["full", "clob"]`. Do not expect `full` alone to export `morpheum_sdk_native::clob`.
- This drive signs a place. It does not build a `CancelOrderBuilder` cancel or a `ModifyOrderBuilder` amend.
- `cargo run` inside the empty netns is the wrong process to watch. Run the example binary there.
- The broadcast call in the example source is a comment. The netns exit code is the check that the process finished without a route.
