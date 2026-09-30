// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Real downstream Engineering Assurance consumer evidence for TC-041 and TC-126.

#![cfg(target_os = "linux")]
#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use engineering_assurance::producer_execution::{
    ArgumentBinding, CancellationBinding, CancellationToken, ContainmentBinding, ContentDigest,
    ContractBinding, ExecutionBudget, ExecutionProcedure, InputBinding, OutputBinding,
    PRODUCER_EXECUTION_REQUEST_PROTOCOL, ProducerDescriptor, ProducerExecutionRequest,
    ProducerExecutionState, ProducerExecutor, StdinBinding,
};
use ix_trace_rs::trace;
use quire_verification::contracts::{VerificationErrorCode, jcs_canonicalize, jcs_sha256, wire};
use quire_verification::qualification_corpus::{
    FrozenQualificationCorpus, QualificationCorpusOutcome, qualify_corpus_manifest,
};
use quire_verification::qualification_oracle::{
    FrozenQualificationOracleManifest, QualificationOracleOutcome, qualify_oracle_manifest,
};
use quire_verification::qualification_oracle_execution::{
    QualificationComparisonAssessment, QualificationExecutionEvidence,
    execute_qualification_comparison, execute_qualification_oracle,
    qualification_comparison_response_binding, qualification_oracle_response_binding,
    record_qualification_comparison_execution, record_qualification_disagreement,
    record_qualification_oracle_execution,
};
use quire_verification::qualification_profile::{
    FrozenQualificationProfile, QualificationProfileOutcome, freeze_qualification_profile,
};
use quire_verification::shared_reference::{
    ArtifactExpectation, ReferenceExpectation, ReferenceSupport, SharedReferenceReader,
    ValidatedSharedReference,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tempfile::TempDir;

const FIXTURE_REVISION: &str = "qualification-oracle-fixture-r1";

#[derive(Clone, Copy)]
enum Operation {
    Oracle,
    Comparison,
}

#[derive(Clone, Copy)]
enum ProducerMode {
    Completed,
    Unavailable,
    Refused,
    ExecutableIdentityRefused,
    Failed,
    TimedOut,
    Malformed,
    ContainmentFailure,
    Cancelled,
}

struct Scenario {
    _root: TempDir,
    profile: Box<FrozenQualificationProfile>,
    corpus: Box<FrozenQualificationCorpus>,
    manifest: Box<FrozenQualificationOracleManifest>,
    request: ProducerExecutionRequest,
    retained: BTreeMap<String, Vec<u8>>,
    response: Value,
}

fn digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn artifact_with_revision(kind: &str, identity: &str, revision: &str, bytes: &[u8]) -> Value {
    json!({
        "wireVersion": "ix.shared-reference/2-draft",
        "kind": "artifact",
        "value": {
            "refVersion": "ix.artifact-ref/3-draft",
            "kind": kind,
            "authority": "qualification.agent-ix.local",
            "identity": identity,
            "revision": {"namespace": "qualification-oracle", "value": revision},
            "digest": digest(bytes),
            "wire": {
                "identity": "quire-verification.qualification-oracle-fixture/v1",
                "version": "1"
            }
        }
    })
}

fn artifact(kind: &str, identity: &str, bytes: &[u8]) -> Value {
    artifact_with_revision(kind, identity, "r1", bytes)
}

fn reference(kind: &str, identity: &str) -> Value {
    artifact(kind, identity, identity.as_bytes())
}

fn collect_artifacts<'a>(value: &'a Value, artifacts: &mut Vec<&'a Value>) {
    match value {
        Value::Object(fields) => {
            if fields.get("kind").and_then(Value::as_str) == Some("artifact")
                && fields.get("wireVersion").is_some()
            {
                artifacts.push(value);
                return;
            }
            for child in fields.values() {
                collect_artifacts(child, artifacts);
            }
        }
        Value::Array(items) => {
            for child in items {
                collect_artifacts(child, artifacts);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

fn proofs(value: &Value, retained: &BTreeMap<String, Vec<u8>>) -> Vec<ValidatedSharedReference> {
    let mut envelopes = Vec::new();
    collect_artifacts(value, &mut envelopes);
    let mut reader = SharedReferenceReader::new(ReferenceSupport {
        wire_versions: BTreeMap::from([(
            "quire-verification.qualification-oracle-fixture/v1".to_owned(),
            BTreeSet::from(["1".to_owned()]),
        )]),
        ..ReferenceSupport::default()
    });
    envelopes
        .into_iter()
        .map(|envelope| {
            let value = &envelope["value"];
            let identity = value["identity"].as_str().expect("artifact identity");
            let bytes = retained
                .get(identity)
                .cloned()
                .unwrap_or_else(|| identity.as_bytes().to_vec());
            reader
                .read(
                    &serde_json::to_vec(envelope).expect("reference JSON"),
                    &ReferenceExpectation::Artifact(ArtifactExpectation {
                        ref_version: value["refVersion"].as_str().expect("refVersion").into(),
                        kind: value["kind"].as_str().expect("kind").into(),
                        authority: value["authority"].as_str().expect("authority").into(),
                        identity: identity.into(),
                        revision_namespace: value["revision"]["namespace"]
                            .as_str()
                            .expect("revision namespace")
                            .into(),
                        revision_value: value["revision"]["value"]
                            .as_str()
                            .expect("revision value")
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
                        bytes: Some(bytes),
                    }),
                )
                .expect("strict reference proof")
        })
        .collect()
}

fn interface_kind(role: &str) -> &'static str {
    match role {
        "parser-compiler" => "executable-projection",
        "production-caller" => "invocation",
        "portable-contracts" | "shared-references" | "engineering-assurance" | "quoin-evidence" => {
            "source"
        }
        _ => panic!("unsupported interface role"),
    }
}

fn profile_value() -> Value {
    let interface = |id: &str, role: &str| {
        let identity = format!("interface:{id}");
        json!({
            "interfaceId": id,
            "role": role,
            "owner": format!("owner:{id}"),
            "status": "accepted",
            "interfaceRef": artifact(interface_kind(role), &identity, identity.as_bytes()),
            "reason": null
        })
    };
    json!({
        "contractVersion": "quire-verification/e02-draft-2",
        "schemaVersion": "quire-verification.qualification-profile/v1",
        "kind": "QualificationProfile",
        "profileId": "QP-EA-CONSUMER-001",
        "profileRevision": "1",
        "candidateRef": reference("executable-projection", "candidate"),
        "dependencyClosureRef": reference("dependency-closure", "closure"),
        "productionCallerRef": reference("invocation", "caller"),
        "commands": [{
            "commandId": "parse",
            "executableRef": reference("executable-projection", "candidate"),
            "arguments": ["parse", "<corpus-item-path>"],
            "workingDirectoryRef": reference("environment", "workspace"),
            "inputBinding": "replace the corpus path",
            "expectedExitCodes": [0, 1],
            "responseKind": "quire-spec.syntax-cli/v1"
        }],
        "inputPopulation": {
            "populationId": "POP-EA-CONSUMER-001",
            "description": "Closed real-consumer population",
            "requiredClasses": ["malformed", "valid"],
            "selectionRationale": "Exercise exact retained producer inputs"
        },
        "semanticGuarantees": [
            {
                "guaranteeId": "syntax-outcome",
                "stage": "syntax",
                "claim": "Return the authored syntax outcome",
                "claimAuthority": {"kind": "non-formal", "reason": "Execution-boundary fixture"}
            },
            {
                "guaranteeId": "diagnostic-span",
                "stage": "syntax",
                "claim": "Return an exact diagnostic span",
                "claimAuthority": {"kind": "non-formal", "reason": "No span oracle selected"}
            }
        ],
        "environmentRef": reference("environment", "environment"),
        "resourceBudgets": [{
            "budgetId": "parse",
            "phase": "parse",
            "maximumWallClockNanoseconds": 20_000_000_000_u64,
            "maximumParallelism": 1,
            "maximumStdoutBytes": 4096,
            "maximumStderrBytes": 4096
        }],
        "evidenceRequirements": [
            {
                "obligationId": "syntax", "guaranteeId": "syntax-outcome",
                "techniqueId": "oracle-comparison", "budgetId": "parse",
                "stoppingRule": "every retained input once", "required": true
            },
            {
                "obligationId": "span", "guaranteeId": "diagnostic-span",
                "techniqueId": "oracle-comparison", "budgetId": "parse",
                "stoppingRule": "every malformed input once", "required": true
            }
        ],
        "exclusions": ["No guarantee outside the two named syntax claims"],
        "interfaces": [
            interface("e02", "portable-contracts"),
            interface("shared", "shared-references"),
            interface("compiler", "parser-compiler"),
            interface("caller", "production-caller"),
            interface("assurance", "engineering-assurance"),
            interface("evidence", "quoin-evidence")
        ]
    })
}

fn freeze_profile() -> (Box<FrozenQualificationProfile>, Value) {
    let value = profile_value();
    let profile =
        match freeze_qualification_profile(value.clone(), &proofs(&value, &BTreeMap::new()))
            .expect("profile decision")
        {
            QualificationProfileOutcome::Frozen(profile) => profile,
            QualificationProfileOutcome::Blocked(blockers) => {
                panic!("profile blocked: {blockers:?}")
            }
        };
    (profile, value)
}

fn corpus_value(profile: &FrozenQualificationProfile) -> (Value, BTreeMap<String, Vec<u8>>) {
    let malformed = b"kind: [".to_vec();
    let valid = b"kind: valid\n".to_vec();
    let entry = |entry_id: &str, class: &str, identity: &str, bytes: &[u8]| {
        json!({
            "entryId": entry_id,
            "artifactRef": artifact("source", identity, bytes),
            "classes": [class], "origin": "repository",
            "provenance": {
                "authority": "agent-ix/quire-specification", "revision": "fixture-r1",
                "path": format!("fixtures/{entry_id}.quire")
            },
            "rights": {
                "owner": "agent-ix", "disposition": "redistributable",
                "basis": "repository fixture"
            },
            "selectionReason": format!("Represent {class} syntax"), "lineage": null
        })
    };
    (
        json!({
            "contractVersion": "quire-verification/e02-draft-2",
            "schemaVersion": "quire-verification.qualification-corpus/v1",
            "kind": "QualificationCorpusManifest", "manifestId": "QC-EA-CONSUMER-001",
            "manifestRevision": "1", "profileIdentity": profile.identity(),
            "populationId": "POP-EA-CONSUMER-001", "requiredClasses": ["malformed", "valid"],
            "entries": [
                entry("valid", "valid", "input:valid", &valid),
                entry("malformed", "malformed", "input:malformed", &malformed)
            ],
            "gaps": [], "exclusions": []
        }),
        BTreeMap::from([
            ("input:malformed".to_owned(), malformed),
            ("input:valid".to_owned(), valid),
        ]),
    )
}

fn freeze_corpus(
    profile: &FrozenQualificationProfile,
) -> (
    Box<FrozenQualificationCorpus>,
    Value,
    BTreeMap<String, Vec<u8>>,
) {
    let (value, retained) = corpus_value(profile);
    let corpus = match qualify_corpus_manifest(value.clone(), profile, &proofs(&value, &retained))
        .expect("corpus decision")
    {
        QualificationCorpusOutcome::Complete(corpus) => corpus,
        QualificationCorpusOutcome::Incomplete { gaps, .. } => {
            panic!("corpus unexpectedly incomplete: {gaps:?}")
        }
    };
    (corpus, value, retained)
}

fn contract(kind: &str) -> ContractBinding {
    ContractBinding {
        kind: kind.to_owned(),
        version: "1".to_owned(),
        revision: FIXTURE_REVISION.to_owned(),
        digest: ContentDigest::of_bytes(kind.as_bytes()),
    }
}

fn fixture_executable() -> PathBuf {
    fs::canonicalize(env!("CARGO_BIN_EXE_qualification-oracle-fixture"))
        .expect("Rust qualification fixture executable")
}

fn input_bindings() -> Vec<InputBinding> {
    vec![
        InputBinding {
            role: "entry:malformed".to_owned(),
            path: "malformed.quire".to_owned(),
            digest: ContentDigest::of_bytes(b"kind: ["),
        },
        InputBinding {
            role: "entry:valid".to_owned(),
            path: "valid.quire".to_owned(),
            digest: ContentDigest::of_bytes(b"kind: valid\n"),
        },
    ]
}

fn producer_arguments(mode: ProducerMode, response: &Value) -> Vec<ArgumentBinding> {
    let literal = |value: &str| ArgumentBinding::Literal {
        value: value.to_owned(),
    };
    let output = || ArgumentBinding::OutputArtifact {
        role: "response".to_owned(),
    };
    match mode {
        ProducerMode::Completed
        | ProducerMode::Unavailable
        | ProducerMode::Refused
        | ProducerMode::ExecutableIdentityRefused
        | ProducerMode::Cancelled => vec![
            literal("emit"),
            output(),
            literal(&serde_json::to_string(response).expect("response JSON")),
        ],
        ProducerMode::Failed => vec![
            literal("emit-exit"),
            output(),
            literal(&serde_json::to_string(response).expect("response JSON")),
            literal("7"),
        ],
        ProducerMode::TimedOut => vec![literal("wait"), literal("2000"), output()],
        ProducerMode::Malformed => vec![literal("malformed"), output()],
        ProducerMode::ContainmentFailure => {
            vec![literal("escape"), literal("escaped.pid"), output()]
        }
    }
}

fn request(
    root: &Path,
    operation: Operation,
    mode: ProducerMode,
    response: &Value,
) -> ProducerExecutionRequest {
    let executable = fixture_executable();
    let executable_bytes = fs::read(&executable).expect("fixture executable bytes");
    let selected_executable = match mode {
        ProducerMode::Unavailable => root.join("missing-executable"),
        ProducerMode::ExecutableIdentityRefused => {
            let changed = root.join("changed-executable");
            fs::write(&changed, b"changed executable").expect("changed executable fixture");
            changed
        }
        _ => executable.clone(),
    };
    let cancellation = if matches!(mode, ProducerMode::Cancelled) {
        CancellationBinding::Event {
            authority: "quire-verification-test".to_owned(),
            event_id: "cancel-before-launch".to_owned(),
        }
    } else {
        CancellationBinding::Disabled
    };
    let mut environment = BTreeMap::new();
    if matches!(mode, ProducerMode::ContainmentFailure) {
        environment.insert(
            "QV_FIXTURE_EXECUTABLE".to_owned(),
            executable.display().to_string(),
        );
    }
    ProducerExecutionRequest {
        protocol: PRODUCER_EXECUTION_REQUEST_PROTOCOL.to_owned(),
        producer: ProducerDescriptor {
            name: "qualification-oracle-rust-fixture".to_owned(),
            version: "1.0.0".to_owned(),
            source_revision: FIXTURE_REVISION.to_owned(),
            executable: selected_executable.display().to_string(),
            executable_digest: ContentDigest::of_bytes(&executable_bytes),
        },
        caller: contract("quire-verification.qualification-oracle-request"),
        procedure: ExecutionProcedure::Direct,
        capability_root: fs::canonicalize(root)
            .expect("canonical root")
            .display()
            .to_string(),
        arguments: producer_arguments(mode, response),
        environment,
        inputs: input_bindings(),
        stdin: StdinBinding::Null,
        outputs: vec![OutputBinding {
            role: "response".to_owned(),
            path: "response.json".to_owned(),
            required: true,
        }],
        containment: ContainmentBinding::ProcessGroupV1 {
            contract: contract("qualification-oracle.cooperative-confinement"),
        },
        cancellation,
        budget: ExecutionBudget {
            timeout_millis: if matches!(mode, ProducerMode::TimedOut) {
                30
            } else {
                2_000
            },
            max_stdout_bytes: 1_024,
            max_stderr_bytes: 1_024,
            max_input_bytes: 32,
            max_output_artifacts: 1,
            max_output_bytes: 16_384,
            max_descendants: 64,
            max_concurrency: if matches!(mode, ProducerMode::Refused) {
                2
            } else {
                1
            },
        },
        response: match operation {
            Operation::Oracle => qualification_oracle_response_binding(),
            Operation::Comparison => qualification_comparison_response_binding(),
        },
    }
}

fn oracle_response(observation_ref: &Value) -> Value {
    json!({
        "contractVersion": "quire-verification/e02-draft-2",
        "schemaVersion": "quire-verification.qualification-oracle-response/v1",
        "kind": "QualificationOracleProducerResponse",
        "bindingId": "binding-ea", "guaranteeId": "syntax-outcome",
        "inputProjection": {"mode": "ordered-batch", "entryIds": ["malformed", "valid"]},
        "relationId": "same-syntax-outcome", "observationRef": observation_ref
    })
}

fn comparison_response(candidate_ref: &Value, oracle_ref: &Value, outcome: &str) -> Value {
    json!({
        "contractVersion": "quire-verification/e02-draft-2",
        "schemaVersion": "quire-verification.qualification-comparison-response/v1",
        "kind": "QualificationComparisonProducerResponse",
        "bindingId": "binding-ea", "guaranteeId": "syntax-outcome",
        "inputProjection": {"mode": "ordered-batch", "entryIds": ["malformed", "valid"]},
        "relationId": "same-syntax-outcome",
        "candidateObservationRef": candidate_ref, "oracleObservationRef": oracle_ref,
        "outcome": outcome
    })
}

fn binding(executable_bytes: &[u8], request_bytes: &[u8]) -> Value {
    json!({
        "bindingId": "binding-ea", "guaranteeId": "syntax-outcome",
        "oracleRef": reference("oracle", "oracle:ea"),
        "executableRef": artifact_with_revision(
            "executable-projection", "executable:oracle", FIXTURE_REVISION, executable_bytes
        ),
        "comparisonProcedureRef": reference("review-procedure", "comparison:ea"),
        "comparisonExecutableRef": artifact_with_revision(
            "executable-projection", "executable:comparison", FIXTURE_REVISION, executable_bytes
        ),
        "inputProjection": {"mode": "ordered-batch", "entryIds": ["malformed", "valid"]},
        "expectedRelation": {
            "relationId": "same-syntax-outcome",
            "description": "Candidate and oracle return the same syntax outcome",
            "expectedOutcome": "agreed", "origin": "oracle"
        },
        "dependence": {
            "class": "independent",
            "evidenceRefs": [reference("source", "independence:ea")],
            "candidateAncestryRefs": [reference("source", "candidate-ancestry:ea")],
            "oracleAncestryRefs": [reference("source", "oracle-ancestry:ea")]
        },
        "limitations": ["Syntax outcome only"],
        "disagreementProcedureRef": reference("review-procedure", "disagreement:ea"),
        "executionRequestRef": artifact("invocation", "request:oracle", request_bytes),
        "comparisonRequestRef": artifact("invocation", "request:comparison", request_bytes)
    })
}

fn manifest_value(
    profile: &FrozenQualificationProfile,
    corpus: &FrozenQualificationCorpus,
    executable_bytes: &[u8],
    request_bytes: &[u8],
) -> Value {
    json!({
        "contractVersion": "quire-verification/e02-draft-2",
        "schemaVersion": "quire-verification.qualification-oracles/v1",
        "kind": "QualificationOracleManifest", "manifestId": "QO-EA-CONSUMER-001",
        "profileIdentity": profile.identity(), "corpusIdentity": corpus.identity(),
        "bindings": [binding(executable_bytes, request_bytes)],
        "gaps": [{
            "gapId": "gap-span", "guaranteeId": "diagnostic-span",
            "owner": "qualification-owner",
            "reason": "No independently implemented span oracle is accepted",
            "limitations": ["Span correctness remains unqualified"]
        }]
    })
}

impl Scenario {
    fn new(operation: Operation, mode: ProducerMode, outcome: &str) -> Self {
        let root = tempfile::tempdir().expect("temporary capability root");
        fs::write(root.path().join("malformed.quire"), b"kind: [").expect("malformed input");
        fs::write(root.path().join("valid.quire"), b"kind: valid\n").expect("valid input");
        let (profile, profile_source) = freeze_profile();
        let (corpus, corpus_source, mut retained) = freeze_corpus(&profile);
        let oracle_observation = artifact("observation", "observation:oracle", b"oracle");
        let candidate_observation = artifact("observation", "observation:candidate", b"candidate");
        let response = match operation {
            Operation::Oracle => oracle_response(&oracle_observation),
            Operation::Comparison => {
                comparison_response(&candidate_observation, &oracle_observation, outcome)
            }
        };
        let request = request(root.path(), operation, mode, &response);
        let request_bytes = jcs_canonicalize(&request).expect("canonical request");
        let executable_bytes = fs::read(fixture_executable()).expect("fixture bytes");
        let manifest_source = manifest_value(&profile, &corpus, &executable_bytes, &request_bytes);
        retained.extend([
            ("executable:oracle".to_owned(), executable_bytes.clone()),
            ("executable:comparison".to_owned(), executable_bytes),
            ("request:oracle".to_owned(), request_bytes.clone()),
            ("request:comparison".to_owned(), request_bytes),
            ("observation:oracle".to_owned(), b"oracle".to_vec()),
            ("observation:candidate".to_owned(), b"candidate".to_vec()),
        ]);
        let manifest = match qualify_oracle_manifest(
            manifest_source.clone(),
            &profile,
            &corpus,
            &proofs(&manifest_source, &retained),
        )
        .expect("oracle manifest decision")
        {
            QualificationOracleOutcome::Complete(manifest)
            | QualificationOracleOutcome::Incomplete { manifest, .. } => manifest,
        };
        for (identity, bytes) in
            collect_retained_defaults(&json!([profile_source, corpus_source, manifest_source]))
        {
            retained.entry(identity).or_insert(bytes);
        }
        Self {
            _root: root,
            profile,
            corpus,
            manifest,
            request,
            retained,
            response,
        }
    }

    fn result_proofs<T: serde::Serialize>(
        &self,
        result: &engineering_assurance::producer_execution::ProducerExecutionResult<T>,
        result_identity: &str,
    ) -> (Value, Vec<ValidatedSharedReference>) {
        let result_bytes = result.canonical_bytes().expect("canonical result");
        let result_ref = artifact("run-artifact", result_identity, &result_bytes);
        let root = json!({
            "profile": self.profile.profile(), "corpus": self.corpus.manifest(),
            "manifest": self.manifest.manifest(), "response": self.response,
            "resultRef": result_ref
        });
        let mut retained = self.retained.clone();
        retained.insert(result_identity.to_owned(), result_bytes);
        let validated = proofs(&root, &retained);
        (root["resultRef"].clone(), validated)
    }
}

fn collect_retained_defaults(value: &Value) -> BTreeMap<String, Vec<u8>> {
    let mut artifacts = Vec::new();
    collect_artifacts(value, &mut artifacts);
    artifacts
        .into_iter()
        .filter_map(|artifact| {
            let identity = artifact["value"]["identity"].as_str()?.to_owned();
            Some((identity.clone(), identity.into_bytes()))
        })
        .collect()
}

fn oracle_execution_state(mode: ProducerMode) -> Value {
    let scenario = Scenario::new(Operation::Oracle, mode, "agreed");
    let executor = ProducerExecutor::new(1).expect("one execution slot");
    let cancellation = CancellationToken::new(scenario.request.cancellation.clone());
    if matches!(mode, ProducerMode::Cancelled) {
        assert!(cancellation.cancel());
    }
    let result = execute_qualification_oracle(&executor, &scenario.request, &cancellation)
        .expect("structurally valid oracle request");
    let (result_ref, validated) = scenario.result_proofs(&result, "result:oracle");
    let evidence = QualificationExecutionEvidence {
        manifest: &scenario.manifest,
        binding_id: "binding-ea",
        profile: &scenario.profile,
        corpus: &scenario.corpus,
        request: &scenario.request,
        result_ref: &result_ref,
        validated_references: &validated,
    };
    let record = record_qualification_oracle_execution(&evidence, &result)
        .expect("bound oracle execution record");
    serde_json::to_value(record).expect("record JSON")["state"].clone()
}

#[test]
#[trace("TC-041", "FR-024-AC-3")]
fn tc_041_tc_126_preserve_every_engineering_assurance_execution_state() {
    let cases = [
        (ProducerMode::Unavailable, json!({"kind": "unavailable"})),
        (
            ProducerMode::Refused,
            json!({"kind": "refused", "reason": "concurrency"}),
        ),
        (
            ProducerMode::ExecutableIdentityRefused,
            json!({"kind": "refused", "reason": "executable_identity"}),
        ),
        (
            ProducerMode::Failed,
            json!({"kind": "failed", "reason": {"kind": "exit_code", "value": 7}}),
        ),
        (ProducerMode::TimedOut, json!({"kind": "timed_out"})),
        (
            ProducerMode::Malformed,
            json!({"kind": "malformed_response"}),
        ),
        (
            ProducerMode::ContainmentFailure,
            json!({"kind": "containment_failure"}),
        ),
        (ProducerMode::Cancelled, json!({"kind": "cancelled"})),
        (
            ProducerMode::Completed,
            json!({
                "kind": "completed",
                "observationRef": artifact("observation", "observation:oracle", b"oracle")
            }),
        ),
    ];
    for (mode, expected) in cases {
        let actual = oracle_execution_state(mode);
        assert_eq!(actual, expected);
        if actual["kind"] != "completed" {
            assert!(actual.get("observationRef").is_none());
        }
    }
}

fn execute_comparison_record(
    scenario: &Scenario,
) -> Box<wire::QualificationOracleComparisonRecord> {
    let executor = ProducerExecutor::new(1).expect("one execution slot");
    let cancellation = CancellationToken::new(scenario.request.cancellation.clone());
    let result = execute_qualification_comparison(&executor, &scenario.request, &cancellation)
        .expect("structurally valid comparison request");
    let (result_ref, validated) = scenario.result_proofs(&result, "result:comparison");
    let evidence = QualificationExecutionEvidence {
        manifest: &scenario.manifest,
        binding_id: "binding-ea",
        profile: &scenario.profile,
        corpus: &scenario.corpus,
        request: &scenario.request,
        result_ref: &result_ref,
        validated_references: &validated,
    };
    let assessment = record_qualification_comparison_execution(&evidence, &result)
        .expect("bound comparison record");
    let QualificationComparisonAssessment::Compared(record) = assessment else {
        panic!("completed comparison became an execution finding");
    };
    record
}

fn comparison_record(outcome: &str) -> Value {
    let scenario = Scenario::new(Operation::Comparison, ProducerMode::Completed, outcome);
    serde_json::to_value(execute_comparison_record(&scenario)).expect("comparison record JSON")
}

#[test]
#[trace("TC-041", "FR-024-AC-3")]
fn tc_041_tc_126_consume_agreement_and_retain_unresolved_disagreement() {
    let agreed = comparison_record("agreed");
    assert_eq!(agreed["outcome"], "agreed");
    assert_eq!(agreed["resolution"], "resolved");

    let disagreed = comparison_record("disagreed");
    assert_eq!(disagreed["outcome"], "disagreed");
    assert_eq!(disagreed["resolution"], "unresolved");
    assert_eq!(
        disagreed["candidateObservationRef"],
        artifact("observation", "observation:candidate", b"candidate")
    );
    assert_eq!(
        disagreed["oracleObservationRef"],
        artifact("observation", "observation:oracle", b"oracle")
    );
}

fn disagreement_response(
    comparison: &wire::QualificationOracleComparisonRecord,
    procedure_ref: &Value,
) -> Value {
    let comparison_value = serde_json::to_value(comparison).expect("comparison JSON");
    json!({
        "contractVersion": "quire-verification/e02-draft-2",
        "schemaVersion": "quire-verification.qualification-disagreement-response/v1",
        "kind": "QualificationDisagreementProcedureResponse",
        "bindingId": "binding-ea",
        "comparisonIdentity": jcs_sha256(&comparison_value).expect("comparison identity"),
        "procedureRef": procedure_ref,
        "attributions": ["candidate accepted malformed syntax"],
        "bounds": ["difference is limited to the malformed entry"]
    })
}

fn record_disagreement(
    scenario: &Scenario,
    comparison: &wire::QualificationOracleComparisonRecord,
    response: &Value,
    identity: &str,
) -> Result<
    wire::QualificationOracleDisagreementRecord,
    quire_verification::contracts::VerificationError,
> {
    let response_bytes = jcs_canonicalize(response).expect("canonical disagreement response");
    let result_ref = artifact("run-artifact", identity, &response_bytes);
    let proof_root = json!({
        "profile": scenario.profile.profile(), "corpus": scenario.corpus.manifest(),
        "manifest": scenario.manifest.manifest(), "response": response,
        "resultRef": result_ref
    });
    let mut retained = scenario.retained.clone();
    retained.insert(identity.to_owned(), response_bytes);
    record_qualification_disagreement(
        &scenario.manifest,
        "binding-ea",
        comparison,
        &proof_root["resultRef"],
        &proofs(&proof_root, &retained),
    )
}

#[test]
#[trace("TC-041", "FR-024-AC-3")]
fn tc_041_binds_only_the_exact_disagreement_procedure_without_a_disposition() {
    let scenario = Scenario::new(Operation::Comparison, ProducerMode::Completed, "disagreed");
    let comparison = execute_comparison_record(&scenario);
    let manifest = serde_json::to_value(scenario.manifest.manifest()).expect("manifest JSON");
    let procedure_ref = manifest["bindings"][0]["disagreementProcedureRef"].clone();

    let exact = disagreement_response(&comparison, &procedure_ref);
    let record = record_disagreement(&scenario, &comparison, &exact, "result:disagreement")
        .expect("exact disagreement procedure");
    let record = serde_json::to_value(record).expect("disagreement record JSON");
    assert_eq!(record["attributions"], exact["attributions"]);
    assert_eq!(record["bounds"], exact["bounds"]);
    assert!(record.get("disposition").is_none());

    let foreign_procedure = reference("review-procedure", "disagreement:foreign");
    let foreign = disagreement_response(&comparison, &foreign_procedure);
    let error = record_disagreement(
        &scenario,
        &comparison,
        &foreign,
        "result:foreign-disagreement",
    )
    .expect_err("foreign disagreement procedure");
    assert_eq!(
        error.code(),
        VerificationErrorCode::InvalidQualificationOracleBinding
    );

    let mut disposition = exact;
    disposition["disposition"] = json!("qualified");
    let error = record_disagreement(&scenario, &comparison, &disposition, "result:disposition")
        .expect_err("disagreement procedure cannot decide qualification");
    assert_eq!(
        error.code(),
        VerificationErrorCode::InvalidQualificationOracleBinding
    );
}

#[test]
#[trace("TC-041", "FR-024-AC-3", "FR-024-AC-4")]
fn tc_041_tc_126_refuse_request_identity_drift() {
    let executor = ProducerExecutor::new(1).expect("one execution slot");
    let exact = Scenario::new(Operation::Oracle, ProducerMode::Completed, "agreed");
    let mut changed_request = exact.request.clone();
    changed_request.arguments.push(ArgumentBinding::Literal {
        value: "foreign".to_owned(),
    });
    let cancellation = CancellationToken::new(changed_request.cancellation.clone());
    let changed_result = execute_qualification_oracle(&executor, &changed_request, &cancellation)
        .expect("changed request remains structurally valid");
    let (changed_result_ref, changed_validated) =
        exact.result_proofs(&changed_result, "result:foreign-request");
    let evidence = QualificationExecutionEvidence {
        manifest: &exact.manifest,
        binding_id: "binding-ea",
        profile: &exact.profile,
        corpus: &exact.corpus,
        request: &changed_request,
        result_ref: &changed_result_ref,
        validated_references: &changed_validated,
    };
    let error = record_qualification_oracle_execution(&evidence, &changed_result)
        .expect_err("request differs from the frozen invocation reference");
    assert_eq!(
        error.code(),
        VerificationErrorCode::InvalidQualificationOracleBinding
    );
}

#[test]
#[trace("TC-041", "FR-024-AC-3", "FR-024-AC-4")]
fn tc_041_tc_126_refuse_result_and_retained_observation_drift() {
    let executor = ProducerExecutor::new(1).expect("one execution slot");
    let exact = Scenario::new(Operation::Oracle, ProducerMode::Completed, "agreed");
    let cancellation = CancellationToken::new(exact.request.cancellation.clone());
    let exact_result = execute_qualification_oracle(&executor, &exact.request, &cancellation)
        .expect("exact request");
    let wrong_result_bytes = b"tampered result";
    let wrong_result_ref = artifact("run-artifact", "result:tampered", wrong_result_bytes);
    let proof_root = json!({
        "profile": exact.profile.profile(), "corpus": exact.corpus.manifest(),
        "manifest": exact.manifest.manifest(), "response": exact.response,
        "resultRef": wrong_result_ref
    });
    let mut retained = exact.retained.clone();
    retained.insert("result:tampered".to_owned(), wrong_result_bytes.to_vec());
    let validated = proofs(&proof_root, &retained);
    let evidence = QualificationExecutionEvidence {
        manifest: &exact.manifest,
        binding_id: "binding-ea",
        profile: &exact.profile,
        corpus: &exact.corpus,
        request: &exact.request,
        result_ref: &proof_root["resultRef"],
        validated_references: &validated,
    };
    let error = record_qualification_oracle_execution(&evidence, &exact_result)
        .expect_err("result reference retains different bytes");
    assert_eq!(
        error.code(),
        VerificationErrorCode::InvalidQualificationOracleBinding
    );

    let mut forged_result = exact_result;
    let mut forged_response = exact.response.clone();
    forged_response["observationRef"] = artifact("observation", "observation:forged", b"forged");
    forged_result.state = ProducerExecutionState::Completed {
        observation: serde_json::from_value(forged_response.clone())
            .expect("schema-derived forged observation"),
    };
    let forged_bytes = forged_result
        .canonical_bytes()
        .expect("canonical forged result");
    let forged_result_ref = artifact("run-artifact", "result:forged-observation", &forged_bytes);
    let proof_root = json!({
        "profile": exact.profile.profile(), "corpus": exact.corpus.manifest(),
        "manifest": exact.manifest.manifest(), "response": forged_response,
        "resultRef": forged_result_ref
    });
    let mut retained = exact.retained.clone();
    retained.insert("observation:forged".to_owned(), b"forged".to_vec());
    retained.insert("result:forged-observation".to_owned(), forged_bytes);
    let validated = proofs(&proof_root, &retained);
    let evidence = QualificationExecutionEvidence {
        manifest: &exact.manifest,
        binding_id: "binding-ea",
        profile: &exact.profile,
        corpus: &exact.corpus,
        request: &exact.request,
        result_ref: &proof_root["resultRef"],
        validated_references: &validated,
    };
    let error = record_qualification_oracle_execution(&evidence, &forged_result)
        .expect_err("typed observation differs from retained producer response");
    assert_eq!(
        error.code(),
        VerificationErrorCode::InvalidQualificationOracleBinding
    );
}
