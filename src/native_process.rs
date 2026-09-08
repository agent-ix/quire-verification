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

use crate::contracts::{VerificationError, VerificationErrorCode, validate_resource_envelope};

/// Largest accepted stdout or stderr stream, in bytes.
pub const MAX_NATIVE_OUTPUT_BYTES: usize = 16 * 1024 * 1024;
/// Largest accepted per-invocation deadline, in seconds.
pub const MAX_NATIVE_DEADLINE_SECONDS: f64 = 10.0;

const MONITOR_INTERVAL: Duration = Duration::from_millis(1);
const PROC_DESCRIPTOR_ROOT: &str = "/proc/self/fd";

/// Exact per-invocation inputs for one admitted native executable.
#[derive(Clone, Debug)]
pub struct NativeProcessRequest {
    /// Separate argument vector passed without shell interpretation.
    pub arguments: Vec<String>,
    /// Complete child environment after inherited entries are cleared.
    pub environment: BTreeMap<String, String>,
    /// Positive finite wall-clock budget no greater than ten seconds.
    pub deadline_seconds: Option<f64>,
}

/// UTF-8 output from one successfully completed native child.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeProcessOutput {
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
            OFlags::PATH | OFlags::CLOEXEC,
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
        Ok(Self {
            root: root_descriptor,
            executable: executable_descriptor,
        })
    }

    /// Executes one request through the retained descriptor in a fresh directory.
    ///
    /// # Errors
    /// Returns a stable typed error before launch for an invalid deadline, or
    /// after group termination and direct-child reaping for deadline, stream,
    /// signal, status, descendant, pipe, spawn, and UTF-8 failures.
    pub fn execute(
        &self,
        request: &NativeProcessRequest,
    ) -> Result<NativeProcessOutput, VerificationError> {
        let deadline = validate_deadline(request.deadline_seconds)?;
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
        )
    }
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
    Limit { stream: Stream, bytes: usize },
    Read { stream: Stream, message: String },
    Monitor(String),
    Exit(ExitStatus),
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
) -> Result<NativeProcessOutput, VerificationError> {
    let started = Instant::now();
    let mut child_exited = false;
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
                Ok(Some(status)) if status.success() => child_exited = true,
                Ok(Some(status)) => {
                    return terminate_group_reap_and_join(running, CaptureFailure::Exit(status));
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
    Ok(NativeProcessOutput { stdout, stderr })
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
    let group_termination = terminate_process_group(running.group);
    let fallback_kill_error = group_termination
        .as_ref()
        .err()
        .and_then(|_| running.child.kill().err());
    let reap_result = running.child.wait();
    let reaped = reap_result.is_ok();
    let _ = running.stdout_reader.join();
    let _ = running.stderr_reader.join();
    let mut error = capture_error(failure, running.pid, reaped, group_termination.is_ok());
    if let Err(termination_error) = group_termination {
        error = error.with_context("group_termination_error", termination_error.to_string());
    }
    if let Some(kill_error) = fallback_kill_error {
        error = error.with_context("fallback_termination_error", kill_error.to_string());
    }
    if let Err(reap_error) = reap_result {
        error = error.with_context("reap_error", reap_error.to_string());
    }
    Err(error)
}

fn terminate_process_group(group: Pid) -> Result<(), Errno> {
    match kill_process_group(group, Signal::KILL) {
        Ok(()) | Err(Errno::SRCH) => Ok(()),
        Err(error) => Err(error),
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
        CaptureFailure::Exit(status) => exit_error(status),
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

fn exit_error(status: ExitStatus) -> VerificationError {
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
    }
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
    let group_termination = process_group_id(pid).map(terminate_process_group);
    let group_terminated = matches!(group_termination, Some(Ok(())));
    let fallback_kill_error = (!group_terminated).then(|| child.kill().err()).flatten();
    let reap_result = child.wait();
    let mut error = VerificationError::new(VerificationErrorCode::IoFailure, message)
        .with_context("pid", pid.to_string())
        .with_context("reaped", reap_result.is_ok().to_string())
        .with_context("process_group_terminated", group_terminated.to_string());
    if let Some(Err(termination_error)) = group_termination {
        error = error.with_context("group_termination_error", termination_error.to_string());
    }
    if let Some(kill_error) = fallback_kill_error {
        error = error.with_context("fallback_termination_error", kill_error.to_string());
    }
    if let Err(reap_error) = reap_result {
        error = error.with_context("reap_error", reap_error.to_string());
    }
    error
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
}
