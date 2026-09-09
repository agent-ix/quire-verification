// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! VP04 native parser portfolio acceptance tests.

#![cfg(target_os = "linux")]

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use ix_trace_rs::trace;
use quire_verification::contracts::VerificationErrorCode;
use quire_verification::evidence::assess_evidence;
use quire_verification::fixtures::validate_fixture_result_references;
use quire_verification::native_process::{NativeProcessExecutor, NativeProcessOutput};
use quire_verification::parser_portfolio::{
    ParserDiagnostic, ParserPortfolioRequest, ParserSource, execute_parser_portfolio,
    validate_diagnostic_response, validate_native_task, validate_parsed_response,
};
use quire_verification::shared_reference::ValidatedSharedReference;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tempfile::TempDir;

struct Candidate {
    root: TempDir,
    executable: PathBuf,
    valid: PathBuf,
    mutation: PathBuf,
}

impl Candidate {
    fn new() -> Self {
        let built = Path::new(env!("CARGO_BIN_EXE_native-process-fixture"));
        let parent = built.parent().expect("fixture build directory");
        let root = tempfile::Builder::new()
            .prefix("quire-verification-parser-")
            .tempdir_in(parent)
            .expect("candidate root");
        let executable = root.path().join("quire-spec");
        fs::hard_link(built, &executable).expect("immutable fixture link");
        let valid = root.path().join("valid.native");
        let mutation = root.path().join("mutation.native");
        fs::write(&valid, b"clause\ninvariant\n").expect("valid source");
        fs::write(&mutation, b"clause\ninvarianx\n").expect("mutation source");
        Self {
            root,
            executable,
            valid,
            mutation,
        }
    }

    fn executor(&self) -> NativeProcessExecutor {
        NativeProcessExecutor::admit(self.root.path(), &self.executable).expect("admit fixture")
    }
}

fn retained() -> Value {
    serde_json::from_str(include_str!("../evidence/local-catalog-portfolio.json"))
        .expect("retained portfolio")
}

fn context(result: &Value, result_id: &str) -> Value {
    let mut context = result.clone();
    let fields = context.as_object_mut().expect("result object");
    for field in [
        "contractVersion",
        "kind",
        "producedAt",
        "execution",
        "logical",
        "activation",
        "participation",
        "nativeArtifacts",
    ] {
        fields.remove(field);
    }
    fields.insert("resultId".into(), json!(result_id));
    context
}

fn request(
    candidate: &Candidate,
    formatted_revision: &str,
) -> (ParserPortfolioRequest, Vec<ValidatedSharedReference>) {
    let retained = retained();
    let results = retained["results"].as_array().expect("retained results");
    let direct = context(&results[0], "VP04-DIRECT");
    let roundtrip = context(&results[1], "VP04-ROUNDTRIP");
    let adverse = context(&results[0], "VP04-ADVERSE");
    let manual = context(&results[2], "VP04-MANUAL");
    let packet = json!({"results": [results[0].clone(), results[1].clone(), results[0].clone(), results[2].clone()]});
    let references = validate_fixture_result_references(&packet).expect("strict contribution refs");
    let valid_bytes = fs::read(&candidate.valid).expect("valid bytes");
    let diagnostic = json!({
        "status": "refused", "phase": "parse", "code": "invalid_syntax",
        "source": {"identity": "vp04:mutation", "revision": "fixture:mutation"},
        "path": candidate.mutation.to_string_lossy(),
        "span": {
            "start": {"byte": 249, "line": 6, "column": 1},
            "end": {"byte": 258, "line": 6, "column": 10}
        },
        "message": "expected a clause keyword"
    });
    (
        ParserPortfolioRequest {
            execution_id: "VP04-EXECUTION".into(),
            candidate_digest: format!(
                "sha256:{:x}",
                Sha256::digest(fs::read(&candidate.executable).expect("candidate bytes"))
            ),
            produced_at: retained["producedAt"].as_str().expect("producedAt").into(),
            valid_source: ParserSource {
                path: candidate.valid.clone(),
                identity: "vp04:valid".into(),
                revision: "fixture:valid".into(),
                digest: format!("sha256:{:x}", Sha256::digest(valid_bytes)),
                clauses: 4,
                imports: 1,
            },
            mutation: ParserDiagnostic {
                identity: "vp04:mutation".into(),
                revision: "fixture:mutation".into(),
                path: candidate.mutation.clone(),
                digest: format!(
                    "sha256:{:x}",
                    Sha256::digest(fs::read(&candidate.mutation).expect("mutation bytes"))
                ),
                expected: diagnostic,
            },
            formatted_revision: formatted_revision.into(),
            environment: BTreeMap::new(),
            deadline_seconds: 2.0,
            plan: retained["plan"].clone(),
            direct_context: direct,
            roundtrip_context: roundtrip,
            adverse_context: adverse,
            manual_context: manual,
        },
        references,
    )
}

#[trace("TC-034", "FR-019-AC-1", "FR-019-AC-2")]
#[test]
fn tc_034_executes_baselines_adverse_and_metamorphic_mismatch() {
    let candidate = Candidate::new();
    let executor = candidate.executor();
    let (baseline_request, references) = request(&candidate, "fixture:formatted");
    let baseline = execute_parser_portfolio(&executor, &baseline_request, &references)
        .expect("baseline portfolio");
    let results = baseline["results"].as_array().expect("results");
    assert_eq!(results[0]["execution"]["status"], "completed");
    assert_eq!(results[0]["logical"]["outcome"], "satisfied");
    assert_eq!(results[1]["logical"]["outcome"], "satisfied");
    assert_eq!(results[2]["execution"]["status"], "completed");
    assert_eq!(results[2]["logical"]["outcome"], "violated");
    assert_eq!(results[3]["execution"]["status"], "not-run");
    assert_eq!(results[3]["logical"]["outcome"], "not-evaluated");
    assert_eq!(results[3]["activation"]["status"], "not-required");
    assert_eq!(results[3]["participation"]["status"], "not-started");
    assert!(
        results[3]["nativeArtifacts"]
            .as_array()
            .is_some_and(Vec::is_empty)
    );
    assert_eq!(baseline["evidenceOwner"], "Quoin");
    assert_eq!(baseline["sufficiencyDecision"], "not-made");
    let commands = baseline["commands"].as_array().expect("commands");
    let runs = baseline["nativeRuns"].as_array().expect("native runs");
    assert_eq!(commands.len(), 5);
    assert_eq!(runs.len(), 5);
    assert_eq!(
        commands
            .iter()
            .map(|command| command["expectedExitCode"].clone())
            .collect::<Vec<_>>(),
        [json!(0), json!(0), json!(0), json!(0), json!(1)]
    );
    assert_eq!(
        runs.iter()
            .map(|run| run["observedExitCode"].clone())
            .collect::<Vec<_>>(),
        [json!(0), json!(0), json!(0), json!(0), json!(1)]
    );
    assert_eq!(runs[0]["stdoutArtifact"], results[0]["nativeArtifacts"][0]);
    assert_eq!(runs[1]["stdoutArtifact"], results[1]["nativeArtifacts"][0]);
    assert_eq!(runs[2]["stdoutArtifact"], results[1]["nativeArtifacts"][1]);
    assert_eq!(runs[3]["stdoutArtifact"], results[1]["nativeArtifacts"][2]);
    assert_eq!(runs[4]["stderrArtifact"], results[2]["nativeArtifacts"][0]);

    let (mismatch_request, mismatch_references) = request(&candidate, "fixture:formatted-mismatch");
    let mismatch = execute_parser_portfolio(&executor, &mismatch_request, &mismatch_references)
        .expect("metamorphic mismatch portfolio");
    assert_eq!(mismatch["results"][1]["execution"]["status"], "completed");
    assert_eq!(mismatch["results"][1]["logical"]["outcome"], "violated");
    assert_eq!(
        mismatch["results"][1]["nativeArtifacts"]
            .as_array()
            .expect("differing artifacts")
            .len(),
        3
    );
}

#[trace("TC-034", "FR-019-AC-1", "FR-019-AC-2")]
#[test]
fn tc_034_discriminates_every_exact_parser_protocol_field() {
    let candidate = Candidate::new();
    let (portfolio, _) = request(&candidate, "fixture:formatted");
    let expected = json!({
        "status": "parsed",
        "source": {"identity": "vp04:valid", "revision": "fixture:valid", "digest": portfolio.valid_source.digest},
        "path": candidate.valid.to_string_lossy(), "imports": 1, "clauses": 4
    });
    let valid_output = NativeProcessOutput {
        exit_code: 0,
        stdout: expected.to_string(),
        stderr: String::new(),
    };
    validate_parsed_response(&valid_output, &portfolio.valid_source).expect("exact parse response");
    for pointer in [
        "/status",
        "/source/identity",
        "/source/revision",
        "/source/digest",
        "/path",
        "/imports",
        "/clauses",
    ] {
        let mut changed = expected.clone();
        *changed.pointer_mut(pointer).expect("field") = json!("wrong");
        let output = NativeProcessOutput {
            stdout: changed.to_string(),
            ..valid_output.clone()
        };
        assert_eq!(
            validate_parsed_response(&output, &portfolio.valid_source)
                .expect_err("field drift")
                .code(),
            VerificationErrorCode::MalformedMethodResponse
        );
    }
    for stdout in [format!("{expected} {{}}"), format!("{expected} trailing")] {
        let output = NativeProcessOutput {
            stdout,
            ..valid_output.clone()
        };
        assert_eq!(
            validate_parsed_response(&output, &portfolio.valid_source)
                .expect_err("trailing data")
                .code(),
            VerificationErrorCode::MalformedMethodResponse
        );
    }
    let wrong_stream = NativeProcessOutput {
        stdout: String::new(),
        stderr: expected.to_string(),
        ..valid_output.clone()
    };
    assert_eq!(
        validate_parsed_response(&wrong_stream, &portfolio.valid_source)
            .expect_err("wrong stream")
            .code(),
        VerificationErrorCode::MalformedMethodResponse
    );

    let diagnostic = portfolio.mutation.expected.clone();
    let valid_diagnostic = NativeProcessOutput {
        exit_code: 1,
        stdout: String::new(),
        stderr: diagnostic.to_string(),
    };
    validate_diagnostic_response(&valid_diagnostic, &diagnostic).expect("exact diagnostic");
    for pointer in [
        "/status",
        "/phase",
        "/code",
        "/source/identity",
        "/source/revision",
        "/path",
        "/span/start/byte",
        "/span/start/line",
        "/span/start/column",
        "/span/end/byte",
        "/span/end/line",
        "/span/end/column",
        "/message",
    ] {
        let mut changed = diagnostic.clone();
        *changed.pointer_mut(pointer).expect("field") = json!("wrong");
        let output = NativeProcessOutput {
            stderr: changed.to_string(),
            ..valid_diagnostic.clone()
        };
        assert_eq!(
            validate_diagnostic_response(&output, &diagnostic)
                .expect_err("diagnostic drift")
                .code(),
            VerificationErrorCode::MalformedMethodResponse
        );
    }
}

#[trace("TC-034", "FR-019-AC-1", "FR-019-AC-2")]
#[test]
fn tc_034_refuses_duplicate_json_members() {
    let candidate = Candidate::new();
    let (portfolio, _) = request(&candidate, "fixture:formatted");
    let expected = json!({
        "status": "parsed", "source": {
            "identity": "vp04:valid", "revision": "fixture:valid",
            "digest": portfolio.valid_source.digest
        }, "path": candidate.valid.to_string_lossy(), "imports": 1, "clauses": 4
    });
    let output = NativeProcessOutput {
        exit_code: 0,
        stdout: expected
            .to_string()
            .replacen("\"status\"", "\"status\":\"parsed\",\"status\"", 1),
        stderr: String::new(),
    };
    assert_eq!(
        validate_parsed_response(&output, &portfolio.valid_source)
            .expect_err("duplicate decoded member")
            .code(),
        VerificationErrorCode::MalformedMethodResponse
    );
    let malformed_diagnostic = json!({"status": "refused"});
    let malformed_output = NativeProcessOutput {
        exit_code: 1,
        stdout: String::new(),
        stderr: malformed_diagnostic.to_string(),
    };
    assert_eq!(
        validate_diagnostic_response(&malformed_output, &malformed_diagnostic)
            .expect_err("matching malformed diagnostic")
            .code(),
        VerificationErrorCode::MalformedMethodResponse
    );
}

#[trace("TC-034", "FR-019-AC-2")]
#[test]
fn tc_034_records_unexpected_process_and_protocol_failures() {
    let candidate = Candidate::new();
    let executor = candidate.executor();
    let (mut protocol_request, references) = request(&candidate, "fixture:formatted");
    protocol_request.valid_source.revision = "fixture:protocol-mismatch".into();
    let protocol = execute_parser_portfolio(&executor, &protocol_request, &references)
        .expect("protocol failure is retained");
    assert_eq!(protocol["results"][0]["execution"]["status"], "failed");
    assert_eq!(
        protocol["results"][0]["logical"]["outcome"],
        "not-evaluated"
    );

    let (mut process_request, process_references) = request(&candidate, "fixture:formatted");
    process_request.valid_source.revision = "fixture:process-failure".into();
    let process = execute_parser_portfolio(&executor, &process_request, &process_references)
        .expect("process failure is retained");
    assert_eq!(process["results"][0]["execution"]["status"], "failed");
    assert_eq!(process["results"][0]["logical"]["outcome"], "not-evaluated");

    let (mut detached_source, detached_references) = request(&candidate, "fixture:formatted");
    detached_source.valid_source.digest = format!("sha256:{}", "0".repeat(64));
    assert_eq!(
        execute_parser_portfolio(&executor, &detached_source, &detached_references)
            .expect_err("detached source bytes")
            .code(),
        VerificationErrorCode::IdentityMismatch
    );
    let (mut malformed_source, malformed_references) = request(&candidate, "fixture:formatted");
    malformed_source.valid_source.digest = "SHA256:UPPERCASE".into();
    assert_eq!(
        execute_parser_portfolio(&executor, &malformed_source, &malformed_references)
            .expect_err("malformed source digest")
            .code(),
        VerificationErrorCode::InvalidInput
    );
    let (mut detached_candidate, detached_references) = request(&candidate, "fixture:formatted");
    detached_candidate.candidate_digest = format!("sha256:{}", "0".repeat(64));
    assert_eq!(
        execute_parser_portfolio(&executor, &detached_candidate, &detached_references)
            .expect_err("detached candidate bytes")
            .code(),
        VerificationErrorCode::IdentityMismatch
    );
}

#[trace("TC-035", "FR-019-AC-3", "FR-019-AC-6")]
#[test]
fn tc_035_refuses_blocked_and_manual_tasks_before_native_launch() {
    let retained = retained();
    let plan = &retained["plan"];
    for task in plan["tasks"].as_array().expect("tasks") {
        if task["readiness"] != "ready" {
            let id = task["taskId"].as_str().expect("task id");
            assert_eq!(
                validate_native_task(plan, id)
                    .expect_err("non-native task")
                    .code(),
                VerificationErrorCode::InvalidInput
            );
        }
    }

    let candidate = Candidate::new();
    let executor = candidate.executor();
    let (mut wrong_manual, references) = request(&candidate, "fixture:formatted");
    wrong_manual.manual_context["taskId"] = wrong_manual.direct_context["taskId"].clone();
    assert_eq!(
        execute_parser_portfolio(&executor, &wrong_manual, &references)
            .expect_err("manual context cannot name native work")
            .code(),
        VerificationErrorCode::InvalidInput
    );

    let (baseline, references) = request(&candidate, "fixture:formatted");
    let mut cases = Vec::new();
    let mut changed = baseline.clone();
    changed.direct_context["techniqueId"] = json!("metamorphic-testing");
    cases.push(changed);
    let mut changed = baseline.clone();
    changed.adverse_context["taskId"] = changed.roundtrip_context["taskId"].clone();
    cases.push(changed);
    let mut changed = baseline;
    changed.manual_context["resultId"] = changed.direct_context["resultId"].clone();
    cases.push(changed);
    for changed in cases {
        assert_eq!(
            execute_parser_portfolio(&executor, &changed, &references)
                .expect_err("cross-role result context")
                .code(),
            VerificationErrorCode::InvalidInput
        );
    }

    let (mut malformed_diagnostic, references) = request(&candidate, "fixture:formatted");
    malformed_diagnostic.mutation.expected["source"]["identity"] = json!("foreign");
    assert_eq!(
        execute_parser_portfolio(&executor, &malformed_diagnostic, &references)
            .expect_err("diagnostic expectation detached from mutation")
            .code(),
        VerificationErrorCode::InvalidInput
    );
}

#[trace("TC-035", "FR-019-AC-4", "FR-019-AC-5")]
#[test]
fn tc_035_retains_freshness_and_correlation_findings() {
    let retained = retained();
    let mut packet = json!({
        "asOf": retained["producedAt"],
        "inventory": retained["plan"]["inventory"],
        "results": retained["results"]
    });
    for result in packet["results"].as_array_mut().expect("results") {
        result["validUntil"] = json!("2026-09-10T00:00:00Z");
    }
    packet["asOf"] = json!("2026-09-10T00:00:00Z");
    let exact_references = validate_fixture_result_references(&packet).expect("strict exact refs");
    let exact = assess_evidence(&packet, &exact_references).expect("exact-expiry assessment");
    assert!(
        exact["obligations"]
            .as_array()
            .expect("exact obligations")
            .iter()
            .all(|item| item["status"] != "stale")
    );
    packet["asOf"] = json!("2026-09-10T00:00:00.000000001Z");
    let shared = packet["results"][0]["oracleContributions"][0]["observationRef"].clone();
    packet["results"][1]["oracleContributions"][0]["observationRef"] = shared;
    let references = validate_fixture_result_references(&packet).expect("strict refs");
    let assessment = assess_evidence(&packet, &references).expect("conservative assessment");
    assert!(
        assessment["obligations"]
            .as_array()
            .expect("obligations")
            .iter()
            .any(|item| item["status"] == "stale")
    );
    assert!(
        assessment["oracleDependenceFindings"]
            .as_array()
            .expect("findings")
            .iter()
            .any(|item| {
                item["kind"] == "correlated-oracle"
                    && item["bases"]
                        .as_array()
                        .is_some_and(|bases| bases.iter().any(|basis| basis == "observation"))
            }),
        "{assessment}"
    );
    assert_eq!(assessment["sufficiencyDecision"], "not-made");
}
