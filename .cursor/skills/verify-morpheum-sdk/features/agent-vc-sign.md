# Agent VC sign

Agent VC sign is the example that builds a trading-key claim and signs a VC issue message with the agent key. On this tree that program does not compile, so a drive ends in `result=FAIL` and the cargo stderr.

## Sub-features

- `agent-vc-claim` builds a `TradingKeyClaim` whose issuer is the owner account and whose subject is the agent.
- `agent-vc-sign` embeds that claim and signs one `/vc.v1.MsgIssue` message.

## How to get to it (user POV)

- Run `cargo run -p morpheum-sdk-examples --example native_agent_vc_issue` from the repo root.

## Driving it with verify-morpheum-sdk

Preconditions:

- `doctor` prints `doctor ok` for this run.
- `examples/Cargo.toml` declares example `native_agent_vc_issue`.
- The example source typechecks against `morpheum-sdk-native` and `morpheum-signing-native`.

- **User command.** The drive session runs `cargo run -p morpheum-sdk-examples --example native_agent_vc_issue`. A passing run exits `0` and stdout contains `Agent VC Issuance Example`, `TradingKeyClaim built successfully`, `Transaction signed successfully by Agent`, and `Agent VC issuance example completed successfully`.
- **Empty netns.** After that command exits `0`, run `unshare --net --map-root-user -- target/debug/examples/native_agent_vc_issue` twice. Both exit `0` and print a 64-hex `TxHash`. The hashes may differ across runs.
- **Current tree.** The preconditions above are unmet. `drive agent-vc-sign` exits non-zero, `proof.txt` says `result=FAIL`, and `cargo-run.stderr` is the cargo error. Do not record this feature as covered by `bip39-sign`.

## Gotchas

- `examples/Cargo.toml` does not declare `native_agent_vc_issue`. Cargo reports no example target of that name.
- The source calls `AccountId::new`. `AccountId` is a tuple struct and has no `new` method.
- The source calls `TxBuilder::new(sdk.signer.clone())`. `MorpheumSdk` has no `signer` field. `agent()` takes the signer by value and drops it.
- `VcClaimBuilder::build` refuses a claim with no signature. The example never calls `.signature()`, so a build that got past the type errors would still exit non-zero at `build(now_secs)`.
- Claim expiry uses `SystemTime::now()`, so a future passing run will not have a stable hash.
- Fix the example and declare the target before treating a drive as a pass.
