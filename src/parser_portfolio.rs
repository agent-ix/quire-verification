// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Bounded VP04 parser protocol execution and conservative result construction.
//!
//! Governing requirement: FR-019.

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rustix::fs::{CWD, FileType, Mode, OFlags, ResolveFlags, fstat, openat2};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::contracts::{
    VerificationError, VerificationErrorCode, contract_version, jcs_equal,
    validate_schema_definition,
};
use crate::evidence::assess_evidence;
use crate::measurement::{measurement_observation, record_measurement_series};
use crate::native_process::{NativeProcessExecutor, NativeProcessOutput, NativeProcessRequest};
use crate::shared_reference::{ValidatedSharedReference, parse_strict_bounded_json};

const MAX_PARSER_SOURCE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_JSON_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

/// Exact source and parse-response identity selected before execution.
#[derive(Clone, Debug)]
pub struct ParserSource {
    /// Absolute source path passed to the parser.
    pub path: PathBuf,
    /// Source identity passed to and expected from the parser.
    pub identity: String,
    /// Source revision passed to and expected from the parser.
    pub revision: String,
    /// Exact SHA-256 byte digest expected from a successful parse.
    pub digest: String,
    /// Exact expected clause count.
    pub clauses: u64,
    /// Exact expected import count.
    pub imports: u64,
}

/// Exact expected syntax-refusal response selected before execution.
#[derive(Clone, Debug)]
pub struct ParserDiagnostic {
    /// Mutated source identity.
    pub identity: String,
    /// Mutated source revision.
    pub revision: String,
    /// Absolute mutated source path.
    pub path: PathBuf,
    /// Exact SHA-256 byte digest of the selected mutation.
    pub digest: String,
    /// Exact diagnostic span and message object.
    pub expected: Value,
}

/// Inputs for one deterministic VP04 parser execution.
#[derive(Clone, Debug)]
pub struct ParserPortfolioRequest {
    /// Unique execution identity.
    pub execution_id: String,
    /// Exact SHA-256 identity of the descriptor-admitted parser candidate.
    pub candidate_digest: String,
    /// RFC 3339 result-production time.
    pub produced_at: String,
    /// Valid source profile.
    pub valid_source: ParserSource,
    /// Seeded mutation and exact diagnostic.
    pub mutation: ParserDiagnostic,
    /// Revision assigned to the first formatted bytes.
    pub formatted_revision: String,
    /// Complete child environment.
    pub environment: BTreeMap<String, String>,
    /// Per-command execution deadline.
    pub deadline_seconds: f64,
    /// Exact validated verification plan.
    pub plan: Value,
    /// Result context for the direct baseline.
    pub direct_context: Value,
    /// Result context for the metamorphic baseline.
    pub roundtrip_context: Value,
    /// Result context for the seeded syntax case.
    pub adverse_context: Value,
    /// Result context for the separately retained manual disposition task.
    pub manual_context: Value,
}

/// Output from the prescribed five-repetition VP04 collection.
#[derive(Clone, Debug)]
pub struct MeasuredParserPortfolio {
    /// The complete portfolio execution from the fifth repetition.
    pub execution: Value,
    /// All 16 raw slots and deterministic phase summaries.
    pub measurements: Value,
}

/// Runs the fixed direct, roundtrip, and complete phases for five sequential repetitions.
///
/// The caller supplies the separately measured setup observation because
/// executable admission begins before an executor exists. Each phase uses a
/// monotonic process-local `Instant`; failed phase results remain invalid timing
/// observations with reasons and cannot enter summaries.
///
/// # Errors
/// Returns an enclosing configuration/schema error or `invalid_measurement_series`.
pub fn collect_parser_portfolio_measurements(
    executor: &NativeProcessExecutor,
    request: &ParserPortfolioRequest,
    validated_references: &[ValidatedSharedReference],
    setup_observation: Value,
    candidate_identity: &str,
    environment_identity: &str,
    context: &Value,
) -> Result<MeasuredParserPortfolio, VerificationError> {
    validate_candidate(executor, request, candidate_identity)?;
    validate_selected_sources(request)?;
    validate_portfolio_tasks(request)?;
    let mut observations = vec![setup_observation];
    let mut last_execution = None;
    let mut last_complete_error = None;
    for repetition in 1..=5 {
        let started = Instant::now();
        let direct = execute_direct(executor, request).map(|(result, _, _)| result);
        observations.push(timed_observation(
            &format!("direct-{repetition}"),
            "direct-parse",
            repetition,
            started,
            direct.as_ref(),
        )?);

        let started = Instant::now();
        let roundtrip = execute_roundtrip(executor, request).map(|(result, _, _)| result);
        observations.push(timed_observation(
            &format!("roundtrip-{repetition}"),
            "parse-format-reparse",
            repetition,
            started,
            roundtrip.as_ref(),
        )?);

        let started = Instant::now();
        let complete = execute_parser_portfolio(executor, request, validated_references);
        observations.push(timed_observation(
            &format!("complete-{repetition}"),
            "complete-run",
            repetition,
            started,
            complete.as_ref(),
        )?);
        match complete {
            Ok(execution) => last_execution = Some(execution),
            Err(error) => last_complete_error = Some(error),
        }
    }
    let execution = last_execution.ok_or_else(|| {
        let mut error = invalid_input(
            "all complete parser-portfolio repetitions failed before producing a record",
        );
        if let Some(cause) = last_complete_error {
            error = error.with_context("last_cause", cause.to_string());
        }
        error
    })?;
    let measurements = record_measurement_series(
        candidate_identity,
        environment_identity,
        &observations,
        context,
    )?;
    Ok(MeasuredParserPortfolio {
        execution,
        measurements,
    })
}

fn timed_observation(
    observation_id: &str,
    phase: &str,
    repetition: u64,
    started: Instant,
    result: Result<&Value, &VerificationError>,
) -> Result<Value, VerificationError> {
    let elapsed = started.elapsed().as_nanos().to_string();
    let reason = match result {
        Ok(outcome) if phase_completed(outcome) => None,
        Ok(_) => Some("phase retained a failed native result"),
        Err(_) => Some("phase returned an enclosing execution error"),
    };
    measurement_observation(observation_id, phase, repetition, &elapsed, reason)
}

fn phase_completed(value: &Value) -> bool {
    if let Some(results) = value.get("results").and_then(Value::as_array) {
        return results.iter().all(|result| {
            let status = result.pointer("/execution/status").and_then(Value::as_str);
            if result["techniqueId"] == "inspection" {
                status == Some("not-run")
            } else {
                status == Some("completed")
            }
        });
    }
    value.pointer("/execution/status").and_then(Value::as_str) == Some("completed")
}

/// Executes direct parse, parse-format-reparse stability, and the seeded syntax case.
///
/// Process and protocol failures become failed `TechniqueResult` values. Invalid
/// configuration, plan/task bindings, result contexts, or assessment inputs fail
/// the enclosing operation.
///
/// # Errors
/// Returns a stable validation or I/O error when the portfolio itself cannot be
/// constructed. Native technique failures are retained inside the returned record.
pub fn execute_parser_portfolio(
    executor: &NativeProcessExecutor,
    request: &ParserPortfolioRequest,
    validated_references: &[ValidatedSharedReference],
) -> Result<Value, VerificationError> {
    validate_candidate(executor, request, &request.candidate_digest)?;
    validate_selected_sources(request)?;
    validate_portfolio_tasks(request)?;

    let (direct, mut commands, mut runs) = execute_direct(executor, request)?;
    let (roundtrip, roundtrip_commands, roundtrip_runs) = execute_roundtrip(executor, request)?;
    commands.extend(roundtrip_commands);
    runs.extend(roundtrip_runs);
    let (adverse, adverse_commands, adverse_runs) = execute_adverse(executor, request)?;
    commands.extend(adverse_commands);
    runs.extend(adverse_runs);
    let manual = manual_technique_result(&request.manual_context, &request.produced_at)?;
    let results = vec![direct, roundtrip, adverse, manual];
    let packet = json!({
        "asOf": request.produced_at,
        "inventory": request.plan.get("inventory").cloned().unwrap_or_else(|| json!([])),
        "results": results,
    });
    let assessment = assess_evidence(&packet, validated_references)?;
    let record = json!({
        "schemaVersion": "quire-verification.vp04-execution/v1",
        "executionId": request.execution_id,
        "plan": request.plan,
        "commands": commands,
        "nativeRuns": runs,
        "results": packet["results"],
        "assessment": assessment,
        "evidenceOwner": "Quoin",
        "sufficiencyDecision": "not-made",
    });
    validate_schema_definition("Vp04ParserPortfolioExecution", &record)?;
    Ok(record)
}

fn validate_candidate(
    executor: &NativeProcessExecutor,
    request: &ParserPortfolioRequest,
    selected_identity: &str,
) -> Result<(), VerificationError> {
    if !valid_byte_digest(&request.candidate_digest)
        || request.candidate_digest != executor.executable_digest()
        || selected_identity != request.candidate_digest
    {
        return Err(VerificationError::new(
            VerificationErrorCode::IdentityMismatch,
            "parser candidate identity differs from the descriptor-admitted executable bytes",
        )
        .with_context("expected", &request.candidate_digest)
        .with_context("admitted", executor.executable_digest())
        .with_context("selected", selected_identity));
    }
    Ok(())
}

fn validate_portfolio_tasks(request: &ParserPortfolioRequest) -> Result<(), VerificationError> {
    validate_schema_definition("VerificationPlan", &request.plan)?;
    validate_result_contexts(request)?;
    validate_context_bindings(request)?;
    validate_native_task(&request.plan, task_id(&request.direct_context)?)?;
    validate_native_task(&request.plan, task_id(&request.roundtrip_context)?)?;
    validate_native_task(&request.plan, task_id(&request.adverse_context)?)?;
    validate_manual_task(&request.plan, task_id(&request.manual_context)?)
}

fn validate_result_contexts(request: &ParserPortfolioRequest) -> Result<(), VerificationError> {
    if request.execution_id.is_empty() {
        return Err(invalid_input("portfolio execution ID is empty"));
    }
    for context in [
        &request.direct_context,
        &request.roundtrip_context,
        &request.adverse_context,
    ] {
        technique_result(
            context,
            &request.produced_at,
            "completed",
            "satisfied",
            "complete",
            Vec::new(),
            None,
        )?;
    }
    manual_technique_result(&request.manual_context, &request.produced_at)?;
    Ok(())
}

fn validate_context_bindings(request: &ParserPortfolioRequest) -> Result<(), VerificationError> {
    let plan_id = request.plan["planId"]
        .as_str()
        .ok_or_else(|| invalid_input("portfolio plan ID is absent"))?;
    let contexts = [
        (&request.direct_context, "unit-testing"),
        (&request.roundtrip_context, "metamorphic-testing"),
        (&request.adverse_context, "unit-testing"),
        (&request.manual_context, "inspection"),
    ];
    let mut result_ids = std::collections::BTreeSet::new();
    for (context, technique) in contexts {
        let result_id = context["resultId"]
            .as_str()
            .ok_or_else(|| invalid_input("portfolio result ID is absent"))?;
        if !result_ids.insert(result_id)
            || context["planId"] != plan_id
            || context["techniqueId"] != technique
        {
            return Err(invalid_input(
                "portfolio result identity or technique differs from its selected role",
            ));
        }
    }
    if task_id(&request.direct_context)? != task_id(&request.adverse_context)?
        || task_id(&request.direct_context)? == task_id(&request.roundtrip_context)?
        || task_id(&request.direct_context)? == task_id(&request.manual_context)?
        || task_id(&request.roundtrip_context)? == task_id(&request.manual_context)?
        || !jcs_equal(
            &request.direct_context["propertyRef"],
            &request.adverse_context["propertyRef"],
        )?
    {
        return Err(invalid_input(
            "portfolio task or adverse-property relationship differs",
        ));
    }
    Ok(())
}

fn validate_manual_task(plan: &Value, task_id: &str) -> Result<(), VerificationError> {
    let task = plan["tasks"]
        .as_array()
        .and_then(|tasks| tasks.iter().find(|task| task["taskId"] == task_id))
        .ok_or_else(|| invalid_input("manual plan task does not exist"))?;
    if task["kind"] != "verification"
        || task["techniqueId"] != "inspection"
        || task["input"]["kind"] != "manual"
        || task["invocationAllowed"] != false
    {
        return Err(invalid_input(
            "manual result context does not name the separate disposition task",
        ));
    }
    Ok(())
}

fn validate_selected_sources(request: &ParserPortfolioRequest) -> Result<(), VerificationError> {
    validate_source_profile(&request.valid_source)?;
    validate_diagnostic_profile(&request.mutation)?;
    if request.formatted_revision.is_empty() {
        return Err(invalid_input("formatted source revision is empty"));
    }
    validate_source_digest(&request.valid_source.path, &request.valid_source.digest)?;
    validate_source_digest(&request.mutation.path, &request.mutation.digest)
}

fn validate_source_profile(source: &ParserSource) -> Result<(), VerificationError> {
    if source.identity.is_empty()
        || source.revision.is_empty()
        || source.clauses > MAX_JSON_SAFE_INTEGER
        || source.imports > MAX_JSON_SAFE_INTEGER
        || !valid_byte_digest(&source.digest)
    {
        return Err(invalid_input(
            "parser source identity, digest, or count is outside its exact domain",
        ));
    }
    source
        .path
        .to_str()
        .ok_or_else(|| invalid_input("parser source path is not UTF-8"))?;
    Ok(())
}

fn validate_diagnostic_profile(diagnostic: &ParserDiagnostic) -> Result<(), VerificationError> {
    if diagnostic.identity.is_empty()
        || diagnostic.revision.is_empty()
        || !valid_byte_digest(&diagnostic.digest)
    {
        return Err(invalid_input(
            "parser mutation identity or digest is outside its exact domain",
        ));
    }
    let path = diagnostic
        .path
        .to_str()
        .ok_or_else(|| invalid_input("parser mutation path is not UTF-8"))?;
    validate_schema_definition("Vp04ParserDiagnosticResponse", &diagnostic.expected)
        .map_err(|_| invalid_input("expected parser diagnostic violates its exact shape"))?;
    if diagnostic.expected["source"]["identity"] != diagnostic.identity
        || diagnostic.expected["source"]["revision"] != diagnostic.revision
        || diagnostic.expected["path"] != path
    {
        return Err(invalid_input(
            "expected parser diagnostic differs from its selected mutation",
        ));
    }
    Ok(())
}

fn valid_byte_digest(value: &str) -> bool {
    value.len() == 71
        && value.strip_prefix("sha256:").is_some_and(|hex| {
            hex.bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        })
}

fn validate_source_digest(path: &Path, expected: &str) -> Result<(), VerificationError> {
    if !path.is_absolute() {
        return Err(invalid_input("parser source path is not absolute"));
    }
    let descriptor = openat2(
        CWD,
        path,
        OFlags::RDONLY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| {
        VerificationError::new(
            VerificationErrorCode::InvalidInput,
            "open exact selected parser source without links",
        )
        .with_context("path", path.display().to_string())
        .with_context("cause", error.to_string())
    })?;
    let metadata = fstat(&descriptor).map_err(|error| {
        VerificationError::new(
            VerificationErrorCode::IoFailure,
            "inspect exact selected parser source descriptor",
        )
        .with_context("path", path.display().to_string())
        .with_context("cause", error.to_string())
    })?;
    if !FileType::from_raw_mode(metadata.st_mode).is_file() {
        return Err(invalid_input(
            "parser source is not a regular non-symlink file",
        ));
    }
    let mut bytes = Vec::with_capacity(
        usize::try_from(metadata.st_size)
            .unwrap_or(usize::MAX)
            .min(64 * 1024),
    );
    File::from(descriptor)
        .take(MAX_PARSER_SOURCE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| io_error("read selected parser source", &error))?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_PARSER_SOURCE_BYTES {
        return Err(invalid_input(
            "parser source exceeds the admitted 16 MiB limit",
        ));
    }
    let actual = byte_digest(&bytes);
    if actual != expected {
        return Err(VerificationError::new(
            VerificationErrorCode::IdentityMismatch,
            "selected parser source bytes differ from their expected digest",
        )
        .with_context("path", path.display().to_string())
        .with_context("expected", expected)
        .with_context("actual", actual));
    }
    Ok(())
}

/// Refuses manual, blocked, unsupported, or otherwise non-invocable plan tasks.
///
/// # Errors
/// Returns `invalid_input` unless the named task is ready native verification work.
pub fn validate_native_task(plan: &Value, task_id: &str) -> Result<(), VerificationError> {
    let tasks = plan
        .get("tasks")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_input("plan tasks are absent"))?;
    let Some(task) = tasks
        .iter()
        .find(|task| task.get("taskId").and_then(Value::as_str) == Some(task_id))
    else {
        return Err(invalid_input("plan task does not exist").with_context("task_id", task_id));
    };
    let invocable = task.get("kind").and_then(Value::as_str) == Some("verification")
        && task.get("readiness").and_then(Value::as_str) == Some("ready")
        && task.get("invocationAllowed").and_then(Value::as_bool) == Some(true)
        && !task.get("runnerRequest").is_none_or(Value::is_null);
    if !invocable {
        return Err(
            invalid_input("plan task is not ready native verification work")
                .with_context("task_id", task_id),
        );
    }
    Ok(())
}

fn execute_direct(
    executor: &NativeProcessExecutor,
    request: &ParserPortfolioRequest,
) -> Result<(Value, Vec<Value>, Vec<Value>), VerificationError> {
    let args = parser_args("parse", &request.valid_source)?;
    let command = command("direct-parse", &args, 0, "parsed-json");
    let process = execute(executor, &args, 0, request).and_then(|output| {
        validate_source_digest(&request.valid_source.path, &request.valid_source.digest)?;
        Ok(output)
    });
    let (result, run) = match process {
        Ok(output) => match validate_parsed_response(&output, &request.valid_source) {
            Ok(()) => (
                technique_result(
                    &request.direct_context,
                    &request.produced_at,
                    "completed",
                    "satisfied",
                    "complete",
                    artifacts("direct-parse", &output, ResponseKind::Json),
                    None,
                )?,
                completed_run(
                    "direct-parse-run",
                    "direct-parse",
                    &output,
                    ResponseKind::Json,
                ),
            ),
            Err(error) => failed_technique(
                &request.direct_context,
                &request.produced_at,
                "direct-parse-run",
                "direct-parse",
                &error,
            )?,
        },
        Err(error) => failed_technique(
            &request.direct_context,
            &request.produced_at,
            "direct-parse-run",
            "direct-parse",
            &error,
        )?,
    };
    Ok((result, vec![command], vec![run]))
}

fn execute_roundtrip(
    executor: &NativeProcessExecutor,
    request: &ParserPortfolioRequest,
) -> Result<(Value, Vec<Value>, Vec<Value>), VerificationError> {
    let first_args = parser_args("format", &request.valid_source)?;
    let first_command = command("roundtrip-format-1", &first_args, 0, "formatted-source");
    let first = match execute(executor, &first_args, 0, request).and_then(|output| {
        validate_source_digest(&request.valid_source.path, &request.valid_source.digest)?;
        verify_format(&output).map(|()| output)
    }) {
        Ok(output) => output,
        Err(error) => {
            let (result, run) = failed_technique(
                &request.roundtrip_context,
                &request.produced_at,
                "roundtrip-format-1-run",
                "roundtrip-format-1",
                &error,
            )?;
            return Ok((result, vec![first_command], vec![run]));
        }
    };
    let workspace = request
        .valid_source
        .path
        .parent()
        .ok_or_else(|| invalid_input("valid source has no parent directory"))?;
    let formatted_file = tempfile::Builder::new()
        .prefix(".quire-verification-formatted-")
        .suffix(".native")
        .tempfile_in(workspace)
        .map_err(|error| io_error("create formatted parser input", &error))?;
    fs::write(formatted_file.path(), first.stdout.as_bytes())
        .map_err(|error| io_error("write formatted parser input", &error))?;
    let formatted_source = ParserSource {
        path: formatted_file.path().to_path_buf(),
        identity: request.valid_source.identity.clone(),
        revision: request.formatted_revision.clone(),
        digest: byte_digest(first.stdout.as_bytes()),
        clauses: request.valid_source.clauses,
        imports: request.valid_source.imports,
    };
    let parse_args = parser_args("parse", &formatted_source)?;
    let second_args = parser_args("format", &formatted_source)?;
    let parse_command = command("roundtrip-parse", &parse_args, 0, "parsed-json");
    let second_command = command("roundtrip-format-2", &second_args, 0, "formatted-source");
    let commands = vec![first_command, parse_command, second_command];
    let mut runs = vec![completed_run(
        "roundtrip-format-1-run",
        "roundtrip-format-1",
        &first,
        ResponseKind::Text,
    )];
    let parsed = match execute(executor, &parse_args, 0, request).and_then(|output| {
        validate_source_digest(&formatted_source.path, &formatted_source.digest)?;
        validate_parsed_response(&output, &formatted_source).map(|()| output)
    }) {
        Ok(output) => output,
        Err(error) => {
            let (result, run) = failed_technique(
                &request.roundtrip_context,
                &request.produced_at,
                "roundtrip-parse-run",
                "roundtrip-parse",
                &error,
            )?;
            runs.push(run);
            return Ok((result, commands, runs));
        }
    };
    runs.push(completed_run(
        "roundtrip-parse-run",
        "roundtrip-parse",
        &parsed,
        ResponseKind::Json,
    ));
    let second = match execute(executor, &second_args, 0, request).and_then(|output| {
        validate_source_digest(&formatted_source.path, &formatted_source.digest)?;
        verify_format(&output).map(|()| output)
    }) {
        Ok(output) => output,
        Err(error) => {
            let (result, run) = failed_technique(
                &request.roundtrip_context,
                &request.produced_at,
                "roundtrip-format-2-run",
                "roundtrip-format-2",
                &error,
            )?;
            runs.push(run);
            return Ok((result, commands, runs));
        }
    };
    runs.push(completed_run(
        "roundtrip-format-2-run",
        "roundtrip-format-2",
        &second,
        ResponseKind::Text,
    ));
    let result = completed_roundtrip_result(request, &first, &parsed, &second)?;
    Ok((result, commands, runs))
}

fn completed_roundtrip_result(
    request: &ParserPortfolioRequest,
    first: &NativeProcessOutput,
    parsed: &NativeProcessOutput,
    second: &NativeProcessOutput,
) -> Result<Value, VerificationError> {
    let matches = first.stdout == second.stdout;
    let mut native_artifacts = artifacts("roundtrip-format-1", first, ResponseKind::Text);
    native_artifacts.extend(artifacts("roundtrip-parse", parsed, ResponseKind::Json));
    native_artifacts.extend(artifacts("roundtrip-format-2", second, ResponseKind::Text));
    technique_result(
        &request.roundtrip_context,
        &request.produced_at,
        "completed",
        if matches { "satisfied" } else { "violated" },
        "complete",
        native_artifacts,
        None,
    )
}

fn execute_adverse(
    executor: &NativeProcessExecutor,
    request: &ParserPortfolioRequest,
) -> Result<(Value, Vec<Value>, Vec<Value>), VerificationError> {
    let source = ParserSource {
        path: request.mutation.path.clone(),
        identity: request.mutation.identity.clone(),
        revision: request.mutation.revision.clone(),
        digest: String::new(),
        clauses: 0,
        imports: 0,
    };
    let args = parser_args("parse", &source)?;
    let command = command("syntax-refusal", &args, 1, "diagnostic-json");
    let process = execute(executor, &args, 1, request).and_then(|output| {
        validate_source_digest(&request.mutation.path, &request.mutation.digest)?;
        Ok(output)
    });
    let (result, run) = match process {
        Ok(output) => match validate_diagnostic_response(&output, &request.mutation.expected) {
            Ok(()) => (
                technique_result(
                    &request.adverse_context,
                    &request.produced_at,
                    "completed",
                    "violated",
                    "complete",
                    artifacts("syntax-refusal", &output, ResponseKind::Json),
                    None,
                )?,
                completed_run(
                    "syntax-refusal-run",
                    "syntax-refusal",
                    &output,
                    ResponseKind::Json,
                ),
            ),
            Err(error) => failed_technique(
                &request.adverse_context,
                &request.produced_at,
                "syntax-refusal-run",
                "syntax-refusal",
                &error,
            )?,
        },
        Err(error) => failed_technique(
            &request.adverse_context,
            &request.produced_at,
            "syntax-refusal-run",
            "syntax-refusal",
            &error,
        )?,
    };
    Ok((result, vec![command], vec![run]))
}

fn execute(
    executor: &NativeProcessExecutor,
    arguments: &[String],
    expected_exit_code: i32,
    request: &ParserPortfolioRequest,
) -> Result<NativeProcessOutput, VerificationError> {
    executor.execute(&NativeProcessRequest {
        arguments: arguments.to_vec(),
        environment: request.environment.clone(),
        deadline_seconds: Some(request.deadline_seconds),
        expected_exit_code: Some(expected_exit_code),
    })
}

/// Validates one successful parse response against its exact selected identity.
///
/// # Errors
/// Returns a protocol error for any status, stream, JSON, field, or value difference.
pub fn validate_parsed_response(
    output: &NativeProcessOutput,
    source: &ParserSource,
) -> Result<(), VerificationError> {
    validate_source_profile(source)
        .map_err(|_| protocol_error("selected parse expectation is outside its exact domain"))?;
    if output.exit_code != 0 || !output.stderr.is_empty() {
        return Err(protocol_error(
            "successful parse used the wrong status or stream",
        ));
    }
    let actual = exact_json(&output.stdout)?;
    let path = source
        .path
        .to_str()
        .ok_or_else(|| protocol_error("selected parser path is not UTF-8"))?;
    let expected = json!({
        "status": "parsed",
        "source": {"identity": source.identity, "revision": source.revision, "digest": source.digest},
        "path": path,
        "imports": source.imports,
        "clauses": source.clauses,
    });
    if !jcs_equal(&actual, &expected)? {
        return Err(protocol_error(
            "parse response differs from the exact expectation",
        ));
    }
    Ok(())
}

fn verify_format(output: &NativeProcessOutput) -> Result<(), VerificationError> {
    if output.exit_code != 0 || !output.stderr.is_empty() || output.stdout.is_empty() {
        return Err(protocol_error(
            "format response used the wrong status or stream",
        ));
    }
    Ok(())
}

/// Validates one expected exit-1 diagnostic against an exact selected object.
///
/// # Errors
/// Returns a protocol error for any status, stream, JSON, field, or value difference.
pub fn validate_diagnostic_response(
    output: &NativeProcessOutput,
    expected: &Value,
) -> Result<(), VerificationError> {
    if output.exit_code != 1 || !output.stdout.is_empty() {
        return Err(protocol_error(
            "syntax diagnostic used the wrong status or stream",
        ));
    }
    let actual = exact_json(&output.stderr)?;
    validate_schema_definition("Vp04ParserDiagnosticResponse", expected)
        .map_err(|_| protocol_error("selected syntax diagnostic expectation is malformed"))?;
    validate_schema_definition("Vp04ParserDiagnosticResponse", &actual)
        .map_err(|_| protocol_error("native syntax diagnostic response is malformed"))?;
    if !jcs_equal(&actual, expected)? {
        return Err(protocol_error(
            "syntax diagnostic differs from the exact expectation",
        ));
    }
    Ok(())
}

fn exact_json(text: &str) -> Result<Value, VerificationError> {
    let value = parse_strict_bounded_json(text.as_bytes()).map_err(|error| {
        protocol_error(format!(
            "native response is not exactly one bounded JSON value: {error}"
        ))
    })?;
    if !value.is_object() {
        return Err(protocol_error("native JSON response is not an object"));
    }
    Ok(value)
}

fn technique_result(
    context: &Value,
    produced_at: &str,
    execution: &str,
    logical: &str,
    participation: &str,
    native_artifacts: Vec<Value>,
    error_code: Option<&str>,
) -> Result<Value, VerificationError> {
    let mut fields = context
        .as_object()
        .cloned()
        .ok_or_else(|| invalid_input("result context is not an object"))?;
    fields.insert("contractVersion".into(), json!(contract_version()));
    fields.insert("kind".into(), json!("TechniqueResult"));
    fields.insert("producedAt".into(), json!(produced_at));
    fields.insert(
        "execution".into(),
        error_code.map_or_else(
            || json!({"status": execution}),
            |code| json!({"status": execution, "errorCode": code}),
        ),
    );
    fields.insert("logical".into(), json!({"outcome": logical}));
    fields.insert("activation".into(), json!({"status": "activated"}));
    fields.insert("participation".into(), json!({"status": participation}));
    fields.insert("nativeArtifacts".into(), Value::Array(native_artifacts));
    let value = Value::Object(fields);
    validate_schema_definition("TechniqueResult", &value)?;
    Ok(value)
}

fn manual_technique_result(context: &Value, produced_at: &str) -> Result<Value, VerificationError> {
    let mut result = technique_result(
        context,
        produced_at,
        "not-run",
        "not-evaluated",
        "not-started",
        Vec::new(),
        None,
    )?;
    result["activation"] = json!({"status": "not-required"});
    validate_schema_definition("TechniqueResult", &result)?;
    Ok(result)
}

fn failed_technique(
    context: &Value,
    produced_at: &str,
    run_id: &str,
    command_id: &str,
    error: &VerificationError,
) -> Result<(Value, Value), VerificationError> {
    let code = error.code().as_str();
    Ok((
        technique_result(
            context,
            produced_at,
            "failed",
            "not-evaluated",
            "partial",
            Vec::new(),
            Some(code),
        )?,
        failed_run(run_id, command_id, code),
    ))
}

fn parser_args(command: &str, source: &ParserSource) -> Result<Vec<String>, VerificationError> {
    let path = source
        .path
        .to_str()
        .ok_or_else(|| invalid_input("parser source path is not UTF-8"))?;
    Ok(vec![
        command.to_owned(),
        source.identity.clone(),
        source.revision.clone(),
        path.to_owned(),
    ])
}

fn task_id(context: &Value) -> Result<&str, VerificationError> {
    context
        .get("taskId")
        .and_then(Value::as_str)
        .ok_or_else(|| invalid_input("result context taskId is absent"))
}

fn command(id: &str, arguments: &[String], status: i32, kind: &str) -> Value {
    json!({"commandId": id, "arguments": arguments, "expectedExitCode": status, "responseKind": kind})
}

fn completed_run(
    run_id: &str,
    command_id: &str,
    output: &NativeProcessOutput,
    response_kind: ResponseKind,
) -> Value {
    let artifacts = artifacts(command_id, output, response_kind);
    json!({
        "runId": run_id,
        "commandId": command_id,
        "status": "completed",
        "observedExitCode": output.exit_code,
        "stdoutArtifact": artifacts.iter().find(|item| item["uri"].as_str().is_some_and(|uri| uri.ends_with("stdout"))).cloned().unwrap_or(Value::Null),
        "stderrArtifact": artifacts.iter().find(|item| item["uri"].as_str().is_some_and(|uri| uri.ends_with("stderr"))).cloned().unwrap_or(Value::Null),
        "errorCode": Value::Null,
    })
}

fn failed_run(run_id: &str, command_id: &str, code: &str) -> Value {
    json!({"runId": run_id, "commandId": command_id, "status": "failed", "observedExitCode": Value::Null, "stdoutArtifact": Value::Null, "stderrArtifact": Value::Null, "errorCode": code})
}

#[derive(Clone, Copy)]
enum ResponseKind {
    Json,
    Text,
}

fn artifacts(
    command_id: &str,
    output: &NativeProcessOutput,
    response_kind: ResponseKind,
) -> Vec<Value> {
    let media_type = match response_kind {
        ResponseKind::Json => "application/json",
        ResponseKind::Text => "text/plain",
    };
    let mut artifacts = Vec::new();
    if !output.stdout.is_empty() {
        artifacts.push(native_artifact(
            command_id,
            "stdout",
            output.stdout.as_bytes(),
            media_type,
        ));
    }
    if !output.stderr.is_empty() {
        artifacts.push(native_artifact(
            command_id,
            "stderr",
            output.stderr.as_bytes(),
            media_type,
        ));
    }
    artifacts
}

fn native_artifact(command_id: &str, stream: &str, bytes: &[u8], media_type: &str) -> Value {
    json!({
        "uri": format!("vp04-native:{command_id}:{stream}"),
        "digest": byte_digest(bytes),
        "mediaType": media_type,
    })
}

fn byte_digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn protocol_error(message: impl Into<String>) -> VerificationError {
    VerificationError::new(
        VerificationErrorCode::MalformedMethodResponse,
        message.into(),
    )
}

fn invalid_input(message: impl Into<String>) -> VerificationError {
    VerificationError::new(VerificationErrorCode::InvalidInput, message.into())
}

fn io_error(action: &str, error: &std::io::Error) -> VerificationError {
    VerificationError::new(
        VerificationErrorCode::IoFailure,
        format!("{action}: {error}"),
    )
}
