# Agent trade sign

Agent trade sign is the example that builds a trading-key claim, checks it, and signs an order message with the agent key. On this tree that program does not compile, so a drive ends in `result=FAIL` and the cargo stderr.

## Sub-features

- `agent-trade-claim` builds a `TradingKeyClaim` for the agent subject.
- `agent-trade-verify` checks the claim against the issuer key.
- `agent-trade-sign` signs one order message and prints `TxHash`.

## How to get to it (user POV)

- Run `cargo run -p morpheum-sdk-examples --example native_agent_trade` from the repo root.

## Driving it with verify-morpheum-sdk

Preconditions:

- `doctor` prints `doctor ok` for this run.
- `examples/Cargo.toml` declares example `native_agent_trade`.
- The example source typechecks against `morpheum-sdk-native` and `morpheum-signing-native`.

- **User command.** The drive session runs `cargo run -p morpheum-sdk-examples --example native_agent_trade`. A passing run exits `0` and stdout contains `Agent Trade with Verified Claim Example`, `TradingKeyClaim built`, `TradingKeyClaim verified successfully`, `Trade transaction signed by Agent`, and `Agent trade example completed successfully`.
- **Empty netns.** After that command exits `0`, run `unshare --net --map-root-user -- target/debug/examples/native_agent_trade` twice. Both exit `0` and print a 64-hex `TxHash`. The hashes may differ across runs.
- **Current tree.** The preconditions above are unmet. `drive agent-trade-sign` exits non-zero, `proof.txt` says `result=FAIL`, and `cargo-run.stderr` is the cargo error. That is the whole result. Do not record this feature as covered by `bip39-sign`.

## Gotchas

- `examples/Cargo.toml` does not declare `native_agent_trade`. Cargo reports no example target of that name.
- The source calls `AccountId::new`. `AccountId` is a tuple struct and has no `new` method.
- The source calls `TxBuilder::new(sdk.signer.clone())`. `MorpheumSdk` has no `signer` field. `agent()` takes the signer by value and drops it.
- The source calls `claim.verify(now_secs, &AccountId::...)`. `TradingKeyClaim::verify` takes `&PublicKey`.
- `VcClaimBuilder::build` refuses a claim with no signature. The example never calls `.signature()`.
- Claim expiry uses `SystemTime::now()`, so a future passing run will not have a stable hash.
- Fix the example and declare the target before treating a drive as a pass. Do not replace this program with a different one and call the feature done.
