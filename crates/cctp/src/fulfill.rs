//! CCTP attestation polling via Circle's IRIS API.
//!
//! After a CCTP message is sent on an EVM chain, Circle's off-chain attesters
//! sign the message. This module polls the IRIS API until the attestation is
//! available, then returns the raw attestation bytes for on-chain submission.
//!
//! Requires the `iris` feature flag.

use std::time::Duration;

use sha3::{Digest, Keccak256};

use crate::error::CctpError;

pub const IRIS_SANDBOX_URL: &str = "https://iris-api-sandbox.circle.com";
pub const IRIS_MAINNET_URL: &str = "https://iris-api.circle.com";

const DEFAULT_POLL_INTERVAL: Duration = Duration::from_secs(10);

/// Polls Circle's IRIS API until the attestation for the given CCTP message
/// is available or the timeout expires.
///
/// # Arguments
/// * `cctp_message` — Raw CCTP message bytes (from `MessageSent` event).
/// * `iris_base_url` — IRIS API base URL (use [`IRIS_SANDBOX_URL`] for testnet
///   or [`IRIS_MAINNET_URL`] for production).
/// * `timeout` — Maximum time to wait for the attestation.
pub async fn wait_for_attestation(
    cctp_message: &[u8],
    iris_base_url: &str,
    timeout: Duration,
) -> Result<Vec<u8>, CctpError> {
    let message_hash = hex::encode(Keccak256::digest(cctp_message));
    let url = format!("{iris_base_url}/v1/attestations/0x{message_hash}");

    let client = reqwest::Client::new();
    let deadline = tokio::time::Instant::now() + timeout;

    tracing::info!(hash = %message_hash, "polling Circle IRIS for attestation");

    loop {
        match client.get(&url).send().await {
            Ok(r) if r.status().is_success() => {
                if let Ok(body) = r.json::<serde_json::Value>().await {
                    let status = body["status"].as_str().unwrap_or("");
                    if status == "complete" {
                        if let Some(att_hex) = body["attestation"].as_str() {
                            let bytes =
                                hex::decode(att_hex.trim_start_matches("0x")).map_err(|e| {
                                    CctpError::Deserialization(format!(
                                        "decode attestation hex: {e}"
                                    ))
                                })?;
                            tracing::info!(
                                hash = %message_hash,
                                att_len = bytes.len(),
                                "attestation received from Circle IRIS"
                            );
                            return Ok(bytes);
                        }
                    }
                    tracing::debug!(status, "attestation not ready yet");
                }
            }
            Ok(r) => {
                tracing::debug!(status = %r.status(), "IRIS returned non-200");
            }
            Err(e) => {
                tracing::warn!(error = %e, "IRIS request failed");
            }
        }

        if tokio::time::Instant::now() >= deadline {
            return Err(CctpError::Query(format!(
                "timed out waiting for Circle IRIS attestation (hash={message_hash})"
            )));
        }
        tokio::time::sleep(DEFAULT_POLL_INTERVAL).await;
    }
}
