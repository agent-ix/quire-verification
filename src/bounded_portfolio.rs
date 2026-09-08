// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Bounded local portfolio execution and self-reporting (FR-009).

use std::time::Instant;

use serde_json::{Map, Value, json};

use crate::catalog::{adapt_quoin_catalog, by_technique_id};
use crate::contracts::{
    VerificationError, VerificationErrorCode, contract_version, jcs_equal, jcs_sha256,
    validate_schema_definition,
};
use crate::evidence::assess_evidence;
use crate::planner::build_verification_plan;
use crate::value::{arrays_field, field, string_field};

const PROFILE: &str = "quire-verification/local-catalog-portfolio-v1";
const PLAN_ID: &str = "PLAN-LOCAL-CATALOG-PORTFOLIO";
const PRODUCER: &str = "quire-verification-local-portfolio";
const ADAPTER: &str = "native-rust";
const SELECTIONS: [(&str, &str, &str); 3] = [
    ("LOCAL-CATALOG-PARITY", "unit-testing", "required"),
    (
        "LOCAL-CATALOG-PERMUTATION",
        "metamorphic-testing",
        "recommended",
    ),
    ("LOCAL-HUMAN-REVIEW", "inspection", "required"),
];

/// Runs the local bounded portfolio at an explicit source revision and timestamp.
///
/// # Errors
/// Returns an input, schema, planning, or result-validation error. Technique
/// execution failures are retained in the returned report.
pub fn run_bounded_portfolio(
    source_revision: &str,
    produced_at: &str,
    catalog: &Value,
) -> Result<Value, VerificationError> {
    run_bounded_portfolio_with_adapter(source_revision, produced_at, catalog, adapt_quoin_catalog)
}

/// Runs the portfolio with an injected adapter for adverse-path verification.
///
/// # Errors
/// Returns an input, schema, planning, or result-validation error. Adapter
/// failures during a technique are retained as failed result records.
pub fn run_bounded_portfolio_with_adapter<F>(
    source_revision: &str,
    produced_at: &str,
    catalog: &Value,
    adapter: F,
) -> Result<Value, VerificationError>
where
    F: Fn(&Value) -> Result<Value, VerificationError>,
{
    let source_digest = validate_portfolio_inputs(source_revision, produced_at, catalog)?;
    let (properties, run_ref) = build_properties(source_revision, &source_digest)?;
    let plan = build_portfolio_plan(source_revision, produced_at, &source_digest, &properties)?;
    let parity = observe(|| check_catalog_parity(catalog, &adapter));
    let permutation = observe(|| check_permutation_invariance(catalog, &adapter));
    let results = build_results(
        &properties,
        produced_at,
        source_revision,
        &run_ref,
        &parity,
        &permutation,
    )?;
    for result in &results {
        validate_schema_definition("TechniqueResult", result)?;
    }
    let assessment = assess_evidence(
        &json!({"inventory": properties, "results": results, "asOf": produced_at}),
    )?;
    Ok(json!({
        "schemaVersion": "quire-verification/local-portfolio-report/v1",
        "claimLimit": "Actual local adapter checks only; no external method, parser, fault, or human review is claimed.",
        "sourceRevision": source_revision, "producedAt": produced_at,
        "setupEffort": {"kind": "one-time", "measuredSeparatelyIn": "docs/evaluation-report.md"},
        "runEffort": {"unitMilliseconds": parity.elapsed_milliseconds, "metamorphicMilliseconds": permutation.elapsed_milliseconds, "manual": "pending verification-owner"},
        "plan": plan, "results": results, "assessment": assessment,
    }))
}

fn validate_portfolio_inputs(
    source_revision: &str,
    produced_at: &str,
    catalog: &Value,
) -> Result<String, VerificationError> {
    if source_revision.is_empty() {
        return Err(VerificationError::new(
            VerificationErrorCode::InvalidInput,
            "source revision must not be empty",
        ));
    }
    chrono::DateTime::parse_from_rfc3339(produced_at).map_err(|error| {
        VerificationError::new(VerificationErrorCode::InvalidTimestamp, error.to_string())
            .with_context("field", "producedAt")
    })?;
    validate_schema_definition("QuoinCatalogFixture", catalog)?;
    let source = field(catalog, "source", "catalog snapshot")?;
    jcs_sha256(&json!({
        "sourceRevision": source_revision,
        "catalogRevision": string_field(source, "quoinRevision", "catalog source")?,
        "moduleRevision": string_field(source, "moduleRevision", "catalog source")?,
        "catalog": field(catalog, "catalog", "catalog snapshot")?,
    }))
}

fn artifact_ref(source_digest: &str, identity: &str, revision: &str) -> Value {
    json!({
        "authority": "github.com/agent-ix/quire-verification", "identity": identity,
        "revision": revision, "digest": source_digest, "profile": PROFILE,
    })
}

fn build_properties(
    source_revision: &str,
    source_digest: &str,
) -> Result<(Vec<Value>, Value), VerificationError> {
    let subject_ref = artifact_ref(
        source_digest,
        "repository:quire-verification",
        source_revision,
    );
    let run_ref = artifact_ref(
        source_digest,
        "run:local-catalog-portfolio",
        source_revision,
    );
    let common = |property_id: &str, property_ref: &str, binding_refs: Vec<Value>, input: Value| {
        json!({
            "propertyId": property_id, "propertyRef": artifact_ref(source_digest, property_ref, source_revision),
            "subjectRefs": [subject_ref], "profile": PROFILE, "bindingRefs": binding_refs,
            "dependencyClosure": source_digest,
            "executionScope": {"repository": "agent-ix/quire-verification", "revision": source_revision, "paths": ["src/catalog.rs", "fixtures/quoin-catalog-export.json"]},
            "freshness": {"maxAgeSeconds": 2_592_000}, "input": input,
        })
    };
    let mut properties = vec![
        common(
            "LOCAL-CATALOG-PARITY",
            "property:quoin-catalog-field-parity",
            vec![artifact_ref(
                source_digest,
                "binding:pinned-quoin-catalog-export",
                source_revision,
            )],
            json!({
                "kind": "state", "snapshotRef": artifact_ref(source_digest, "snapshot:pinned-quoin-catalog", source_revision), "property": "all pinned method IDs and fields survive adaptation",
            }),
        ),
        common(
            "LOCAL-CATALOG-PERMUTATION",
            "property:catalog-permutation-invariance",
            vec![artifact_ref(
                source_digest,
                "binding:catalog-adapter",
                source_revision,
            )],
            json!({
                "kind": "relational", "leftSubject": "catalog:original-order", "rightSubject": "catalog:permuted-order", "relation": "adapted technique registry is bytewise equal",
            }),
        ),
        common(
            "LOCAL-HUMAN-REVIEW",
            "property:e02-boundary-human-review",
            Vec::new(),
            json!({
                "kind": "manual", "reviewProcedure": "Review identity ownership, compatibility behavior, claim limits, and rights disposition.", "requiredDisposition": ["accept", "revise", "reject"],
            }),
        ),
    ];
    for (property, (_, technique, _)) in properties.iter_mut().zip(SELECTIONS) {
        object_mut(property)?.insert(
            "techniqueId".to_owned(),
            Value::String(technique.to_owned()),
        );
        let property_id = string_field(property, "propertyId", "property")?.to_owned();
        object_mut(property)?.insert(
            "resultBinding".to_owned(),
            result_binding(&property_id, technique, &run_ref),
        );
    }
    Ok((properties, run_ref))
}

fn build_portfolio_plan(
    source_revision: &str,
    produced_at: &str,
    source_digest: &str,
    properties: &[Value],
) -> Result<Value, VerificationError> {
    let policy = json!({
        "contractVersion": contract_version(), "kind": "SelectionPolicy", "policyId": "POLICY-LOCAL-CATALOG-PORTFOLIO", "version": "1",
        "validUntil": "2026-10-07T00:00:00Z", "sourceRef": artifact_ref(source_digest, "policy:local-catalog-portfolio", source_revision),
        "provenance": {"kind": "project-authored", "standardDerived": false, "rights": "AGPL-3.0-or-later"},
        "rules": [
            {"ruleId": "fixed-parity", "priority": "required", "explanation": "Compare every field with the pinned existing behavior."},
            {"ruleId": "permutation-invariance", "priority": "recommended", "explanation": "Compare related executions under input-order changes."},
            {"ruleId": "human-boundary-review", "priority": "required", "explanation": "A human owner decides whether the provisional boundary is accepted."},
        ],
    });
    let capabilities = vec![
        capability("CAP-LOCAL-UNIT", "unit-testing", "state", false),
        capability(
            "CAP-LOCAL-METAMORPHIC",
            "metamorphic-testing",
            "relational",
            false,
        ),
        capability("CAP-HUMAN-REVIEW", "inspection", "manual", true),
    ];
    let selections = SELECTIONS.into_iter().map(|(property, technique, priority)| json!({
        "propertyId": property, "techniqueId": technique, "applicability": "applicable", "priority": priority,
        "reasons": [{"rule": "project-authored", "value": property}],
    })).collect::<Vec<_>>();
    build_verification_plan(&json!({
        "planId": PLAN_ID, "asOf": produced_at, "policy": policy, "properties": properties,
        "capabilities": capabilities, "selections": selections, "availablePrerequisites": [],
        "budget": {"wallClockSeconds": 10, "parallelism": 1},
    }))
}

fn build_results(
    properties: &[Value],
    produced_at: &str,
    source_revision: &str,
    run_ref: &Value,
    parity: &Observation,
    permutation: &Observation,
) -> Result<Vec<Value>, VerificationError> {
    let mut property = properties.iter();
    let parity_property = next_property(&mut property)?;
    let permutation_property = next_property(&mut property)?;
    let manual_property = next_property(&mut property)?;
    Ok(vec![
        observed_result(
            parity_property,
            "unit-testing",
            "RESULT-LOCAL-UNIT",
            produced_at,
            source_revision,
            run_ref,
            parity,
        )?,
        observed_result(
            permutation_property,
            "metamorphic-testing",
            "RESULT-LOCAL-METAMORPHIC",
            produced_at,
            source_revision,
            run_ref,
            permutation,
        )?,
        manual_result(manual_property, produced_at, source_revision, run_ref)?,
    ])
}

fn next_property<'a>(
    properties: &mut impl Iterator<Item = &'a Value>,
) -> Result<&'a Value, VerificationError> {
    properties.next().ok_or_else(|| {
        VerificationError::new(
            VerificationErrorCode::InvalidInput,
            "portfolio property set is incomplete",
        )
    })
}

#[derive(Debug)]
struct Observation {
    execution_status: &'static str,
    passed: bool,
    detail: String,
    elapsed_milliseconds: f64,
}

fn observe<F>(check: F) -> Observation
where
    F: FnOnce() -> Result<(bool, String), VerificationError>,
{
    let started = Instant::now();
    match check() {
        Ok((passed, detail)) => Observation {
            execution_status: "completed",
            passed,
            detail,
            elapsed_milliseconds: started.elapsed().as_secs_f64() * 1000.0,
        },
        Err(error) => Observation {
            execution_status: "failed",
            passed: false,
            detail: error.to_string(),
            elapsed_milliseconds: started.elapsed().as_secs_f64() * 1000.0,
        },
    }
}

fn check_catalog_parity<F>(
    catalog: &Value,
    adapter: &F,
) -> Result<(bool, String), VerificationError>
where
    F: Fn(&Value) -> Result<Value, VerificationError>,
{
    let adapted_catalog = adapter(catalog)?;
    let methods = arrays_field(
        field(catalog, "catalog", "catalog snapshot")?,
        "methods",
        "catalog",
    )?;
    let techniques = arrays_field(&adapted_catalog, "techniques", "adapted catalog")?;
    let source_by_id = by_technique_id(methods, "id", "source method")?;
    let adapted_by_id = by_technique_id(techniques, "techniqueId", "adapted technique")?;
    let mut failures = Vec::new();
    if adapted_catalog.get("qualified").and_then(Value::as_bool) != Some(true) {
        failures.push("adapted catalog is unqualified".to_owned());
    }
    for (id, method) in &source_by_id {
        let Some(technique) = adapted_by_id.get(id) else {
            failures.push(format!("missing adapted technique {}", id.as_str()));
            continue;
        };
        let expected = json!({
            "techniqueId": method.get("id").cloned().unwrap_or(Value::Null), "name": method.get("name").cloned().unwrap_or(Value::Null),
            "class": method.get("class").cloned().unwrap_or(Value::Null), "purpose": method.get("definition").cloned().unwrap_or(Value::Null),
            "evidenceKind": method.get("evidenceKind").cloned().unwrap_or(Value::Null), "applicability": method.get("applicability").cloned().unwrap_or_else(|| json!({})),
            "legacyToolingHints": method.get("tooling").cloned().unwrap_or_else(|| json!([])), "sourceModule": method.get("moduleName").cloned().unwrap_or(Value::Null),
        });
        let actual = expected
            .as_object()
            .map(|fields| {
                fields
                    .keys()
                    .map(|name| {
                        (
                            name.clone(),
                            technique.get(name).cloned().unwrap_or(Value::Null),
                        )
                    })
                    .collect::<Map<_, _>>()
            })
            .map_or(Value::Null, Value::Object);
        if !jcs_equal(&actual, &expected)? {
            failures.push(format!("field mismatch for {}", id.as_str()));
        }
    }
    for id in adapted_by_id.keys() {
        if !source_by_id.contains_key(id) {
            failures.push(format!("unexpected adapted technique {}", id.as_str()));
        }
    }
    if failures.is_empty() {
        Ok((
            true,
            format!(
                "checked {} method definitions by techniqueId",
                source_by_id.len()
            ),
        ))
    } else {
        Ok((false, failures.join("; ")))
    }
}

fn check_permutation_invariance<F>(
    catalog: &Value,
    adapter: &F,
) -> Result<(bool, String), VerificationError>
where
    F: Fn(&Value) -> Result<Value, VerificationError>,
{
    let baseline = field(&adapter(catalog)?, "techniques", "adapted catalog")?.clone();
    let mut reversed = catalog.clone();
    methods_mut(&mut reversed)?.reverse();
    let mut rotated = catalog.clone();
    if !methods_mut(&mut rotated)?.is_empty() {
        methods_mut(&mut rotated)?.rotate_left(1);
    }
    let reversed_output = field(&adapter(&reversed)?, "techniques", "adapted catalog")?.clone();
    let rotated_output = field(&adapter(&rotated)?, "techniques", "adapted catalog")?.clone();
    let passed = jcs_equal(&baseline, &reversed_output)? && jcs_equal(&baseline, &rotated_output)?;
    Ok((
        passed,
        if passed {
            "checked reversed and rotated catalog orders"
        } else {
            "catalog adaptation changed under input permutation"
        }
        .to_owned(),
    ))
}

fn capability(capability_id: &str, technique_id: &str, input_kind: &str, manual: bool) -> Value {
    json!({
        "contractVersion": contract_version(), "kind": "ToolCapability", "capabilityId": capability_id, "techniqueId": technique_id,
        "qualification": "qualified", "inputKinds": [input_kind], "profiles": [PROFILE],
        "prerequisites": if manual { json!([{"kind": "reviewer", "id": "verification-owner", "required": true}]) } else { json!([]) },
        "runner": {"kind": if manual { "manual" } else { "native-producer" }, "adapterId": if manual { "human-review-record" } else { "quire-verification-local-portfolio" }, "adapterVersion": "1"},
        "environmentPermissions": if manual { json!([]) } else { json!(["read-repository-fixtures"]) },
        "nativeArtifactInterface": {"mode": "stdout-json", "mediaTypes": ["application/json"]}, "limits": {"subject": "catalog adapter at one source revision"},
    })
}

fn observed_result(
    property: &Value,
    technique: &str,
    result_id: &str,
    produced_at: &str,
    revision: &str,
    run_ref: &Value,
    observation: &Observation,
) -> Result<Value, VerificationError> {
    let completed = observation.execution_status == "completed";
    result_base(
        property,
        technique,
        result_id,
        produced_at,
        revision,
        run_ref,
        json!({
            "execution": {"status": observation.execution_status, "detail": observation.detail, "elapsedMilliseconds": observation.elapsed_milliseconds},
            "logical": {"outcome": if completed { if observation.passed { "satisfied" } else { "violated" } } else { "not-evaluated" }},
            "activation": {"status": "not-required"}, "participation": {"status": "complete", "participants": ["catalog-adapter"]},
            "assumptions": [{"assumptionId": "pinned-source-fixture", "status": "met"}],
            "adequacy": if completed { json!({"status": "adequate", "observations": [observation.detail]}) } else { json!({"status": "not-assessed", "observations": []}) },
        }),
    )
}

fn manual_result(
    property: &Value,
    produced_at: &str,
    revision: &str,
    run_ref: &Value,
) -> Result<Value, VerificationError> {
    result_base(
        property,
        "inspection",
        "RESULT-LOCAL-HUMAN-REVIEW-PENDING",
        produced_at,
        revision,
        run_ref,
        json!({
            "execution": {"status": "not-run", "detail": "verification-owner review pending"}, "logical": {"outcome": "not-evaluated"},
            "activation": {"status": "not-required"}, "participation": {"status": "not-started", "participants": []},
            "assumptions": [], "adequacy": {"status": "not-assessed", "observations": []},
        }),
    )
}

fn result_base(
    property: &Value,
    technique: &str,
    result_id: &str,
    produced_at: &str,
    revision: &str,
    run_ref: &Value,
    dimensions: Value,
) -> Result<Value, VerificationError> {
    let property_id = string_field(property, "propertyId", "portfolio property")?;
    let mut result = json!({
        "contractVersion": contract_version(), "kind": "TechniqueResult", "resultId": result_id, "planId": PLAN_ID,
        "taskId": format!("verify:{property_id}:{technique}"), "techniqueId": technique,
        "producedAt": produced_at, "runRef": run_ref,
        "propertyRef": field(property, "propertyRef", "portfolio property")?,
        "subjectRefs": field(property, "subjectRefs", "portfolio property")?, "profile": PROFILE,
        "bindingRefs": field(property, "bindingRefs", "portfolio property")?,
        "dependencyClosure": field(property, "dependencyClosure", "portfolio property")?,
        "executionScope": field(property, "executionScope", "portfolio property")?,
        "provenance": {"producer": PRODUCER, "producerVersion": "1", "adapter": ADAPTER, "adapterVersion": "1", "sourceRevision": revision, "executionMode": "native-local"}, "nativeArtifacts": [],
    });
    let target = object_mut(&mut result)?;
    let Value::Object(extra) = dimensions else {
        return Err(VerificationError::new(
            VerificationErrorCode::InvalidInput,
            "result dimensions must be an object",
        ));
    };
    target.extend(extra);
    Ok(result)
}

fn result_binding(property_id: &str, technique: &str, run_ref: &Value) -> Value {
    json!({"planId": PLAN_ID, "taskId": format!("verify:{property_id}:{technique}"), "runRef": run_ref, "producer": PRODUCER, "producerVersion": "1", "adapter": ADAPTER, "adapterVersion": "1"})
}

fn object_mut(value: &mut Value) -> Result<&mut Map<String, Value>, VerificationError> {
    value.as_object_mut().ok_or_else(|| {
        VerificationError::new(VerificationErrorCode::InvalidInput, "expected object")
    })
}

fn methods_mut(value: &mut Value) -> Result<&mut Vec<Value>, VerificationError> {
    value
        .pointer_mut("/catalog/methods")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| {
            VerificationError::new(
                VerificationErrorCode::InvalidInput,
                "catalog methods must be an array",
            )
        })
}
