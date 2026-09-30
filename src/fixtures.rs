// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Fixture validation and rendering helpers (FR-003).

use std::fs;
use std::path::Path;

use serde::Serialize;
use serde_json::{Value, json};

use crate::catalog::{adapt_quoin_advice, adapt_quoin_catalog};
use crate::contracts::{
    VerificationError, VerificationErrorCode, parse_bounded_json, validate_contract,
    validate_schema_definition,
};
use crate::evidence::assess_evidence;
use crate::planner::build_verification_plan;
use crate::shared_reference::{
    ArtifactExpectation, ReferenceExpectation, ReferenceSupport, SharedReferenceReader,
    ValidatedSharedReference,
};
use crate::value::{arrays_field, field, string_field};

/// Validation count for one discovered JSON fixture.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FixtureSummary {
    /// Direct-child fixture filename.
    pub name: String,
    /// Public contract instances validated while handling the fixture.
    pub contracts: usize,
}

/// Reads a JSON file through the bounded public ingestion path.
///
/// # Errors
/// Returns an I/O, JSON, or resource-limit error.
pub fn read_bounded_json(path: &Path) -> Result<Value, VerificationError> {
    let bytes = fs::read(path).map_err(|error| io_error(path, &error))?;
    parse_bounded_json(&bytes)
}

/// Discovers and validates every direct-child JSON fixture.
///
/// # Errors
/// Returns an I/O, unhandled-fixture, schema, or semantic validation error.
pub fn validate_all_fixtures(directory: &Path) -> Result<Vec<FixtureSummary>, VerificationError> {
    let mut names = fs::read_dir(directory)
        .map_err(|error| io_error(directory, &error))?
        .map(|entry| {
            entry.map_err(|error| {
                VerificationError::new(VerificationErrorCode::IoFailure, error.to_string())
            })
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter_map(|entry| {
            let path = entry.path();
            (path.extension().and_then(|value| value.to_str()) == Some("json"))
                .then(|| entry.file_name().to_string_lossy().into_owned())
        })
        .collect::<Vec<_>>();
    names.sort();
    let mut summaries = Vec::with_capacity(names.len());
    for name in names {
        let value = read_bounded_json(&directory.join(&name))?;
        let contracts = match name.as_str() {
            "portfolio.json" => validate_portfolio(&value)?,
            "quoin-advice-export.json" => validate_advice(&value)?,
            "quoin-catalog-export.json" => validate_catalog(&value)?,
            "results.json" => validate_results(&value)?,
            _ => {
                return Err(VerificationError::new(
                    VerificationErrorCode::UnhandledFixture,
                    "fixture JSON has no schema handler",
                )
                .with_context("fixture", name));
            }
        };
        summaries.push(FixtureSummary { name, contracts });
    }
    Ok(summaries)
}

/// Renders the retained evidence packet from its source fixtures.
///
/// # Errors
/// Returns a planning or evidence-assessment error.
pub fn render_fixture_packet(
    portfolio: &Value,
    results: &Value,
) -> Result<Value, VerificationError> {
    let plan_request = field(portfolio, "planRequest", "portfolio fixture")?;
    let references = validate_fixture_planning_references(plan_request)?;
    let evidence_references = validate_fixture_result_references(results)?;
    Ok(json!({
        "schemaVersion": "quire-verification/e02-evidence-packet/v1",
        "claimLimit": "Fixture planning and intake only; no represented external verification or fault tool executed.",
        "plan": build_verification_plan(plan_request, &references)?,
        "assessment": assess_evidence(results, &evidence_references)?,
    }))
}

/// Validates every oracle and observation envelope embedded in a results fixture.
///
/// Fixture artifact bytes are their declared identity bytes, matching the other
/// synthetic fixture references in this module.
///
/// # Errors
/// Returns a strict shared-reference refusal for any unsupported or mismatched envelope.
pub fn validate_fixture_result_references(
    results: &Value,
) -> Result<Vec<ValidatedSharedReference>, VerificationError> {
    let mut envelopes = Vec::new();
    for result in arrays_field(results, "results", "results fixture")? {
        for contribution in arrays_field(result, "oracleContributions", "result")? {
            envelopes.push(field(contribution, "oracleRef", "oracle contribution")?);
            envelopes.push(field(
                contribution,
                "observationRef",
                "oracle contribution",
            )?);
        }
    }
    validate_fixture_envelopes(&envelopes)
}

fn validate_portfolio(value: &Value) -> Result<usize, VerificationError> {
    validate_schema_definition("PortfolioFixture", value)?;
    let request = field(value, "planRequest", "portfolio fixture")?;
    validate_contract(field(request, "policy", "plan request")?.clone())?;
    let capabilities = arrays_field(request, "capabilities", "plan request")?;
    for capability in capabilities {
        validate_contract(capability.clone())?;
    }
    for property in arrays_field(request, "properties", "plan request")? {
        validate_schema_definition("PropertyInventoryItem", property)?;
    }
    let references = validate_fixture_planning_references(request)?;
    validate_contract(build_verification_plan(request, &references)?)?;
    Ok(2 + capabilities.len())
}

fn validate_fixture_planning_references(
    request: &Value,
) -> Result<Vec<ValidatedSharedReference>, VerificationError> {
    let mut envelopes = Vec::new();
    for property in arrays_field(request, "properties", "plan request")? {
        let input = field(property, "input", "property")?;
        match string_field(input, "kind", "property input")? {
            "state" => envelopes.push(field(input, "snapshotRef", "state input")?),
            "trace" => envelopes.push(field(input, "traceRef", "trace input")?),
            "fault" => {
                envelopes.push(field(input, "faultModelRef", "fault input")?);
                envelopes.push(field(input, "mitigationOracleRef", "fault input")?);
            }
            "manual" => {
                envelopes.push(field(input, "reviewProcedureRef", "manual input")?);
            }
            "relational" => {}
            kind => {
                return Err(VerificationError::new(
                    VerificationErrorCode::SchemaViolation,
                    "unknown property input kind",
                )
                .with_context("input_kind", kind));
            }
        }
    }
    validate_fixture_envelopes(&envelopes)
}

fn validate_fixture_envelopes(
    envelopes: &[&Value],
) -> Result<Vec<ValidatedSharedReference>, VerificationError> {
    let mut support = ReferenceSupport::default();
    let mut expectations = Vec::with_capacity(envelopes.len());
    for envelope in envelopes {
        let value = field(envelope, "value", "shared artifact envelope")?;
        let wire = field(value, "wire", "shared artifact value")?;
        let wire_identity = string_field(wire, "identity", "artifact wire")?.to_owned();
        let wire_version = string_field(wire, "version", "artifact wire")?.to_owned();
        support
            .wire_versions
            .entry(wire_identity.clone())
            .or_default()
            .insert(wire_version.clone());
        let revision = field(value, "revision", "shared artifact value")?;
        let identity = string_field(value, "identity", "shared artifact value")?.to_owned();
        expectations.push(ArtifactExpectation {
            ref_version: string_field(value, "refVersion", "shared artifact value")?.to_owned(),
            kind: string_field(value, "kind", "shared artifact value")?.to_owned(),
            authority: string_field(value, "authority", "shared artifact value")?.to_owned(),
            identity: identity.clone(),
            revision_namespace: string_field(revision, "namespace", "artifact revision")?
                .to_owned(),
            revision_value: string_field(revision, "value", "artifact revision")?.to_owned(),
            digest: string_field(value, "digest", "shared artifact value")?.to_owned(),
            wire_identity,
            wire_version,
            bytes: Some(identity.into_bytes()),
        });
    }
    let mut reader = SharedReferenceReader::new(support);
    envelopes
        .iter()
        .zip(expectations)
        .map(|(envelope, expectation)| {
            let bytes = serde_json::to_vec(envelope).map_err(|error| {
                VerificationError::new(VerificationErrorCode::InvalidJson, error.to_string())
            })?;
            reader.read(&bytes, &ReferenceExpectation::Artifact(expectation))
        })
        .collect()
}

fn validate_results(value: &Value) -> Result<usize, VerificationError> {
    validate_schema_definition("ResultsFixture", value)?;
    for property in arrays_field(value, "inventory", "results fixture")? {
        validate_schema_definition("EvidenceObligation", property)?;
    }
    let results = arrays_field(value, "results", "results fixture")?;
    for result in results {
        validate_contract(result.clone())?;
    }
    Ok(results.len())
}

fn validate_catalog(value: &Value) -> Result<usize, VerificationError> {
    validate_schema_definition("QuoinCatalogFixture", value)?;
    let snapshot = adapt_quoin_catalog(value)?;
    let techniques = arrays_field(&snapshot, "techniques", "adapted catalog")?;
    for technique in techniques {
        validate_contract(technique.clone())?;
    }
    Ok(techniques.len())
}

fn validate_advice(value: &Value) -> Result<usize, VerificationError> {
    validate_schema_definition("QuoinAdviceFixture", value)?;
    let cases = arrays_field(value, "cases", "advice fixture")?;
    let advice = arrays_field(value, "advice", "advice fixture")?;
    if cases.len() != advice.len() {
        return Err(VerificationError::new(
            VerificationErrorCode::InvalidInput,
            "Quoin advice fixture cases and advice must have equal lengths",
        ));
    }
    for item in advice {
        adapt_quoin_advice(item)?;
    }
    Ok(0)
}

fn io_error(path: &Path, error: &std::io::Error) -> VerificationError {
    VerificationError::new(VerificationErrorCode::IoFailure, error.to_string())
        .with_context("path", path.display().to_string())
}
