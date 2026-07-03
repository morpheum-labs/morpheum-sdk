//! Request and response wrappers for the VC module.
//!
//! These provide clean, type-safe Rust APIs around the raw protobuf messages.
//! They use `AccountId` for addresses, offer ergonomic constructors and helpers,
//! and include `to_any()` methods for seamless integration with `TxBuilder`.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;

use morpheum_proto::vc::v1 as proto;
use morpheum_sdk_core::AccountId;

use crate::types::VcClaims;

// ====================== TRANSACTION REQUESTS ======================

/// Request to issue a new Verifiable Credential (issuer → subject).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IssueVcRequest {
    pub issuer: AccountId,
    pub subject: AccountId,
    pub claims: VcClaims,
    pub expiry_timestamp: u64, // 0 = use module default
    /// zkClaims commitment binding the hidden owner-issued limits (WS3-D).
    /// Empty for a plaintext credential (the default); a 32-byte Pedersen
    /// commitment for a privacy-mode credential, in which case the numeric
    /// `claims` fields MUST be zero (mode disjointness, enforced on-chain at
    /// issuance). Build privacy credentials with
    /// [`VcIssueBuilder::privacy_limits`](crate::VcIssueBuilder::privacy_limits)
    /// (with the `zk` feature) or set a pre-computed commitment via
    /// [`with_claims_commitment`](Self::with_claims_commitment).
    pub claims_commitment: Vec<u8>,
}

impl IssueVcRequest {
    /// Creates a new VC issuance request.
    pub fn new(
        issuer: impl Into<AccountId>,
        subject: impl Into<AccountId>,
        claims: VcClaims,
    ) -> Self {
        Self {
            issuer: issuer.into(),
            subject: subject.into(),
            claims,
            expiry_timestamp: 0,
            claims_commitment: Vec::new(),
        }
    }

    /// Sets a custom expiry timestamp (0 = use module default from Params).
    pub fn with_expiry(mut self, timestamp: u64) -> Self {
        self.expiry_timestamp = timestamp;
        self
    }

    /// Sets the zkClaims commitment for a privacy-mode credential (WS3-D). The
    /// numeric `claims` fields must be zero alongside a non-empty commitment.
    pub fn with_claims_commitment(mut self, commitment: Vec<u8>) -> Self {
        self.claims_commitment = commitment;
        self
    }

    /// Converts this request into a protobuf `Any` for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgIssue = self.clone().into();
        ProtoAny {
            type_url: "/vc.v1.MsgIssue".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<IssueVcRequest> for proto::MsgIssue {
    fn from(req: IssueVcRequest) -> Self {
        Self {
            issuer_agent_hash: req.issuer.to_string(),
            subject_agent_hash: req.subject.to_string(),
            claims: Some(req.claims.into()),
            expiry_timestamp: req.expiry_timestamp,
            claims_commitment: req.claims_commitment,
        }
    }
}

/// Request to revoke a VC (issuer-initiated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RevokeVcRequest {
    pub vc_id: String,
    pub reason: Option<String>,
}

impl RevokeVcRequest {
    pub fn new(vc_id: impl Into<String>) -> Self {
        Self {
            vc_id: vc_id.into(),
            reason: None,
        }
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgRevoke = self.clone().into();
        ProtoAny {
            type_url: "/vc.v1.MsgRevoke".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<RevokeVcRequest> for proto::MsgRevoke {
    fn from(req: RevokeVcRequest) -> Self {
        Self {
            vc_id: req.vc_id,
            reason: req.reason.unwrap_or_default(),
        }
    }
}

/// Request for an agent to self-revoke its own VC.
///
/// The subject's identity is established by the transaction signature through
/// the auth pipeline — no explicit `subject` field is needed in the message.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SelfRevokeVcRequest {
    pub vc_id: String,
    pub reason: Option<String>,
}

impl SelfRevokeVcRequest {
    pub fn new(vc_id: impl Into<String>) -> Self {
        Self {
            vc_id: vc_id.into(),
            reason: None,
        }
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgSelfRevoke = self.clone().into();
        ProtoAny {
            type_url: "/vc.v1.MsgSelfRevoke".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<SelfRevokeVcRequest> for proto::MsgSelfRevoke {
    fn from(req: SelfRevokeVcRequest) -> Self {
        Self {
            vc_id: req.vc_id,
            reason: req.reason.unwrap_or_default(),
        }
    }
}

/// Request to update claims on an existing VC (issuer-initiated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateClaimsRequest {
    pub vc_id: String,
    pub new_claims: VcClaims,
    /// zkClaims commitment for the updated claims (WS3-D). Empty keeps/rotates
    /// the credential into plaintext mode; a 32-byte commitment (with zeroed
    /// numeric `new_claims`) rotates it into privacy mode.
    pub claims_commitment: Vec<u8>,
}

impl UpdateClaimsRequest {
    pub fn new(vc_id: impl Into<String>, new_claims: VcClaims) -> Self {
        Self {
            vc_id: vc_id.into(),
            new_claims,
            claims_commitment: Vec::new(),
        }
    }

    /// Sets the zkClaims commitment for a privacy-mode claims update (WS3-D).
    pub fn with_claims_commitment(mut self, commitment: Vec<u8>) -> Self {
        self.claims_commitment = commitment;
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdateClaims = self.clone().into();
        ProtoAny {
            type_url: "/vc.v1.MsgUpdateClaims".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdateClaimsRequest> for proto::MsgUpdateClaims {
    fn from(req: UpdateClaimsRequest) -> Self {
        Self {
            vc_id: req.vc_id,
            new_claims: Some(req.new_claims.into()),
            claims_commitment: req.claims_commitment,
        }
    }
}

/// Update global VC module parameters (governance-only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsRequest {
    pub authority: String,
    pub params: crate::types::Params,
}

impl UpdateParamsRequest {
    pub fn new(authority: impl Into<String>, params: crate::types::Params) -> Self {
        Self {
            authority: authority.into(),
            params,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgUpdateParams {
            authority: self.authority.clone(),
            params: Some(self.params.clone().into()),
        };
        ProtoAny {
            type_url: "/vc.v1.MsgUpdateParams".into(),
            value: msg.encode_to_vec(),
        }
    }
}

// ====================== QUERY REQUESTS & RESPONSES ======================

/// Query a specific Verifiable Credential by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryVcRequest {
    pub vc_id: String,
}

impl QueryVcRequest {
    pub fn new(vc_id: String) -> Self {
        Self { vc_id }
    }
}

impl From<QueryVcRequest> for proto::QueryVcRequest {
    fn from(req: QueryVcRequest) -> Self {
        Self { vc_id: req.vc_id }
    }
}

/// Response containing a single VC and its status.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryVcResponse {
    pub vc: crate::types::Vc,
    pub is_valid: bool,
    pub is_revoked: bool,
    pub is_expired: bool,
}

impl From<proto::QueryVcResponse> for QueryVcResponse {
    fn from(res: proto::QueryVcResponse) -> Self {
        Self {
            vc: res.vc.map(crate::types::Vc::from).unwrap_or_default(),
            is_valid: res.is_valid,
            is_revoked: res.is_revoked,
            is_expired: res.is_expired,
        }
    }
}

/// Query the current status of a VC.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryVcStatusRequest {
    pub vc_id: String,
}

impl QueryVcStatusRequest {
    pub fn new(vc_id: String) -> Self {
        Self { vc_id }
    }
}

impl From<QueryVcStatusRequest> for proto::QueryVcStatusRequest {
    fn from(req: QueryVcStatusRequest) -> Self {
        Self { vc_id: req.vc_id }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryVcStatusResponse {
    pub vc_id: String,
    pub is_valid: bool,
    pub is_revoked: bool,
    pub is_expired: bool,
    pub revoked_at: u64,
    pub reason: String,
}

impl From<proto::QueryVcStatusResponse> for QueryVcStatusResponse {
    fn from(res: proto::QueryVcStatusResponse) -> Self {
        Self {
            vc_id: res.vc_id,
            is_valid: res.is_valid,
            is_revoked: res.is_revoked,
            is_expired: res.is_expired,
            revoked_at: res.revoked_at,
            reason: res.reason,
        }
    }
}

/// Query VCs issued by a specific issuer (paginated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryVcsByIssuerRequest {
    pub issuer: String,
    pub limit: u32,
    pub offset: u32,
}

impl QueryVcsByIssuerRequest {
    pub fn new(issuer: impl Into<String>, limit: u32, offset: u32) -> Self {
        Self {
            issuer: issuer.into(),
            limit,
            offset,
        }
    }
}

impl From<QueryVcsByIssuerRequest> for proto::QueryVcsByIssuerRequest {
    fn from(req: QueryVcsByIssuerRequest) -> Self {
        Self {
            issuer_agent_hash: req.issuer,
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Query VCs issued to a specific subject (paginated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryVcsBySubjectRequest {
    pub subject: String,
    pub limit: u32,
    pub offset: u32,
}

impl QueryVcsBySubjectRequest {
    pub fn new(subject: impl Into<String>, limit: u32, offset: u32) -> Self {
        Self {
            subject: subject.into(),
            limit,
            offset,
        }
    }
}

impl From<QueryVcsBySubjectRequest> for proto::QueryVcsBySubjectRequest {
    fn from(req: QueryVcsBySubjectRequest) -> Self {
        Self {
            subject_agent_hash: req.subject,
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Query the revocation bitmap for an issuer.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryRevocationBitmapRequest {
    pub issuer: String,
}

impl QueryRevocationBitmapRequest {
    pub fn new(issuer: impl Into<String>) -> Self {
        Self {
            issuer: issuer.into(),
        }
    }
}

impl From<QueryRevocationBitmapRequest> for proto::QueryRevocationBitmapRequest {
    fn from(req: QueryRevocationBitmapRequest) -> Self {
        Self {
            issuer_agent_hash: req.issuer,
        }
    }
}

/// Query the current VC module parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsRequest {}

impl From<QueryParamsRequest> for proto::QueryParamsRequest {
    fn from(_req: QueryParamsRequest) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use morpheum_sdk_core::AccountId;

    #[test]
    fn issue_request_to_any() {
        let issuer = AccountId::new([1u8; 32]);
        let subject = AccountId::new([2u8; 32]);
        let claims = crate::types::VcClaims {
            max_daily_usd: 100_000,
            allowed_pairs_bitflags: 1,
            max_slippage_bps: 50,
            max_position_usd: 1_000_000,
            custom_constraints: None,
        };

        let req = IssueVcRequest::new(issuer, subject, claims).with_expiry(1_800_000_000);

        let any = req.to_any();
        assert_eq!(any.type_url, "/vc.v1.MsgIssue");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn conversions_work() {
        let req = RevokeVcRequest::new("vc_123").with_reason("Test revocation");

        let proto: proto::MsgRevoke = req.into();
        assert_eq!(proto.reason, "Test revocation");
    }
}
