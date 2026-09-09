// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Real-candidate VP04 qualification integration.

#![cfg(target_os = "linux")]

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use ix_trace_rs::trace;
use quire_verification::contracts::{VerificationErrorCode, contract_version};
use quire_verification::evidence::assess_evidence;
use quire_verification::measurement::{
    measurement_observation, record_measurement_series, validate_measurement_series,
};
use quire_verification::native_process::{NativeProcessExecutor, NativeProcessRequest};
use quire_verification::parser_portfolio::{
    ParserDiagnostic, ParserPortfolioRequest, ParserSource, collect_parser_portfolio_measurements,
};
use quire_verification::planner::{build_verification_plan, verification_task_id};
use quire_verification::report::{DispositionInput, assemble_portfolio_report};
use quire_verification::shared_reference::{
    ArtifactExpectation, ReferenceExpectation, ReferenceSupport, SharedReferenceReader,
    ValidatedSharedReference, parse_strict_bounded_json,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const CANDIDATE_REVISION: &str = "2be73504edff1224d6396da15edca3775905d37b";
const CANDIDATE_DIGEST: &str =
    "sha256:c6bd75613f85dfd38aaf37169aed0a3d7409466d4b2f34dcc5397896c46579e5";
const FIXTURE_DIGEST: &str =
    "sha256:bb7a187662ee136b70c27a82204ecd41b446dcac56fbc183d32bc94b8ff28018";
const MUTATION_DIGEST: &str =
    "sha256:2f6c5dde78cb3c1c666ebb21a0c993da26b8ac6dfd762c83ef63203d9b5085fa";
const LOCK_DIGEST: &str = "sha256:26c8dc235777ff79a5882e01a6c700d2feab9fe4d9f4397662490aa37fb9d444";
const PRODUCED_AT: &str = "2026-09-09T20:00:00Z";
const REVIEWER: &str = "agent-ix-verification-owner";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn artifact_reference(kind: &str, identity: &str, wire: &str, bytes: &[u8]) -> Value {
    let revision = digest(bytes);
    artifact_reference_at(
        kind,
        identity,
        wire,
        "sha256",
        revision.strip_prefix("sha256:").expect("digest prefix"),
        bytes,
    )
}

fn artifact_reference_at(
    kind: &str,
    identity: &str,
    wire: &str,
    revision_namespace: &str,
    revision_value: &str,
    bytes: &[u8],
) -> Value {
    json!({
        "wireVersion": "ix.shared-reference/2-draft",
        "kind": "artifact",
        "value": {
            "refVersion": "ix.artifact-ref/3-draft",
            "kind": kind,
            "authority": "github.com/agent-ix/quire-verification",
            "identity": identity,
            "revision": {"namespace": revision_namespace, "value": revision_value},
            "digest": digest(bytes),
            "wire": {"identity": wire, "version": "1"}
        }
    })
}

fn validate_references<'a>(
    references: impl IntoIterator<Item = (&'a Value, &'a [u8])>,
) -> Vec<quire_verification::shared_reference::ValidatedSharedReference> {
    let references = references.into_iter().collect::<Vec<_>>();
    let mut support = ReferenceSupport::default();
    for (reference, _) in &references {
        support
            .wire_versions
            .entry(
                reference["value"]["wire"]["identity"]
                    .as_str()
                    .expect("wire identity")
                    .into(),
            )
            .or_default()
            .insert(
                reference["value"]["wire"]["version"]
                    .as_str()
                    .expect("wire version")
                    .into(),
            );
    }
    let mut reader = SharedReferenceReader::new(support);
    references
        .into_iter()
        .map(|(reference, bytes)| {
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
                wire_identity: value["wire"]["identity"]
                    .as_str()
                    .expect("wire identity")
                    .into(),
                wire_version: value["wire"]["version"]
                    .as_str()
                    .expect("wire version")
                    .into(),
                bytes: Some(bytes.to_vec()),
            };
            reader
                .read(
                    &serde_json::to_vec(reference).expect("reference JSON"),
                    &ReferenceExpectation::Artifact(expectation),
                )
                .expect("strict exact reference")
        })
        .collect()
}

fn measurement_context() -> Value {
    json!({
        "population": "16 VP04 slots for quire-spec at the selected revision and fixture pair",
        "exclusions": ["build and download time", "other machines", "other fixtures", "human review latency"],
        "clock": "monotonic-process-local",
        "commandOrder": ["setup", "direct-parse", "parse-format-reparse", "complete-run"],
        "cacheTreatment": "five sequential repetitions with naturally warmed filesystem cache",
        "environmentalDifferences": [],
        "uncertainty": "process scheduling, filesystem cache, clock granularity, build residue, and thermal state",
        "alternativeExplanations": ["process scheduling", "filesystem cache state", "thermal state"]
    })
}

struct QualificationInputs {
    executor: NativeProcessExecutor,
    setup: Value,
    candidate_ref: Value,
    subject_ref: Value,
    mutation_ref: Value,
    closure_ref: Value,
    environment_ref: Value,
    procedure_ref: Value,
    validated_procedure: ValidatedSharedReference,
    request: ParserPortfolioRequest,
    result_references: Vec<ValidatedSharedReference>,
}

struct PortfolioReferences {
    subject: Value,
    mutation: Value,
    snapshot: Value,
    procedure: Value,
    run: Value,
    oracle: Value,
    direct_observation: Value,
    roundtrip_observation: Value,
    adverse_observation: Value,
}

struct QualificationAssets {
    candidate: Vec<u8>,
    valid: Vec<u8>,
    mutation: Vec<u8>,
    lock: Vec<u8>,
    procedure: Vec<u8>,
}

struct ReferenceBundle {
    candidate: Value,
    subject: Value,
    mutation: Value,
    closure: Value,
    environment: Value,
    procedure: Value,
    portfolio: PortfolioReferences,
    validated: Vec<ValidatedSharedReference>,
}

fn load_assets(executable: &PathBuf) -> QualificationAssets {
    let candidate_bytes = fs::read(executable).expect("qualified parser binary");
    let valid_bytes = fs::read(root().join("fixtures/vp04/parent.native")).expect("valid fixture");
    let mutation_bytes =
        fs::read(root().join("fixtures/vp04/parent-invalid.native")).expect("mutation fixture");
    let lock_bytes =
        fs::read(root().join("fixtures/vp04/quire-spec-Cargo.lock")).expect("dependency closure");
    assert_eq!(digest(&candidate_bytes), CANDIDATE_DIGEST);
    assert_eq!(digest(&valid_bytes), FIXTURE_DIGEST);
    assert_eq!(digest(&mutation_bytes), MUTATION_DIGEST);
    assert_eq!(digest(&lock_bytes), LOCK_DIGEST);
    let procedure_bytes =
        fs::read(root().join("evidence/vp04-review-procedure.md")).expect("review procedure");
    QualificationAssets {
        candidate: candidate_bytes,
        valid: valid_bytes,
        mutation: mutation_bytes,
        lock: lock_bytes,
        procedure: procedure_bytes,
    }
}

fn build_references(assets: &QualificationAssets) -> ReferenceBundle {
    let environment = b"linux-x86_64;kernel=6.18.33.2-microsoft-standard-WSL2;rustc=1.94.1";
    let candidate_ref = artifact_reference_at(
        "executable-projection",
        "quire-spec-language:quire-spec",
        "application/vnd.agent-ix.rust-elf",
        "git",
        CANDIDATE_REVISION,
        &assets.candidate,
    );
    let subject_ref = artifact_reference_at(
        "source",
        "quire-spec:parent.native",
        "text/plain",
        "git",
        CANDIDATE_REVISION,
        &assets.valid,
    );
    let mutation_ref = artifact_reference_at(
        "source",
        "quire-spec:parent-invalid.native",
        "text/plain",
        "vp04-mutation-of-git",
        CANDIDATE_REVISION,
        &assets.mutation,
    );
    let closure_ref = artifact_reference_at(
        "dependency-closure",
        "quire-spec-language:Cargo.lock",
        "application/toml",
        "git",
        CANDIDATE_REVISION,
        &assets.lock,
    );
    let environment_ref = artifact_reference(
        "environment",
        "vp04:linux-x86_64",
        "text/plain",
        environment,
    );
    let procedure_ref = artifact_reference(
        "review-procedure",
        "procedure:vp04-verification-owner",
        "text/markdown",
        &assets.procedure,
    );
    let references =
        portfolio_references(&subject_ref, &mutation_ref, &procedure_ref, &assets.valid);
    let validated = validate_qualification_references(
        assets,
        [
            &candidate_ref,
            &subject_ref,
            &mutation_ref,
            &closure_ref,
            &environment_ref,
            &procedure_ref,
        ],
        &references,
    );
    ReferenceBundle {
        candidate: candidate_ref,
        subject: subject_ref,
        mutation: mutation_ref,
        closure: closure_ref,
        environment: environment_ref,
        procedure: procedure_ref,
        portfolio: references,
        validated,
    }
}

fn portfolio_references(
    subject: &Value,
    mutation: &Value,
    procedure: &Value,
    valid_bytes: &[u8],
) -> PortfolioReferences {
    let run_bytes = b"VP04 real parser run";
    let oracle_bytes = b"quire-spec exact parse, format, and syntax-refusal protocol";
    let direct_bytes = b"valid source parsed with exact identity, digest, and counts";
    let roundtrip_bytes = b"first and second formatted byte strings are equal";
    let adverse_bytes = b"declared syntax mutation produced the exact located refusal";
    PortfolioReferences {
        subject: subject.clone(),
        mutation: mutation.clone(),
        snapshot: artifact_reference(
            "snapshot",
            "snapshot:parent.native",
            "text/plain",
            valid_bytes,
        ),
        procedure: procedure.clone(),
        run: artifact_reference(
            "run-artifact",
            "run:vp04-real-parser",
            "application/json",
            run_bytes,
        ),
        oracle: artifact_reference(
            "oracle",
            "oracle:quire-spec-parser-protocol",
            "text/plain",
            oracle_bytes,
        ),
        direct_observation: artifact_reference(
            "observation",
            "observation:vp04-direct",
            "text/plain",
            direct_bytes,
        ),
        roundtrip_observation: artifact_reference(
            "observation",
            "observation:vp04-roundtrip",
            "text/plain",
            roundtrip_bytes,
        ),
        adverse_observation: artifact_reference(
            "observation",
            "observation:vp04-adverse",
            "text/plain",
            adverse_bytes,
        ),
    }
}

fn validate_qualification_references(
    assets: &QualificationAssets,
    core: [&Value; 6],
    references: &PortfolioReferences,
) -> Vec<ValidatedSharedReference> {
    let environment_bytes = b"linux-x86_64;kernel=6.18.33.2-microsoft-standard-WSL2;rustc=1.94.1";
    let run_bytes = b"VP04 real parser run";
    let oracle_bytes = b"quire-spec exact parse, format, and syntax-refusal protocol";
    let direct_bytes = b"valid source parsed with exact identity, digest, and counts";
    let roundtrip_bytes = b"first and second formatted byte strings are equal";
    let adverse_bytes = b"declared syntax mutation produced the exact located refusal";
    validate_references([
        (core[0], assets.candidate.as_slice()),
        (core[1], assets.valid.as_slice()),
        (core[2], assets.mutation.as_slice()),
        (core[3], assets.lock.as_slice()),
        (core[4], environment_bytes.as_slice()),
        (core[5], assets.procedure.as_slice()),
        (&references.snapshot, assets.valid.as_slice()),
        (&references.run, run_bytes.as_slice()),
        (&references.oracle, oracle_bytes.as_slice()),
        (&references.direct_observation, direct_bytes.as_slice()),
        (
            &references.roundtrip_observation,
            roundtrip_bytes.as_slice(),
        ),
        (&references.adverse_observation, adverse_bytes.as_slice()),
    ])
}

fn prepare_inputs() -> QualificationInputs {
    let started = Instant::now();
    let executable = std::env::var_os("VP04_PARSER").map_or_else(
        || PathBuf::from("/tmp/quire-spec-language-agent-b-target/release/quire-spec"),
        PathBuf::from,
    );
    let assets = load_assets(&executable);
    let references = build_references(&assets);
    let validated_procedure = references.validated[5].clone();
    let request = parser_request(&references.portfolio, &references.validated);
    let executor =
        NativeProcessExecutor::admit(executable.parent().expect("candidate root"), &executable)
            .expect("admitted parser");
    let setup = measurement_observation(
        "setup-0",
        "setup",
        0,
        &started.elapsed().as_nanos().to_string(),
        None,
    )
    .expect("setup measurement");
    QualificationInputs {
        executor,
        setup,
        candidate_ref: references.candidate,
        subject_ref: references.subject,
        mutation_ref: references.mutation,
        closure_ref: references.closure,
        environment_ref: references.environment,
        procedure_ref: references.procedure,
        validated_procedure,
        request,
        result_references: references.validated,
    }
}

fn parser_request(
    references: &PortfolioReferences,
    validated: &[ValidatedSharedReference],
) -> ParserPortfolioRequest {
    let plan = parser_plan(references, validated);
    let mutation_path = root().join("fixtures/vp04/parent-invalid.native");
    let direct_context = result_context(
        &plan,
        "VP04-DIRECT",
        "unit-testing",
        "VP04-REAL-DIRECT",
        &references.oracle,
        &references.direct_observation,
    );
    let roundtrip_context = result_context(
        &plan,
        "VP04-ROUNDTRIP",
        "metamorphic-testing",
        "VP04-REAL-ROUNDTRIP",
        &references.oracle,
        &references.roundtrip_observation,
    );
    let adverse_context = result_context(
        &plan,
        "VP04-DIRECT",
        "unit-testing",
        "VP04-REAL-ADVERSE",
        &references.oracle,
        &references.adverse_observation,
    );
    let mut manual_context = result_context(
        &plan,
        "VP04-DISPOSITION",
        "inspection",
        "VP04-REAL-DISPOSITION-PENDING",
        &references.oracle,
        &references.direct_observation,
    );
    manual_context["oracleContributions"] = json!([]);
    manual_context["adequacy"] = json!({"status": "not-assessed"});
    ParserPortfolioRequest {
        execution_id: "VP04-REAL-EXECUTION".into(),
        candidate_digest: CANDIDATE_DIGEST.into(),
        produced_at: PRODUCED_AT.into(),
        valid_source: ParserSource {
            path: root().join("fixtures/vp04/parent.native"),
            identity: "test:parent".into(),
            revision: "fixture:1".into(),
            digest: FIXTURE_DIGEST.into(),
            clauses: 4,
            imports: 1,
        },
        mutation: ParserDiagnostic {
            identity: "test:parent".into(),
            revision: "fixture:invalid".into(),
            path: mutation_path.clone(),
            digest: MUTATION_DIGEST.into(),
            expected: json!({
                "status": "refused", "phase": "parse", "code": "invalid_syntax",
                "source": {"identity": "test:parent", "revision": "fixture:invalid"},
                "path": mutation_path.to_string_lossy(),
                "span": {
                    "start": {"byte": 249, "line": 6, "column": 1},
                    "end": {"byte": 258, "line": 6, "column": 10}
                },
                "message": "expected invariant, pre or post clause"
            }),
        },
        formatted_revision: "fixture:formatted".into(),
        environment: BTreeMap::new(),
        deadline_seconds: 2.0,
        plan,
        direct_context,
        roundtrip_context,
        adverse_context,
        manual_context,
    }
}

fn parser_plan(references: &PortfolioReferences, validated: &[ValidatedSharedReference]) -> Value {
    build_verification_plan(&parser_plan_request(references), validated)
        .expect("parser-specific portable plan")
}

fn parser_policy(references: &PortfolioReferences) -> Value {
    json!({
        "contractVersion": contract_version(), "kind": "SelectionPolicy",
        "policyId": "POLICY-VP04-QUIRE-SPEC", "version": "1",
        "validUntil": "2026-10-09T20:00:00Z", "sourceRef": references.subject,
        "provenance": {"kind": "project-authored", "standardDerived": false, "rights": "AGPL-3.0-or-later"},
        "rules": [
            {"ruleId": "exact-direct", "priority": "required", "explanation": "Require the exact successful parser protocol."},
            {"ruleId": "exact-roundtrip", "priority": "required", "explanation": "Require byte-stable parse-format-reparse behavior."},
            {"ruleId": "human-disposition", "priority": "required", "explanation": "Leave qualification to the external verification owner."}
        ]
    })
}

fn parser_capability(
    id: &str,
    technique: &str,
    input_kind: &str,
    profile: &str,
    manual: bool,
) -> Value {
    let mut capability = json!({
        "contractVersion": contract_version(), "kind": "ToolCapability",
        "capabilityId": id, "techniqueId": technique, "qualification": "qualified",
        "inputKinds": [input_kind], "profiles": [profile],
        "prerequisites": if manual {
            json!([{"kind": "reviewer", "id": REVIEWER, "required": true}])
        } else {
            json!([
                {"kind": "runner", "id": "native-rust", "required": true},
                {"kind": "binding", "id": "quire-spec-cli", "required": true},
                {"kind": "tool", "id": "quire-spec", "required": true},
                {"kind": "resource", "id": "vp04-fixture-pair", "required": true}
            ])
        },
        "runner": {"kind": if manual { "manual" } else { "native-producer" }, "adapterId": "native-rust", "adapterVersion": contract_version()},
        "environmentPermissions": ["read-qualified-fixture"],
        "nativeArtifactInterface": {"mode": "reference-only", "mediaTypes": ["application/json", "text/plain"]},
        "limits": {"executionClaim": "selected quire-spec syntax commands and exact fixtures only"}
    });
    if !manual {
        capability["resourceLimits"] = json!({"maxWallClockSeconds": 10, "maxParallelism": 1});
    }
    capability
}

fn parser_plan_request(references: &PortfolioReferences) -> Value {
    let plan_id = "PLAN-VP04-QUIRE-SPEC";
    let direct_task = verification_task_id("VP04-DIRECT", "unit-testing");
    let roundtrip_task = verification_task_id("VP04-ROUNDTRIP", "metamorphic-testing");
    let manual_task = verification_task_id("VP04-DISPOSITION", "inspection");
    let binding = |task_id: &str| {
        json!({
            "planId": plan_id, "taskId": task_id, "runRef": references.run,
            "producer": "quire-spec-language", "producerVersion": CANDIDATE_REVISION,
            "adapter": "native-rust", "adapterVersion": contract_version()
        })
    };
    let property = |id: &str,
                    name: &str,
                    task_id: &str,
                    technique_id: &str,
                    profile: &str,
                    input: Value| {
        json!({
            "propertyId": id, "techniqueId": technique_id,
            "propertyRef": artifact_reference("property", name, "text/plain", name.as_bytes()),
            "subjectRefs": [references.subject, references.mutation],
            "profile": profile,
            "bindingRefs": [artifact_reference("binding", &format!("binding:{id}"), "text/plain", id.as_bytes())],
            "dependencyClosure": LOCK_DIGEST,
            "executionScope": {"repository": "agent-ix/quire-spec-language", "revision": CANDIDATE_REVISION, "paths": ["src/main.rs", "src/parser.rs", "src/format.rs"]},
            "freshness": {"maxAgeSeconds": 2_592_000},
            "resultBinding": binding(task_id),
            "input": input
        })
    };
    let properties = vec![
        property(
            "VP04-DIRECT",
            "property:vp04-direct-parse",
            &direct_task,
            "unit-testing",
            "quire-native-parser/v1",
            json!({"kind": "state", "snapshotRef": references.snapshot, "property": "exact successful parse response"}),
        ),
        property(
            "VP04-ROUNDTRIP",
            "property:vp04-format-stability",
            &roundtrip_task,
            "metamorphic-testing",
            "quire-native-roundtrip/v1",
            json!({"kind": "relational", "leftSubject": "source:valid", "rightSubject": "source:formatted", "relation": "first and second format outputs are byte-equal"}),
        ),
        property(
            "VP04-DISPOSITION",
            "property:vp04-human-disposition",
            &manual_task,
            "inspection",
            "quire-native-review/v1",
            json!({"kind": "manual", "reviewProcedureRef": references.procedure, "requiredDisposition": ["accept", "revise", "reject"]}),
        ),
    ];
    json!({
        "planId": plan_id, "asOf": PRODUCED_AT,
        "policy": parser_policy(references),
        "properties": properties,
        "capabilities": [
            parser_capability("CAP-VP04-DIRECT", "unit-testing", "state", "quire-native-parser/v1", false),
            parser_capability("CAP-VP04-ROUNDTRIP", "metamorphic-testing", "relational", "quire-native-roundtrip/v1", false),
            parser_capability("CAP-VP04-DISPOSITION", "inspection", "manual", "quire-native-review/v1", true)
        ],
        "selections": [
            {"propertyId": "VP04-DIRECT", "techniqueId": "unit-testing", "applicability": "applicable", "priority": "required", "reasons": [{"rule": "exact-direct", "value": "reviewed parser CLI"}]},
            {"propertyId": "VP04-ROUNDTRIP", "techniqueId": "metamorphic-testing", "applicability": "applicable", "priority": "required", "reasons": [{"rule": "exact-roundtrip", "value": "reviewed parser CLI"}]},
            {"propertyId": "VP04-DISPOSITION", "techniqueId": "inspection", "applicability": "applicable", "priority": "required", "reasons": [{"rule": "human-disposition", "value": "external owner"}]}
        ],
        "availablePrerequisites": [
            {"kind": "runner", "id": "native-rust"},
            {"kind": "binding", "id": "quire-spec-cli"},
            {"kind": "tool", "id": "quire-spec"},
            {"kind": "resource", "id": "vp04-fixture-pair"}
        ],
        "budget": {"wallClockSeconds": 10, "parallelism": 1}
    })
}

fn result_context(
    plan: &Value,
    property_id: &str,
    technique_id: &str,
    result_id: &str,
    oracle_ref: &Value,
    observation_ref: &Value,
) -> Value {
    let property = plan["inventory"]
        .as_array()
        .expect("inventory")
        .iter()
        .find(|property| property["propertyId"] == property_id)
        .expect("selected property");
    let binding = &property["resultBinding"];
    json!({
        "resultId": result_id, "planId": binding["planId"], "taskId": binding["taskId"],
        "runRef": binding["runRef"], "techniqueId": technique_id,
        "propertyRef": property["propertyRef"], "subjectRefs": property["subjectRefs"],
        "profile": property["profile"], "bindingRefs": property["bindingRefs"],
        "dependencyClosure": property["dependencyClosure"], "executionScope": property["executionScope"],
        "validUntil": "2026-10-09T20:00:00Z",
        "assumptions": [], "adequacy": {"status": "adequate"},
        "provenance": {
            "producer": binding["producer"], "producerVersion": binding["producerVersion"],
            "adapter": binding["adapter"], "adapterVersion": binding["adapterVersion"],
            "sourceRevision": CANDIDATE_REVISION
        },
        "oracleContributions": [{
            "contributionId": format!("contribution:{result_id}"),
            "oracleRef": oracle_ref, "observationRef": observation_ref,
            "relation": "shared", "dependenceGroupId": "quire-spec-parser-candidate"
        }]
    })
}

fn report_core(
    inputs: &QualificationInputs,
    request: &ParserPortfolioRequest,
    measured: &quire_verification::parser_portfolio::MeasuredParserPortfolio,
) -> Value {
    let task_ids = request.plan["tasks"]
        .as_array()
        .expect("plan tasks")
        .iter()
        .map(|task| task["taskId"].clone())
        .collect::<Vec<_>>();
    json!({
        "schemaVersion": "quire-verification.vp04-report-core/v1",
        "reportId": "VP04-QUIRE-SPEC-LC01",
        "candidateRef": inputs.candidate_ref,
        "subjectRef": inputs.subject_ref,
        "mutationRef": inputs.mutation_ref,
        "dependencyClosureRef": inputs.closure_ref,
        "environmentRef": inputs.environment_ref,
        "commands": measured.execution["commands"],
        "plan": measured.execution["plan"],
        "taskIds": task_ids,
        "nativeRuns": measured.execution["nativeRuns"],
        "results": measured.execution["results"],
        "assessment": measured.execution["assessment"],
        "assessedAt": PRODUCED_AT,
        "measurements": measured.measurements,
        "interpretation": "These observations describe only the selected quire-spec revision and exact fixture pair.",
        "claimLimits": [
            "The run establishes only the declared parser syntax responses and format stability for these fixtures.",
            "Quoin retains ownership of evidence interpretation and sufficiency."
        ],
        "producers": ["quire-spec-language@2be73504edff1224d6396da15edca3775905d37b", "quire-verification"],
        "adapters": ["native-rust"],
        "evidenceOwner": "Quoin",
        "sufficiencyDecision": "not-made"
    })
}

fn assert_plan_and_failure_scenarios(inputs: &QualificationInputs) {
    let tasks = inputs.request.plan["tasks"].as_array().expect("plan tasks");
    assert_eq!(
        tasks
            .iter()
            .filter(|task| task["readiness"] == "ready" && task["invocationAllowed"] == true)
            .count(),
        2
    );
    let manual = tasks
        .iter()
        .find(|task| task.pointer("/input/kind") == Some(&json!("manual")))
        .expect("separate manual task");
    assert_eq!(manual["readiness"], "blocked");
    assert_eq!(manual["invocationAllowed"], false);
    assert_eq!(manual["input"]["reviewProcedureRef"], inputs.procedure_ref);
    assert_eq!(
        manual["input"]["requiredDisposition"],
        json!(["accept", "revise", "reject"])
    );

    let invalid_status = inputs
        .executor
        .execute(&NativeProcessRequest {
            arguments: vec!["parse".into()],
            environment: BTreeMap::new(),
            deadline_seconds: Some(1.0),
            expected_exit_code: None,
        })
        .expect_err("missing status must refuse before launch");
    assert_eq!(
        invalid_status.code(),
        VerificationErrorCode::InvalidExpectedExitStatus
    );

    let base = parser_plan_request(&build_portfolio_references_for_plan(inputs));
    for (kind, id) in [
        ("runner", "native-rust"),
        ("binding", "quire-spec-cli"),
        ("tool", "quire-spec"),
        ("resource", "vp04-fixture-pair"),
    ] {
        let mut missing = base.clone();
        missing["availablePrerequisites"]
            .as_array_mut()
            .expect("available prerequisites")
            .retain(|item| item["kind"] != kind || item["id"] != id);
        let plan = build_verification_plan(&missing, &inputs.result_references)
            .expect("blocked plan remains representable");
        let affected = plan["tasks"]
            .as_array()
            .expect("blocked tasks")
            .iter()
            .filter(|task| task["kind"] == "verification" && task["techniqueId"] != "inspection")
            .collect::<Vec<_>>();
        assert_eq!(affected.len(), 2);
        assert!(affected.iter().all(|task| {
            task["readiness"] == "blocked"
                && task["invocationAllowed"] == false
                && task["runnerRequest"].is_null()
                && task["missingPrerequisites"]
                    .as_array()
                    .is_some_and(|items| {
                        items
                            .iter()
                            .any(|item| item["kind"] == kind && item["id"] == id)
                    })
        }));
    }
}

fn build_portfolio_references_for_plan(inputs: &QualificationInputs) -> PortfolioReferences {
    let valid = fs::read(root().join("fixtures/vp04/parent.native")).expect("valid fixture");
    portfolio_references(
        &inputs.subject_ref,
        &inputs.mutation_ref,
        &inputs.procedure_ref,
        &valid,
    )
}

fn assert_freshness_and_correlation_scenarios(
    measured: &quire_verification::parser_portfolio::MeasuredParserPortfolio,
    validated: &[ValidatedSharedReference],
) {
    let inventory = &measured.execution["plan"]["inventory"];
    let results = &measured.execution["results"];
    let exact = assess_evidence(
        &json!({"asOf": "2026-10-09T20:00:00Z", "inventory": inventory, "results": results}),
        validated,
    )
    .expect("fresh at exact expiry");
    assert!(
        exact["obligations"]
            .as_array()
            .expect("exact-expiry obligations")
            .iter()
            .all(|item| item["status"] != "stale")
    );

    let stale = assess_evidence(
        &json!({"asOf": "2026-10-09T20:00:00.000000001Z", "inventory": inventory, "results": results}),
        validated,
    )
    .expect("stale after exact expiry");
    assert!(
        stale["obligations"]
            .as_array()
            .expect("stale obligations")
            .iter()
            .any(|item| item["status"] == "stale")
    );
    assert!(
        !stale["oracleDependenceFindings"]
            .as_array()
            .expect("independent findings")
            .is_empty()
    );

    let mut unknown_results = results.clone();
    unknown_results[1]["oracleContributions"][0]["relation"] = json!("unknown");
    unknown_results[1]["oracleContributions"][0]
        .as_object_mut()
        .expect("oracle contribution")
        .remove("dependenceGroupId");
    let unknown = assess_evidence(
        &json!({"asOf": PRODUCED_AT, "inventory": inventory, "results": unknown_results}),
        validated,
    )
    .expect("unknown relationship remains representable");
    assert!(
        unknown["oracleDependenceFindings"]
            .as_array()
            .expect("unknown findings")
            .iter()
            .any(|finding| finding["kind"] == "oracle-dependence-unknown")
    );
    assert_eq!(unknown["sufficiencyDecision"], "not-made");
}

fn assert_invalid_measurement_scenario(
    measured: &quire_verification::parser_portfolio::MeasuredParserPortfolio,
) {
    let mut observations = measured.measurements["observations"]
        .as_array()
        .expect("observations")
        .clone();
    observations[13] = measurement_observation("direct-5", "direct-parse", 5, "-1", None)
        .expect("negative duration is retained as invalid");
    let candidate = measured.measurements["candidateIdentity"]
        .as_str()
        .expect("candidate identity");
    let environment = measured.measurements["environmentIdentity"]
        .as_str()
        .expect("environment identity");
    let series = record_measurement_series(
        candidate,
        environment,
        &observations,
        &measured.measurements["context"],
    )
    .expect("invalid observation remains in the series");
    validate_measurement_series(&series, candidate, environment).expect("recomputed series");
    assert_eq!(series["observations"][13]["status"], "invalid");
    assert!(series["observations"][13]["elapsedNanoseconds"].is_null());
    assert_eq!(series["summaries"][1]["validSampleCount"], 4);
}

#[trace("IT-003")]
#[test]
#[ignore = "VP04 automated-core collection lane; requires the exact qualified Rust parser"]
fn vp04_collects_and_retains_automated_core() {
    let inputs = prepare_inputs();
    assert_plan_and_failure_scenarios(&inputs);
    let request = &inputs.request;
    let measured = collect_parser_portfolio_measurements(
        &inputs.executor,
        request,
        &inputs.result_references,
        inputs.setup.clone(),
        CANDIDATE_DIGEST,
        &digest(b"linux-x86_64;kernel=6.18.33.2-microsoft-standard-WSL2;rustc=1.94.1"),
        &measurement_context(),
    )
    .expect("measured real parser portfolio");
    assert!(
        measured.execution["results"]
            .as_array()
            .expect("results")
            .iter()
            .filter(|result| result["techniqueId"] != "inspection")
            .all(|result| result["execution"]["status"] == "completed")
    );
    assert_eq!(
        measured.execution["results"][0]["logical"]["outcome"],
        "satisfied"
    );
    assert_eq!(
        measured.execution["results"][1]["logical"]["outcome"],
        "satisfied"
    );
    assert_eq!(
        measured.execution["results"][2]["logical"]["outcome"],
        "violated"
    );
    assert_eq!(
        measured.execution["results"][3]["execution"]["status"],
        "not-run"
    );
    assert_eq!(
        measured.measurements["observations"]
            .as_array()
            .expect("observations")
            .len(),
        16
    );
    assert_freshness_and_correlation_scenarios(&measured, &inputs.result_references);
    assert_invalid_measurement_scenario(&measured);

    let core = report_core(&inputs, request, &measured);
    let report = assemble_portfolio_report(
        &core,
        &inputs.result_references,
        &inputs.procedure_ref,
        &inputs.validated_procedure,
        REVIEWER,
        &[],
    )
    .expect("sealed incomplete report");
    assert_eq!(report["status"], "incomplete");
    let output = std::env::var_os("VP04_CORE_OUTPUT").map_or_else(
        || root().join("evidence/vp04-parser-portfolio-core.json"),
        PathBuf::from,
    );
    fs::write(
        output,
        serde_json::to_vec_pretty(&report).expect("report JSON"),
    )
    .expect("retain VP04 automated core");
}

#[trace("IT-003")]
#[test]
#[ignore = "VP04 disposition-finalization lane; requires retained core and external disposition"]
fn vp04_binds_external_disposition_to_retained_core() {
    let executable = std::env::var_os("VP04_PARSER").map_or_else(
        || PathBuf::from("/tmp/quire-spec-language-agent-b-target/release/quire-spec"),
        PathBuf::from,
    );
    let assets = load_assets(&executable);
    let references = build_references(&assets);
    let core_path = std::env::var_os("VP04_CORE_INPUT").map_or_else(
        || root().join("evidence/vp04-parser-portfolio-core.json"),
        PathBuf::from,
    );
    let retained_bytes = fs::read(core_path).expect("retained automated-core report");
    let retained = parse_strict_bounded_json(&retained_bytes).expect("strict retained report");
    assert_eq!(retained["status"], "incomplete");
    let core = &retained["core"];

    let disposition_path = std::env::var_os("VP04_DISPOSITION_INPUT").map_or_else(
        || root().join("evidence/vp04-review-disposition.json"),
        PathBuf::from,
    );
    let disposition_bytes = fs::read(disposition_path).expect("external VP04 disposition");
    let disposition =
        parse_strict_bounded_json(&disposition_bytes).expect("strict external disposition payload");
    let disposition_id = disposition["dispositionId"]
        .as_str()
        .expect("disposition ID");
    let disposition_ref = artifact_reference(
        "review-disposition",
        &format!("disposition:{disposition_id}"),
        "application/json",
        &disposition_bytes,
    );
    let validated_disposition =
        validate_references([(&disposition_ref, disposition_bytes.as_slice())])
            .pop()
            .expect("validated disposition reference");
    let final_report = assemble_portfolio_report(
        core,
        &references.validated,
        &references.procedure,
        &references.validated[5],
        REVIEWER,
        &[DispositionInput {
            reference: &disposition_ref,
            validated_reference: &validated_disposition,
            bytes: &disposition_bytes,
        }],
    )
    .expect("final report with exact external disposition");
    assert_eq!(final_report["status"], "final");
    assert_eq!(final_report["coreDigest"], retained["coreDigest"]);
    assert_eq!(final_report["disposition"], disposition);
    assert_eq!(final_report["core"]["evidenceOwner"], "Quoin");
    assert_eq!(final_report["core"]["sufficiencyDecision"], "not-made");

    let output = std::env::var_os("VP04_EVIDENCE_OUTPUT").map_or_else(
        || root().join("evidence/vp04-parser-portfolio.json"),
        PathBuf::from,
    );
    fs::write(
        output,
        serde_json::to_vec_pretty(&final_report).expect("final report JSON"),
    )
    .expect("retain final VP04 report");
}
