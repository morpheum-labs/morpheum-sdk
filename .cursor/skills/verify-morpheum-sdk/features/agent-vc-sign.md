# Agent VC sign

Agent VC sign builds a trading-key claim for an agent subject and signs one VC issue message with the agent key. The program prints the tx hash and exits. It does not broadcast. The hash can change between runs because claim expiry uses the clock.

## Sub-features

- `agent-vc-claim` builds a `TradingKeyClaim` whose issuer is the owner account and whose subject is the agent.
- `agent-vc-sign` embeds that claim and signs one `/vc.v1.MsgIssue` message.

## How to get to it (user POV)

- Run `cargo run -p morpheum-sdk-examples --example native_agent_vc_issue` from the repo root.

## Driving it with verify-morpheum-sdk

Preconditions:

- `doctor` prints `doctor ok` for this run.
- `examples/Cargo.toml` declares example `native_agent_vc_issue`.
- The sibling manifests listed in the index exist.

- **User command.** The drive session runs `cargo run -p morpheum-sdk-examples --example native_agent_vc_issue`. Exit code `0`. Stdout contains `Agent VC Issuance Example`, `TradingKeyClaim built successfully`, `TradingKeyClaim verified successfully`, `Transaction signed successfully by Agent`, and `Agent VC issuance example completed successfully`.
- **Hash.** Read `TxHash` from that stdout. It is 64 hex characters. A later run of the same binary may print a different hash.
- **Empty netns, twice.** Run `unshare --net --map-root-user -- target/debug/examples/native_agent_vc_issue` twice. Both exit `0` and print a 64-hex `TxHash`. The three hashes may differ.
- **Proof.** `evidence` prints `result=PASS` and `feature=agent-vc-sign`. `cargo_exit`, `netns_1_exit`, and `netns_2_exit` are `0`.

## Gotchas

- `agent()` drops the signer it is given. The example clones the signer, passes one clone to `agent()`, and passes the signer to `TxBuilder::new`. `MorpheumSdk` has no `signer` field.
- `AgentSigner::new` and `VcClaimBuilder` take the signing library's `AccountId`, a tuple struct. The example derives those ids from `NativeSigner::account_id()`. The SDK `AccountId::new` is a different type. The signing `AccountId` has no `Display` impl, so the example prints it with `{:?}`.
- `VcClaimBuilder::build` refuses a claim with no signature. The example calls `.signature(Signature::Ed25519([1u8; 64]))`. `build` checks that the signature is present. It does not check the curve.
- `TradingKeyClaim::verify` takes `&PublicKey`. The issuer on the claim is `owner.account_id()`, and the check passes that owner's `public_key()`.
- Claim expiry uses `SystemTime::now()`, so three runs do not have to share a hash. Differing hashes are not a failure for this feature.
- The chain id in the program is `morpheum-test-1`. The genesis hash is 32 zero bytes. The issue message is the placeholder `/vc.v1.MsgIssue` bytes in the example.
- `cargo run` inside the empty netns is the wrong process to watch. Run the example binary there.
- The broadcast call in the example source is a comment. The netns exit code is the check that the process finished without a route.
