# Morpheum native SDK verification map

This directory is the source for verifying the native example programs. Read the index, then follow one feature file.

## Baseline preconditions

- Clone `morpheum-proto`, `morpheum-primitives`, `morpheum-signing`, `cryptogram`, and `contracts` as siblings of this repo. `doctor` fails while a sibling marker is missing. The first four need a root `Cargo.toml`. `contracts` needs `contracts/cosmwasm/Cargo.toml`, or any other `Cargo.toml` under that tree.
- `rustc` is at least the workspace `rust-version`. CI pins 1.98.1.
- `protoc`, `tmux`, and `unshare` are on `PATH`. The host must allow the `uid_map` write from `unshare --map-root-user`. Ubuntu 24.04 blocks it until `kernel.apparmor_restrict_unprivileged_userns` is `0`.
- Run `launch`, then `doctor`, and require `doctor ok` for this run id.
- One verification run owns the workspace `target/` directory. A second `launch` exits while the lock at `/tmp/verify-morpheum-sdk/lock` is held.
- The example programs use the all-zero genesis hash baked into their source. That is the chain id binding under test. They do not submit to a node.

## Driving conventions

- Start from a finished `launch` unless the feature file says otherwise.
- Treat every command as literal.
- Run drives through `.cursor/skills/verify-morpheum-sdk/bin/verify-morpheum-sdk drive <feature>`.
- `cleanup` deletes the tmux session and `/tmp` state for the run. It leaves `evidence/<run_id>/` in place.
- A feature that fails to build stays failed. Record the compiler output. Do not mark it passed through a different program.

## Proof and skip reporting

- Capture the command, stdout, stderr, and exit code.
- For a signing example, also capture the same binary's stdout inside `unshare --net --map-root-user`.
- `bip39-sign`, `market-sign`, and `cancel-order-sign` quote one tx hash from three runs. Differing hashes fail the proof.
- Record the feature id, example name, binary path, and sdk git head in `proof.txt`.
- An unreachable example records `result=FAIL` and the cargo stderr. It is not verified by another feature.

## Feature entry contract

Each feature file has an H1, one paragraph, then four H2 sections in order: `Sub-features`, `How to get to it (user POV)`, `Driving it with verify-morpheum-sdk`, and `Gotchas`.

## Features

- [BIP-39 sign](./bip39-sign.md) signs one transaction from the standard test mnemonic and prints a stable hash. Offline.
- [Market sign](./market-sign.md) builds a perp market create request and signs it. Offline. Stable hash.
- [Agent trade sign](./agent-trade-sign.md) builds a trading-key claim, checks it against the issuer key, and signs one order. Offline. The tx hash may change between runs.
- [Agent VC sign](./agent-vc-sign.md) builds a trading-key claim and signs one VC issue message with the agent key. Offline. The tx hash may change between runs.
- [Cancel order sign](./cancel-order-sign.md) builds a CLOB cancel request (`/clob.v1.MsgCancelOrderRequest`) and signs it. Offline. Stable hash.
