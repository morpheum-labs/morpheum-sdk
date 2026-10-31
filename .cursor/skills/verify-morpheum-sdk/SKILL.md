---
name: verify-morpheum-sdk
description: "Drive the Morpheum native SDK example binaries (cargo run -p morpheum-sdk-examples --example …) and save a re-runnable transcript. Use after a signing or example change, or when a review needs proof that an example signed offline."
---

# Verify morpheum-sdk

The user-facing surface is the native example programs in `examples/`. Each one is a short process: it builds a signer, signs one transaction, prints the tx hash, and exits. No port is opened. The browser files `examples/wasm_browser_*.ts` need a wallet page; this skill leaves them alone.

The workspace has one Cargo target directory. A second `launch` refuses to start while `/tmp/verify-morpheum-sdk/lock` belongs to another run.

Sibling crates must sit beside this repo before `launch`. `doctor` fails while any of these markers is missing:

- `../morpheum-proto/Cargo.toml`
- `../morpheum-primitives/Cargo.toml`
- `../morpheum-signing/Cargo.toml` (crates `core`, `native`, and `wasm-lib`)
- `../cryptogram/Cargo.toml`
- `../contracts/cosmwasm/Cargo.toml`. The `contracts` repo has no root manifest. Any `Cargo.toml` under `../contracts/` also counts.

`morpheum-signing` is public. The other four are private; CI checks them out with `CI_REPO_READ_TOKEN`. A stand-in crate is a failed run.

## Launch

From the repo root:

```bash
.cursor/skills/verify-morpheum-sdk/bin/verify-morpheum-sdk launch
```

`launch` checks `rustc` against the workspace `rust-version`, and checks that `cargo`, `protoc`, `tmux`, `unshare`, and `git` are on `PATH`. It then builds the five example targets the drive can run:

```bash
cargo build -p morpheum-sdk-examples --example native_bip39 --example native_market --example native_agent_trade --example native_agent_vc_issue --example native_cancel_order
```

Ready is the line `ready run_id=<id>`. The id is stored in `/tmp/verify-morpheum-sdk/current`. The debug binaries are `target/debug/examples/native_bip39`, `target/debug/examples/native_market`, `target/debug/examples/native_agent_trade`, `target/debug/examples/native_agent_vc_issue`, and `target/debug/examples/native_cancel_order`. `launch` records their sha256.

## Doctor

```bash
.cursor/skills/verify-morpheum-sdk/bin/verify-morpheum-sdk doctor
```

`doctor` only reads. It re-checks the toolchain and the sibling manifests, then checks that the current run's binaries and the example sources still match the hashes `launch` recorded. `drive` runs `doctor` before it opens a session. Exit 0 prints `doctor ok run_id=<id> head=<sha> rustc=<version>`.

## Drive

```bash
.cursor/skills/verify-morpheum-sdk/bin/verify-morpheum-sdk drive bip39-sign
```

Feature ids: `bip39-sign`, `market-sign`, `agent-trade-sign`, `agent-vc-sign`, `cancel-order-sign`. The map in `features/` is the recipe. One drive uses one tmux session named `verify-morpheum-sdk-<run_id>-<feature>`.

Inside that session the helper runs, from the repo root:

```bash
cargo run -p morpheum-sdk-examples --example <name>
unshare --net --map-root-user -- target/debug/examples/<name>
unshare --net --map-root-user -- target/debug/examples/<name>
```

`unshare --net --map-root-user` gives the example a network namespace with no external route. Cargo stays outside that namespace: Cargo talks to the registry, and a Cargo failure there says nothing about the example. The second and third commands are the built example binary, the same file `cargo run` executes.

`--map-root-user` writes `/proc/self/uid_map`. `unshare` on `PATH` is not enough when the kernel refuses that write. Ubuntu 24.04 does so while `kernel.apparmor_restrict_unprivileged_userns` is `1`. The verify workflow sets that sysctl to `0`, then runs the same `unshare` command, before `launch`.

`bip39-sign`, `market-sign`, and `cancel-order-sign` must print the same tx hash on all three runs. `bip39-sign` must also print the same byte size. `agent-trade-sign` and `agent-vc-sign` call `SystemTime::now()` for claim expiry, so a matching hash is not required. The success lines and exit 0 are.

## Evidence

Transcripts land in:

```text
.cursor/skills/verify-morpheum-sdk/evidence/<run_id>/
```

| File | Contents |
| --- | --- |
| `launch.log` | toolchain check and `cargo build` output |
| `doctor.txt` | the doctor report used before the drive |
| `commands.txt` | the cargo command and the two `unshare` commands |
| `cargo-run.stdout`, `cargo-run.stderr`, `cargo-run.exit` | the user command |
| `netns-1.*`, `netns-2.*` | the same binary inside the empty netns |
| `proof.txt` | feature id, exits, tx hash, size, `result=PASS` or `result=FAIL` |

```bash
.cursor/skills/verify-morpheum-sdk/bin/verify-morpheum-sdk evidence
```

prints `proof.txt` and exits 0 only when it contains `result=PASS`.

A passing `bip39-sign` proof shows exit 0 for the cargo run and both netns runs, one 64-hex tx hash shared by all three, and one positive byte size shared by all three. The empty netns is the observation that the example did not dial out. The printed hash is the observation of the signed transaction. The example's broadcast call is commented out in source; the netns run is what shows the process still finishes with that hash.

## Cleanup

```bash
.cursor/skills/verify-morpheum-sdk/bin/verify-morpheum-sdk cleanup
```

`cleanup` kills tmux sessions named `verify-morpheum-sdk-<run_id>-*`, deletes `/tmp/verify-morpheum-sdk/runs/<run_id>`, and releases the lock when it belongs to that run. The evidence directory stays. `cleanup` prints `evidence=<absolute path>` after checking that directory is still there.

Run `cleanup` after a failed `launch` or `drive` as well, then start a new `launch`.

## Helpers

The helper is `.cursor/skills/verify-morpheum-sdk/bin/verify-morpheum-sdk`. It is executable. The commands above are the whole interface. Set `VERIFY_RUN_ID` to point `doctor`, `drive`, `evidence`, or `cleanup` at a run other than `/tmp/verify-morpheum-sdk/current`.

When the examples change, update the map with `/maintain-verification-skill`.
