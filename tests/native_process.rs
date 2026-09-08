// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Acceptance tests for the bounded Linux native-process boundary.

#![cfg(target_os = "linux")]

use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use ix_trace_rs::trace;
use quire_verification::contracts::VerificationErrorCode;
use quire_verification::native_process::{
    MAX_NATIVE_DEADLINE_SECONDS, MAX_NATIVE_OUTPUT_BYTES, NativeProcessExecutor,
    NativeProcessRequest, parse_native_json_output,
};
use serde_json::Value;
use tempfile::TempDir;

struct Candidate {
    root: TempDir,
    executable: PathBuf,
}

impl Candidate {
    fn new() -> Self {
        let built_fixture = Path::new(env!("CARGO_BIN_EXE_native-process-fixture"));
        let fixture_directory = built_fixture.parent().expect("fixture build directory");
        let root = tempfile::Builder::new()
            .prefix("quire-verification-native-process-")
            .tempdir_in(fixture_directory)
            .expect("candidate root beside completed Cargo fixture");
        let executable = root.path().join("native-process-fixture");
        fs::hard_link(built_fixture, &executable)
            .expect("link completed controlled native child without a writer");
        Self { root, executable }
    }

    fn executor(&self) -> NativeProcessExecutor {
        NativeProcessExecutor::admit(self.root.path(), &self.executable)
            .expect("admit controlled native child")
    }

    fn request(arguments: &[&str]) -> NativeProcessRequest {
        NativeProcessRequest {
            arguments: arguments.iter().map(ToString::to_string).collect(),
            environment: BTreeMap::from([("DECLARED".to_owned(), "exact".to_owned())]),
            deadline_seconds: Some(MAX_NATIVE_DEADLINE_SECONDS),
        }
    }
}

fn process_is_absent(error_context: &BTreeMap<String, String>) {
    let pid = error_context
        .get("pid")
        .expect("bounded child error carries pid");
    assert_eq!(
        error_context.get("reaped").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        error_context
            .get("process_group_terminated")
            .map(String::as_str),
        Some("true")
    );
    assert!(
        !Path::new("/proc").join(pid).exists(),
        "child {pid} remains"
    );
}

fn process_is_live(pid: &str) -> bool {
    let Ok(stat) = fs::read_to_string(Path::new("/proc").join(pid).join("stat")) else {
        return false;
    };
    let Some(close) = stat.rfind(')') else {
        return true;
    };
    !matches!(
        stat[close + 1..].trim_start().chars().next(),
        Some('Z' | 'X')
    )
}

#[trace("TC-038", "NFR-006-AC-1")]
#[test]
fn tc_038_executes_exact_argv_in_a_fresh_isolated_directory() {
    let candidate = Candidate::new();
    let executor = candidate.executor();
    let request = Candidate::request(&["inspect", "argument with spaces", "$()", "semi;colon"]);
    let first = executor.execute(&request).expect("exact request");
    let payload = parse_native_json_output(&first).expect("fixture protocol");
    assert_eq!(
        payload["arguments"],
        serde_json::json!(["argument with spaces", "$()", "semi;colon"])
    );
    assert_eq!(payload["declared"], "exact");
    assert_eq!(payload["homePresent"], false);
    let first_directory = PathBuf::from(
        payload["workingDirectory"]
            .as_str()
            .expect("working directory string"),
    );
    assert!(first_directory.starts_with(candidate.root.path()));
    assert_ne!(first_directory, candidate.root.path());

    let second = executor
        .execute(&Candidate::request(&["inspect"]))
        .expect("second exact request");
    let second_payload = parse_native_json_output(&second).expect("second fixture protocol");
    assert_ne!(
        payload["workingDirectory"],
        second_payload["workingDirectory"]
    );
}

#[trace("TC-038", "NFR-006-AC-6", "NFR-006-AC-7")]
#[test]
fn tc_038_pins_the_admitted_descriptor_and_refuses_path_escape() {
    let candidate = Candidate::new();
    let mut adverse = vec![
        PathBuf::from("relative"),
        candidate.root.path().join("absent"),
    ];
    let outside = tempfile::NamedTempFile::new().expect("outside regular file");
    adverse.push(outside.path().to_path_buf());
    let link = candidate.root.path().join("linked-candidate");
    symlink(&candidate.executable, &link).expect("candidate symlink");
    adverse.push(link);

    for executable in adverse {
        let error = NativeProcessExecutor::admit(candidate.root.path(), &executable)
            .expect_err("path must refuse");
        assert_eq!(error.code(), VerificationErrorCode::InvalidExecutablePath);
    }
    let directory_error =
        NativeProcessExecutor::admit(candidate.root.path(), candidate.root.path())
            .expect_err("directory candidate must refuse");
    assert_eq!(
        directory_error.code(),
        VerificationErrorCode::InvalidExecutablePath
    );

    let linked_root_parent = tempfile::tempdir().expect("linked-root parent");
    let root_link = linked_root_parent.path().join("linked-root");
    symlink(candidate.root.path(), &root_link).expect("root symlink");
    let root_error = NativeProcessExecutor::admit(&root_link, &root_link.join("candidate"))
        .expect_err("linked root must refuse");
    assert_eq!(
        root_error.code(),
        VerificationErrorCode::InvalidExecutablePath
    );

    let executor = candidate.executor();
    let admitted_path = candidate.root.path().join("admitted-original");
    fs::rename(&candidate.executable, &admitted_path).expect("move admitted inode");
    fs::write(&candidate.executable, b"replacement must not execute")
        .expect("replace request pathname");
    let output = executor
        .execute(&Candidate::request(&["valid-json"]))
        .expect("descriptor-pinned executable");
    assert_eq!(
        parse_native_json_output(&output).expect("admitted fixture response")["status"],
        "ok"
    );
}

#[trace(
    "TC-038",
    "NFR-006-AC-2",
    "NFR-006-AC-3",
    "NFR-006-AC-5",
    "NFR-006-AC-8"
)]
#[test]
fn tc_038_refuses_invalid_deadlines_and_contains_the_process_group() {
    let candidate = Candidate::new();
    let executor = candidate.executor();
    let marker = candidate.root.path().join("deadline-marker");
    for deadline in [
        None,
        Some(0.0),
        Some(-1.0),
        Some(f64::NAN),
        Some(f64::INFINITY),
        Some(MAX_NATIVE_DEADLINE_SECONDS + 0.001),
    ] {
        let mut request = Candidate::request(&["write-marker", marker.to_str().expect("marker")]);
        request.deadline_seconds = deadline;
        let error = executor
            .execute(&request)
            .expect_err("deadline must refuse");
        assert_eq!(
            error.code(),
            VerificationErrorCode::InvalidExecutionDeadline
        );
        assert!(!marker.exists(), "invalid deadline launched a child");
    }

    let mut request =
        Candidate::request(&["sleep-mark", "250", marker.to_str().expect("marker path")]);
    request.deadline_seconds = Some(0.02);
    let error = executor.execute(&request).expect_err("child must time out");
    assert_eq!(error.code(), VerificationErrorCode::ExecutionTimeout);
    process_is_absent(error.context());
    assert!(!marker.exists(), "timed-out child remained live");

    let descendant_pid_file = candidate.root.path().join("descendant.pid");
    let descendant_marker = candidate.root.path().join("descendant-marker");
    let mut descendant_request = Candidate::request(&[
        "descendant-holds-pipes",
        "500",
        descendant_pid_file.to_str().expect("pid file"),
        descendant_marker.to_str().expect("descendant marker"),
    ]);
    descendant_request.deadline_seconds = Some(0.05);
    let descendant_error = executor
        .execute(&descendant_request)
        .expect_err("descendant-held pipes must not escape deadline");
    assert_eq!(
        descendant_error.code(),
        VerificationErrorCode::ExecutionTimeout
    );
    process_is_absent(descendant_error.context());
    let descendant_pid = fs::read_to_string(descendant_pid_file).expect("descendant pid");
    assert!(!process_is_live(&descendant_pid), "descendant remains live");
    assert!(
        !descendant_marker.exists(),
        "descendant completed after return"
    );

    let detached_pid_file = candidate.root.path().join("closed-pipes-descendant.pid");
    let detached_marker = candidate.root.path().join("closed-pipes-descendant-marker");
    let detached_request = Candidate::request(&[
        "descendant-closes-pipes",
        "500",
        detached_pid_file.to_str().expect("pid file"),
        detached_marker.to_str().expect("descendant marker"),
    ]);
    let detached_error = executor
        .execute(&detached_request)
        .expect_err("surviving descendant must be reported");
    assert_eq!(
        detached_error.code(),
        VerificationErrorCode::UnexpectedProcessDescendant
    );
    process_is_absent(detached_error.context());
    let detached_pid = fs::read_to_string(detached_pid_file).expect("detached pid");
    assert!(
        !process_is_live(&detached_pid),
        "detached descendant remains live"
    );
    assert!(
        !detached_marker.exists(),
        "detached descendant completed after return"
    );
}

#[trace("TC-038", "NFR-006-AC-4")]
#[test]
fn tc_038_accepts_exact_stream_limits_and_reaps_on_the_first_excess_byte() {
    let candidate = Candidate::new();
    let executor = candidate.executor();
    let exact = MAX_NATIVE_OUTPUT_BYTES.to_string();
    let both = executor
        .execute(&Candidate::request(&["both-bytes", &exact]))
        .expect("both exact limits");
    assert_eq!(both.stdout.len(), MAX_NATIVE_OUTPUT_BYTES);
    assert_eq!(both.stderr.len(), MAX_NATIVE_OUTPUT_BYTES);

    let excess = (MAX_NATIVE_OUTPUT_BYTES + 1).to_string();
    for (mode, stream) in [("stdout-bytes", "stdout"), ("stderr-bytes", "stderr")] {
        let error = executor
            .execute(&Candidate::request(&[mode, &excess]))
            .expect_err("first excess byte must fail");
        assert_eq!(error.code(), VerificationErrorCode::OutputLimitExceeded);
        assert_eq!(
            error.context().get("stream").map(String::as_str),
            Some(stream)
        );
        assert_eq!(
            error.context().get("bytes").map(String::as_str),
            Some(excess.as_str())
        );
        assert_eq!(
            error.context().get("limit").map(String::as_str),
            Some(exact.as_str())
        );
        process_is_absent(error.context());
    }
}

#[trace("TC-038", "NFR-006-AC-5")]
#[test]
fn tc_038_returns_distinct_status_signal_encoding_and_protocol_failures() {
    let candidate = Candidate::new();
    let executor = candidate.executor();
    let status = executor
        .execute(&Candidate::request(&["exit", "7"]))
        .expect_err("unexpected status");
    assert_eq!(
        status.code(),
        VerificationErrorCode::UnexpectedExitStatus,
        "{status:?}"
    );
    assert_eq!(
        status.context().get("status").map(String::as_str),
        Some("7")
    );

    let signal = executor
        .execute(&Candidate::request(&["abort"]))
        .expect_err("signal must fail");
    assert_eq!(signal.code(), VerificationErrorCode::ProcessSignaled);
    process_is_absent(signal.context());

    for (mode, stream) in [("invalid-stdout", "stdout"), ("invalid-stderr", "stderr")] {
        let error = executor
            .execute(&Candidate::request(&[mode]))
            .expect_err("invalid UTF-8 must fail");
        assert_eq!(error.code(), VerificationErrorCode::InvalidProcessOutput);
        assert_eq!(
            error.context().get("stream").map(String::as_str),
            Some(stream)
        );
    }

    let malformed = executor
        .execute(&Candidate::request(&["malformed-json"]))
        .expect("process itself succeeds");
    assert_eq!(
        parse_native_json_output(&malformed)
            .expect_err("malformed method response")
            .code(),
        VerificationErrorCode::MalformedMethodResponse
    );
    let valid = executor
        .execute(&Candidate::request(&["valid-json"]))
        .expect("valid process response");
    assert_eq!(
        parse_native_json_output(&valid).expect("valid method response"),
        Value::Object(serde_json::Map::from_iter([(
            "status".to_owned(),
            Value::String("ok".to_owned())
        )]))
    );
}
