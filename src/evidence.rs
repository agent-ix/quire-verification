// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Exact-scope evidence matching without sufficiency decisions (FR-006, FR-007, FR-017).

use std::collections::{BTreeMap, BTreeSet};

use chrono::{DateTime, Datelike, FixedOffset, TimeDelta};
use serde_json::{Value, json};

use crate::contracts::{
    VerificationError, VerificationErrorCode, jcs_canonicalize, jcs_equal,
    validate_schema_definition,
};
use crate::shared_reference::ValidatedSharedReference;
use crate::value::{arrays_field, field, string_field};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ContributionId {
    result_id: String,
    contribution_id: String,
}

impl ContributionId {
    fn new(result_id: &str, contribution_id: &str) -> Self {
        Self {
            result_id: result_id.to_owned(),
            contribution_id: contribution_id.to_owned(),
        }
    }
}

type DependenceGraph = BTreeMap<ContributionId, Vec<(ContributionId, &'static str)>>;

#[derive(Debug)]
struct ContributionRecord<'a> {
    oracle_key: Vec<u8>,
    observation_key: Vec<u8>,
    relation: &'a str,
    dependence_group: Option<&'a str>,
    derived_from: Option<ContributionId>,
}

/// Assesses result compatibility for every inventory obligation.
///
/// # Errors
/// Returns a schema, timestamp, duplicate-identifier, strict-reference,
/// invalid-lineage, or canonicalization error.
pub fn assess_evidence(
    packet: &Value,
    validated_references: &[ValidatedSharedReference],
) -> Result<Value, VerificationError> {
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
    let contributions = validate_contributions(results, validated_references)?;

    let mut obligations = Vec::with_capacity(inventory.len());
    let mut selected_results = BTreeSet::new();
    for property in inventory {
        let (obligation, selected_result) = assess_obligation(property, results, as_of)?;
        if let Some(result_id) = selected_result {
            selected_results.insert(result_id);
        }
        obligations.push(obligation);
    }
    sort_canonical(&mut obligations)?;
    let oracle_dependence_findings = dependence_findings(&contributions, &selected_results)?;
    Ok(json!({
        "asOf": as_of_text,
        "denominator": inventory.len(),
        "obligations": obligations,
        "oracleDependenceFindings": oracle_dependence_findings,
        "evidenceOwner": "Quoin",
        "sufficiencyDecision": "not-made",
        "statusSemantics": "compatibility summary only; gaps enumerate every applicable independent result finding",
    }))
}

fn assess_obligation(
    property: &Value,
    results: &[Value],
    as_of: DateTime<FixedOffset>,
) -> Result<(Value, Option<String>), VerificationError> {
    validate_schema_definition("EvidenceObligation", property)?;
    let property_identity = artifact_identity(field(property, "propertyRef", "property")?)?;
    let candidates = results
        .iter()
        .filter(|result| {
            result
                .pointer("/propertyRef/value/identity")
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
        return Ok((
            json!({
                "propertyId": string_field(property, "propertyId", "property")?,
                "status": "missing", "resultId": null, "gaps": gaps,
            }),
            None,
        ));
    }
    if exact.len() > 1 {
        let ids = exact
            .iter()
            .map(|result| {
                string_field(result, "resultId", "result").map(|id| Value::String(id.to_owned()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        return Ok((
            json!({
                "propertyId": string_field(property, "propertyId", "property")?,
                "status": "ambiguous", "resultId": null,
                "candidateResultIds": ids,
                "gaps": [{"kind": "ambiguous-results", "resultIds": ids}],
            }),
            None,
        ));
    }
    let result = exact[0];
    let result_id = string_field(result, "resultId", "result")?.to_owned();
    let mut findings = findings_for(result)?;
    let status = if is_stale(property, result, as_of)? {
        findings.insert(0, json!({"kind": "freshness-expired"}));
        "stale"
    } else {
        classify(result)?
    };
    Ok((
        result_view(property, result, status, &findings)?,
        Some(result_id),
    ))
}

fn validate_contributions<'a>(
    results: &'a [Value],
    validated_references: &[ValidatedSharedReference],
) -> Result<BTreeMap<ContributionId, ContributionRecord<'a>>, VerificationError> {
    let reference_index = validated_references
        .iter()
        .filter_map(ValidatedSharedReference::reference_key)
        .collect::<BTreeMap<_, _>>();
    let mut records = BTreeMap::new();
    for result in results {
        let result_id = string_field(result, "resultId", "result")?;
        for contribution in arrays_field(result, "oracleContributions", "result")? {
            let contribution_id =
                string_field(contribution, "contributionId", "oracle contribution")?;
            let key = ContributionId::new(result_id, contribution_id);
            if records.contains_key(&key) {
                return Err(VerificationError::new(
                    VerificationErrorCode::DuplicateOracleContributionId,
                    "duplicate oracle contribution identifier within a result",
                )
                .with_context("result_id", result_id)
                .with_context("contribution_id", contribution_id));
            }
            let oracle_key = validate_contribution_reference(
                field(contribution, "oracleRef", "oracle contribution")?,
                "oracle",
                &reference_index,
            )?;
            let observation_key = validate_contribution_reference(
                field(contribution, "observationRef", "oracle contribution")?,
                "observation",
                &reference_index,
            )?;
            let relation = string_field(contribution, "relation", "oracle contribution")?;
            let dependence_group = contribution
                .get("dependenceGroupId")
                .and_then(Value::as_str);
            let derived_from = contribution
                .get("derivedFrom")
                .map(|source| {
                    Ok(ContributionId::new(
                        string_field(source, "resultId", "derived source")?,
                        string_field(source, "contributionId", "derived source")?,
                    ))
                })
                .transpose()?;
            records.insert(
                key,
                ContributionRecord {
                    oracle_key,
                    observation_key,
                    relation,
                    dependence_group,
                    derived_from,
                },
            );
        }
    }
    validate_lineage(&records)?;
    Ok(records)
}

fn validate_contribution_reference(
    reference: &Value,
    expected_role: &str,
    validated_references: &BTreeMap<&[u8], &str>,
) -> Result<Vec<u8>, VerificationError> {
    let canonical = jcs_canonicalize(reference)?;
    match validated_references.get(canonical.as_slice()) {
        Some(role) if *role == expected_role => Ok(canonical),
        Some(role) => Err(VerificationError::new(
            VerificationErrorCode::IdentityMismatch,
            "validated shared reference has the wrong contribution role",
        )
        .with_context("expected_role", expected_role)
        .with_context("actual_role", *role)),
        None => Err(VerificationError::new(
            VerificationErrorCode::IdentityMismatch,
            "contribution reference has no exact strict-reader output",
        )
        .with_context("expected_role", expected_role)),
    }
}

fn validate_lineage(
    records: &BTreeMap<ContributionId, ContributionRecord<'_>>,
) -> Result<(), VerificationError> {
    let derived = records
        .iter()
        .filter_map(|(id, record)| {
            record
                .derived_from
                .as_ref()
                .map(|source| (id.clone(), source.clone()))
        })
        .collect::<BTreeMap<_, _>>();
    for (id, source) in &derived {
        if id == source || !records.contains_key(source) {
            return Err(VerificationError::new(
                VerificationErrorCode::InvalidOracleLineage,
                "derived oracle contribution has an absent or self source",
            )
            .with_context("result_id", id.result_id.clone())
            .with_context("contribution_id", id.contribution_id.clone()));
        }
    }

    let mut complete = BTreeSet::new();
    for start in derived.keys() {
        if complete.contains(start) {
            continue;
        }
        let mut path = BTreeSet::new();
        let mut traversed = Vec::new();
        let mut current = start.clone();
        loop {
            if complete.contains(&current) {
                break;
            }
            if !path.insert(current.clone()) {
                return Err(VerificationError::new(
                    VerificationErrorCode::InvalidOracleLineage,
                    "derived oracle contribution graph contains a cycle",
                )
                .with_context("result_id", current.result_id)
                .with_context("contribution_id", current.contribution_id));
            }
            traversed.push(current.clone());
            let Some(next) = derived.get(&current) else {
                break;
            };
            current.clone_from(next);
        }
        complete.extend(traversed);
    }
    Ok(())
}

fn dependence_findings(
    records: &BTreeMap<ContributionId, ContributionRecord<'_>>,
    selected_results: &BTreeSet<String>,
) -> Result<Vec<Value>, VerificationError> {
    let selected = records
        .iter()
        .filter(|(id, _)| selected_results.contains(&id.result_id))
        .collect::<BTreeMap<_, _>>();
    let (mut adjacency, by_group) = build_base_dependence_graph(&selected);
    let mut findings = BTreeMap::<Vec<u8>, Value>::new();
    add_declared_findings(&selected, &by_group, &mut adjacency, &mut findings)?;
    add_component_findings(&adjacency, &mut findings)?;
    let mut values = findings.into_values().collect::<Vec<_>>();
    sort_canonical(&mut values)?;
    Ok(values)
}

fn build_base_dependence_graph(
    selected: &BTreeMap<&ContributionId, &ContributionRecord<'_>>,
) -> (DependenceGraph, BTreeMap<String, Vec<ContributionId>>) {
    let mut adjacency = selected
        .keys()
        .map(|id| ((*id).clone(), Vec::<(ContributionId, &'static str)>::new()))
        .collect::<BTreeMap<_, _>>();
    let mut by_oracle = BTreeMap::<Vec<u8>, Vec<ContributionId>>::new();
    let mut by_observation = BTreeMap::<Vec<u8>, Vec<ContributionId>>::new();
    let mut by_group = BTreeMap::<String, Vec<ContributionId>>::new();
    for (id, record) in selected {
        by_oracle
            .entry(record.oracle_key.clone())
            .or_default()
            .push((*id).clone());
        by_observation
            .entry(record.observation_key.clone())
            .or_default()
            .push((*id).clone());
        if let Some(group) = record.dependence_group {
            by_group
                .entry(group.to_owned())
                .or_default()
                .push((*id).clone());
        }
    }
    connect_groups(by_oracle.values(), "oracle", &mut adjacency);
    connect_groups(by_observation.values(), "observation", &mut adjacency);
    connect_groups(by_group.values(), "declared-group", &mut adjacency);
    (adjacency, by_group)
}

fn add_declared_findings(
    selected: &BTreeMap<&ContributionId, &ContributionRecord<'_>>,
    by_group: &BTreeMap<String, Vec<ContributionId>>,
    adjacency: &mut DependenceGraph,
    findings: &mut BTreeMap<Vec<u8>, Value>,
) -> Result<(), VerificationError> {
    for (id, record) in selected {
        match record.relation {
            "unknown" => insert_finding(findings, unknown_finding(id, "declared-unknown"))?,
            "shared" => {
                let group = record.dependence_group.ok_or_else(|| {
                    VerificationError::new(
                        VerificationErrorCode::SchemaViolation,
                        "shared contribution has no dependence group",
                    )
                })?;
                if by_group
                    .get(group)
                    .is_some_and(|members| members.len() == 1)
                {
                    insert_finding(findings, unknown_finding(id, "shared-group-unresolved"))?;
                }
            }
            "derived" => {
                let source = record.derived_from.as_ref().ok_or_else(|| {
                    VerificationError::new(
                        VerificationErrorCode::SchemaViolation,
                        "derived contribution has no source",
                    )
                })?;
                if adjacency.contains_key(source) {
                    add_edge(id, source, "derived", adjacency);
                } else {
                    insert_finding(findings, unknown_finding(id, "source-not-assessed"))?;
                }
            }
            "independent" => {}
            _ => {
                return Err(VerificationError::new(
                    VerificationErrorCode::SchemaViolation,
                    "unknown oracle contribution relation",
                ));
            }
        }
    }
    Ok(())
}

fn add_component_findings(
    adjacency: &DependenceGraph,
    findings: &mut BTreeMap<Vec<u8>, Value>,
) -> Result<(), VerificationError> {
    let mut visited = BTreeSet::new();
    for start in adjacency.keys() {
        if !visited.insert(start.clone()) {
            continue;
        }
        let mut stack = vec![start.clone()];
        let mut members = BTreeSet::new();
        let mut bases = BTreeSet::new();
        while let Some(current) = stack.pop() {
            members.insert(current.clone());
            if let Some(edges) = adjacency.get(&current) {
                for (next, basis) in edges {
                    bases.insert(*basis);
                    if visited.insert(next.clone()) {
                        stack.push(next.clone());
                    }
                }
            }
        }
        if members.len() >= 2 {
            let finding = json!({
                "kind": "correlated-oracle",
                "members": members.into_iter().map(|id| member_value(&id)).collect::<Vec<_>>(),
                "bases": bases.into_iter().collect::<Vec<_>>(),
            });
            insert_finding(findings, finding)?;
        }
    }
    Ok(())
}

fn connect_groups<'a, I>(groups: I, basis: &'static str, adjacency: &mut DependenceGraph)
where
    I: Iterator<Item = &'a Vec<ContributionId>>,
{
    for members in groups {
        if let Some(first) = members.first() {
            for member in &members[1..] {
                add_edge(first, member, basis, adjacency);
            }
        }
    }
}

fn add_edge(
    left: &ContributionId,
    right: &ContributionId,
    basis: &'static str,
    adjacency: &mut DependenceGraph,
) {
    if left == right {
        return;
    }
    if let Some(edges) = adjacency.get_mut(left) {
        edges.push((right.clone(), basis));
    }
    if let Some(edges) = adjacency.get_mut(right) {
        edges.push((left.clone(), basis));
    }
}

fn unknown_finding(id: &ContributionId, reason: &str) -> Value {
    json!({
        "kind": "oracle-dependence-unknown",
        "member": member_value(id),
        "reason": reason,
    })
}

fn member_value(id: &ContributionId) -> Value {
    json!({"resultId": id.result_id, "contributionId": id.contribution_id})
}

fn insert_finding(
    findings: &mut BTreeMap<Vec<u8>, Value>,
    finding: Value,
) -> Result<(), VerificationError> {
    findings.insert(jcs_canonicalize(&finding)?, finding);
    Ok(())
}

fn artifact_identity(reference: &Value) -> Result<&str, VerificationError> {
    string_field(
        field(reference, "value", "artifact envelope")?,
        "identity",
        "artifact envelope.value",
    )
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
    let mut oracle_contributions = arrays_field(result, "oracleContributions", "result")?.to_vec();
    oracle_contributions.sort_by(|left, right| {
        left.get("contributionId")
            .and_then(Value::as_str)
            .cmp(&right.get("contributionId").and_then(Value::as_str))
    });
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
        "oracleContributions": oracle_contributions,
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
