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
    IssueVcRequest, RevokeVcRequest, SelfRevokeVcRequest, UpdateClaimsRequest, UpdateParamsRequest,
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
    claims_commitment: Vec<u8>,
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

    /// Sets a pre-computed zkClaims commitment for a privacy-mode credential
    /// (WS3-D). Use this when the 32-byte Pedersen commitment was computed
    /// elsewhere (e.g. the prover crate). The numeric [`claims`](Self::claims)
    /// must be zero alongside a commitment (mode disjointness, enforced on
    /// chain). Prefer [`privacy_limits`](Self::privacy_limits) (with the `zk`
    /// feature) which computes the commitment and zeroes the numeric fields for
    /// you.
    pub fn claims_commitment(mut self, commitment: Vec<u8>) -> Self {
        self.claims_commitment = commitment;
        self
    }

    /// Configures this builder for a **privacy-mode** credential (WS3-D): the
    /// four owner-issued limits stay hidden behind a Pedersen commitment and the
    /// on-chain numeric claims are zeroed (mode disjointness). Computes
    /// `claims_commitment` from the supplied limits and a 32-byte `blinding`
    /// (a canonical JubJub scalar) via the same primitive the zkClaims circuit
    /// opens, so the resulting commitment verifies against proofs produced for
    /// these limits.
    ///
    /// Any previously set [`claims`](Self::claims) are replaced; a custom
    /// constraints string, if present, is preserved (it is not part of the ZK
    /// statement). Requires the `zk` feature.
    #[cfg(feature = "zk")]
    pub fn privacy_limits(
        mut self,
        max_position_usd: u64,
        max_daily_usd: u64,
        max_slippage_bps: u32,
        allowed_pairs_bitflags: u64,
        blinding: &[u8; 32],
    ) -> Result<Self, SdkError> {
        let commitment = morpheum_primitives::crypto::zk::claims_pedersen_commit(
            max_position_usd,
            max_daily_usd,
            max_slippage_bps,
            allowed_pairs_bitflags,
            blinding,
        )
        .map_err(|_| {
            SdkError::invalid_input(
                "failed to compute zkClaims commitment (non-canonical blinding scalar)",
            )
        })?;

        let custom_constraints = self.claims.and_then(|c| c.custom_constraints);
        self.claims = Some(VcClaims {
            custom_constraints,
            ..Default::default()
        });
        self.claims_commitment = commitment.to_vec();
        Ok(self)
    }

    /// Builds the issuance request, performing validation.
    pub fn build(self) -> Result<IssueVcRequest, SdkError> {
        let issuer = self
            .issuer
            .ok_or_else(|| SdkError::invalid_input("issuer is required for VC issuance"))?;

        let subject = self
            .subject
            .ok_or_else(|| SdkError::invalid_input("subject is required for VC issuance"))?;

        let claims = self
            .claims
            .ok_or_else(|| SdkError::invalid_input("claims are required for VC issuance"))?;

        let mut req = IssueVcRequest::new(issuer, subject, claims);

        if let Some(expiry) = self.expiry_timestamp {
            req = req.with_expiry(expiry);
        }

        if !self.claims_commitment.is_empty() {
            req = req.with_claims_commitment(self.claims_commitment);
        }

        Ok(req)
    }
}

/// Fluent builder for revoking a VC (issuer-initiated).
#[derive(Default)]
pub struct VcRevokeBuilder {
    vc_id: Option<String>,
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

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn build(self) -> Result<RevokeVcRequest, SdkError> {
        let vc_id = self
            .vc_id
            .ok_or_else(|| SdkError::invalid_input("vc_id is required for revocation"))?;

        Ok(RevokeVcRequest {
            vc_id,
            reason: self.reason,
        })
    }
}

/// Fluent builder for self-revocation of a VC by the subject agent.
#[derive(Default)]
pub struct VcSelfRevokeBuilder {
    vc_id: Option<String>,
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

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn build(self) -> Result<SelfRevokeVcRequest, SdkError> {
        let vc_id = self
            .vc_id
            .ok_or_else(|| SdkError::invalid_input("vc_id is required for self-revocation"))?;

        Ok(SelfRevokeVcRequest {
            vc_id,
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
///     .new_claims(VcClaims {
///         max_daily_usd: 200_000,
///         ..Default::default()
///     })
///     .build()?;
/// ```
#[derive(Default)]
pub struct UpdateClaimsBuilder {
    vc_id: Option<String>,
    new_claims: Option<VcClaims>,
    claims_commitment: Vec<u8>,
}

impl UpdateClaimsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vc_id(mut self, vc_id: impl Into<String>) -> Self {
        self.vc_id = Some(vc_id.into());
        self
    }

    /// Sets the new claims to replace the existing ones.
    pub fn new_claims(mut self, claims: VcClaims) -> Self {
        self.new_claims = Some(claims);
        self
    }

    /// Sets a pre-computed zkClaims commitment, rotating the credential into
    /// privacy mode (WS3-D). The numeric [`new_claims`](Self::new_claims) must
    /// be zero alongside a commitment. Prefer
    /// [`privacy_limits`](Self::privacy_limits) (with the `zk` feature).
    pub fn claims_commitment(mut self, commitment: Vec<u8>) -> Self {
        self.claims_commitment = commitment;
        self
    }

    /// Rotates this claims update into **privacy mode** (WS3-D): computes the
    /// `claims_commitment` from the supplied limits and a 32-byte `blinding`
    /// scalar and zeroes the on-chain numeric claims. Requires the `zk` feature.
    #[cfg(feature = "zk")]
    pub fn privacy_limits(
        mut self,
        max_position_usd: u64,
        max_daily_usd: u64,
        max_slippage_bps: u32,
        allowed_pairs_bitflags: u64,
        blinding: &[u8; 32],
    ) -> Result<Self, SdkError> {
        let commitment = morpheum_primitives::crypto::zk::claims_pedersen_commit(
            max_position_usd,
            max_daily_usd,
            max_slippage_bps,
            allowed_pairs_bitflags,
            blinding,
        )
        .map_err(|_| {
            SdkError::invalid_input(
                "failed to compute zkClaims commitment (non-canonical blinding scalar)",
            )
        })?;

        let custom_constraints = self.new_claims.and_then(|c| c.custom_constraints);
        self.new_claims = Some(VcClaims {
            custom_constraints,
            ..Default::default()
        });
        self.claims_commitment = commitment.to_vec();
        Ok(self)
    }

    /// Builds the update claims request, performing validation.
    pub fn build(self) -> Result<UpdateClaimsRequest, SdkError> {
        let vc_id = self
            .vc_id
            .ok_or_else(|| SdkError::invalid_input("vc_id is required for claims update"))?;

        let new_claims = self
            .new_claims
            .ok_or_else(|| SdkError::invalid_input("new_claims are required for claims update"))?;

        let mut req = UpdateClaimsRequest::new(vc_id, new_claims);

        if !self.claims_commitment.is_empty() {
            req = req.with_claims_commitment(self.claims_commitment);
        }

        Ok(req)
    }
}

/// Fluent builder for updating VC module parameters (governance-only).
#[derive(Default)]
pub struct UpdateModuleParamsBuilder {
    authority: Option<String>,
    params: Option<crate::types::Params>,
}

impl UpdateModuleParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn authority(mut self, authority: impl Into<String>) -> Self {
        self.authority = Some(authority.into());
        self
    }

    pub fn params(mut self, params: crate::types::Params) -> Self {
        self.params = Some(params);
        self
    }

    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        let authority = self
            .authority
            .ok_or_else(|| SdkError::invalid_input("authority is required for UpdateParams"))?;

        let params = self
            .params
            .ok_or_else(|| SdkError::invalid_input("params are required for UpdateParams"))?;

        Ok(UpdateParamsRequest::new(authority, params))
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
        let request = VcRevokeBuilder::new()
            .vc_id("vc_test_001")
            .reason("Test revocation")
            .build()
            .unwrap();

        assert_eq!(request.vc_id, "vc_test_001");
        assert_eq!(request.reason, Some("Test revocation".into()));
    }

    #[test]
    fn update_claims_builder_works() {
        let request = UpdateClaimsBuilder::new()
            .vc_id("vc_test_002")
            .new_claims(VcClaims {
                max_daily_usd: 200_000,
                ..Default::default()
            })
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

    #[test]
    fn issue_builder_accepts_precomputed_commitment() {
        let issuer = AccountId::new([1u8; 32]);
        let subject = AccountId::new([2u8; 32]);
        let request = VcIssueBuilder::new()
            .issuer(issuer)
            .subject(subject)
            .claims(VcClaims::default())
            .claims_commitment(vec![7u8; 32])
            .build()
            .unwrap();

        assert_eq!(request.claims_commitment, vec![7u8; 32]);
        assert_eq!(request.claims.max_position_usd, 0);
    }

    #[cfg(feature = "zk")]
    #[test]
    fn privacy_limits_zeroes_claims_and_sets_commitment() {
        let issuer = AccountId::new([1u8; 32]);
        let subject = AccountId::new([2u8; 32]);
        let blinding = [3u8; 32];

        let request = VcIssueBuilder::new()
            .issuer(issuer)
            .subject(subject)
            .claims(VcClaims {
                custom_constraints: Some("{\"k\":1}".into()),
                ..Default::default()
            })
            .privacy_limits(500_000, 100_000, 50, 0b0011, &blinding)
            .unwrap()
            .build()
            .unwrap();

        // Numeric limits are hidden (zeroed) in privacy mode; custom constraints kept.
        assert_eq!(request.claims.max_position_usd, 0);
        assert_eq!(request.claims.max_daily_usd, 0);
        assert_eq!(request.claims.max_slippage_bps, 0);
        assert_eq!(request.claims.allowed_pairs_bitflags, 0);
        assert_eq!(request.claims.custom_constraints.as_deref(), Some("{\"k\":1}"));

        // Commitment is the deterministic 32-byte Pedersen commitment.
        assert_eq!(request.claims_commitment.len(), 32);
        let expected = morpheum_primitives::crypto::zk::claims_pedersen_commit(
            500_000, 100_000, 50, 0b0011, &blinding,
        )
        .unwrap();
        assert_eq!(request.claims_commitment, expected.to_vec());
    }
}
