//! Hyperlane message relay — embedded relayer for bidirectional message passing.
//!
//! Supports both directions:
//!   - **Inbound (EVM → Morpheum)**: [`inbound_process_msg`] extracts the
//!     `Dispatch` event, signs a checkpoint, and builds the `Mailbox.process()`
//!     `MsgExecuteContract`. Delivering it is an ordinary signed Morpheum
//!     transaction, submitted by the caller's key:
//!
//!     ```ignore
//!     let msg = relay::inbound_process_msg(&evm_provider, InboundRelayRequest {
//!         morpheum_sender: &submitter.account_hex(),
//!         // ...
//!     })
//!     .await?;
//!     let admitted = submitter.submit(msg.to_any()).await?;
//!     match submitter.wait_final(&admitted.txhash, timeout).await? {
//!         TxOutcome::Confirmed { .. } => { /* delivered */ }
//!         TxOutcome::Failed { status } => { /* executed and failed, or skipped */ }
//!     }
//!     ```
//!   - **Outbound (Morpheum → EVM)**: Reconstructs the Hyperlane message,
//!     queries Morpheum MerkleTreeHook, signs checkpoint, submits
//!     `Mailbox.process()` on the target EVM chain.
//!
//! This performs the same role as a Hyperlane relayer but is embedded for
//! programmatic use — suitable for E2E tests, CLI tools, and services.
//!
//! Requires the `relay` feature flag.

use morpheum_sdk_evm::alloy::primitives::{Address, B256};
use morpheum_sdk_evm::alloy::providers::Provider;
use morpheum_sdk_evm::alloy::sol_types::SolCall;
use morpheum_sdk_evm::contracts::{IMailbox, IMerkleTreeHook};
use morpheum_sdk_evm::provider::EvmProvider;
use sha3::{Digest, Keccak256};

use morpheum_sdk_cosmwasm::{grpc, ExecuteContractRequest};

/// Errors from relay operations.
#[derive(Debug, thiserror::Error)]
pub enum RelayError {
    #[error("EVM error: {0}")]
    Evm(String),
    #[error("Morpheum gRPC error: {0}")]
    Grpc(String),
    #[error("message validation: {0}")]
    Validation(String),
    #[error("signing error: {0}")]
    Signing(String),
}

fn keccak(data: &[u8]) -> [u8; 32] {
    Keccak256::digest(data).into()
}

/// Pads a 20-byte EVM address to 32 bytes (left-padded with zeros).
pub fn pad_address_to_32(addr: Address) -> [u8; 32] {
    let mut padded = [0u8; 32];
    padded[12..].copy_from_slice(addr.as_slice());
    padded
}

/// Reads the Merkle root and latest index from an EVM MerkleTreeHook contract.
pub async fn read_evm_merkle_state(
    provider: &EvmProvider,
    hook_address: Address,
) -> Result<([u8; 32], u32), RelayError> {
    let hook = IMerkleTreeHook::new(hook_address, provider);

    let root: [u8; 32] = hook
        .root()
        .call()
        .await
        .map_err(|e| RelayError::Evm(format!("MerkleTreeHook.root(): {e}")))?
        .0;

    let count: u32 = hook
        .count()
        .call()
        .await
        .map_err(|e| RelayError::Evm(format!("MerkleTreeHook.count(): {e}")))?;

    let index = count.saturating_sub(1);
    Ok((root, index))
}

/// Signs a Hyperlane `MessageIdMultisigIsm` checkpoint.
///
/// The checkpoint hash follows the Hyperlane spec:
///   `domain_hash = keccak256(origin_domain || origin_merkle_tree || "HYPERLANE")`
///   `signing_hash = keccak256(domain_hash || merkle_root || merkle_index || message_id)`
///   `eth_hash = EIP-191(signing_hash)`
///   `signature = ECDSA(eth_hash)`
pub fn sign_checkpoint(
    private_key: &[u8; 32],
    origin_domain: u32,
    origin_merkle_tree: &[u8; 32],
    merkle_root: &[u8; 32],
    merkle_index: u32,
    message_id: &[u8; 32],
) -> Result<[u8; 65], RelayError> {
    let domain_hash = {
        let mut hasher = Keccak256::new();
        hasher.update(origin_domain.to_be_bytes());
        hasher.update(origin_merkle_tree);
        hasher.update(b"HYPERLANE");
        hasher.finalize()
    };

    let signing_hash = {
        let mut hasher = Keccak256::new();
        hasher.update(domain_hash);
        hasher.update(merkle_root);
        hasher.update(merkle_index.to_be_bytes());
        hasher.update(message_id);
        hasher.finalize()
    };

    let eth_hash = {
        let prefix = format!("\x19Ethereum Signed Message:\n{}", signing_hash.len());
        let mut hasher = Keccak256::new();
        hasher.update(prefix.as_bytes());
        hasher.update(signing_hash);
        hasher.finalize()
    };

    let signing_key = k256::ecdsa::SigningKey::from_bytes(private_key.into())
        .map_err(|e| RelayError::Signing(format!("invalid validator key: {e}")))?;

    let (sig, recovery_id) = signing_key
        .sign_prehash_recoverable(&eth_hash)
        .map_err(|e| RelayError::Signing(format!("ECDSA sign: {e}")))?;

    let mut signature = [0u8; 65];
    signature[..64].copy_from_slice(&sig.to_bytes());
    signature[64] = recovery_id.to_byte() + 27;

    Ok(signature)
}

/// Builds `MessageIdMultisigIsm` metadata from checkpoint components:
/// `origin_merkle_tree || merkle_root || merkle_index || signature`.
pub fn build_ism_metadata(
    origin_merkle_tree: &[u8; 32],
    merkle_root: &[u8; 32],
    merkle_index: u32,
    signature: &[u8; 65],
) -> Vec<u8> {
    let mut metadata = Vec::with_capacity(68 + 65);
    metadata.extend_from_slice(origin_merkle_tree);
    metadata.extend_from_slice(merkle_root);
    metadata.extend_from_slice(&merkle_index.to_be_bytes());
    metadata.extend_from_slice(signature);
    metadata
}

/// Builds the `Mailbox.process()` call that delivers a Hyperlane `message`
/// to the Morpheum mailbox contract, as a `MsgExecuteContract` from `sender`.
///
/// `sender` is the account that signs the transaction carrying the Msg: its
/// hex account id (as `TxSubmitter::account_hex()` returns) or its bech32
/// address. `metadata` is the ISM metadata, e.g. from [`build_ism_metadata`].
/// No funds are attached.
pub fn mailbox_process_msg(
    sender: &str,
    mailbox: &str,
    metadata: &[u8],
    message: &[u8],
) -> ExecuteContractRequest {
    let process_msg = serde_json::json!({
        "process": {
            "metadata": hex::encode(metadata),
            "message": hex::encode(message)
        }
    });

    ExecuteContractRequest {
        sender: sender.to_owned(),
        contract: mailbox.to_owned(),
        msg: process_msg.to_string().into_bytes(),
        funds: Vec::new(),
    }
}

/// Decodes the `bytes message` argument from the data of a Hyperlane
/// `Dispatch(address,uint32,bytes32,bytes)` log: an ABI head word holding the
/// offset of the tail, then the tail's length word and the bytes. Returns
/// `None` if the data is malformed (an offset or length that does not fit in
/// the data).
fn decode_dispatch_message(data: &[u8]) -> Option<&[u8]> {
    let word = |at: usize| -> Option<usize> {
        let bytes = data.get(at..at.checked_add(32)?)?;
        let (high, low) = bytes.split_at(24);
        if high.iter().any(|&b| b != 0) {
            return None;
        }
        usize::try_from(u64::from_be_bytes(low.try_into().ok()?)).ok()
    };
    let offset = word(0)?;
    let length = word(offset)?;
    let start = offset.checked_add(32)?;
    data.get(start..start.checked_add(length)?)
}

/// Parameters required to relay a Hyperlane message from EVM into Morpheum.
pub struct InboundRelayRequest<'a> {
    /// The account that signs the relay transaction: its hex account id
    /// (as `TxSubmitter::account_hex()` returns) or its bech32 address.
    pub morpheum_sender: &'a str,
    pub morpheum_mailbox: &'a str,
    pub tx_hash: B256,
    pub validator_private_key: &'a [u8; 32],
    pub origin_domain: u32,
    pub merkle_tree_hook: Address,
}

/// Builds the Morpheum `Mailbox.process()` Msg that relays a Hyperlane
/// message dispatched on an EVM chain.
///
/// Extracts the `Dispatch` event from the given EVM tx, reads the
/// MerkleTreeHook state, signs a checkpoint, and returns the
/// `MsgExecuteContract` from [`InboundRelayRequest::morpheum_sender`] to the
/// mailbox. Nothing is sent to Morpheum: the relay is complete once the
/// caller has signed and submitted `msg.to_any()` with that sender's key
/// (e.g. `TxSubmitter::submit`) and the transaction has executed (see the
/// [module docs](self)).
pub async fn inbound_process_msg(
    evm_provider: &EvmProvider,
    request: InboundRelayRequest<'_>,
) -> Result<ExecuteContractRequest, RelayError> {
    let receipt = evm_provider
        .get_transaction_receipt(request.tx_hash)
        .await
        .map_err(|e| RelayError::Evm(format!("get tx receipt: {e}")))?
        .ok_or_else(|| RelayError::Evm("receipt not found".into()))?;

    let dispatch_topic = B256::from(keccak(b"Dispatch(address,uint32,bytes32,bytes)"));

    let message_bytes = receipt
        .inner
        .logs()
        .iter()
        .filter(|log| log.topics().first() == Some(&dispatch_topic))
        .find_map(|log| decode_dispatch_message(log.data().data.as_ref()))
        .map(<[u8]>::to_vec)
        .ok_or_else(|| RelayError::Evm("no well-formed Dispatch event in tx receipt".into()))?;

    if message_bytes.len() < 77 {
        return Err(RelayError::Validation(format!(
            "Hyperlane message too short: {} bytes",
            message_bytes.len()
        )));
    }

    let message_id = keccak(&message_bytes);
    tracing::info!(
        message_id = hex::encode(message_id),
        msg_len = message_bytes.len(),
        "extracted Dispatch message"
    );

    let (merkle_root, merkle_index) =
        read_evm_merkle_state(evm_provider, request.merkle_tree_hook).await?;

    let origin_merkle_tree = pad_address_to_32(request.merkle_tree_hook);
    let signature = sign_checkpoint(
        request.validator_private_key,
        request.origin_domain,
        &origin_merkle_tree,
        &merkle_root,
        merkle_index,
        &message_id,
    )?;

    let metadata = build_ism_metadata(&origin_merkle_tree, &merkle_root, merkle_index, &signature);

    Ok(mailbox_process_msg(
        request.morpheum_sender,
        request.morpheum_mailbox,
        &metadata,
        &message_bytes,
    ))
}

/// Relays a Hyperlane message from Morpheum to a target EVM chain.
///
/// Reconstructs the Hyperlane V3 message from known parameters, queries
/// the Morpheum MerkleTreeHook, signs a checkpoint, and submits
/// `Mailbox.process()` on the target EVM chain.
#[allow(clippy::too_many_arguments)]
pub async fn relay_outbound(
    channel: &tonic::transport::Channel,
    evm_provider: &EvmProvider,
    evm_mailbox: Address,
    morpheum_mailbox: &str,
    morpheum_merkle_hook: &str,
    morpheum_warp_route_raw: &[u8; 20],
    evm_warp_collateral: Address,
    morpheum_domain: u32,
    evm_domain: u32,
    recipient_bytes: [u8; 32],
    amount: u64,
    validator_private_key: &[u8; 32],
) -> Result<(), RelayError> {
    let nonce_resp =
        grpc::wasm_smart_query(channel, morpheum_mailbox, br#"{"mailbox":{"nonce":{}}}"#)
            .await
            .map_err(|e| RelayError::Grpc(format!("query Morpheum Mailbox nonce: {e}")))?;

    let nonce_val: serde_json::Value = serde_json::from_slice(&nonce_resp)
        .map_err(|e| RelayError::Grpc(format!("parse nonce response: {e}")))?;
    let nonce = nonce_val["nonce"]
        .as_u64()
        .ok_or_else(|| RelayError::Grpc("nonce not found in response".into()))?
        as u32;
    let msg_nonce = nonce.saturating_sub(1);

    // Reconstruct Hyperlane V3 message:
    //   version(1) || nonce(4) || origin(4) || sender(32) ||
    //   destination(4) || recipient(32) || body(variable)
    let mut sender_padded = [0u8; 32];
    sender_padded[12..].copy_from_slice(morpheum_warp_route_raw);

    let warp_body = {
        let mut buf = Vec::with_capacity(64);
        buf.extend_from_slice(&recipient_bytes);
        let mut amount_bytes = [0u8; 32];
        amount_bytes[24..].copy_from_slice(&amount.to_be_bytes());
        buf.extend_from_slice(&amount_bytes);
        buf
    };

    let mut message = Vec::new();
    message.push(3u8); // version
    message.extend_from_slice(&msg_nonce.to_be_bytes());
    message.extend_from_slice(&morpheum_domain.to_be_bytes());
    message.extend_from_slice(&sender_padded);
    message.extend_from_slice(&evm_domain.to_be_bytes());
    let evm_recipient = pad_address_to_32(evm_warp_collateral);
    message.extend_from_slice(&evm_recipient);
    message.extend_from_slice(&warp_body);

    let message_id = keccak(&message);
    tracing::info!(
        message_id = hex::encode(message_id),
        nonce = msg_nonce,
        "reconstructed outbound Hyperlane message"
    );

    let root_resp = grpc::wasm_smart_query(
        channel,
        morpheum_merkle_hook,
        br#"{"merkle_hook":{"root":{}}}"#,
    )
    .await
    .map_err(|e| RelayError::Grpc(format!("query MerkleTreeHook root: {e}")))?;

    let count_resp = grpc::wasm_smart_query(
        channel,
        morpheum_merkle_hook,
        br#"{"merkle_hook":{"count":{}}}"#,
    )
    .await
    .map_err(|e| RelayError::Grpc(format!("query MerkleTreeHook count: {e}")))?;

    let root_val: serde_json::Value = serde_json::from_slice(&root_resp)
        .map_err(|e| RelayError::Grpc(format!("parse root: {e}")))?;
    let count_val: serde_json::Value = serde_json::from_slice(&count_resp)
        .map_err(|e| RelayError::Grpc(format!("parse count: {e}")))?;

    let root_hex = root_val["root"]
        .as_str()
        .ok_or_else(|| RelayError::Grpc("root not found".into()))?;
    let index = count_val["count"]
        .as_u64()
        .ok_or_else(|| RelayError::Grpc("count not found".into()))? as u32;

    let mut root = [0u8; 32];
    hex::decode_to_slice(root_hex, &mut root)
        .map_err(|e| RelayError::Grpc(format!("decode root hex: {e}")))?;

    tracing::info!(
        root = root_hex,
        count = index,
        "Morpheum MerkleTreeHook state"
    );

    let hook_raw = morpheum_primitives::address::decode_address(morpheum_merkle_hook)
        .ok_or_else(|| RelayError::Grpc(format!("invalid bech32: {morpheum_merkle_hook}")))?;
    let mut origin_merkle_tree = [0u8; 32];
    origin_merkle_tree[12..].copy_from_slice(&hook_raw);

    let merkle_index = index.saturating_sub(1);
    let signature = sign_checkpoint(
        validator_private_key,
        morpheum_domain,
        &origin_merkle_tree,
        &root,
        merkle_index,
        &message_id,
    )?;

    let metadata = build_ism_metadata(&origin_merkle_tree, &root, merkle_index, &signature);

    let calldata = IMailbox::processCall {
        _metadata: metadata.into(),
        _message: message.into(),
    }
    .abi_encode();

    let tx = morpheum_sdk_evm::alloy::rpc::types::TransactionRequest::default()
        .to(evm_mailbox)
        .input(calldata.into());

    let pending = evm_provider
        .send_transaction(tx)
        .await
        .map_err(|e| RelayError::Evm(format!("send Mailbox.process tx: {e}")))?;
    let receipt = pending
        .get_receipt()
        .await
        .map_err(|e| RelayError::Evm(format!("await Mailbox.process receipt: {e}")))?;

    if !receipt.status() {
        return Err(RelayError::Evm(
            "Mailbox.process on target EVM chain reverted".into(),
        ));
    }

    tracing::info!(
        tx_hash = %receipt.transaction_hash,
        "outbound Hyperlane message delivered on target EVM chain"
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ism_metadata_is_tree_root_index_signature() {
        let metadata = build_ism_metadata(&[1; 32], &[2; 32], 0x0102_0304, &[3; 65]);
        assert_eq!(metadata.len(), 32 + 32 + 4 + 65);
        assert_eq!(&metadata[..32], &[1; 32]);
        assert_eq!(&metadata[32..64], &[2; 32]);
        assert_eq!(&metadata[64..68], &[1, 2, 3, 4]);
        assert_eq!(&metadata[68..], &[3; 65]);
    }

    /// ABI-encodes `message` as the single dynamic `bytes` argument.
    fn abi_bytes(message: &[u8]) -> Vec<u8> {
        let mut data = vec![0u8; 64];
        data[31] = 0x20;
        data[56..64].copy_from_slice(&(message.len() as u64).to_be_bytes());
        data.extend_from_slice(message);
        data.resize(64 + message.len().div_ceil(32) * 32, 0);
        data
    }

    #[test]
    fn dispatch_message_decodes_from_abi_bytes() {
        let message: Vec<u8> = (0..77).collect();
        assert_eq!(
            decode_dispatch_message(&abi_bytes(&message)),
            Some(message.as_slice())
        );
        assert_eq!(decode_dispatch_message(&abi_bytes(&[])), Some(&b""[..]));
    }

    #[test]
    fn malformed_dispatch_data_is_rejected_without_panicking() {
        let valid = abi_bytes(&[7; 40]);

        // Too short for a head word.
        assert_eq!(decode_dispatch_message(&valid[..31]), None);
        // Length runs past the end of the data.
        assert_eq!(decode_dispatch_message(&valid[..64 + 39]), None);
        // Offset points past the end of the data.
        let mut far_offset = valid.clone();
        far_offset[31] = 0xff;
        assert_eq!(decode_dispatch_message(&far_offset), None);
        // Offset word wider than 64 bits.
        let mut huge_offset = valid.clone();
        huge_offset[0] = 1;
        assert_eq!(decode_dispatch_message(&huge_offset), None);
        // Length whose end offset overflows.
        let mut huge_length = valid;
        huge_length[56..64].copy_from_slice(&u64::MAX.to_be_bytes());
        assert_eq!(decode_dispatch_message(&huge_length), None);
    }

    #[test]
    fn process_msg_executes_the_mailbox_from_the_sender_without_funds() {
        let any =
            mailbox_process_msg("ab01", "morm1mailbox", &[0xaa, 0xbb], &[0x03, 0x00]).to_any();
        assert_eq!(any.type_url, "/cosmwasm.wasm.v1.MsgExecuteContract");

        let body: serde_json::Value = serde_json::from_slice(&any.value).unwrap();
        assert_eq!(body["sender"], "ab01");
        assert_eq!(body["contract"], "morm1mailbox");
        assert_eq!(body["funds"], serde_json::json!([]));

        let msg: Vec<u8> = serde_json::from_value(body["msg"].clone()).unwrap();
        let process: serde_json::Value = serde_json::from_slice(&msg).unwrap();
        assert_eq!(
            process,
            serde_json::json!({ "process": { "metadata": "aabb", "message": "0300" } })
        );
    }
}
