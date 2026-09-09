// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Sealing and external-disposition binding for the VP04 domain report.
//!
//! Governing requirement: FR-021.

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

use crate::contracts::{
    VerificationError, VerificationErrorCode, jcs_equal, jcs_sha256, validate_schema_definition,
};
use crate::evidence::assess_evidence;
use crate::shared_reference::{ValidatedSharedReference, parse_strict_bounded_json};

/// One exact externally validated human disposition artifact.
#[derive(Clone, Copy, Debug)]
pub struct DispositionInput<'a> {
    /// Exact shared artifact-reference envelope retained in the report.
    pub reference: &'a Value,
    /// Opaque proof that the strict shared-reference reader accepted the envelope and bytes.
    pub validated_reference: &'a ValidatedSharedReference,
    /// Exact disposition payload bytes whose digest is named by `reference`.
    pub bytes: &'a [u8],
}

/// Seals a validated automated core and binds zero or one exact human disposition.
///
/// The digest covers the core alone. An absent disposition yields an incomplete
/// report; the function does not create, authenticate, persist, or orchestrate
/// human evidence.
///
/// # Errors
/// Returns the stable FR-021 refusal codes for duplicate, foreign, unsupported,
/// or digest-invalid dispositions, and schema errors for an invalid core.
pub fn assemble_portfolio_report(
    core: &Value,
    validated_references: &[ValidatedSharedReference],
    selected_procedure: &Value,
    validated_procedure: &ValidatedSharedReference,
    expected_reviewer: &str,
    dispositions: &[DispositionInput<'_>],
) -> Result<Value, VerificationError> {
    validate_schema_definition("Vp04AutomatedReportCore", core)?;
    validate_core_references(core, validated_references)?;
    validate_measurement_bindings(core)?;
    validate_reproduction_bindings(core)?;
    validate_selected_reference(selected_procedure, validated_procedure, "review-procedure")?;
    validate_core_bindings(core, selected_procedure)?;
    validate_core_assessment(core, validated_references)?;
    if expected_reviewer.is_empty() {
        return Err(foreign("selected reviewer identity is empty"));
    }
    if dispositions.len() > 1 {
        return Err(VerificationError::new(
            VerificationErrorCode::DuplicateDisposition,
            "more than one disposition was supplied",
        ));
    }

    let core_digest = jcs_sha256(core).map_err(|error| {
        VerificationError::new(
            VerificationErrorCode::InvalidReportDigest,
            "automated report core cannot be digested",
        )
        .with_context("cause", error.code().as_str())
    })?;
    let (disposition_ref, disposition, status) = if let Some(input) = dispositions.first() {
        let disposition =
            validate_disposition(input, &core_digest, selected_procedure, expected_reviewer)?;
        (input.reference.clone(), disposition, "final")
    } else {
        (Value::Null, Value::Null, "incomplete")
    };
    let report = json!({
        "schemaVersion": "quire-verification.vp04-report/v1",
        "core": core,
        "coreDigest": core_digest,
        "dispositionRef": disposition_ref,
        "disposition": disposition,
        "status": status,
    });
    validate_schema_definition("Vp04PortfolioReport", &report)?;
    Ok(report)
}

fn validate_reproduction_bindings(core: &Value) -> Result<(), VerificationError> {
    let candidate_revision = core
        .pointer("/candidateRef/value/revision/value")
        .and_then(Value::as_str)
        .ok_or_else(|| invalid_core("candidate revision is absent"))?;
    let closure_digest = core
        .pointer("/dependencyClosureRef/value/digest")
        .and_then(Value::as_str)
        .ok_or_else(|| invalid_core("dependency-closure digest is absent"))?;
    let selected_subjects = json!([core["subjectRef"], core["mutationRef"]]);
    let inventory = core["plan"]["inventory"]
        .as_array()
        .ok_or_else(|| invalid_core("report plan inventory is absent"))?;
    let results = core["results"]
        .as_array()
        .ok_or_else(|| invalid_core("report results are absent"))?;
    if inventory.is_empty() || results.is_empty() {
        return Err(invalid_core(
            "report reproduction inventory or results are empty",
        ));
    }
    for property in inventory {
        if property["dependencyClosure"] != closure_digest
            || property
                .pointer("/executionScope/revision")
                .and_then(Value::as_str)
                != Some(candidate_revision)
            || property
                .pointer("/resultBinding/producerVersion")
                .and_then(Value::as_str)
                != Some(candidate_revision)
            || !jcs_equal(&property["subjectRefs"], &selected_subjects)?
        {
            return Err(invalid_core(
                "plan inventory differs from the selected candidate, sources, or closure",
            ));
        }
    }
    for result in results {
        if result["dependencyClosure"] != closure_digest
            || result
                .pointer("/executionScope/revision")
                .and_then(Value::as_str)
                != Some(candidate_revision)
            || result
                .pointer("/provenance/sourceRevision")
                .and_then(Value::as_str)
                != Some(candidate_revision)
            || result
                .pointer("/provenance/producerVersion")
                .and_then(Value::as_str)
                != Some(candidate_revision)
            || !jcs_equal(&result["subjectRefs"], &selected_subjects)?
        {
            return Err(invalid_core(
                "result differs from the selected candidate, sources, or closure",
            ));
        }
    }
    Ok(())
}

fn validate_measurement_bindings(core: &Value) -> Result<(), VerificationError> {
    for (measurement_field, reference_field) in [
        ("candidateIdentity", "candidateRef"),
        ("environmentIdentity", "environmentRef"),
    ] {
        let measurement_identity = core
            .pointer(&format!("/measurements/{measurement_field}"))
            .and_then(Value::as_str)
            .ok_or_else(|| invalid_core("report measurement identity is absent"))?;
        let reference_digest = core
            .pointer(&format!("/{reference_field}/value/digest"))
            .and_then(Value::as_str)
            .ok_or_else(|| invalid_core("report identity reference has no digest"))?;
        if measurement_identity != reference_digest {
            return Err(invalid_core(
                "report measurement identity differs from its exact reference digest",
            ));
        }
    }
    Ok(())
}

fn validate_core_references(
    core: &Value,
    validated_references: &[ValidatedSharedReference],
) -> Result<(), VerificationError> {
    for (field, expected_role) in [
        ("candidateRef", "executable-projection"),
        ("subjectRef", "source"),
        ("mutationRef", "source"),
        ("dependencyClosureRef", "dependency-closure"),
        ("environmentRef", "environment"),
    ] {
        let reference = core
            .get(field)
            .ok_or_else(|| invalid_core("report core reference is absent"))?;
        let canonical = crate::contracts::jcs_canonicalize(reference)?;
        let accepted = validated_references.iter().any(|validated| {
            validated
                .reference_key()
                .is_some_and(|(key, role)| key == canonical && role == expected_role)
        });
        if !accepted {
            return Err(invalid_core(
                "report core reference has no exact strict-reader proof",
            ));
        }
    }
    Ok(())
}

fn validate_core_assessment(
    core: &Value,
    validated_references: &[ValidatedSharedReference],
) -> Result<(), VerificationError> {
    let packet = json!({
        "asOf": core["assessedAt"],
        "inventory": core["plan"]["inventory"],
        "results": core["results"],
    });
    let expected = assess_evidence(&packet, validated_references)?;
    if !jcs_equal(&core["assessment"], &expected)? {
        return Err(invalid_core(
            "report core assessment does not recompute from its plan and results",
        ));
    }
    Ok(())
}

fn validate_core_bindings(
    core: &Value,
    selected_procedure: &Value,
) -> Result<(), VerificationError> {
    let tasks = core
        .pointer("/plan/tasks")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_core("report core has no plan tasks"))?;
    let task_ids = tasks
        .iter()
        .filter_map(|task| task.get("taskId").and_then(Value::as_str))
        .collect::<Vec<_>>();
    let reported_ids = core["taskIds"]
        .as_array()
        .ok_or_else(|| invalid_core("report core task IDs are absent"))?;
    if reported_ids.len() != task_ids.len()
        || reported_ids
            .iter()
            .zip(&task_ids)
            .any(|(reported, planned)| reported.as_str() != Some(planned))
    {
        return Err(invalid_core("report core task IDs differ from its plan"));
    }
    validate_result_bindings(core, tasks)?;

    let manual_tasks = tasks
        .iter()
        .filter(|task| task.pointer("/input/kind").and_then(Value::as_str) == Some("manual"))
        .collect::<Vec<_>>();
    if manual_tasks.len() != 1 {
        return Err(invalid_core(
            "report core must retain exactly one manual disposition task",
        ));
    }
    let manual = manual_tasks[0];
    if manual["invocationAllowed"] != false
        || !jcs_equal(&manual["input"]["reviewProcedureRef"], selected_procedure)?
    {
        return Err(foreign(
            "report core manual task differs from the selected review procedure",
        ));
    }
    let manual_id = manual["taskId"]
        .as_str()
        .ok_or_else(|| invalid_core("manual task has no ID"))?;
    for result in core["results"]
        .as_array()
        .ok_or_else(|| invalid_core("report core results are absent"))?
    {
        if result["taskId"] == manual_id
            && (result.pointer("/execution/status").and_then(Value::as_str) != Some("not-run")
                || result.pointer("/logical/outcome").and_then(Value::as_str)
                    != Some("not-evaluated"))
        {
            return Err(invalid_core(
                "manual disposition task has a fabricated execution result",
            ));
        }
    }
    validate_run_bindings(core)
}

fn validate_result_bindings(core: &Value, tasks: &[Value]) -> Result<(), VerificationError> {
    let plan_id = core["plan"]["planId"]
        .as_str()
        .ok_or_else(|| invalid_core("report plan ID is absent"))?;
    let results = core["results"]
        .as_array()
        .ok_or_else(|| invalid_core("report core results are absent"))?;
    let mut result_ids = BTreeSet::new();
    let mut represented_tasks = BTreeSet::new();
    for result in results {
        let result_id = result["resultId"]
            .as_str()
            .ok_or_else(|| invalid_core("result ID is absent"))?;
        let task_id = result["taskId"]
            .as_str()
            .ok_or_else(|| invalid_core("result task ID is absent"))?;
        let technique_id = result["techniqueId"]
            .as_str()
            .ok_or_else(|| invalid_core("result technique ID is absent"))?;
        let task = tasks
            .iter()
            .find(|task| task["taskId"] == task_id)
            .ok_or_else(|| invalid_core("result names no retained plan task"))?;
        let manual = task.pointer("/input/kind").and_then(Value::as_str) == Some("manual");
        let valid_execution_binding = if manual {
            task["invocationAllowed"] == false
                && result.pointer("/execution/status").and_then(Value::as_str) == Some("not-run")
                && result.pointer("/logical/outcome").and_then(Value::as_str)
                    == Some("not-evaluated")
        } else {
            task["readiness"] == "ready" && task["invocationAllowed"] == true
        };
        if !result_ids.insert(result_id)
            || result["planId"] != plan_id
            || task["kind"] != "verification"
            || task["techniqueId"] != technique_id
            || !valid_execution_binding
        {
            return Err(invalid_core(
                "result identity or execution binding differs from its ready plan task",
            ));
        }
        if !manual {
            represented_tasks.insert(task_id);
        }
    }
    for task in tasks.iter().filter(|task| {
        task["kind"] == "verification"
            && task["readiness"] == "ready"
            && task["invocationAllowed"] == true
    }) {
        let task_id = task["taskId"]
            .as_str()
            .ok_or_else(|| invalid_core("ready task ID is absent"))?;
        if !represented_tasks.contains(task_id) {
            return Err(invalid_core("ready plan task has no retained result"));
        }
    }
    let manual_task_ids = tasks
        .iter()
        .filter(|task| task.pointer("/input/kind").and_then(Value::as_str) == Some("manual"))
        .filter_map(|task| task["taskId"].as_str())
        .collect::<BTreeSet<_>>();
    let manual_result_count = results
        .iter()
        .filter(|result| {
            result["taskId"]
                .as_str()
                .is_some_and(|task_id| manual_task_ids.contains(task_id))
        })
        .count();
    if manual_result_count != manual_task_ids.len() {
        return Err(invalid_core(
            "each manual plan task must have one retained not-run result",
        ));
    }
    Ok(())
}

fn validate_run_bindings(core: &Value) -> Result<(), VerificationError> {
    let commands = core["commands"]
        .as_array()
        .ok_or_else(|| invalid_core("report core commands are absent"))?;
    let command_ids = commands
        .iter()
        .filter_map(|command| command.get("commandId").and_then(Value::as_str))
        .collect::<BTreeSet<_>>();
    if command_ids.len() != commands.len() {
        return Err(invalid_core("report core repeats a command ID"));
    }
    let runs = core["nativeRuns"]
        .as_array()
        .ok_or_else(|| invalid_core("report core native runs are absent"))?;
    let mut run_ids = BTreeSet::new();
    let mut run_command_ids = BTreeSet::new();
    for run in runs {
        let run_id = run["runId"]
            .as_str()
            .ok_or_else(|| invalid_core("native run has no ID"))?;
        let command_id = run["commandId"]
            .as_str()
            .ok_or_else(|| invalid_core("native run has no command ID"))?;
        if !run_ids.insert(run_id)
            || !run_command_ids.insert(command_id)
            || !command_ids.contains(command_id)
        {
            return Err(invalid_core(
                "native run or command binding is duplicated or foreign",
            ));
        }
        if run["status"] == "completed" {
            let command = commands
                .iter()
                .find(|command| command["commandId"] == command_id)
                .ok_or_else(|| invalid_core("completed run command is absent"))?;
            if run["observedExitCode"] != command["expectedExitCode"] {
                return Err(invalid_core(
                    "completed native run observed a different exit status",
                ));
            }
        }
    }
    if run_command_ids != command_ids {
        return Err(invalid_core(
            "report core commands and native runs are not one-to-one",
        ));
    }
    Ok(())
}

fn validate_disposition(
    input: &DispositionInput<'_>,
    core_digest: &str,
    selected_procedure: &Value,
    expected_reviewer: &str,
) -> Result<Value, VerificationError> {
    validate_selected_reference(
        input.reference,
        input.validated_reference,
        "review-disposition",
    )?;
    let expected_byte_digest = input
        .reference
        .pointer("/value/digest")
        .and_then(Value::as_str)
        .ok_or_else(|| unsupported("disposition reference has no byte digest"))?;
    let actual_byte_digest = format!("sha256:{:x}", Sha256::digest(input.bytes));
    if actual_byte_digest != expected_byte_digest {
        return Err(VerificationError::new(
            VerificationErrorCode::InvalidReportDigest,
            "disposition bytes do not match their artifact digest",
        ));
    }
    let disposition = parse_strict_bounded_json(input.bytes)
        .map_err(|_| unsupported("disposition bytes are not strict bounded JSON"))?;
    if let Err(error) = validate_schema_definition("Vp04ReviewDisposition", &disposition) {
        let invalid_digest = disposition
            .get("coreDigest")
            .and_then(Value::as_str)
            .is_none_or(|digest| !valid_jcs_digest(digest));
        let code = if invalid_digest {
            VerificationErrorCode::InvalidReportDigest
        } else {
            VerificationErrorCode::UnsupportedDisposition
        };
        return Err(
            VerificationError::new(code, "disposition payload is unsupported")
                .with_context("cause", error.code().as_str()),
        );
    }
    if disposition["coreDigest"] != core_digest {
        return Err(foreign("disposition names another automated report core"));
    }
    if disposition["reviewer"] != expected_reviewer {
        return Err(foreign("disposition names another reviewer"));
    }
    if !jcs_equal(&disposition["procedureRef"], selected_procedure)? {
        return Err(foreign("disposition names another review procedure"));
    }
    Ok(disposition)
}

fn validate_selected_reference(
    reference: &Value,
    validated: &ValidatedSharedReference,
    expected_role: &str,
) -> Result<(), VerificationError> {
    let Some((canonical, role)) = validated.reference_key() else {
        return Err(unsupported("shared reference is not an artifact reference"));
    };
    if role != expected_role {
        return Err(unsupported("shared reference has the wrong role"));
    }
    let validated_value: Value = serde_json::from_slice(canonical)
        .map_err(|_| unsupported("validated reference cannot be decoded"))?;
    if !jcs_equal(reference, &validated_value)? {
        return Err(unsupported(
            "supplied reference differs from its strict-reader proof",
        ));
    }
    Ok(())
}

fn valid_jcs_digest(value: &str) -> bool {
    value.len() == 75
        && value.strip_prefix("sha256-jcs:").is_some_and(|hex| {
            hex.bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        })
}

fn foreign(message: impl Into<String>) -> VerificationError {
    VerificationError::new(VerificationErrorCode::ForeignDisposition, message.into())
}

fn unsupported(message: impl Into<String>) -> VerificationError {
    VerificationError::new(
        VerificationErrorCode::UnsupportedDisposition,
        message.into(),
    )
}

fn invalid_core(message: impl Into<String>) -> VerificationError {
    VerificationError::new(VerificationErrorCode::SchemaViolation, message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::measurement::validate_measurement_series;
    use crate::shared_reference::{
        ArtifactExpectation, ReferenceExpectation, ReferenceSupport, SharedReferenceReader,
    };
    use ix_trace_rs::trace;
    use std::collections::{BTreeMap, BTreeSet};

    const CORE_DIGEST: &str =
        "sha256-jcs:db03d0c81141e06135fa68c84125253ed28b0e92e961abb5b0c44c8a7abd56f2";
    const CANDIDATE_DIGEST: &str =
        "sha256:c6bd75613f85dfd38aaf37169aed0a3d7409466d4b2f34dcc5397896c46579e5";

    fn validate_artifact(reference: &Value, bytes: &[u8]) -> ValidatedSharedReference {
        let value = &reference["value"];
        let wire_identity = value["wire"]["identity"]
            .as_str()
            .expect("wire identity")
            .to_owned();
        let wire_version = value["wire"]["version"]
            .as_str()
            .expect("wire version")
            .to_owned();
        let support = ReferenceSupport {
            wire_versions: BTreeMap::from([(
                wire_identity.clone(),
                BTreeSet::from([wire_version.clone()]),
            )]),
            ..ReferenceSupport::default()
        };
        SharedReferenceReader::new(support)
            .read(
                &serde_json::to_vec(reference).expect("reference JSON"),
                &ReferenceExpectation::Artifact(ArtifactExpectation {
                    ref_version: value["refVersion"].as_str().expect("refVersion").into(),
                    kind: value["kind"].as_str().expect("kind").into(),
                    authority: value["authority"].as_str().expect("authority").into(),
                    identity: value["identity"].as_str().expect("identity").into(),
                    revision_namespace: value["revision"]["namespace"]
                        .as_str()
                        .expect("revision namespace")
                        .into(),
                    revision_value: value["revision"]["value"]
                        .as_str()
                        .expect("revision value")
                        .into(),
                    digest: value["digest"].as_str().expect("digest").into(),
                    wire_identity,
                    wire_version,
                    bytes: Some(bytes.to_vec()),
                }),
            )
            .expect("exact retained artifact reference")
    }

    fn contribution_reference<'a>(core: &'a Value, field: &str, identity: &str) -> &'a Value {
        core["results"]
            .as_array()
            .expect("results")
            .iter()
            .flat_map(|result| {
                result["oracleContributions"]
                    .as_array()
                    .expect("oracle contributions")
            })
            .map(|contribution| &contribution[field])
            .find(|reference| {
                reference.pointer("/value/identity").and_then(Value::as_str) == Some(identity)
            })
            .expect("retained contribution reference")
    }

    #[trace("TC-037", "FR-021-AC-1", "FR-021-AC-2", "FR-021-AC-5")]
    #[test]
    fn tc_037_ratchets_the_committed_vp04_core_and_recomputes_its_assessment() {
        let retained = parse_strict_bounded_json(include_bytes!(
            "../evidence/vp04-parser-portfolio-core.json"
        ))
        .expect("strict committed VP04 report");
        validate_schema_definition("Vp04PortfolioReport", &retained)
            .expect("committed report schema");
        assert_eq!(retained["status"], "incomplete");
        assert!(retained["disposition"].is_null());
        let core = &retained["core"];
        validate_schema_definition("Vp04AutomatedReportCore", core).expect("committed core schema");
        let digest = jcs_sha256(core).expect("committed core JCS digest");
        assert_eq!(digest, CORE_DIGEST);
        assert_eq!(retained["coreDigest"], CORE_DIGEST);

        for (field, bytes) in [
            (
                "subjectRef",
                include_bytes!("../fixtures/vp04/parent.native").as_slice(),
            ),
            (
                "mutationRef",
                include_bytes!("../fixtures/vp04/parent-invalid.native").as_slice(),
            ),
            (
                "dependencyClosureRef",
                include_bytes!("../fixtures/vp04/quire-spec-Cargo.lock").as_slice(),
            ),
            (
                "environmentRef",
                b"linux-x86_64;kernel=6.18.33.2-microsoft-standard-WSL2;rustc=1.94.1".as_slice(),
            ),
        ] {
            validate_artifact(&core[field], bytes);
        }
        assert_eq!(core["candidateRef"]["value"]["digest"], CANDIDATE_DIGEST);
        assert_eq!(
            core["candidateRef"]["value"]["revision"]["value"],
            "2be73504edff1224d6396da15edca3775905d37b"
        );

        let procedure = core["plan"]["inventory"]
            .as_array()
            .expect("inventory")
            .iter()
            .find(|property| property["propertyId"] == "VP04-DISPOSITION")
            .expect("manual property")
            .pointer("/input/reviewProcedureRef")
            .expect("review procedure");
        let validated_procedure = validate_artifact(
            procedure,
            include_bytes!("../evidence/vp04-review-procedure.md"),
        );
        validate_selected_reference(procedure, &validated_procedure, "review-procedure")
            .expect("selected review procedure binding");

        let validated_contributions = [
            (
                "oracleRef",
                "oracle:quire-spec-parser-protocol",
                b"quire-spec exact parse, format, and syntax-refusal protocol".as_slice(),
            ),
            (
                "observationRef",
                "observation:vp04-direct",
                b"valid source parsed with exact identity, digest, and counts".as_slice(),
            ),
            (
                "observationRef",
                "observation:vp04-roundtrip",
                b"first and second formatted byte strings are equal".as_slice(),
            ),
            (
                "observationRef",
                "observation:vp04-adverse",
                b"declared syntax mutation produced the exact located refusal".as_slice(),
            ),
        ]
        .into_iter()
        .map(|(field, identity, bytes)| {
            validate_artifact(contribution_reference(core, field, identity), bytes)
        })
        .collect::<Vec<_>>();

        validate_measurement_bindings(core).expect("measurement reference bindings");
        validate_reproduction_bindings(core).expect("reproduction identity bindings");
        validate_core_bindings(core, procedure).expect("plan, task, result, and run bindings");
        validate_core_assessment(core, &validated_contributions)
            .expect("assessment recomputes from retained evidence");
        validate_measurement_series(
            &core["measurements"],
            CANDIDATE_DIGEST,
            core["environmentRef"]["value"]["digest"]
                .as_str()
                .expect("environment digest"),
        )
        .expect("measurement series recomputes");
    }
}
