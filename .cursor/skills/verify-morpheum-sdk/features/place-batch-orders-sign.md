# Place batch orders sign

Place batch orders sign builds one CLOB place request, wraps it in a batch, and signs that batch. The message type URL is `/clob.v1.MsgPlaceBatchOrdersRequest`. The program prints the tx hash and exits. It does not broadcast. The hash can change between runs because `PlaceBatchOrdersRequest::to_any()` under `std` stamps each order with the clock.

## Sub-features

- `place-batch-orders-build` builds one `PlaceOrderRequest` with `PlaceOrderBuilder` and a `PlaceBatchOrdersRequest` with `PlaceBatchOrdersBuilder`.
- `place-batch-orders-sign` signs that request and prints `TxHash`.
- `place-batch-orders-offline` the example binary exits 0 with a 64-hex `TxHash` inside an empty network namespace.

## How to get to it (user POV)

- Run `cargo run -p morpheum-sdk-examples --example native_place_batch_orders` from the repo root.

## Driving it with verify-morpheum-sdk

Preconditions:

- `doctor` prints `doctor ok` for this run.
- `examples/Cargo.toml` declares example `native_place_batch_orders`.
- The examples package enables the native `clob` feature. `full` does not include `clob`.
- The sibling manifests listed in the index exist.

- **User command.** The drive session runs `cargo run -p morpheum-sdk-examples --example native_place_batch_orders`. Exit code `0`. Stdout contains `Place Batch Orders Example`, `Place batch orders request built successfully`, `Transaction signed successfully`, and `Example completed successfully`.
- **Hash.** Read `TxHash` from that stdout. It is 64 hex characters. A later run of the same binary may print a different hash.
- **Empty netns, twice.** Run `unshare --net --map-root-user -- target/debug/examples/native_place_batch_orders` twice. Both exit `0` and print a 64-hex `TxHash`. The three hashes may differ.
- **Proof.** `evidence` prints `result=PASS` and `feature=place-batch-orders-sign`. `cargo_exit`, `netns_1_exit`, and `netns_2_exit` are `0`.

## Gotchas

- The signer seed is `[0x42; 32]`. The chain id comes from `sdk.config().default_chain_id`, which `native()` sets to `morpheum-1`. The genesis hash is 32 zero bytes.
- `PlaceOrderBuilder::address` and `PlaceBatchOrdersBuilder::from_address` take `impl Into<String>`. The signing `AccountId` has no `Display`. The example passes the SDK `AccountId` hex of `signer.account_id()`.
- The batch holds one order. The market index is `0`. The price is `100`. The quantity is `1`. The side is `Side::Buy`. The order type is `OrderType::Limit`. TimeInForce stays the default `Gtc`. `client_order_id`, `credential_proof`, and `expiry_timestamp` stay unset. `orders_hash` stays unset.
- `PlaceBatchOrdersRequest::to_any()` under `std` calls `to_proto_with_canonical_hash()`. That stamps every order with `SystemTime::now()` and, when `orders_hash` is empty, computes `orders_hash` from that timestamp. Three runs do not have to share a hash. Differing hashes are not a failure for this feature. This feature is not in the stable-hash set.
- Do not freeze the clock in library code to make this hash stable. The example leaves the timestamp path as the library implements it.
- The examples package depends on `morpheum-sdk-native` with `features = ["full", "clob"]`. Do not expect `full` alone to export `morpheum_sdk_native::clob`.
- This drive signs a batch of one place. It does not build a `CancelOrderBuilder` cancel, a `ModifyOrderBuilder` amend, or a lone `PlaceOrderRequest` message.
- `cargo run` inside the empty netns is the wrong process to watch. Run the example binary there.
- The broadcast call in the example source is a comment. The netns exit code is the check that the process finished without a route.
