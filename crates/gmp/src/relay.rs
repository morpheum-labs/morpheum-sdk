//! Hyperlane message relay — embedded relayer for bidirectional message passing.
//!
//! Supports both directions:
//!   - **Inbound (EVM → Morpheum)**: Extracts `Dispatch` event, signs checkpoint,
//!     submits `Mailbox.process()` to Morpheum via `dev_messages`.
//!   - **Outbound (Morpheum → EVM)**: Reconstructs the Hyperlane message,
//!     queries Morpheum MerkleTreeHook, signs checkpoint, submits
//!     `Mailbox.process()` on the target EVM chain.
//!
//! This performs the same role as a Hyperlane relayer but is embedded for
//! programmatic use — suitable for E2E tests, CLI tools, and services.
//!
//! Requires the `relay` feature flag.

use morpheum_sdk_evm::alloy::primitives::{Address, B256, U256};
use morpheum_sdk_evm::alloy::providers::Provider;
use morpheum_sdk_evm::alloy::sol_types::SolCall;
use morpheum_sdk_evm::contracts::{IMailbox, IMerkleTreeHook};
use morpheum_sdk_evm::provider::EvmProvider;
use sha3::{Digest, Keccak256};

use morpheum_sdk_cosmwasm::grpc;

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

/// Builds MessageIdMultisigIsm metadata from checkpoint components.
fn build_ism_metadata(
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

/// Relays a dispatched Hyperlane message from an EVM chain to Morpheum.
///
/// Extracts the `Dispatch` event from the given EVM tx, reads the
/// MerkleTreeHook state, signs a checkpoint, and submits `Mailbox.process()`
/// to Morpheum via `dev_messages`.
pub async fn relay_inbound(
    evm_provider: &EvmProvider,
    channel: &tonic::transport::Channel,
    morpheum_sender: &str,
    morpheum_mailbox: &str,
    tx_hash: B256,
    validator_private_key: &[u8; 32],
    origin_domain: u32,
    merkle_tree_hook: Address,
) -> Result<(), RelayError> {
    let receipt = evm_provider
        .get_transaction_receipt(tx_hash)
        .await
        .map_err(|e| RelayError::Evm(format!("get tx receipt: {e}")))?
        .ok_or_else(|| RelayError::Evm("receipt not found".into()))?;

    let dispatch_topic = B256::from(keccak(
        b"Dispatch(address,uint32,bytes32,bytes)",
    ));

    let message_bytes = receipt
        .inner
        .logs()
        .iter()
        .find_map(|log| {
            if log.topics().first() == Some(&dispatch_topic) {
                let data = log.data().data.as_ref();
                if data.len() >= 64 {
                    let offset = U256::from_be_slice(&data[0..32]).to::<usize>();
                    let length = U256::from_be_slice(&data[offset..offset + 32]).to::<usize>();
                    return Some(data[offset + 32..offset + 32 + length].to_vec());
                }
            }
            None
        })
        .ok_or_else(|| RelayError::Evm("Dispatch event not found in tx receipt".into()))?;

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
        read_evm_merkle_state(evm_provider, merkle_tree_hook).await?;

    let origin_merkle_tree = pad_address_to_32(merkle_tree_hook);
    let signature = sign_checkpoint(
        validator_private_key,
        origin_domain,
        &origin_merkle_tree,
        &merkle_root,
        merkle_index,
        &message_id,
    )?;

    let metadata = build_ism_metadata(&origin_merkle_tree, &merkle_root, merkle_index, &signature);

    let process_msg = serde_json::json!({
        "process": {
            "metadata": hex::encode(&metadata),
            "message": hex::encode(&message_bytes)
        }
    });
    let msg_json = serde_json::to_vec(&process_msg)
        .map_err(|e| RelayError::Validation(format!("serialize process msg: {e}")))?;

    grpc::broadcast_execute_contract(channel, morpheum_sender, morpheum_mailbox, &msg_json)
        .await
        .map_err(|e| RelayError::Grpc(format!("Mailbox.process BroadcastTx: {e}")))?;

    tracing::info!(
        message_id = hex::encode(message_id),
        "Hyperlane message relayed to Morpheum"
    );

    Ok(())
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
    let nonce_resp = grpc::wasm_smart_query(
        channel,
        morpheum_mailbox,
        br#"{"mailbox":{"nonce":{}}}"#,
    )
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

    tracing::info!(root = root_hex, count = index, "Morpheum MerkleTreeHook state");

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
