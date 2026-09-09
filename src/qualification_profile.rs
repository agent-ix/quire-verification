// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Frozen qualification-profile identity and external-interface readiness.
//!
//! Governing requirement: FR-022.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::contracts::{
    VerificationError, VerificationErrorCode, jcs_canonicalize, jcs_sha256,
    validate_schema_definition, wire,
};
use crate::shared_reference::ValidatedSharedReference;

/// An immutable, schema-derived qualification profile and its RFC 8785 identity.
#[derive(Clone, Debug)]
pub struct FrozenQualificationProfile {
    profile: wire::QualificationProfile,
    identity: Box<str>,
}

impl FrozenQualificationProfile {
    /// Returns the schema-derived profile value.
    #[must_use]
    pub const fn profile(&self) -> &wire::QualificationProfile {
        &self.profile
    }

    /// Returns the domain-separated JCS identity of the complete profile.
    #[must_use]
    pub const fn identity(&self) -> &str {
        &self.identity
    }
}

/// One producer-owned interface that prevents a profile from being frozen.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualificationProfileBlocker {
    interface_id: Box<str>,
    role: Box<str>,
    owner: Box<str>,
    status: Box<str>,
    reason: Box<str>,
}

impl QualificationProfileBlocker {
    /// Returns the profile-local interface identifier.
    #[must_use]
    pub const fn interface_id(&self) -> &str {
        &self.interface_id
    }

    /// Returns the interface role.
    #[must_use]
    pub const fn role(&self) -> &str {
        &self.role
    }

    /// Returns the producer or decision owner.
    #[must_use]
    pub const fn owner(&self) -> &str {
        &self.owner
    }

    /// Returns the non-accepted interface status.
    #[must_use]
    pub const fn status(&self) -> &str {
        &self.status
    }

    /// Returns the exact reason the interface is unavailable.
    #[must_use]
    pub const fn reason(&self) -> &str {
        &self.reason
    }
}

/// Result of evaluating a complete profile candidate.
#[derive(Clone, Debug)]
pub enum QualificationProfileOutcome {
    /// Every selected interface is accepted and the profile has been frozen.
    Frozen(Box<FrozenQualificationProfile>),
    /// One or more producer-owned interfaces are not accepted.
    Blocked(Box<[QualificationProfileBlocker]>),
}

/// Validates a profile against the authoritative schema and strict-reader proofs.
///
/// # Errors
/// Returns a bounded-input, identity, or schema error when any required dimension is
/// missing, open, contradictory, unsupported, unproved, or carries an unknown field.
pub fn validate_qualification_profile(
    value: &Value,
    validated_references: &[ValidatedSharedReference],
) -> Result<(), VerificationError> {
    validate_schema_definition("QualificationProfile", value)?;
    ensure_strict_references(value, validated_references)?;
    ensure_unique_ids(value, "commands", "commandId")?;
    ensure_unique_ids(value, "semanticGuarantees", "guaranteeId")?;
    ensure_unique_ids(value, "resourceBudgets", "budgetId")?;
    ensure_unique_ids(value, "evidenceRequirements", "obligationId")?;
    ensure_unique_ids(value, "interfaces", "interfaceId")?;
    ensure_known_guarantees(value)
}

/// Freezes a complete profile or returns every unresolved producer interface.
///
/// Blocked interfaces never become local replacement definitions. A frozen profile
/// owns its generated Rust value and exposes no mutation path, so its stored identity
/// always names the same canonical bytes.
///
/// # Errors
/// Returns a bounded-input, identity, schema, decoding, or canonicalization error when
/// the candidate is malformed, lacks an exact strict-reader proof, or falls outside the
/// RFC 8785 domain.
pub fn freeze_qualification_profile(
    value: Value,
    validated_references: &[ValidatedSharedReference],
) -> Result<QualificationProfileOutcome, VerificationError> {
    validate_qualification_profile(&value, validated_references)?;
    let interfaces = value["interfaces"].as_array().ok_or_else(|| {
        invalid_profile(
            "QualificationProfile schema lost its required interfaces array",
            "interfaces",
        )
    })?;
    let mut blockers = interfaces
        .iter()
        .filter(|interface| interface["status"] != "accepted")
        .map(blocker_from_value)
        .collect::<Result<Vec<_>, _>>()?;
    blockers.sort_unstable_by(|left, right| left.interface_id.cmp(&right.interface_id));
    if !blockers.is_empty() {
        return Ok(QualificationProfileOutcome::Blocked(
            blockers.into_boxed_slice(),
        ));
    }

    let identity = jcs_sha256(&value)?.into_boxed_str();
    let profile = serde_json::from_value(value).map_err(|error| {
        VerificationError::new(
            VerificationErrorCode::SchemaViolation,
            format!("QualificationProfile schema-derived decoding failed: {error}"),
        )
    })?;
    Ok(QualificationProfileOutcome::Frozen(Box::new(
        FrozenQualificationProfile { profile, identity },
    )))
}

fn ensure_strict_references(
    profile: &Value,
    validated_references: &[ValidatedSharedReference],
) -> Result<(), VerificationError> {
    let reference_index = validated_references
        .iter()
        .filter_map(ValidatedSharedReference::reference_key)
        .collect::<BTreeMap<_, _>>();
    validate_bound_reference(
        &profile["candidateRef"],
        "candidateRef",
        "executable-projection",
        &reference_index,
    )?;
    validate_bound_reference(
        &profile["dependencyClosureRef"],
        "dependencyClosureRef",
        "dependency-closure",
        &reference_index,
    )?;
    validate_bound_reference(
        &profile["productionCallerRef"],
        "productionCallerRef",
        "invocation",
        &reference_index,
    )?;
    validate_bound_reference(
        &profile["environmentRef"],
        "environmentRef",
        "environment",
        &reference_index,
    )?;
    for command in profile["commands"]
        .as_array()
        .ok_or_else(|| invalid_profile("profile commands are not an array", "commands"))?
    {
        validate_bound_reference(
            &command["executableRef"],
            "commands.executableRef",
            "executable-projection",
            &reference_index,
        )?;
        validate_bound_reference(
            &command["workingDirectoryRef"],
            "commands.workingDirectoryRef",
            "environment",
            &reference_index,
        )?;
    }
    for interface in profile["interfaces"]
        .as_array()
        .ok_or_else(|| invalid_profile("profile interfaces are not an array", "interfaces"))?
    {
        if interface["status"] == "accepted" {
            let interface_role = interface["role"].as_str().ok_or_else(|| {
                invalid_profile("accepted interface has no semantic role", "interfaces.role")
            })?;
            let expected_role = expected_interface_artifact_kind(interface_role)?;
            validate_bound_reference(
                &interface["interfaceRef"],
                "interfaces.interfaceRef",
                expected_role,
                &reference_index,
            )?;
        }
    }
    Ok(())
}

fn expected_interface_artifact_kind(role: &str) -> Result<&'static str, VerificationError> {
    match role {
        "portable-contracts" | "shared-references" | "engineering-assurance" | "quoin-evidence" => {
            Ok("source")
        }
        "parser-compiler" => Ok("executable-projection"),
        "production-caller" => Ok("invocation"),
        _ => Err(invalid_profile(
            "qualification interface has an unsupported semantic role",
            role,
        )),
    }
}

fn validate_bound_reference(
    reference: &Value,
    field: &str,
    expected_role: &str,
    validated_references: &BTreeMap<&[u8], &str>,
) -> Result<(), VerificationError> {
    let canonical = jcs_canonicalize(reference)?;
    let Some(actual_role) = validated_references.get(canonical.as_slice()) else {
        return Err(VerificationError::new(
            VerificationErrorCode::IdentityMismatch,
            "qualification profile reference was not accepted by the strict reader",
        )
        .with_context("field", field)
        .with_context("expected_role", expected_role));
    };
    if *actual_role != expected_role {
        return Err(VerificationError::new(
            VerificationErrorCode::IdentityMismatch,
            "qualification profile reference has the wrong validated role",
        )
        .with_context("field", field)
        .with_context("actual_role", *actual_role)
        .with_context("expected_role", expected_role));
    }
    Ok(())
}

fn blocker_from_value(value: &Value) -> Result<QualificationProfileBlocker, VerificationError> {
    let string = |field: &str| {
        value[field].as_str().map(str::to_owned).ok_or_else(|| {
            VerificationError::new(
                VerificationErrorCode::SchemaViolation,
                "QualificationInterface lost a schema-required string",
            )
            .with_context("field", field)
        })
    };
    Ok(QualificationProfileBlocker {
        interface_id: string("interfaceId")?.into_boxed_str(),
        role: string("role")?.into_boxed_str(),
        owner: string("owner")?.into_boxed_str(),
        status: string("status")?.into_boxed_str(),
        reason: string("reason")?.into_boxed_str(),
    })
}

fn ensure_unique_ids(
    profile: &Value,
    collection: &str,
    identity: &str,
) -> Result<(), VerificationError> {
    let values = profile[collection]
        .as_array()
        .ok_or_else(|| invalid_profile("profile collection is not an array", collection))?;
    let mut seen = BTreeSet::new();
    for value in values {
        let identifier = value[identity]
            .as_str()
            .ok_or_else(|| invalid_profile("profile item identity is not a string", identity))?;
        if !seen.insert(identifier) {
            return Err(
                invalid_profile("duplicate profile item identity", identifier)
                    .with_context("collection", collection),
            );
        }
    }
    Ok(())
}

fn ensure_known_guarantees(profile: &Value) -> Result<(), VerificationError> {
    let guarantee_ids = profile["semanticGuarantees"]
        .as_array()
        .ok_or_else(|| {
            invalid_profile("semantic guarantees are not an array", "semanticGuarantees")
        })?
        .iter()
        .filter_map(|guarantee| guarantee["guaranteeId"].as_str())
        .collect::<BTreeSet<_>>();
    for requirement in profile["evidenceRequirements"].as_array().ok_or_else(|| {
        invalid_profile(
            "evidence requirements are not an array",
            "evidenceRequirements",
        )
    })? {
        let guarantee_id = requirement["guaranteeId"].as_str().ok_or_else(|| {
            invalid_profile("evidence guarantee identity is not a string", "guaranteeId")
        })?;
        if !guarantee_ids.contains(guarantee_id) {
            return Err(invalid_profile(
                "evidence requirement names no semantic guarantee",
                guarantee_id,
            ));
        }
    }
    Ok(())
}

fn invalid_profile(message: &str, value: &str) -> VerificationError {
    VerificationError::new(VerificationErrorCode::InvalidInput, message)
        .with_context("value", value)
}
