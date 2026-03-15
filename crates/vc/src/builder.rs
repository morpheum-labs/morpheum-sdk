//! Fluent builders for the VC module.
//!
//! This module provides ergonomic, type-safe fluent builders for the most
//! common and complex VC operations, particularly issuing Verifiable Credentials
//! with rich claims. All builders follow the classic Builder pattern and
//! return the corresponding request type from `requests.rs` for seamless
//! integration with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::{AccountId, SdkError};

use crate::requests::{
    IssueVcRequest,
    RevokeVcRequest,
    SelfRevokeVcRequest,
    UpdateClaimsRequest,
};
use crate::types::VcClaims;

/// Fluent builder for issuing a new Verifiable Credential.
///
/// This is the primary and most feature-rich builder in the VC module.
/// It supports rich claims, custom expiry, and full validation.
#[derive(Default)]
pub struct VcIssueBuilder {
    issuer: Option<AccountId>,
    subject: Option<AccountId>,
    claims: Option<VcClaims>,
    expiry_timestamp: Option<u64>,
    issuer_signature: Option<Vec<u8>>,
}

impl VcIssueBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the issuer of the VC (the agent issuing the credential).
    ///
    /// Accepts any type that converts into `AccountId`, including
    /// `morpheum_signing_core::types::AccountId` from a `Signer`.
    pub fn issuer(mut self, issuer: impl Into<AccountId>) -> Self {
        self.issuer = Some(issuer.into());
        self
    }

    /// Sets the subject of the VC (the agent receiving the credential).
    ///
    /// Accepts any type that converts into `AccountId`.
    pub fn subject(mut self, subject: impl Into<AccountId>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Sets the claims / permissions for this VC.
    pub fn claims(mut self, claims: VcClaims) -> Self {
        self.claims = Some(claims);
        self
    }

    /// Sets a custom expiry timestamp (0 = use module default from Params).
    pub fn expiry(mut self, timestamp: u64) -> Self {
        self.expiry_timestamp = Some(timestamp);
        self
    }

    /// Sets the issuer's signature (required for issuance).
    pub fn issuer_signature(mut self, signature: Vec<u8>) -> Self {
        self.issuer_signature = Some(signature);
        self
    }

    /// Builds the issuance request, performing validation.
    pub fn build(self) -> Result<IssueVcRequest, SdkError> {
        let issuer = self.issuer.ok_or_else(|| {
            SdkError::invalid_input("issuer is required for VC issuance")
        })?;

        let subject = self.subject.ok_or_else(|| {
            SdkError::invalid_input("subject is required for VC issuance")
        })?;

        let claims = self.claims.ok_or_else(|| {
            SdkError::invalid_input("claims are required for VC issuance")
        })?;

        let issuer_signature = self.issuer_signature.ok_or_else(|| {
            SdkError::invalid_input("issuer_signature is required for VC issuance")
        })?;

        let mut req = IssueVcRequest::new(
            issuer,
            subject,
            claims,
            issuer_signature,
        );

        if let Some(expiry) = self.expiry_timestamp {
            req = req.with_expiry(expiry);
        }

        Ok(req)
    }
}

/// Fluent builder for revoking a VC (issuer-initiated).
#[derive(Default)]
pub struct VcRevokeBuilder {
    vc_id: Option<String>,
    issuer: Option<AccountId>,
    issuer_signature: Option<Vec<u8>>,
    reason: Option<String>,
}

impl VcRevokeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vc_id(mut self, vc_id: impl Into<String>) -> Self {
        self.vc_id = Some(vc_id.into());
        self
    }

    /// Sets the issuer address.
    ///
    /// Accepts any type that converts into `AccountId`.
    pub fn issuer(mut self, issuer: impl Into<AccountId>) -> Self {
        self.issuer = Some(issuer.into());
        self
    }

    pub fn issuer_signature(mut self, signature: Vec<u8>) -> Self {
        self.issuer_signature = Some(signature);
        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn build(self) -> Result<RevokeVcRequest, SdkError> {
        let vc_id = self.vc_id.ok_or_else(|| {
            SdkError::invalid_input("vc_id is required for revocation")
        })?;

        let issuer = self.issuer.ok_or_else(|| {
            SdkError::invalid_input("issuer is required for revocation")
        })?;

        let issuer_signature = self.issuer_signature.ok_or_else(|| {
            SdkError::invalid_input("issuer_signature is required for revocation")
        })?;

        Ok(RevokeVcRequest {
            vc_id,
            issuer,
            issuer_signature,
            reason: self.reason,
        })
    }
}

/// Fluent builder for self-revocation of a VC by the subject agent.
#[derive(Default)]
pub struct VcSelfRevokeBuilder {
    vc_id: Option<String>,
    agent_signature: Option<Vec<u8>>,
    reason: Option<String>,
}

impl VcSelfRevokeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vc_id(mut self, vc_id: impl Into<String>) -> Self {
        self.vc_id = Some(vc_id.into());
        self
    }

    pub fn agent_signature(mut self, signature: Vec<u8>) -> Self {
        self.agent_signature = Some(signature);
        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn build(self) -> Result<SelfRevokeVcRequest, SdkError> {
        let vc_id = self.vc_id.ok_or_else(|| {
            SdkError::invalid_input("vc_id is required for self-revocation")
        })?;

        let agent_signature = self.agent_signature.ok_or_else(|| {
            SdkError::invalid_input("agent_signature is required for self-revocation")
        })?;

        Ok(SelfRevokeVcRequest {
            vc_id,
            agent_signature,
            reason: self.reason,
        })
    }
}

/// Fluent builder for updating claims on an existing VC (issuer-initiated).
///
/// # Example
/// ```rust,ignore
/// let request = UpdateClaimsBuilder::new()
///     .vc_id("vc_test_001")
///     .issuer(signer.account_id())
///     .new_claims(VcClaims {
///         max_daily_usd: 200_000,
///         ..Default::default()
///     })
///     .issuer_signature(sig_bytes)
///     .build()?;
/// ```
#[derive(Default)]
pub struct UpdateClaimsBuilder {
    vc_id: Option<String>,
    issuer: Option<AccountId>,
    new_claims: Option<VcClaims>,
    issuer_signature: Option<Vec<u8>>,
}

impl UpdateClaimsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vc_id(mut self, vc_id: impl Into<String>) -> Self {
        self.vc_id = Some(vc_id.into());
        self
    }

    /// Sets the issuer address.
    ///
    /// Accepts any type that converts into `AccountId`.
    pub fn issuer(mut self, issuer: impl Into<AccountId>) -> Self {
        self.issuer = Some(issuer.into());
        self
    }

    /// Sets the new claims to replace the existing ones.
    pub fn new_claims(mut self, claims: VcClaims) -> Self {
        self.new_claims = Some(claims);
        self
    }

    /// Sets the issuer's signature authorizing the update.
    pub fn issuer_signature(mut self, signature: Vec<u8>) -> Self {
        self.issuer_signature = Some(signature);
        self
    }

    /// Builds the update claims request, performing validation.
    pub fn build(self) -> Result<UpdateClaimsRequest, SdkError> {
        let vc_id = self.vc_id.ok_or_else(|| {
            SdkError::invalid_input("vc_id is required for claims update")
        })?;

        let issuer = self.issuer.ok_or_else(|| {
            SdkError::invalid_input("issuer is required for claims update")
        })?;

        let new_claims = self.new_claims.ok_or_else(|| {
            SdkError::invalid_input("new_claims are required for claims update")
        })?;

        let issuer_signature = self.issuer_signature.ok_or_else(|| {
            SdkError::invalid_input("issuer_signature is required for claims update")
        })?;

        Ok(UpdateClaimsRequest::new(vc_id, issuer, new_claims, issuer_signature))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use morpheum_sdk_core::AccountId;

    #[test]
    fn vc_issue_builder_full_flow() {
        let issuer = AccountId::new([1u8; 32]);
        let subject = AccountId::new([2u8; 32]);
        let claims = VcClaims {
            max_daily_usd: 100_000,
            allowed_pairs_bitflags: 0b0011,
            max_slippage_bps: 50,
            max_position_usd: 500_000,
            custom_constraints: Some("{\"max_leverage\": 20}".into()),
        };

        let request = VcIssueBuilder::new()
            .issuer(issuer.clone())
            .subject(subject.clone())
            .claims(claims)
            .expiry(1_800_000_000)
            .issuer_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.issuer, issuer);
        assert_eq!(request.subject, subject);
        assert_eq!(request.expiry_timestamp, 1_800_000_000);
    }

    #[test]
    fn vc_issue_builder_validation() {
        let result = VcIssueBuilder::new().build();
        assert!(result.is_err());
    }

    #[test]
    fn vc_revoke_builder_works() {
        let issuer = AccountId::new([1u8; 32]);
        let request = VcRevokeBuilder::new()
            .vc_id("vc_test_001")
            .issuer(issuer)
            .issuer_signature(vec![0u8; 64])
            .reason("Test revocation")
            .build()
            .unwrap();

        assert_eq!(request.vc_id, "vc_test_001");
        assert_eq!(request.reason, Some("Test revocation".into()));
    }

    #[test]
    fn update_claims_builder_works() {
        let issuer = AccountId::new([1u8; 32]);
        let request = UpdateClaimsBuilder::new()
            .vc_id("vc_test_002")
            .issuer(issuer)
            .new_claims(VcClaims {
                max_daily_usd: 200_000,
                ..Default::default()
            })
            .issuer_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.vc_id, "vc_test_002");
        assert_eq!(request.new_claims.max_daily_usd, 200_000);
        assert_eq!(request.new_claims.max_slippage_bps, 0); // default
    }

    #[test]
    fn update_claims_builder_validation() {
        let result = UpdateClaimsBuilder::new().build();
        assert!(result.is_err());
    }
}