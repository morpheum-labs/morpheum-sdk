//! gRPC transport for the Morpheum SDK, backed by a `tonic::transport::Channel`.
//!
//! Implements the core `Transport` trait using raw tonic unary calls with a
//! generic bytes codec. This allows any SDK module client (BankClient,
//! IdentityClient, etc.) to issue queries and broadcast transactions through
//! a real gRPC connection without the module needing to know about tonic.

use std::str::FromStr;

use async_trait::async_trait;
use prost::bytes::{Buf, BufMut};
use prost::Message;
use tonic::codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};
use tonic::transport::Channel;

use morpheum_sdk_core::{BroadcastResult, SdkError, Transport};

/// gRPC transport backed by a `tonic::transport::Channel`.
///
/// Satisfies the SDK's `Transport` trait so that any `MorpheumClient`
/// implementation can query or broadcast through a live Mormcore node.
pub struct GrpcTransport {
    channel: Channel,
}

impl GrpcTransport {
    /// Connects to the given gRPC endpoint (e.g. `http://127.0.0.1:26657`).
    pub async fn connect(endpoint: &str) -> Result<Self, SdkError> {
        let channel = Channel::from_shared(endpoint.to_string())
            .map_err(|e| SdkError::transport(format!("invalid endpoint: {e}")))?
            .connect()
            .await
            .map_err(|e| SdkError::transport(format!("gRPC connect to {endpoint} failed: {e}")))?;
        Ok(Self { channel })
    }

    /// Issues a raw unary gRPC call at the given method path.
    async fn raw_unary(&self, path: &str, data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
        let path = http::uri::PathAndQuery::from_str(path)
            .map_err(|e| SdkError::transport(format!("invalid gRPC path '{path}': {e}")))?;

        let mut grpc = tonic::client::Grpc::new(self.channel.clone());
        grpc.ready()
            .await
            .map_err(|e| SdkError::transport(format!("channel not ready: {e}")))?;

        let resp = grpc
            .unary(tonic::Request::new(data), path, RawBytesCodec)
            .await
            .map_err(|e| SdkError::transport(format!("gRPC call failed: {e}")))?;

        Ok(resp.into_inner())
    }
}

#[async_trait(?Send)]
impl Transport for GrpcTransport {
    async fn broadcast_tx(&self, tx_bytes: Vec<u8>) -> Result<BroadcastResult, SdkError> {
        use morpheum_proto::tx::v1::{SubmitTxRequest, SubmitTxResponse, Tx};

        let tx = Tx::decode(tx_bytes.as_slice())?;
        let req = SubmitTxRequest {
            tx: Some(tx),
            ..Default::default()
        };
        let resp_bytes = self
            .raw_unary("/tx.v1.IngressService/SubmitTx", req.encode_to_vec())
            .await?;

        let resp = SubmitTxResponse::decode(resp_bytes.as_slice())?;

        if !resp.accepted {
            return Err(SdkError::transport(format!(
                "transaction rejected: {}",
                resp.error_message
            )));
        }

        Ok(BroadcastResult {
            txhash: resp.txhash,
            raw_response: None,
        })
    }

    async fn query(&self, path: &str, data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
        self.raw_unary(path, data).await
    }
}

// ── Raw bytes codec for transport-agnostic gRPC calls ──────────────────────

/// A tonic `Codec` that passes raw byte vectors without any proto (de)serialization.
///
/// This enables `GrpcTransport` to call *any* gRPC method path generically —
/// the proto encode/decode is handled by the SDK module client, not the transport.
#[derive(Debug, Clone, Default)]
struct RawBytesCodec;

impl Codec for RawBytesCodec {
    type Encode = Vec<u8>;
    type Decode = Vec<u8>;
    type Encoder = RawBytesEncoder;
    type Decoder = RawBytesDecoder;

    fn encoder(&mut self) -> Self::Encoder {
        RawBytesEncoder
    }

    fn decoder(&mut self) -> Self::Decoder {
        RawBytesDecoder
    }
}

#[derive(Debug, Clone)]
struct RawBytesEncoder;

impl Encoder for RawBytesEncoder {
    type Item = Vec<u8>;
    type Error = tonic::Status;

    fn encode(
        &mut self,
        item: Self::Item,
        dst: &mut EncodeBuf<'_>,
    ) -> Result<(), Self::Error> {
        dst.put_slice(&item);
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct RawBytesDecoder;

impl Decoder for RawBytesDecoder {
    type Item = Vec<u8>;
    type Error = tonic::Status;

    fn decode(
        &mut self,
        src: &mut DecodeBuf<'_>,
    ) -> Result<Option<Self::Item>, Self::Error> {
        let remaining = src.remaining();
        let mut buf = vec![0u8; remaining];
        src.copy_to_slice(&mut buf);
        Ok(Some(buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grpc_transport_compiles() {
        fn _assert_transport<T: Transport>() {}
        _assert_transport::<GrpcTransport>();
    }
}
