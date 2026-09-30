// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Versioned adapters for Quoin-owned catalog and advice JSON (FR-004).

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value, json};

use crate::contracts::{
    VerificationError, VerificationErrorCode, contract_version, jcs_canonicalize,
    validate_contract, validate_schema_definition,
};
use crate::value::{arrays_field, field, object, optional_array, string, string_field};

/// A technique identifier used for deterministic catalog joins.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct TechniqueId(String);

impl TechniqueId {
    /// Returns the wire representation.
    #[must_use]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::str::FromStr for TechniqueId {
    type Err = VerificationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(VerificationError::new(
                VerificationErrorCode::InvalidInput,
                "technique identifier must not be empty",
            ));
        }
        Ok(Self(value.to_owned()))
    }
}

/// Converts one schema-validated Quoin catalog export into portable technique contracts.
///
/// # Errors
/// Returns a schema, duplicate-identifier, or canonicalization error.
pub fn adapt_quoin_catalog(snapshot: &Value) -> Result<Value, VerificationError> {
    validate_schema_definition("QuoinCatalogFixture", snapshot)?;
    let source = field(snapshot, "source", "snapshot")?;
    let source_ref = field(source, "sourceRef", "snapshot.source")?.clone();
    let catalog = field(snapshot, "catalog", "snapshot")?;
    let methods = arrays_field(catalog, "methods", "snapshot.catalog")?;
    let mut techniques = Vec::with_capacity(methods.len());
    let mut identifiers = BTreeSet::new();

    for (index, method) in methods.iter().enumerate() {
        let label = format!("snapshot.catalog.methods[{index}]");
        object(method, &label)?;
        let identifier = string_field(method, "id", &label)?.parse::<TechniqueId>()?;
        if !identifiers.insert(identifier.clone()) {
            return Err(VerificationError::new(
                VerificationErrorCode::DuplicateTechniqueId,
                "post-merge Quoin snapshot contains duplicate active method IDs",
            )
            .with_context("technique_id", identifier.as_str()));
        }
        let applicability = normalize_rules(object(method, &label)?.get("applicability"), &label)?;
        let tooling = optional_array(method, "tooling", &label)?
            .iter()
            .map(stringify_scalar)
            .collect::<Vec<_>>();
        let definition = json!({
            "contractVersion": contract_version(),
            "kind": "TechniqueDefinition",
            "techniqueId": identifier.as_str(),
            "name": string_field(method, "name", &label)?,
            "class": string_field(method, "class", &label)?,
            "purpose": string_field(method, "definition", &label)?,
            "evidenceKind": object(method, &label)?.get("evidenceKind").cloned().unwrap_or(Value::Null),
            "applicability": applicability,
            "possibleClaims": [],
            "legacyToolingHints": tooling,
            "sourceModule": string_field(method, "moduleName", &label)?,
            "sourceRef": source_ref,
        });
        validate_contract(definition.clone())?;
        techniques.push((identifier, definition));
    }
    techniques.sort_by(|(left, _), (right, _)| left.cmp(right));
    let techniques = techniques
        .into_iter()
        .map(|(_, definition)| definition)
        .collect::<Vec<_>>();

    let mut diagnostics = Vec::new();
    for duplicate in optional_array(catalog, "duplicates", "snapshot.catalog")? {
        diagnostics.push(json!({
            "kind": "conflicting-definition",
            "techniqueId": scalar_field(duplicate, "id", "duplicate")?,
            "modules": optional_array(duplicate, "modules", "duplicate")?.iter().map(stringify_scalar).collect::<Vec<_>>(),
            "effect": "active-first-wins-definition-preserved; snapshot-unqualified",
        }));
    }
    for unreadable in optional_array(catalog, "unreadable", "snapshot.catalog")? {
        diagnostics.push(json!({
            "kind": "unreadable-source",
            "source": scalar_field(unreadable, "moduleRoot", "unreadable")?,
            "reason": scalar_field(unreadable, "reason", "unreadable")?,
            "effect": "available-definitions-preserved; snapshot-unqualified",
        }));
    }
    sort_by_jcs(&mut diagnostics)?;
    Ok(json!({
        "contractVersion": contract_version(),
        "kind": "TechniqueRegistrySnapshot",
        "sourceRef": source_ref,
        "qualified": diagnostics.is_empty(),
        "techniques": techniques,
        "diagnostics": diagnostics,
    }))
}

/// Converts one Quoin advice result into a portable, task-scoped selection record.
///
/// # Errors
/// Returns `invalid_input` when required advice fields have the wrong shape.
pub fn adapt_quoin_advice(advice: &Value) -> Result<Value, VerificationError> {
    let obligation = string_field(advice, "obligation", "advice")?;
    let recommendations = arrays_field(advice, "recommended", "advice")?;
    let mut selections = Vec::with_capacity(recommendations.len());
    for recommendation in recommendations {
        let method = string_field(recommendation, "method", "recommendation")?;
        let mut reasons = Vec::new();
        for reason in optional_array(recommendation, "reasons", "recommendation")? {
            reasons.push(json!({
                "rule": scalar_field(reason, "rule", "reason")?,
                "value": scalar_field(reason, "value", "reason")?,
            }));
        }
        selections.push(json!({
            "propertyId": obligation,
            "techniqueId": method,
            "applicability": "applicable",
            "reasons": reasons,
        }));
    }
    let input = object(advice, "advice")?;
    let residual = if selections.is_empty()
        && input.get("inconclusive").and_then(Value::as_bool) == Some(true)
    {
        Value::String("unknown".to_owned())
    } else {
        Value::Null
    };
    Ok(json!({
        "propertyId": obligation,
        "selections": selections,
        "residualApplicability": residual,
        "authored": input.get("authored").cloned().unwrap_or(Value::Null),
        "mismatch": input.get("mismatch").and_then(Value::as_bool) == Some(true),
        "uncatalogued": input.get("uncatalogued").and_then(Value::as_bool) == Some(true),
    }))
}

fn normalize_rules(value: Option<&Value>, label: &str) -> Result<Value, VerificationError> {
    let Some(Value::Object(fields)) = value else {
        return Ok(json!({}));
    };
    let normalized = fields
        .iter()
        .map(|(axis, values)| {
            let items = values
                .as_array()
                .ok_or_else(|| {
                    VerificationError::new(
                        VerificationErrorCode::InvalidInput,
                        "catalog applicability axis must be an array",
                    )
                    .with_context("field", format!("{label}.applicability.{axis}"))
                })?
                .iter()
                .map(stringify_scalar)
                .map(Value::String)
                .collect();
            Ok((axis.clone(), Value::Array(items)))
        })
        .collect::<Result<Map<_, _>, VerificationError>>()?;
    Ok(Value::Object(normalized))
}

fn sort_by_jcs(values: &mut Vec<Value>) -> Result<(), VerificationError> {
    let mut keyed = values
        .drain(..)
        .map(|value| jcs_canonicalize(&value).map(|key| (key, value)))
        .collect::<Result<Vec<_>, _>>()?;
    keyed.sort_by(|(left, _), (right, _)| left.cmp(right));
    values.extend(keyed.into_iter().map(|(_, value)| value));
    Ok(())
}

fn stringify_scalar(value: &Value) -> String {
    match value {
        Value::String(item) => item.clone(),
        other => other.to_string(),
    }
}

fn scalar_field(value: &Value, name: &str, label: &str) -> Result<String, VerificationError> {
    Ok(stringify_scalar(field(value, name, label)?))
}

/// Indexes values by unique technique identifier for order-independent parity checks.
///
/// # Errors
/// Returns an input or duplicate-identifier error.
pub(crate) fn by_technique_id<'a>(
    values: &'a [Value],
    id_field: &str,
    label: &str,
) -> Result<BTreeMap<TechniqueId, &'a Value>, VerificationError> {
    let mut indexed = BTreeMap::new();
    for value in values {
        let id = string(
            field(value, id_field, label)?,
            &format!("{label}.{id_field}"),
        )?
        .parse::<TechniqueId>()?;
        if indexed.insert(id.clone(), value).is_some() {
            return Err(VerificationError::new(
                VerificationErrorCode::DuplicateTechniqueId,
                format!("duplicate {label} ID"),
            )
            .with_context("technique_id", id.as_str()));
        }
    }
    Ok(indexed)
}
