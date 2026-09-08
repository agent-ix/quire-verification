// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Fixture validation, rendering, and external catalog baseline helpers (FR-003, IT-001).

use std::fs;
use std::path::Path;

use serde::Serialize;
use serde_json::{Value, json};

use crate::catalog::{adapt_quoin_advice, adapt_quoin_catalog, by_technique_id};
use crate::contracts::{
    VerificationError, VerificationErrorCode, jcs_equal, parse_bounded_json, validate_contract,
    validate_schema_definition,
};
use crate::evidence::assess_evidence;
use crate::planner::build_verification_plan;
use crate::value::{arrays_field, field};

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
    Ok(json!({
        "schemaVersion": "quire-verification/e02-evidence-packet/v1",
        "claimLimit": "Fixture planning and intake only; no represented external verification or fault tool executed.",
        "plan": build_verification_plan(plan_request)?,
        "assessment": assess_evidence(results)?,
    }))
}

/// Compares real Quoin catalog output with the retained fixture by technique identifier.
///
/// # Errors
/// Returns an input, duplicate-identifier, canonicalization, or parity error.
pub fn check_quoin_baseline(actual: &Value, fixture: &Value) -> Result<usize, VerificationError> {
    let expected = field(fixture, "catalog", "catalog fixture")?;
    let actual_methods = arrays_field(actual, "methods", "actual catalog")?;
    let expected_methods = arrays_field(expected, "methods", "expected catalog")?;
    let actual_by_id = by_technique_id(actual_methods, "id", "actual method")?;
    let expected_by_id = by_technique_id(expected_methods, "id", "expected method")?;
    if actual_by_id.len() != expected_by_id.len() {
        return Err(VerificationError::new(
            VerificationErrorCode::CatalogParityMismatch,
            "Quoin baseline method count differs",
        ));
    }
    for (id, expected_method) in expected_by_id {
        let Some(actual_method) = actual_by_id.get(&id) else {
            return Err(VerificationError::new(
                VerificationErrorCode::CatalogParityMismatch,
                "Quoin baseline is missing a technique",
            )
            .with_context("technique_id", id.as_str()));
        };
        if !jcs_equal(*actual_method, expected_method)? {
            return Err(VerificationError::new(
                VerificationErrorCode::CatalogParityMismatch,
                "Quoin baseline method differs",
            )
            .with_context("technique_id", id.as_str()));
        }
    }
    for field_name in ["duplicates", "unreadable"] {
        if !jcs_equal(
            field(actual, field_name, "actual catalog")?,
            field(expected, field_name, "expected catalog")?,
        )? {
            return Err(VerificationError::new(
                VerificationErrorCode::CatalogParityMismatch,
                "Quoin baseline diagnostics differ",
            )
            .with_context("field", field_name));
        }
    }
    Ok(actual_methods.len())
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
    validate_contract(build_verification_plan(request)?)?;
    Ok(2 + capabilities.len())
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
