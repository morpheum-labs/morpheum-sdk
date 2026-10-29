# BIP-39 sign

BIP-39 sign derives a native signer from the standard 12-word test mnemonic and signs one transaction on this machine. The program prints the tx hash and the signed byte length, then exits.

## Sub-features

- `bip39-derive` builds a `NativeSigner` from the test mnemonic and an empty passphrase.
- `bip39-sign` signs one message and prints `TxHash` and `Size`.
- `bip39-stable` a second run prints the same hash and the same size.
- `bip39-offline` the example binary still exits 0 with that hash inside an empty network namespace.

## How to get to it (user POV)

- Run `cargo run -p morpheum-sdk-examples --example native_bip39` from the repo root.

## Driving it with verify-morpheum-sdk

Preconditions:

- `doctor` prints `doctor ok` for this run.
- `examples/Cargo.toml` declares example `native_bip39`.
- The sibling manifests listed in the index exist.

- **User command.** Run the example the way a caller does. The drive session runs `cargo run -p morpheum-sdk-examples --example native_bip39`. Exit code `0`. Stdout contains `BIP-39 Mnemonic Example`, the mnemonic `abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about`, `NativeSigner created successfully`, `Transaction signed successfully`, and `BIP-39 mnemonic example completed successfully`.
- **Hash and size.** Read `TxHash` and `Size` from that stdout. The hash is 64 hex characters. The size is a positive integer followed by `bytes`.
- **Empty netns, first run.** Run `unshare --net --map-root-user -- target/debug/examples/native_bip39`. Exit code `0`. Hash and size match the cargo run.
- **Empty netns, second run.** Run the same `unshare` command again. Exit code `0`. Hash and size match the first netns run.
- **Proof.** `evidence` prints `result=PASS`, `feature=bip39-sign`, and the shared `txhash` and `size`.

## Gotchas

- `native()` drops the signer it is given. The example clones the signer, passes one clone to `native()`, and passes the signer to `TxBuilder::new`. `MorpheumSdk` has no `signer` field.
- The mnemonic in the program is the public BIP-39 test vector. The proof records it because the program prints it.
- The genesis hash in the program is 32 zero bytes, and the chain id is `morpheum-test-1`. With the default zero nonce, the hash does not depend on the clock.
- `cargo run` inside the empty netns fails when Cargo tries to reach the registry. Only the example binary goes in the netns.
- The broadcast call in the example source is a comment. The netns exit code is the check that the process finished without a route.
