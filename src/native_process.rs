// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Descriptor-pinned, process-group-bounded native execution for Linux.
//!
//! Governing requirement: NFR-006.

use std::collections::BTreeMap;
use std::io::Read;
use std::os::fd::{AsRawFd, OwnedFd};
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::{Component, Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use rustix::fs::{CWD, FileType, Mode, OFlags, ResolveFlags, fstat, openat2};
use rustix::io::Errno;
use rustix::process::{Pid, Signal, kill_process_group, test_kill_process_group};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::contracts::{VerificationError, VerificationErrorCode, validate_resource_envelope};

/// Largest accepted stdout or stderr stream, in bytes.
pub const MAX_NATIVE_OUTPUT_BYTES: usize = 16 * 1024 * 1024;
/// Largest accepted per-invocation deadline, in seconds.
pub const MAX_NATIVE_DEADLINE_SECONDS: f64 = 10.0;

const MONITOR_INTERVAL: Duration = Duration::from_millis(1);
const PROCESS_GROUP_CLEANUP_BUDGET: Duration = Duration::from_secs(1);
const PROC_DESCRIPTOR_ROOT: &str = "/proc/self/fd";
const MAX_NATIVE_EXECUTABLE_BYTES: usize = 128 * 1024 * 1024;

/// Exact per-invocation inputs for one admitted native executable.
#[derive(Clone, Debug)]
pub struct NativeProcessRequest {
    /// Separate argument vector passed without shell interpretation.
    pub arguments: Vec<String>,
    /// Complete child environment after inherited entries are cleared.
    pub environment: BTreeMap<String, String>,
    /// Positive finite wall-clock budget no greater than ten seconds.
    pub deadline_seconds: Option<f64>,
    /// Exact numeric exit code admitted for this invocation.
    pub expected_exit_code: Option<i32>,
}

/// UTF-8 output from one successfully completed native child.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeProcessOutput {
    /// Exact numeric exit code observed from the direct child.
    pub exit_code: i32,
    /// Complete standard output through the admitted limit.
    pub stdout: String,
    /// Complete standard error through the admitted limit.
    pub stderr: String,
}

/// A root and executable pinned by Linux descriptors before any invocation.
#[derive(Debug)]
pub struct NativeProcessExecutor {
    root: OwnedFd,
    executable: OwnedFd,
    executable_digest: String,
}

impl NativeProcessExecutor {
    /// Admits one exact absolute regular executable beneath an exact absolute root.
    ///
    /// Linux `openat2` resolves the root and root-relative executable without
    /// symbolic links or magic links. The returned executor retains both
    /// descriptors, so later pathname replacement cannot substitute a candidate.
    ///
    /// # Errors
    /// Returns `invalid_executable_path` for a non-exact, escaping, missing,
    /// linked, or non-regular candidate. Returns
    /// `unsupported_execution_profile` when required Linux facilities are absent.
    pub fn admit(root: &Path, executable: &Path) -> Result<Self, VerificationError> {
        validate_lexical_paths(root, executable)?;
        if !Path::new(PROC_DESCRIPTOR_ROOT).is_dir() {
            return Err(unsupported_profile("/proc/self/fd is unavailable"));
        }
        let root_descriptor = openat2(
            CWD,
            root,
            OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
            ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
        )
        .map_err(|error| descriptor_open_error(error, root))?;
        let relative = executable
            .strip_prefix(root)
            .map_err(|_| invalid_executable(executable, "candidate escapes execution root"))?;
        if relative.as_os_str().is_empty() || has_non_normal_component(relative) {
            return Err(invalid_executable(
                executable,
                "candidate is not an exact root-relative path",
            ));
        }
        let executable_descriptor = openat2(
            &root_descriptor,
            relative,
            OFlags::RDONLY | OFlags::CLOEXEC,
            Mode::empty(),
            ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
        )
        .map_err(|error| descriptor_open_error(error, executable))?;
        let metadata = fstat(&executable_descriptor)
            .map_err(|error| invalid_executable(executable, error.to_string()))?;
        if !FileType::from_raw_mode(metadata.st_mode).is_file() {
            return Err(invalid_executable(
                executable,
                "candidate is not a regular file",
            ));
        }
        let executable_digest = digest_executable(&executable_descriptor, executable)?;
        Ok(Self {
            root: root_descriptor,
            executable: executable_descriptor,
            executable_digest,
        })
    }

    /// Returns the SHA-256 identity of the descriptor-admitted executable bytes.
    #[must_use]
    pub fn executable_digest(&self) -> &str {
        &self.executable_digest
    }

    /// Executes one request through the retained descriptor in a fresh directory.
    ///
    /// # Errors
    /// Returns a stable typed error before launch for an invalid deadline or exit status, or
    /// after group termination and direct-child reaping for deadline, stream,
    /// signal, status, descendant, pipe, spawn, and UTF-8 failures.
    pub fn execute(
        &self,
        request: &NativeProcessRequest,
    ) -> Result<NativeProcessOutput, VerificationError> {
        let deadline = validate_deadline(request.deadline_seconds)?;
        let expected_exit_code = validate_expected_exit_code(request.expected_exit_code)?;
        let root_path = descriptor_path(&self.root);
        let run_directory = tempfile::Builder::new()
            .prefix(".quire-verification-run-")
            .tempdir_in(&root_path)
            .map_err(|error| io_error("create fresh native run directory", &error))?;
        let executable_path = descriptor_path(&self.executable);
        let mut command = Command::new(executable_path);
        command
            .args(&request.arguments)
            .env_clear()
            .envs(&request.environment)
            .current_dir(run_directory.path())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .process_group(0);
        let mut child = command
            .spawn()
            .map_err(|error| io_error("spawn descriptor-admitted native child", &error))?;
        let pid = child.id();
        let Some(group) = process_group_id(pid) else {
            return Err(reap_after_setup_failure(
                &mut child,
                pid,
                "native child pid is outside the Linux process-id domain",
            ));
        };
        let Some(stdout) = child.stdout.take() else {
            return Err(reap_after_setup_failure(
                &mut child,
                pid,
                "native child stdout pipe was unavailable",
            ));
        };
        let Some(stderr) = child.stderr.take() else {
            return Err(reap_after_setup_failure(
                &mut child,
                pid,
                "native child stderr pipe was unavailable",
            ));
        };
        let (sender, receiver) = mpsc::channel();
        let stdout_reader = spawn_reader(stdout, Stream::Stdout, sender.clone());
        let stderr_reader = spawn_reader(stderr, Stream::Stderr, sender);
        monitor_child(
            RunningProcess {
                child,
                group,
                pid,
                receiver,
                stdout_reader,
                stderr_reader,
            },
            deadline,
            expected_exit_code,
        )
    }
}

fn digest_executable(descriptor: &OwnedFd, path: &Path) -> Result<String, VerificationError> {
    let mut digest = Sha256::new();
    let mut total = 0_usize;
    let mut buffer = vec![0_u8; 64 * 1024].into_boxed_slice();
    loop {
        let read = rustix::io::read(descriptor, &mut buffer[..])
            .map_err(|error| invalid_executable(path, format!("read candidate bytes: {error}")))?;
        if read == 0 {
            break;
        }
        total = total
            .checked_add(read)
            .ok_or_else(|| invalid_executable(path, "candidate size overflow"))?;
        if total > MAX_NATIVE_EXECUTABLE_BYTES {
            return Err(invalid_executable(
                path,
                "candidate exceeds the 128 MiB admission limit",
            ));
        }
        digest.update(&buffer[..read]);
    }
    Ok(format!("sha256:{:x}", digest.finalize()))
}

/// Parses a successful stdout stream as one bounded JSON method response.
///
/// # Errors
/// Returns `malformed_method_response` for invalid JSON or a bounded-resource
/// error when the parsed value exceeds the shared JSON envelope.
pub fn parse_native_json_output(output: &NativeProcessOutput) -> Result<Value, VerificationError> {
    let value = serde_json::from_str(&output.stdout).map_err(|error| {
        VerificationError::new(
            VerificationErrorCode::MalformedMethodResponse,
            error.to_string(),
        )
    })?;
    validate_resource_envelope(&value)?;
    Ok(value)
}

#[derive(Clone, Copy, Debug)]
enum Stream {
    Stdout,
    Stderr,
}

impl Stream {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Stdout => "stdout",
            Self::Stderr => "stderr",
        }
    }
}

#[derive(Debug)]
enum CaptureEvent {
    Done(Stream),
    Failed(CaptureFailure),
}

#[derive(Clone, Debug)]
enum CaptureFailure {
    Timeout,
    Limit {
        stream: Stream,
        bytes: usize,
    },
    Read {
        stream: Stream,
        message: String,
    },
    Monitor(String),
    Exit {
        status: ExitStatus,
        expected_status: i32,
    },
    UnexpectedDescendant,
}

type Reader = JoinHandle<Result<Vec<u8>, CaptureFailure>>;

struct RunningProcess {
    child: Child,
    group: Pid,
    pid: u32,
    receiver: Receiver<CaptureEvent>,
    stdout_reader: Reader,
    stderr_reader: Reader,
}

fn monitor_child(
    mut running: RunningProcess,
    deadline: Duration,
    expected_exit_code: i32,
) -> Result<NativeProcessOutput, VerificationError> {
    let started = Instant::now();
    let mut child_exited = false;
    let mut observed_exit_code = None;
    let mut stdout_done = false;
    let mut stderr_done = false;
    loop {
        while let Ok(event) = running.receiver.try_recv() {
            if let Some(failure) = apply_event(event, &mut stdout_done, &mut stderr_done) {
                return terminate_group_reap_and_join(running, failure);
            }
        }
        if !child_exited {
            match running.child.try_wait() {
                Ok(Some(status)) if status.code() == Some(expected_exit_code) => {
                    child_exited = true;
                    observed_exit_code = Some(expected_exit_code);
                }
                Ok(Some(status)) => {
                    return terminate_group_reap_and_join(
                        running,
                        CaptureFailure::Exit {
                            status,
                            expected_status: expected_exit_code,
                        },
                    );
                }
                Ok(None) => {}
                Err(error) => {
                    return terminate_group_reap_and_join(
                        running,
                        CaptureFailure::Monitor(error.to_string()),
                    );
                }
            }
        }
        if child_exited && stdout_done && stderr_done {
            match process_group_exists(running.group) {
                Ok(false) => break,
                Ok(true) => {
                    return terminate_group_reap_and_join(
                        running,
                        CaptureFailure::UnexpectedDescendant,
                    );
                }
                Err(error) => {
                    return terminate_group_reap_and_join(
                        running,
                        CaptureFailure::Monitor(error.to_string()),
                    );
                }
            }
        }
        let elapsed = started.elapsed();
        if elapsed >= deadline {
            return terminate_group_reap_and_join(running, CaptureFailure::Timeout);
        }
        let wait = deadline.saturating_sub(elapsed).min(MONITOR_INTERVAL);
        match running.receiver.recv_timeout(wait) {
            Ok(event) => {
                if let Some(failure) = apply_event(event, &mut stdout_done, &mut stderr_done) {
                    return terminate_group_reap_and_join(running, failure);
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) if stdout_done && stderr_done => {}
            Err(RecvTimeoutError::Disconnected) => {
                return terminate_group_reap_and_join(
                    running,
                    CaptureFailure::Monitor(
                        "native stream monitors disconnected before completion".to_owned(),
                    ),
                );
            }
        }
    }

    let stdout = join_reader(running.stdout_reader, Stream::Stdout, running.pid);
    let stderr = join_reader(running.stderr_reader, Stream::Stderr, running.pid);
    let stdout = decode_stream(stdout?, Stream::Stdout, running.pid)?;
    let stderr = decode_stream(stderr?, Stream::Stderr, running.pid)?;
    let exit_code = observed_exit_code.ok_or_else(|| {
        VerificationError::new(
            VerificationErrorCode::IoFailure,
            "native child completed without an observed exit status",
        )
    })?;
    Ok(NativeProcessOutput {
        exit_code,
        stdout,
        stderr,
    })
}

fn apply_event(
    event: CaptureEvent,
    stdout_done: &mut bool,
    stderr_done: &mut bool,
) -> Option<CaptureFailure> {
    match event {
        CaptureEvent::Done(Stream::Stdout) => *stdout_done = true,
        CaptureEvent::Done(Stream::Stderr) => *stderr_done = true,
        CaptureEvent::Failed(failure) => return Some(failure),
    }
    None
}

fn spawn_reader<R>(reader: R, stream: Stream, sender: Sender<CaptureEvent>) -> Reader
where
    R: Read + Send + 'static,
{
    thread::spawn(move || read_capped(reader, stream, &sender))
}

fn read_capped(
    mut reader: impl Read,
    stream: Stream,
    sender: &Sender<CaptureEvent>,
) -> Result<Vec<u8>, CaptureFailure> {
    let mut output = Vec::with_capacity(MAX_NATIVE_OUTPUT_BYTES.min(64 * 1024));
    let mut buffer = [0_u8; 16 * 1024];
    loop {
        let allowed_read = (MAX_NATIVE_OUTPUT_BYTES + 1)
            .saturating_sub(output.len())
            .min(buffer.len());
        let read = match reader.read(&mut buffer[..allowed_read]) {
            Ok(read) => read,
            Err(error) => {
                let failure = CaptureFailure::Read {
                    stream,
                    message: error.to_string(),
                };
                let _ = sender.send(CaptureEvent::Failed(failure.clone()));
                return Err(failure);
            }
        };
        if read == 0 {
            let _ = sender.send(CaptureEvent::Done(stream));
            return Ok(output);
        }
        output.extend_from_slice(&buffer[..read]);
        if output.len() > MAX_NATIVE_OUTPUT_BYTES {
            let failure = CaptureFailure::Limit {
                stream,
                bytes: output.len(),
            };
            let _ = sender.send(CaptureEvent::Failed(failure.clone()));
            output.truncate(MAX_NATIVE_OUTPUT_BYTES);
            return Err(failure);
        }
    }
}

fn join_reader(reader: Reader, stream: Stream, pid: u32) -> Result<Vec<u8>, VerificationError> {
    match reader.join() {
        Ok(Ok(output)) => Ok(output),
        Ok(Err(failure)) => Err(capture_error(failure, pid, true, true)),
        Err(_) => Err(VerificationError::new(
            VerificationErrorCode::IoFailure,
            "native stream reader panicked",
        )
        .with_context("stream", stream.as_str())
        .with_context("pid", pid.to_string())
        .with_context("reaped", "true")
        .with_context("process_group_terminated", "true")),
    }
}

fn terminate_group_reap_and_join(
    mut running: RunningProcess,
    failure: CaptureFailure,
) -> Result<NativeProcessOutput, VerificationError> {
    let cleanup = cleanup_process_group(&mut running.child, running.group);
    if cleanup.group_terminated {
        let _ = running.stdout_reader.join();
        let _ = running.stderr_reader.join();
        return Err(capture_error(failure, running.pid, cleanup.reaped, true));
    }
    Err(containment_failure(&failure, running.pid, cleanup))
}

fn signal_process_group(group: Pid) -> Result<(), Errno> {
    match kill_process_group(group, Signal::KILL) {
        Ok(()) | Err(Errno::SRCH) => Ok(()),
        Err(error) => Err(error),
    }
}

#[derive(Debug)]
struct CleanupStatus {
    reaped: bool,
    group_terminated: bool,
    signal_error: Option<String>,
    fallback_error: Option<String>,
    reap_error: Option<String>,
    observation_error: Option<Errno>,
}

fn cleanup_process_group(child: &mut Child, group: Pid) -> CleanupStatus {
    let cleanup_deadline = Instant::now()
        .checked_add(PROCESS_GROUP_CLEANUP_BUDGET)
        .unwrap_or_else(Instant::now);
    let signal_error = signal_process_group(group)
        .err()
        .map(|error| error.to_string());
    let fallback_error = signal_error
        .as_ref()
        .and_then(|_| child.kill().err())
        .map(|error| error.to_string());
    let reap_error = wait_for_child_reap_until(child, cleanup_deadline).err();
    let mut observation_error = reap_error.as_ref().map_or_else(
        || wait_for_process_group_exit_until(group, cleanup_deadline).err(),
        |_| Some(Errno::TIMEDOUT),
    );
    if Instant::now() > cleanup_deadline && observation_error.is_none() {
        observation_error = Some(Errno::TIMEDOUT);
    }
    let group_terminated = signal_error.is_none()
        && fallback_error.is_none()
        && reap_error.is_none()
        && observation_error.is_none();
    CleanupStatus {
        reaped: reap_error.is_none(),
        group_terminated,
        signal_error,
        fallback_error,
        reap_error,
        observation_error,
    }
}

fn wait_for_child_reap_until(child: &mut Child, deadline: Instant) -> Result<(), String> {
    loop {
        match child.try_wait() {
            Ok(Some(_)) => return Ok(()),
            Ok(None) if Instant::now() < deadline => {
                thread::park_timeout(
                    MONITOR_INTERVAL.min(deadline.saturating_duration_since(Instant::now())),
                );
            }
            Ok(None) => return Err("cleanup budget expired before direct-child reap".to_owned()),
            Err(error) => return Err(error.to_string()),
        }
    }
}

fn wait_for_process_group_exit_until(group: Pid, deadline: Instant) -> Result<(), Errno> {
    wait_for_process_group_exit_with(deadline.saturating_duration_since(Instant::now()), || {
        process_group_exists(group)
    })
}

fn wait_for_process_group_exit_with(
    budget: Duration,
    mut group_exists: impl FnMut() -> Result<bool, Errno>,
) -> Result<(), Errno> {
    let started = Instant::now();
    loop {
        if !group_exists()? {
            return Ok(());
        }
        let elapsed = started.elapsed();
        if elapsed >= budget {
            return Err(Errno::TIMEDOUT);
        }
        thread::park_timeout(budget.saturating_sub(elapsed).min(MONITOR_INTERVAL));
    }
}

fn process_group_exists(group: Pid) -> Result<bool, Errno> {
    match test_kill_process_group(group) {
        Ok(()) => Ok(true),
        Err(Errno::SRCH) => Ok(false),
        Err(error) => Err(error),
    }
}

fn capture_error(
    failure: CaptureFailure,
    pid: u32,
    reaped: bool,
    group_terminated: bool,
) -> VerificationError {
    let error = match failure {
        CaptureFailure::Timeout => VerificationError::new(
            VerificationErrorCode::ExecutionTimeout,
            "native process group exceeded its execution deadline",
        ),
        CaptureFailure::Limit { stream, bytes } => VerificationError::new(
            VerificationErrorCode::OutputLimitExceeded,
            "native process group exceeded a stream output limit",
        )
        .with_context("stream", stream.as_str())
        .with_context("bytes", bytes.to_string())
        .with_context("limit", MAX_NATIVE_OUTPUT_BYTES.to_string()),
        CaptureFailure::Read { stream, message } => {
            VerificationError::new(VerificationErrorCode::IoFailure, message)
                .with_context("stream", stream.as_str())
        }
        CaptureFailure::Monitor(message) => VerificationError::new(
            VerificationErrorCode::IoFailure,
            format!("monitor native process group: {message}"),
        ),
        CaptureFailure::Exit {
            status,
            expected_status,
        } => exit_error(status, expected_status),
        CaptureFailure::UnexpectedDescendant => VerificationError::new(
            VerificationErrorCode::UnexpectedProcessDescendant,
            "native process group remained after its direct child completed",
        ),
    };
    error
        .with_context("pid", pid.to_string())
        .with_context("process_group", pid.to_string())
        .with_context("reaped", reaped.to_string())
        .with_context("process_group_terminated", group_terminated.to_string())
}

fn containment_failure(
    trigger: &CaptureFailure,
    pid: u32,
    cleanup: CleanupStatus,
) -> VerificationError {
    let trigger_code = capture_failure_code(trigger);
    let mut error = VerificationError::new(
        VerificationErrorCode::ProcessContainmentFailure,
        "native process-group containment could not be observed",
    )
    .with_context("trigger", trigger_code.as_str())
    .with_context("pid", pid.to_string())
    .with_context("process_group", pid.to_string())
    .with_context("reaped", cleanup.reaped.to_string())
    .with_context("process_group_terminated", "false");
    if let Some(error_value) = cleanup.signal_error {
        error = error.with_context("group_signal_error", error_value);
    }
    if let Some(error_value) = cleanup.fallback_error {
        error = error.with_context("fallback_termination_error", error_value);
    }
    if let Some(error_value) = cleanup.reap_error {
        error = error.with_context("reap_error", error_value);
    }
    if let Some(error_value) = cleanup.observation_error {
        error = error.with_context("group_observation_error", error_value.to_string());
    }
    error
}

fn capture_failure_code(failure: &CaptureFailure) -> VerificationErrorCode {
    match failure {
        CaptureFailure::Timeout => VerificationErrorCode::ExecutionTimeout,
        CaptureFailure::Limit { .. } => VerificationErrorCode::OutputLimitExceeded,
        CaptureFailure::Read { .. } | CaptureFailure::Monitor(_) => {
            VerificationErrorCode::IoFailure
        }
        CaptureFailure::Exit { status, .. } if status.signal().is_some() => {
            VerificationErrorCode::ProcessSignaled
        }
        CaptureFailure::Exit { .. } => VerificationErrorCode::UnexpectedExitStatus,
        CaptureFailure::UnexpectedDescendant => VerificationErrorCode::UnexpectedProcessDescendant,
    }
}

fn exit_error(status: ExitStatus, expected_status: i32) -> VerificationError {
    if let Some(signal) = status.signal() {
        VerificationError::new(
            VerificationErrorCode::ProcessSignaled,
            "native child terminated by signal",
        )
        .with_context("signal", signal.to_string())
    } else {
        VerificationError::new(
            VerificationErrorCode::UnexpectedExitStatus,
            "native child returned an unexpected exit status",
        )
        .with_context(
            "status",
            status
                .code()
                .map_or_else(|| "unknown".to_owned(), |code| code.to_string()),
        )
        .with_context("expected_status", expected_status.to_string())
    }
}

fn validate_expected_exit_code(value: Option<i32>) -> Result<i32, VerificationError> {
    let Some(value) = value else {
        return Err(invalid_expected_exit_status("missing"));
    };
    if !(0..=255).contains(&value) {
        return Err(invalid_expected_exit_status(value.to_string()));
    }
    Ok(value)
}

fn invalid_expected_exit_status(value: impl Into<String>) -> VerificationError {
    VerificationError::new(
        VerificationErrorCode::InvalidExpectedExitStatus,
        "native expected exit status must be one integer in [0, 255]",
    )
    .with_context("expected_status", value.into())
}

fn validate_deadline(seconds: Option<f64>) -> Result<Duration, VerificationError> {
    let Some(seconds) = seconds else {
        return Err(invalid_deadline("missing"));
    };
    if !seconds.is_finite() || seconds <= 0.0 || seconds > MAX_NATIVE_DEADLINE_SECONDS {
        return Err(invalid_deadline(seconds.to_string()));
    }
    Ok(Duration::from_secs_f64(seconds))
}

fn invalid_deadline(value: impl Into<String>) -> VerificationError {
    VerificationError::new(
        VerificationErrorCode::InvalidExecutionDeadline,
        "native execution deadline must be finite and in (0, 10] seconds",
    )
    .with_context("deadline_seconds", value.into())
    .with_context("maximum_seconds", MAX_NATIVE_DEADLINE_SECONDS.to_string())
}

fn validate_lexical_paths(root: &Path, executable: &Path) -> Result<(), VerificationError> {
    if !root.is_absolute()
        || !executable.is_absolute()
        || has_non_normal_component(root)
        || has_non_normal_component(executable)
        || !executable.starts_with(root)
    {
        return Err(invalid_executable(
            executable,
            "path is not exact and contained",
        ));
    }
    Ok(())
}

fn has_non_normal_component(path: &Path) -> bool {
    path.components()
        .any(|component| matches!(component, Component::CurDir | Component::ParentDir))
}

fn descriptor_open_error(error: Errno, path: &Path) -> VerificationError {
    if matches!(error, Errno::NOSYS | Errno::INVAL) {
        unsupported_profile(error.to_string())
    } else {
        invalid_executable(path, error.to_string())
    }
}

fn unsupported_profile(reason: impl Into<String>) -> VerificationError {
    VerificationError::new(
        VerificationErrorCode::UnsupportedExecutionProfile,
        "required Linux native-execution facilities are unavailable",
    )
    .with_context("reason", reason.into())
}

fn invalid_executable(path: &Path, reason: impl Into<String>) -> VerificationError {
    VerificationError::new(
        VerificationErrorCode::InvalidExecutablePath,
        "native executable path is not admitted",
    )
    .with_context("path", path.display().to_string())
    .with_context("reason", reason.into())
}

fn descriptor_path(descriptor: &OwnedFd) -> PathBuf {
    Path::new(PROC_DESCRIPTOR_ROOT).join(descriptor.as_raw_fd().to_string())
}

fn process_group_id(pid: u32) -> Option<Pid> {
    i32::try_from(pid).ok().and_then(Pid::from_raw)
}

fn reap_after_setup_failure(child: &mut Child, pid: u32, message: &str) -> VerificationError {
    let Some(group) = process_group_id(pid) else {
        return VerificationError::new(VerificationErrorCode::ProcessContainmentFailure, message)
            .with_context("trigger", VerificationErrorCode::IoFailure.as_str())
            .with_context("pid", pid.to_string())
            .with_context("reaped", "false")
            .with_context("process_group_terminated", "false");
    };
    let cleanup = cleanup_process_group(child, group);
    if cleanup.group_terminated {
        VerificationError::new(VerificationErrorCode::IoFailure, message)
            .with_context("pid", pid.to_string())
            .with_context("reaped", cleanup.reaped.to_string())
            .with_context("process_group_terminated", "true")
    } else {
        containment_failure(&CaptureFailure::Monitor(message.to_owned()), pid, cleanup)
    }
}

fn decode_stream(bytes: Vec<u8>, stream: Stream, pid: u32) -> Result<String, VerificationError> {
    String::from_utf8(bytes).map_err(|error| {
        VerificationError::new(
            VerificationErrorCode::InvalidProcessOutput,
            "native child stream is not valid UTF-8",
        )
        .with_context("stream", stream.as_str())
        .with_context("valid_up_to", error.utf8_error().valid_up_to().to_string())
        .with_context("pid", pid.to_string())
        .with_context("reaped", "true")
        .with_context("process_group_terminated", "true")
    })
}

fn io_error(action: &str, error: &impl std::fmt::Display) -> VerificationError {
    VerificationError::new(
        VerificationErrorCode::IoFailure,
        format!("{action}: {error}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;

    #[trace("TC-038", "NFR-006-AC-6")]
    #[test]
    fn tc_038_classifies_missing_linux_profile_without_fallback() {
        assert_eq!(
            descriptor_open_error(Errno::NOSYS, Path::new("/candidate")).code(),
            VerificationErrorCode::UnsupportedExecutionProfile
        );
        assert_eq!(
            descriptor_open_error(Errno::INVAL, Path::new("/candidate")).code(),
            VerificationErrorCode::UnsupportedExecutionProfile
        );
        assert_eq!(
            descriptor_open_error(Errno::LOOP, Path::new("/candidate")).code(),
            VerificationErrorCode::InvalidExecutablePath
        );
    }

    #[trace("TC-038", "NFR-006-AC-8")]
    #[test]
    fn tc_038_containment_observation_timeout_preserves_the_trigger() {
        let wait_error = wait_for_process_group_exit_with(Duration::ZERO, || Ok(true))
            .expect_err("persisting group must exhaust cleanup budget");
        let error = containment_failure(
            &CaptureFailure::Timeout,
            41,
            CleanupStatus {
                reaped: false,
                group_terminated: false,
                signal_error: None,
                fallback_error: None,
                reap_error: None,
                observation_error: Some(wait_error),
            },
        );
        assert_eq!(
            error.code(),
            VerificationErrorCode::ProcessContainmentFailure
        );
        assert_eq!(
            VerificationErrorCode::from_code("process_containment_failure"),
            Some(VerificationErrorCode::ProcessContainmentFailure)
        );
        assert_eq!(
            error.context().get("trigger").map(String::as_str),
            Some("execution_timeout")
        );
        assert_eq!(
            error
                .context()
                .get("process_group_terminated")
                .map(String::as_str),
            Some("false")
        );
    }
}
