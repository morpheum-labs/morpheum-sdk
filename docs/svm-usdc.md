# SVM USDC Native Program — SDK & CLI Reference

> Last updated: 2026-03-21

This document covers the client-side tooling for interacting with Morpheum's internal SVM USDC native program. The native program lives inside the Morpheum node (`mormcore/crates/modules/svm/src/programs/usdc.rs`) and translates compact byte-encoded instructions into `NativeOp` dispatches against the shared Bank module.

---

## Overview

Morpheum canonical USDC (bank asset index 1) is accessible from three VM contexts:

| VM | Mechanism | Client SDK |
|----|-----------|------------|
| CosmWasm | CCTP Handler contract | `morpheum-sdk-cctp` |
| EVM | Precompile at `0x...C1` | `morpheum-sdk-evm::cctp` |
| SVM | Native program (intercepted in `SvmExecutor`) | `morpheum-sdk-svm::usdc` |

All three converge on the same `NativeDispatcher` → `TokenBridge` → `BankKeeper` pipeline. The precompile and native program are pure translators with zero independent state.

---

## Program ID

The USDC native program ID is derived deterministically:

```
blake3::hash(b"morpheum_usdc_native_program")[..20] → hex-encoded (40 chars)
```

Use `morpheum_sdk_svm::usdc::usdc_program_id()` to compute it.

---

## Instruction Encoding

| Byte 0 (discriminator) | Bytes 1..17 (LE u128) | Accounts |
|------------------------|-----------------------|----------|
| 0 = Transfer | amount | [from, to] |
| 1 = Approve | amount | [owner, spender] |
| 2 = TransferFrom | amount | [from, to] (sender = approved spender) |
| 3 = BalanceOf | -- | [owner] |
| 4 = Allowance | -- | [owner, spender] |

### SDK Encoding Functions

```rust
use morpheum_sdk_svm::usdc;

let transfer_data = usdc::encode_transfer(1_000_000u128);     // 17 bytes
let approve_data  = usdc::encode_approve(500_000u128);         // 17 bytes
let balance_data  = usdc::encode_balance_of();                 // 1 byte
let allowance_data = usdc::encode_allowance();                 // 1 byte
```

---

## Account Metadata

Each instruction requires specific accounts:

```rust
use morpheum_sdk_svm::usdc::AccountMeta;

// Writable account (for transfer source/destination)
let from = AccountMeta::writable("sender_hex_address");

// Read-only account (for balance queries)
let owner = AccountMeta::readonly("owner_hex_address");
```

---

## Building `MsgExecute`

Instructions are submitted as `MsgExecute` transactions through Morpheum's Ingress gRPC endpoint:

```rust
use morpheum_sdk_svm::usdc;

let msg_any = usdc::build_usdc_execute(
    "sender_hex_address",
    usdc::encode_transfer(1_000_000),
    vec![
        usdc::AccountMeta::writable("sender_hex"),
        usdc::AccountMeta::writable("recipient_hex"),
    ],
    usdc::DEFAULT_COMPUTE_LIMIT,
)?;

// msg_any is a google.protobuf.Any with type_url "/morpheum.svm.v1.MsgExecute"
// Submit via IngressService/SubmitTx
```

The `MsgExecute` is JSON-serialized (matching the SVM actor's `serde_json::from_slice` deserialization) and wrapped in a `google.protobuf.Any`.

---

## Return Data

Read operations (`BalanceOf`, `Allowance`) produce LE u128 return data:

```rust
use morpheum_sdk_svm::usdc;

let balance = usdc::decode_u128(&return_data_bytes)?;
```

Note: The current `SubmitTxResponse` proto does not carry return data. For balance queries, use `morpheum query bank balance` or the bank gRPC endpoint as the canonical source. The `MsgExecute` submission proves the native program path works.

---

## CLI Commands

### Transaction Commands (feature: `svm`)

```bash
# Transfer USDC via SVM native program
morpheum tx svm-usdc transfer --to <hex_address> --amount 1000000

# Approve a spender
morpheum tx svm-usdc approve --spender <hex_address> --amount 500000

# TransferFrom (requires prior approval)
morpheum tx svm-usdc transfer-from --from <hex_address> --to <hex_address> --amount 100000
```

### Query Commands (feature: `svm`)

```bash
# Display the USDC native program ID
morpheum query svm-usdc program-id

# Query balance (submits MsgExecute through consensus)
morpheum query svm-usdc balance --address <hex_address>

# Query allowance (submits MsgExecute through consensus)
morpheum query svm-usdc allowance --owner <hex_address> --spender <hex_address>
```

---

## E2E Testing

The `CctpSvmHarness` in `orchestrator/tests/e2e/gmp/cctp/src/svm_harness.rs` provides:

- `usdc_transfer(to, amount)` — Submit Transfer via `MsgExecute`
- `usdc_approve(spender, amount)` — Submit Approve via `MsgExecute`
- `usdc_balance_of(owner)` — Submit BalanceOf + verify via bank query
- `usdc_allowance(owner, spender)` — Submit Allowance via `MsgExecute`
- `raw_execute(data, accounts)` — Submit arbitrary instruction data (for negative tests)

### Test coverage

| Test | File | What it verifies |
|------|------|-----------------|
| `test_cctp_svm_native_program_balance` | `cctp_multi_vm_e2e.rs` | BalanceOf via MsgExecute matches bank |
| `test_cctp_cross_vm_balance_consistency` | `cctp_multi_vm_e2e.rs` | Bank == SVM balance |
| `test_svm_usdc_transfer_updates_bank` | `cctp_multi_vm_e2e.rs` | Transfer decrements sender, credits recipient |
| `test_svm_usdc_transfer_insufficient_balance` | `cctp_negative_e2e.rs` | Overdraft rejected, balance unchanged |
| `test_svm_usdc_invalid_instruction_rejected` | `cctp_negative_e2e.rs` | Invalid discriminator (0xFF) rejected |
| `test_svm_usdc_empty_instruction_rejected` | `cctp_negative_e2e.rs` | Empty data rejected |

---

## Relationship to External Solana

This module covers **Morpheum's internal SVM engine** — the USDC native program running inside the Morpheum node. It does NOT cover external Solana chain CCTP bridging (which would require a Solana program for CCTP — future work).

For bridging tokens FROM Solana TO Morpheum via Warp Route, use `morpheum-sdk-svm::bridge`.
