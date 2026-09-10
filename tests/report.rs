// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! VP04 report-core and external-disposition acceptance tests.

use std::collections::{BTreeMap, BTreeSet};

use ix_trace_rs::trace;
use quire_verification::contracts::{VerificationErrorCode, wire};
use quire_verification::evidence::assess_evidence;
use quire_verification::fixtures::validate_fixture_result_references;
use quire_verification::measurement::{measurement_observation, record_measurement_series};
use quire_verification::report::{DispositionInput, assemble_portfolio_report as assemble_report};
use quire_verification::shared_reference::{
    ArtifactExpectation, ReferenceExpectation, ReferenceSupport, SharedReferenceReader,
    ValidatedSharedReference,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const REVIEWER: &str = "agent-ix-verification-owner";

fn retained() -> Value {
    serde_json::from_str(include_str!("../evidence/local-catalog-portfolio.json"))
        .expect("retained portfolio")
}

fn artifact_reference(kind: &str, identity: &str, wire: &str, bytes: &[u8]) -> Value {
    json!({
        "wireVersion": "ix.shared-reference/2-draft",
        "kind": "artifact",
        "value": {
            "refVersion": "ix.artifact-ref/3-draft",
            "kind": kind,
            "authority": "github.com/agent-ix/quire-verification",
            "identity": identity,
            "revision": {"namespace": "vp04-test", "value": "1"},
            "digest": format!("sha256:{:x}", Sha256::digest(bytes)),
            "wire": {"identity": wire, "version": "1"}
        }
    })
}

fn validate_reference(reference: &Value, bytes: &[u8]) -> ValidatedSharedReference {
    let value = &reference["value"];
    let expectation = ArtifactExpectation {
        ref_version: value["refVersion"].as_str().expect("refVersion").into(),
        kind: value["kind"].as_str().expect("kind").into(),
        authority: value["authority"].as_str().expect("authority").into(),
        identity: value["identity"].as_str().expect("identity").into(),
        revision_namespace: value["revision"]["namespace"]
            .as_str()
            .expect("namespace")
            .into(),
        revision_value: value["revision"]["value"]
            .as_str()
            .expect("revision")
            .into(),
        digest: value["digest"].as_str().expect("digest").into(),
        wire_identity: value["wire"]["identity"].as_str().expect("wire").into(),
        wire_version: value["wire"]["version"].as_str().expect("version").into(),
        bytes: Some(bytes.to_vec()),
    };
    let support = ReferenceSupport {
        wire_versions: BTreeMap::from([(
            expectation.wire_identity.clone(),
            BTreeSet::from([expectation.wire_version.clone()]),
        )]),
        ..ReferenceSupport::default()
    };
    SharedReferenceReader::new(support)
        .read(
            &serde_json::to_vec(reference).expect("reference JSON"),
            &ReferenceExpectation::Artifact(expectation),
        )
        .expect("strict reference")
}

fn replace_review_procedure(value: &mut Value, procedure: &Value) {
    match value {
        Value::Object(fields) => {
            if fields
                .get("value")
                .and_then(|value| value.get("kind"))
                .and_then(Value::as_str)
                == Some("review-procedure")
            {
                *value = procedure.clone();
                return;
            }
            for child in fields.values_mut() {
                replace_review_procedure(child, procedure);
            }
        }
        Value::Array(items) => {
            for child in items {
                replace_review_procedure(child, procedure);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

fn measurements(candidate_identity: &str, environment_identity: &str) -> Value {
    let mut observations =
        vec![measurement_observation("setup-0", "setup", 0, "10", None).expect("setup")];
    for repetition in 1..=5 {
        for (phase, elapsed) in [
            ("direct-parse", repetition * 10),
            ("parse-format-reparse", repetition * 20),
            ("complete-run", repetition * 30),
        ] {
            observations.push(
                measurement_observation(
                    &format!("{phase}-{repetition}"),
                    phase,
                    repetition,
                    &elapsed.to_string(),
                    None,
                )
                .expect("observation"),
            );
        }
    }
    record_measurement_series(
        candidate_identity,
        environment_identity,
        &observations,
        &json!({
            "population": "this exact candidate and fixture pair",
            "exclusions": ["other candidates", "other fixtures"],
            "clock": "monotonic-process-local",
            "commandOrder": ["setup", "direct-parse", "parse-format-reparse", "complete-run"],
            "cacheTreatment": "naturally warmed",
            "environmentalDifferences": [],
            "uncertainty": "scheduler and filesystem cache state",
            "alternativeExplanations": ["scheduler variation", "cache variation"]
        }),
    )
    .expect("measurements")
}

fn core(procedure: &Value) -> Value {
    let mut retained = retained();
    replace_review_procedure(&mut retained["plan"], procedure);
    let candidate_ref = artifact_reference(
        "executable-projection",
        "candidate:vp04",
        "application/octet-stream",
        b"candidate",
    );
    let subject_ref = artifact_reference("source", "source:valid", "text/plain", b"valid source");
    let mutation_ref = artifact_reference(
        "source",
        "source:mutation",
        "text/plain",
        b"mutation source",
    );
    let closure_ref = artifact_reference(
        "dependency-closure",
        "closure:vp04",
        "application/toml",
        b"dependency closure",
    );
    let closure_digest = closure_ref["value"]["digest"].clone();
    for property in retained["plan"]["inventory"]
        .as_array_mut()
        .expect("inventory")
    {
        property["subjectRefs"] = json!([subject_ref, mutation_ref]);
        property["dependencyClosure"] = closure_digest.clone();
        property["executionScope"]["revision"] = json!("1");
        property["resultBinding"]["producerVersion"] = json!("1");
    }
    for result in retained["results"].as_array_mut().expect("results") {
        result["subjectRefs"] = json!([subject_ref, mutation_ref]);
        result["dependencyClosure"] = closure_digest.clone();
        result["executionScope"]["revision"] = json!("1");
        result["provenance"]["sourceRevision"] = json!("1");
        result["provenance"]["producerVersion"] = json!("1");
    }
    let results = retained["results"].as_array().expect("results");
    let packet = json!({
        "asOf": retained["producedAt"],
        "inventory": retained["plan"]["inventory"],
        "results": results,
    });
    let references = validate_fixture_result_references(&packet).expect("evidence references");
    let assessment = assess_evidence(&packet, &references).expect("assessment");
    let environment_ref = artifact_reference(
        "environment",
        "environment:vp04",
        "text/plain",
        b"environment",
    );
    let candidate_identity = candidate_ref["value"]["digest"]
        .as_str()
        .expect("candidate digest");
    let environment_identity = environment_ref["value"]["digest"]
        .as_str()
        .expect("environment digest");
    json!({
        "schemaVersion": "quire-verification.vp04-report-core/v1",
        "reportId": "VP04-REPORT",
        "candidateRef": candidate_ref,
        "subjectRef": subject_ref,
        "mutationRef": mutation_ref,
        "dependencyClosureRef": closure_ref,
        "environmentRef": environment_ref,
        "commands": [{"commandId": "direct", "arguments": ["parse", "fixture.native"], "expectedExitCode": 0, "responseKind": "parsed-json"}],
        "plan": retained["plan"],
        "taskIds": retained["plan"]["tasks"].as_array().expect("tasks").iter().map(|task| task["taskId"].clone()).collect::<Vec<_>>(),
        "nativeRuns": [{"runId": "direct-run", "commandId": "direct", "status": "failed", "observedExitCode": null, "stdoutArtifact": null, "stderrArtifact": null, "errorCode": "controlled_fixture"}],
        "results": results,
        "assessment": assessment,
        "assessedAt": retained["producedAt"],
        "measurements": measurements(candidate_identity, environment_identity),
        "interpretation": "These observations describe only the selected quire-spec revision and exact fixture pair.",
        "claimLimits": [
            "The run establishes only the declared parser syntax responses and format stability for these fixtures.",
            "Quoin retains ownership of evidence interpretation and sufficiency."
        ],
        "producers": ["quire-spec-language", "quire-verification"],
        "adapters": ["native-rust"],
        "evidenceOwner": "Quoin",
        "sufficiencyDecision": "not-made"
    })
}

fn assemble_portfolio_report(
    core: &Value,
    procedure: &Value,
    validated_procedure: &ValidatedSharedReference,
    reviewer: &str,
    dispositions: &[DispositionInput<'_>],
) -> Result<Value, quire_verification::contracts::VerificationError> {
    let packet = json!({"results": core["results"]});
    let mut references = validate_fixture_result_references(&packet)?;
    for (field, bytes) in [
        ("candidateRef", b"candidate".as_slice()),
        ("subjectRef", b"valid source".as_slice()),
        ("mutationRef", b"mutation source".as_slice()),
        ("dependencyClosureRef", b"dependency closure".as_slice()),
        ("environmentRef", b"environment".as_slice()),
    ] {
        references.push(validate_reference(&core[field], bytes));
    }
    assemble_report(
        core,
        &references,
        procedure,
        validated_procedure,
        reviewer,
        dispositions,
    )
}

fn disposition_bytes(core_digest: &str, procedure: &Value, reviewer: &str, value: &str) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "schemaVersion": "quire-verification.vp04-disposition/v1",
        "dispositionId": format!("VP04-{value}"),
        "coreDigest": core_digest,
        "reviewer": reviewer,
        "procedureRef": procedure,
        "observedAt": "2026-09-09T20:00:00Z",
        "value": value
    }))
    .expect("disposition JSON")
}

fn disposition_case<'a>(
    bytes: &'a [u8],
    reference: &'a Value,
    validated: &'a ValidatedSharedReference,
) -> DispositionInput<'a> {
    DispositionInput {
        reference,
        validated_reference: validated,
        bytes,
    }
}

#[trace("TC-037", "FR-020-AC-6")]
#[trace("TC-037", "FR-020-AC-7")]
#[trace("TC-037", "FR-021-AC-1")]
#[trace("TC-037", "FR-021-AC-2")]
#[trace("TC-037", "FR-021-AC-3")]
#[trace("TC-037", "FR-021-AC-5")]
#[test]
fn tc_037_seals_schema_derived_core_and_binds_each_human_value() {
    let procedure_bytes = b"VP04 external review procedure";
    let procedure = artifact_reference(
        "review-procedure",
        "procedure:vp04",
        "text/plain",
        procedure_bytes,
    );
    let validated_procedure = validate_reference(&procedure, procedure_bytes);
    let core = core(&procedure);
    let incomplete =
        assemble_portfolio_report(&core, &procedure, &validated_procedure, REVIEWER, &[])
            .expect("incomplete report");
    assert_eq!(incomplete["status"], "incomplete");
    assert!(incomplete["disposition"].is_null());
    let schema: Value =
        serde_json::from_str(include_str!("../contracts/e02-draft-2.schema.json")).expect("schema");
    assert_eq!(schema["oneOf"].as_array().expect("public root").len(), 5);
    assert!(schema["$defs"].get("Vp04PortfolioReport").is_some());
    let _: wire::Vp04PortfolioReport =
        serde_json::from_value(incomplete.clone()).expect("schema-generated report type");

    for value in ["accept", "revise", "reject"] {
        let bytes = disposition_bytes(
            incomplete["coreDigest"].as_str().expect("core digest"),
            &procedure,
            REVIEWER,
            value,
        );
        let reference = artifact_reference(
            "review-disposition",
            &format!("disposition:{value}"),
            "application/json",
            &bytes,
        );
        let validated = validate_reference(&reference, &bytes);
        let report = assemble_portfolio_report(
            &core,
            &procedure,
            &validated_procedure,
            REVIEWER,
            &[disposition_case(&bytes, &reference, &validated)],
        )
        .expect("final report");
        assert_eq!(report["status"], "final");
        assert_eq!(report["coreDigest"], incomplete["coreDigest"]);
        assert_eq!(report["core"], incomplete["core"]);
        assert_eq!(report["disposition"]["value"], value);
        assert_eq!(report["core"]["evidenceOwner"], "Quoin");
        assert_eq!(report["core"]["sufficiencyDecision"], "not-made");
    }
}

#[trace("TC-037", "FR-021-AC-4")]
#[test]
fn tc_037_refuses_duplicate_foreign_unsupported_and_invalid_digest_cases() {
    let procedure_bytes = b"VP04 external review procedure";
    let procedure = artifact_reference(
        "review-procedure",
        "procedure:vp04",
        "text/plain",
        procedure_bytes,
    );
    let validated_procedure = validate_reference(&procedure, procedure_bytes);
    let core = core(&procedure);
    let incomplete =
        assemble_portfolio_report(&core, &procedure, &validated_procedure, REVIEWER, &[])
            .expect("sealed core");
    let digest = incomplete["coreDigest"].as_str().expect("digest");

    let make = |bytes: &[u8], kind: &str| {
        let reference = artifact_reference(kind, "disposition:test", "application/json", bytes);
        let validated = validate_reference(&reference, bytes);
        (reference, validated)
    };
    let exact_bytes = disposition_bytes(digest, &procedure, REVIEWER, "accept");
    let (exact_ref, exact_validated) = make(&exact_bytes, "review-disposition");
    let exact = disposition_case(&exact_bytes, &exact_ref, &exact_validated);
    assert_eq!(
        assemble_portfolio_report(
            &core,
            &procedure,
            &validated_procedure,
            REVIEWER,
            &[exact, exact]
        )
        .expect_err("duplicate")
        .code(),
        VerificationErrorCode::DuplicateDisposition
    );

    for (bytes, expected) in [
        (
            disposition_bytes(
                &format!("sha256-jcs:{}", "0".repeat(64)),
                &procedure,
                REVIEWER,
                "accept",
            ),
            VerificationErrorCode::ForeignDisposition,
        ),
        (
            disposition_bytes("sha256-jcs:wrong", &procedure, REVIEWER, "accept"),
            VerificationErrorCode::InvalidReportDigest,
        ),
        (
            disposition_bytes(digest, &procedure, REVIEWER, "abstain"),
            VerificationErrorCode::UnsupportedDisposition,
        ),
        (
            disposition_bytes(digest, &procedure, "another-reviewer", "accept"),
            VerificationErrorCode::ForeignDisposition,
        ),
    ] {
        let (reference, validated) = make(&bytes, "review-disposition");
        assert_eq!(
            assemble_portfolio_report(
                &core,
                &procedure,
                &validated_procedure,
                REVIEWER,
                &[disposition_case(&bytes, &reference, &validated)],
            )
            .expect_err("refusal")
            .code(),
            expected
        );
    }
}

#[trace("TC-037", "FR-021-AC-4")]
#[test]
fn tc_037_refuses_wrong_role_and_payload_digest_drift() {
    let procedure_bytes = b"VP04 external review procedure";
    let procedure = artifact_reference(
        "review-procedure",
        "procedure:vp04",
        "text/plain",
        procedure_bytes,
    );
    let validated_procedure = validate_reference(&procedure, procedure_bytes);
    let core = core(&procedure);
    let incomplete =
        assemble_portfolio_report(&core, &procedure, &validated_procedure, REVIEWER, &[])
            .expect("sealed core");
    let digest = incomplete["coreDigest"].as_str().expect("digest");
    let exact_bytes = disposition_bytes(digest, &procedure, REVIEWER, "accept");
    let exact_ref = artifact_reference(
        "review-disposition",
        "disposition:test",
        "application/json",
        &exact_bytes,
    );
    let exact_validated = validate_reference(&exact_ref, &exact_bytes);
    let wrong_role_ref = artifact_reference(
        "observation",
        "disposition:test",
        "application/json",
        &exact_bytes,
    );
    let wrong_role_validated = validate_reference(&wrong_role_ref, &exact_bytes);
    assert_eq!(
        assemble_portfolio_report(
            &core,
            &procedure,
            &validated_procedure,
            REVIEWER,
            &[disposition_case(
                &exact_bytes,
                &wrong_role_ref,
                &wrong_role_validated
            )],
        )
        .expect_err("wrong role")
        .code(),
        VerificationErrorCode::UnsupportedDisposition
    );
    let changed_bytes = disposition_bytes(digest, &procedure, REVIEWER, "reject");
    assert_eq!(
        assemble_portfolio_report(
            &core,
            &procedure,
            &validated_procedure,
            REVIEWER,
            &[disposition_case(
                &changed_bytes,
                &exact_ref,
                &exact_validated
            )],
        )
        .expect_err("payload digest")
        .code(),
        VerificationErrorCode::InvalidReportDigest
    );
}

#[trace("TC-037", "FR-021-AC-4")]
#[test]
fn tc_037_refuses_missing_digest_and_foreign_procedure() {
    let procedure_bytes = b"VP04 external review procedure";
    let procedure = artifact_reference(
        "review-procedure",
        "procedure:vp04",
        "text/plain",
        procedure_bytes,
    );
    let validated_procedure = validate_reference(&procedure, procedure_bytes);
    let core = core(&procedure);
    let incomplete =
        assemble_portfolio_report(&core, &procedure, &validated_procedure, REVIEWER, &[])
            .expect("sealed core");
    let digest = incomplete["coreDigest"].as_str().expect("digest");

    let mut missing: Value =
        serde_json::from_slice(&disposition_bytes(digest, &procedure, REVIEWER, "accept"))
            .expect("disposition");
    missing
        .as_object_mut()
        .expect("object")
        .remove("coreDigest");
    let missing_bytes = serde_json::to_vec(&missing).expect("missing digest bytes");
    let missing_ref = artifact_reference(
        "review-disposition",
        "disposition:missing-digest",
        "application/json",
        &missing_bytes,
    );
    let missing_validated = validate_reference(&missing_ref, &missing_bytes);
    assert_eq!(
        assemble_portfolio_report(
            &core,
            &procedure,
            &validated_procedure,
            REVIEWER,
            &[disposition_case(
                &missing_bytes,
                &missing_ref,
                &missing_validated
            )],
        )
        .expect_err("missing digest")
        .code(),
        VerificationErrorCode::InvalidReportDigest
    );

    let other = artifact_reference(
        "review-procedure",
        "procedure:other",
        "text/plain",
        b"other procedure",
    );
    let foreign_bytes = disposition_bytes(digest, &other, REVIEWER, "accept");
    let foreign_ref = artifact_reference(
        "review-disposition",
        "disposition:foreign-procedure",
        "application/json",
        &foreign_bytes,
    );
    let foreign_validated = validate_reference(&foreign_ref, &foreign_bytes);
    assert_eq!(
        assemble_portfolio_report(
            &core,
            &procedure,
            &validated_procedure,
            REVIEWER,
            &[disposition_case(
                &foreign_bytes,
                &foreign_ref,
                &foreign_validated
            )],
        )
        .expect_err("foreign procedure")
        .code(),
        VerificationErrorCode::ForeignDisposition
    );
}

#[trace("TC-037", "FR-021-AC-2", "FR-021-AC-5")]
#[test]
fn tc_037_refuses_an_assessment_that_does_not_recompute() {
    let procedure_bytes = b"VP04 external review procedure";
    let procedure = artifact_reference(
        "review-procedure",
        "procedure:vp04",
        "text/plain",
        procedure_bytes,
    );
    let validated_procedure = validate_reference(&procedure, procedure_bytes);
    let mut changed = core(&procedure);
    changed["assessment"]["sufficiencyDecision"] = json!("made-by-adapter");
    assert_eq!(
        assemble_portfolio_report(&changed, &procedure, &validated_procedure, REVIEWER, &[])
            .expect_err("fabricated assessment")
            .code(),
        VerificationErrorCode::SchemaViolation
    );
}

#[trace("TC-037", "FR-020-AC-2", "FR-021-AC-2")]
#[test]
fn tc_037_refuses_measurement_identity_detached_from_core_references() {
    let procedure_bytes = b"VP04 external review procedure";
    let procedure = artifact_reference(
        "review-procedure",
        "procedure:vp04",
        "text/plain",
        procedure_bytes,
    );
    let validated_procedure = validate_reference(&procedure, procedure_bytes);
    for field in ["candidateIdentity", "environmentIdentity"] {
        let mut changed = core(&procedure);
        changed["measurements"][field] = json!("sha256:detached");
        assert_eq!(
            assemble_portfolio_report(&changed, &procedure, &validated_procedure, REVIEWER, &[])
                .expect_err("detached measurement identity")
                .code(),
            VerificationErrorCode::SchemaViolation
        );
    }
}

#[trace("TC-037", "FR-021-AC-2")]
#[test]
fn tc_037_refuses_core_task_binding_drift() {
    let procedure_bytes = b"VP04 external review procedure";
    let procedure = artifact_reference(
        "review-procedure",
        "procedure:vp04",
        "text/plain",
        procedure_bytes,
    );
    let validated_procedure = validate_reference(&procedure, procedure_bytes);
    let mut changed = core(&procedure);
    changed["taskIds"][0] = json!("detached-task");
    assert_eq!(
        assemble_portfolio_report(&changed, &procedure, &validated_procedure, REVIEWER, &[])
            .expect_err("detached task binding")
            .code(),
        VerificationErrorCode::SchemaViolation
    );
}

#[trace("TC-037", "FR-021-AC-2")]
#[test]
fn tc_037_refuses_reproduction_identity_drift_inside_the_core() {
    let procedure_bytes = b"VP04 external review procedure";
    let procedure = artifact_reference(
        "review-procedure",
        "procedure:vp04",
        "text/plain",
        procedure_bytes,
    );
    let validated_procedure = validate_reference(&procedure, procedure_bytes);
    for pointer in [
        "/plan/inventory/0/dependencyClosure",
        "/plan/inventory/0/executionScope/revision",
        "/plan/inventory/0/resultBinding/producerVersion",
        "/results/0/dependencyClosure",
        "/results/0/executionScope/revision",
        "/results/0/provenance/sourceRevision",
        "/results/0/provenance/producerVersion",
        "/results/0/subjectRefs/0/value/identity",
    ] {
        let mut changed = core(&procedure);
        *changed.pointer_mut(pointer).expect("identity field") = json!("detached");
        assert_eq!(
            assemble_portfolio_report(&changed, &procedure, &validated_procedure, REVIEWER, &[])
                .expect_err("detached reproduction identity")
                .code(),
            VerificationErrorCode::SchemaViolation
        );
    }
}

#[trace("TC-037", "FR-020-AC-7", "FR-021-AC-5")]
#[test]
fn tc_037_refuses_claim_text_outside_the_reviewed_parser_boundary() {
    let procedure_bytes = b"VP04 external review procedure";
    let procedure = artifact_reference(
        "review-procedure",
        "procedure:vp04",
        "text/plain",
        procedure_bytes,
    );
    let validated_procedure = validate_reference(&procedure, procedure_bytes);
    for (field, value) in [
        (
            "interpretation",
            json!("This certifies general parser effectiveness."),
        ),
        ("claimLimits", json!(["Production throughput is proven."])),
    ] {
        let mut changed = core(&procedure);
        changed[field] = value;
        assert_eq!(
            assemble_portfolio_report(&changed, &procedure, &validated_procedure, REVIEWER, &[])
                .expect_err("expanded claim")
                .code(),
            VerificationErrorCode::SchemaViolation
        );
    }
}

#[trace("TC-037", "FR-021-AC-1")]
#[test]
fn tc_037_native_run_variants_refuse_cross_status_fields() {
    let procedure_bytes = b"VP04 external review procedure";
    let procedure = artifact_reference(
        "review-procedure",
        "procedure:vp04",
        "text/plain",
        procedure_bytes,
    );
    let validated_procedure = validate_reference(&procedure, procedure_bytes);
    for (status, observed, error) in [
        ("completed", Value::Null, Value::Null),
        ("completed", json!(0), json!("foreign")),
        ("failed", json!(1), json!("failure")),
        ("failed", Value::Null, Value::Null),
    ] {
        let mut changed = core(&procedure);
        changed["nativeRuns"][0]["status"] = json!(status);
        changed["nativeRuns"][0]["observedExitCode"] = observed;
        changed["nativeRuns"][0]["errorCode"] = error;
        assert_eq!(
            assemble_portfolio_report(&changed, &procedure, &validated_procedure, REVIEWER, &[])
                .expect_err("cross-status field")
                .code(),
            VerificationErrorCode::SchemaViolation
        );
    }

    let mut missing_run = core(&procedure);
    missing_run["commands"]
        .as_array_mut()
        .expect("commands")
        .push(json!({
            "commandId": "unreported",
            "arguments": ["parse", "unreported.native"],
            "expectedExitCode": 0,
            "responseKind": "parsed-json"
        }));
    assert_eq!(
        assemble_portfolio_report(
            &missing_run,
            &procedure,
            &validated_procedure,
            REVIEWER,
            &[]
        )
        .expect_err("command without a run")
        .code(),
        VerificationErrorCode::SchemaViolation
    );
}
