# Market sign

Market sign builds a perp market-create request from a fixed seed and signs that request. The program prints the tx hash and exits. It does not broadcast.

## Sub-features

- `market-build` builds a `CreateMarketRequest` for a perp market on the `clob` book.
- `market-sign` signs that request and prints `TxHash`.
- `market-stable` a second run prints the same hash.
- `market-offline` the example binary exits 0 with that hash inside an empty network namespace.

## How to get to it (user POV)

- Run `cargo run -p morpheum-sdk-examples --example native_market` from the repo root.

## Driving it with verify-morpheum-sdk

Preconditions:

- `doctor` prints `doctor ok` for this run.
- `examples/Cargo.toml` declares example `native_market`.
- The sibling manifests listed in the index exist.

- **User command.** The drive session runs `cargo run -p morpheum-sdk-examples --example native_market`. Exit code `0`. Stdout contains `Market Creation Example`, `Market creation request built successfully`, `Transaction signed successfully`, and `Example completed successfully`.
- **Hash.** Read `TxHash` from that stdout. It is 64 hex characters. This example does not print `Size`.
- **Empty netns, twice.** Run `unshare --net --map-root-user -- target/debug/examples/native_market` twice. Both exit `0` and print the same hash as the cargo run.
- **Proof.** `evidence` prints `result=PASS` and `feature=market-sign`.

## Gotchas

- The signer seed is `[0x42; 32]`. The chain id comes from `sdk.config().default_chain_id`, which `native()` sets to `morpheum-1`. The genesis hash is 32 zero bytes.
- The default nonce is all zeros, so the hash does not depend on the clock.
- Terms are the integer strings in the example (`tick_size` `100`, `max_leverage` `20`, orderbook `clob`, market type perp). A change to those literals changes the hash. Compare three runs of the same binary; do not freeze a hash in this file.
- `cargo run` inside the empty netns is the wrong process to watch. Run the example binary there.
