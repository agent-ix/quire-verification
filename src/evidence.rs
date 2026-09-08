// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Exact-scope evidence matching without sufficiency decisions (FR-006, FR-007).

use std::collections::BTreeSet;

use chrono::{DateTime, Datelike, FixedOffset, TimeDelta};
use serde_json::{Value, json};

use crate::contracts::{
    VerificationError, VerificationErrorCode, jcs_canonicalize, jcs_equal,
    validate_schema_definition,
};
use crate::value::{arrays_field, field, string_field};

/// Assesses result compatibility for every inventory obligation.
///
/// # Errors
/// Returns a schema, timestamp, duplicate-identifier, or canonicalization error.
pub fn assess_evidence(packet: &Value) -> Result<Value, VerificationError> {
    let inventory = arrays_field(packet, "inventory", "evidence packet")?;
    let results = arrays_field(packet, "results", "evidence packet")?;
    let as_of_text = string_field(packet, "asOf", "evidence packet")?;
    let as_of = parse_timestamp(as_of_text, "asOf")?;
    let mut result_ids = BTreeSet::new();
    for result in results {
        validate_schema_definition("TechniqueResult", result)?;
        let result_id = string_field(result, "resultId", "result")?;
        if !result_ids.insert(result_id) {
            return Err(VerificationError::new(
                VerificationErrorCode::DuplicateResultId,
                "duplicate result id",
            )
            .with_context("result_id", result_id));
        }
    }

    let mut obligations = Vec::with_capacity(inventory.len());
    for property in inventory {
        validate_schema_definition("EvidenceObligation", property)?;
        let property_identity = string_field(
            field(property, "propertyRef", "property")?,
            "identity",
            "property.propertyRef",
        )?;
        let candidates = results
            .iter()
            .filter(|result| {
                result
                    .pointer("/propertyRef/identity")
                    .and_then(Value::as_str)
                    == Some(property_identity)
            })
            .collect::<Vec<_>>();
        let mut exact = Vec::new();
        for result in &candidates {
            if scope_differences(property, result)?.is_empty() {
                exact.push(*result);
            }
        }
        exact.sort_by(|left, right| {
            left.get("resultId")
                .and_then(Value::as_str)
                .cmp(&right.get("resultId").and_then(Value::as_str))
        });
        if exact.is_empty() {
            let mut gaps = Vec::new();
            for result in candidates {
                for dimension in scope_differences(property, result)? {
                    gaps.push(json!({"kind": "scope-mismatch", "dimension": dimension, "resultId": string_field(result, "resultId", "result")?}));
                }
            }
            sort_canonical(&mut gaps)?;
            obligations.push(json!({
                "propertyId": string_field(property, "propertyId", "property")?,
                "status": "missing",
                "resultId": null,
                "gaps": gaps,
            }));
            continue;
        }
        if exact.len() > 1 {
            let ids = exact
                .iter()
                .map(|result| {
                    string_field(result, "resultId", "result")
                        .map(|id| Value::String(id.to_owned()))
                })
                .collect::<Result<Vec<_>, _>>()?;
            obligations.push(json!({
                "propertyId": string_field(property, "propertyId", "property")?,
                "status": "ambiguous",
                "resultId": null,
                "candidateResultIds": ids,
                "gaps": [{"kind": "ambiguous-results", "resultIds": ids}],
            }));
            continue;
        }
        let result = exact.into_iter().next().ok_or_else(|| {
            VerificationError::new(
                VerificationErrorCode::InvalidInput,
                "exact result set unexpectedly became empty",
            )
        })?;
        let mut findings = findings_for(result)?;
        if is_stale(property, result, as_of)? {
            findings.insert(0, json!({"kind": "freshness-expired"}));
            obligations.push(result_view(property, result, "stale", &findings)?);
        } else {
            obligations.push(result_view(property, result, classify(result)?, &findings)?);
        }
    }
    Ok(json!({
        "asOf": as_of_text,
        "denominator": inventory.len(),
        "obligations": obligations,
        "evidenceOwner": "Quoin",
        "sufficiencyDecision": "not-made",
        "statusSemantics": "compatibility summary only; gaps enumerate every applicable independent result finding",
    }))
}

/// Returns every identity or scope dimension that differs under RFC 8785 JCS.
///
/// # Errors
/// Returns an input or canonicalization error.
pub fn scope_differences(
    property: &Value,
    result: &Value,
) -> Result<Vec<&'static str>, VerificationError> {
    let binding = field(property, "resultBinding", "property")?;
    let provenance = field(result, "provenance", "result")?;
    let pairs = [
        (
            "techniqueId",
            field(property, "techniqueId", "property")?,
            field(result, "techniqueId", "result")?,
        ),
        (
            "propertyRef",
            field(property, "propertyRef", "property")?,
            field(result, "propertyRef", "result")?,
        ),
        (
            "subjectRefs",
            field(property, "subjectRefs", "property")?,
            field(result, "subjectRefs", "result")?,
        ),
        (
            "profile",
            field(property, "profile", "property")?,
            field(result, "profile", "result")?,
        ),
        (
            "bindingRefs",
            field(property, "bindingRefs", "property")?,
            field(result, "bindingRefs", "result")?,
        ),
        (
            "dependencyClosure",
            field(property, "dependencyClosure", "property")?,
            field(result, "dependencyClosure", "result")?,
        ),
        (
            "executionScope",
            field(property, "executionScope", "property")?,
            field(result, "executionScope", "result")?,
        ),
        (
            "planId",
            field(binding, "planId", "resultBinding")?,
            field(result, "planId", "result")?,
        ),
        (
            "taskId",
            field(binding, "taskId", "resultBinding")?,
            field(result, "taskId", "result")?,
        ),
        (
            "runRef",
            field(binding, "runRef", "resultBinding")?,
            field(result, "runRef", "result")?,
        ),
        (
            "producer",
            field(binding, "producer", "resultBinding")?,
            field(provenance, "producer", "provenance")?,
        ),
        (
            "producerVersion",
            field(binding, "producerVersion", "resultBinding")?,
            field(provenance, "producerVersion", "provenance")?,
        ),
        (
            "adapter",
            field(binding, "adapter", "resultBinding")?,
            field(provenance, "adapter", "provenance")?,
        ),
        (
            "adapterVersion",
            field(binding, "adapterVersion", "resultBinding")?,
            field(provenance, "adapterVersion", "provenance")?,
        ),
    ];
    let mut differences = Vec::new();
    for (dimension, expected, actual) in pairs {
        if !jcs_equal(expected, actual)? {
            differences.push(dimension);
        }
    }
    Ok(differences)
}

/// Enumerates all independent non-success findings in deterministic order.
///
/// # Errors
/// Returns an input or canonicalization error.
pub fn findings_for(result: &Value) -> Result<Vec<Value>, VerificationError> {
    let execution = field(result, "execution", "result")?;
    let activation = field(result, "activation", "result")?;
    let adequacy = field(result, "adequacy", "result")?;
    let participation = field(result, "participation", "result")?;
    let logical = field(result, "logical", "result")?;
    let execution_status = string_field(execution, "status", "execution")?;
    let detail = execution.get("detail").cloned().unwrap_or(Value::Null);
    let mut gaps = Vec::new();
    match execution_status {
        "unsupported" => gaps.push(json!({"kind": "unsupported", "detail": detail})),
        "failed" => gaps.push(json!({"kind": "failed", "detail": detail})),
        "completed" => {}
        _ => gaps.push(json!({"kind": "incomplete", "detail": detail})),
    }
    match string_field(activation, "status", "activation")? {
        "not-activated" => gaps.push(json!({"kind": "required-activation-absent"})),
        "unknown" => gaps.push(json!({"kind": "activation-unknown"})),
        _ => {}
    }
    if matches!(
        string_field(adequacy, "status", "adequacy")?,
        "inadequate" | "unknown" | "not-assessed"
    ) {
        gaps.push(json!({"kind": "observation-inadequate", "missing": adequacy.get("missingObservations").cloned().unwrap_or_else(|| json!([]))}));
    }
    let participation_status = string_field(participation, "status", "participation")?;
    if matches!(participation_status, "partial" | "not-started" | "unknown") {
        gaps.push(json!({"kind": "partial-participation", "status": participation_status, "missing": participation.get("missingParticipants").cloned().unwrap_or_else(|| json!([]))}));
    }
    for assumption in arrays_field(result, "assumptions", "result")? {
        let status = string_field(assumption, "status", "assumption")?;
        let assumption_id = string_field(assumption, "assumptionId", "assumption")?;
        if status == "not-met" {
            gaps.push(json!({"kind": "assumption-unmet", "assumptionId": assumption_id}));
        } else if status != "met" {
            gaps.push(json!({"kind": "assumption-inconclusive", "assumptionId": assumption_id, "status": status}));
        }
    }
    let outcome = string_field(logical, "outcome", "logical")?;
    if outcome == "violated" {
        gaps.push(json!({"kind": "logical-violation"}));
    } else if execution_status == "completed" && !matches!(outcome, "satisfied" | "violated") {
        gaps.push(json!({"kind": "logical-inconclusive", "outcome": outcome}));
    }
    sort_canonical(&mut gaps)?;
    Ok(gaps)
}

fn classify(result: &Value) -> Result<&'static str, VerificationError> {
    let execution = string_field(field(result, "execution", "result")?, "status", "execution")?;
    if execution == "unsupported" {
        return Ok("unsupported");
    }
    if execution == "failed" {
        return Ok("failed");
    }
    if execution != "completed" {
        return Ok("incomplete");
    }
    let activation = string_field(
        field(result, "activation", "result")?,
        "status",
        "activation",
    )?;
    if activation == "not-activated" {
        return Ok("untriggered");
    }
    let adequacy = string_field(field(result, "adequacy", "result")?, "status", "adequacy")?;
    if matches!(adequacy, "inadequate" | "unknown" | "not-assessed") {
        return Ok("inadequate");
    }
    if activation == "unknown" {
        return Ok("incomplete");
    }
    let participation = string_field(
        field(result, "participation", "result")?,
        "status",
        "participation",
    )?;
    if matches!(participation, "partial" | "not-started" | "unknown") {
        return Ok("partial");
    }
    let assumptions = arrays_field(result, "assumptions", "result")?;
    if assumptions
        .iter()
        .any(|item| item.get("status").and_then(Value::as_str) == Some("not-met"))
    {
        return Ok("assumption-unmet");
    }
    if assumptions
        .iter()
        .any(|item| item.get("status").and_then(Value::as_str) != Some("met"))
    {
        return Ok("inconclusive");
    }
    Ok(
        match string_field(field(result, "logical", "result")?, "outcome", "logical")? {
            "violated" => "violated",
            "satisfied" => "supported",
            _ => "inconclusive",
        },
    )
}

fn is_stale(
    property: &Value,
    result: &Value,
    as_of: DateTime<FixedOffset>,
) -> Result<bool, VerificationError> {
    let produced = parse_timestamp(
        string_field(result, "producedAt", "result")?,
        "result.producedAt",
    )?;
    if produced > as_of {
        return Err(VerificationError::new(
            VerificationErrorCode::InvalidTemporalOrder,
            "result production time is later than assessment time",
        )
        .with_context("assessment_time", as_of.to_string())
        .with_context("produced_at", produced.to_string()));
    }

    let max_age = property
        .pointer("/freshness/maxAgeSeconds")
        .and_then(Value::as_i64)
        .ok_or_else(|| {
            VerificationError::new(
                VerificationErrorCode::InvalidInput,
                "evidence obligation has no valid maximum age",
            )
        })?;
    let age = TimeDelta::try_seconds(max_age).ok_or_else(|| {
        VerificationError::new(
            VerificationErrorCode::InvalidTimestamp,
            "maximum age cannot be represented as a timestamp duration",
        )
        .with_context("max_age_seconds", max_age.to_string())
    })?;
    let age_expiry = produced.checked_add_signed(age).ok_or_else(|| {
        VerificationError::new(
            VerificationErrorCode::InvalidTimestamp,
            "maximum age produces an expiry outside the timestamp domain",
        )
        .with_context("max_age_seconds", max_age.to_string())
        .with_context("produced_at", produced.to_string())
    })?;
    if !(0..=9999).contains(&age_expiry.year()) {
        return Err(VerificationError::new(
            VerificationErrorCode::InvalidTimestamp,
            "maximum age produces an expiry outside the RFC 3339 year domain",
        )
        .with_context("expiry", age_expiry.to_string()));
    }

    let effective_expiry = if let Some(valid_until) = result.get("validUntil") {
        let valid_until = parse_timestamp(
            valid_until.as_str().ok_or_else(|| {
                VerificationError::new(
                    VerificationErrorCode::InvalidTimestamp,
                    "result.validUntil is not a timestamp string",
                )
            })?,
            "result.validUntil",
        )?;
        if valid_until < produced {
            return Err(VerificationError::new(
                VerificationErrorCode::InvalidTemporalOrder,
                "result validity ends before result production",
            )
            .with_context("produced_at", produced.to_string())
            .with_context("valid_until", valid_until.to_string()));
        }
        std::cmp::min(age_expiry, valid_until)
    } else {
        age_expiry
    };

    Ok(as_of > effective_expiry)
}

fn result_view(
    property: &Value,
    result: &Value,
    status: &str,
    gaps: &[Value],
) -> Result<Value, VerificationError> {
    Ok(json!({
        "propertyId": string_field(property, "propertyId", "property")?,
        "status": status,
        "freshness": if status == "stale" { "stale" } else { "fresh" },
        "resultId": field(result, "resultId", "result")?,
        "execution": field(result, "execution", "result")?,
        "logical": field(result, "logical", "result")?,
        "activation": field(result, "activation", "result")?,
        "participation": field(result, "participation", "result")?,
        "assumptions": field(result, "assumptions", "result")?,
        "adequacy": field(result, "adequacy", "result")?,
        "provenance": field(result, "provenance", "result")?,
        "nativeArtifacts": field(result, "nativeArtifacts", "result")?,
        "gaps": gaps,
    }))
}

fn sort_canonical(values: &mut [Value]) -> Result<(), VerificationError> {
    let mut keyed = values
        .iter()
        .map(|value| jcs_canonicalize(value).map(|key| (key, value.clone())))
        .collect::<Result<Vec<_>, _>>()?;
    keyed.sort_by(|left, right| left.0.cmp(&right.0));
    for (slot, (_, value)) in values.iter_mut().zip(keyed) {
        *slot = value;
    }
    Ok(())
}

fn parse_timestamp(value: &str, name: &str) -> Result<DateTime<FixedOffset>, VerificationError> {
    DateTime::parse_from_rfc3339(value).map_err(|error| {
        VerificationError::new(VerificationErrorCode::InvalidTimestamp, error.to_string())
            .with_context("field", name)
    })
}
