---
id: Task-020
title: "Bounded native process executor"
type: Task
status: in_progress
track: P
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/NFR-006", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-038", type: verifies }
---
# Task-020: Bounded Native Process Executor

## Scope

Add the Linux Rust child-process boundary for descriptor-admitted executables,
direct arguments, fresh working directories, declared environments, dedicated
process groups, finite deadlines, concurrent bounded output capture, typed
process/protocol failures, observed group termination, and direct-child reaping.

## Subtasks

- [x] Write TC-038 first with controlled Rust child fixtures for every exact and adverse process boundary.
- [x] Pin the root-relative executable with Linux `openat2` beneath/no-magiclink/no-symlink resolution and launch only its retained descriptor-backed path.
- [x] Launch a dedicated process group without a shell or inherited environment and drain both streams concurrently.
- [x] Enforce the accepted deadline and per-stream byte caps, then signal the group, reap the direct child, and observe group disappearance within the separate cleanup budget.
- [x] Prove pathname replacement cannot substitute admitted bytes and descendants with retained or closed pipes cannot escape the deadline or the containment observation boundary.
- [x] Prepare concurrent test candidates as isolated immutable links to the completed Cargo-built fixture, leaving no writable executable handle.
- [x] Return distinct typed failures for timeout, stream excess, signal, status, invalid UTF-8, malformed protocol, and failed containment observation while retaining the original trigger.
- [x] Bind TC-038 and every NFR-006 criterion, including AC-8, through compiler-checked `ix-trace-rs` markers whose arguments are all discovered by Quire coverage.
- [ ] Admit exactly one expected exit code from 0 through 255 per invocation, return the observed matching status with complete streams, and reject absent, out-of-range, signaled, or different statuses without weakening containment.
- [ ] Extend TC-038 and its compiler-checked trace binding across expected statuses zero and one plus declaration and mismatch boundaries.

## Deliverables

- Rust descriptor/process-group adapter, controlled child fixture binary, TC-038 tests, and trace bindings.

## Notes

- This task can use controlled children before selecting the exact parser candidate.
- No shell, network service, production invocation, or cross-platform claim is in scope.
- Reopened after hosted CI run `34277066256` observed `ETXTBSY` fixture launch and a live descendant after the executor returned; reviewed amendment SR-071 through SR-078 governs remediation.
- Reopened again after current coverage proved the multiline marker compiled but
  left NFR-006-AC-2 and NFR-006-AC-3 unbound; reviewed amendment SR-081 through
  SR-088 governs this trace-only remediation.
- Restored after implementation `760bbbd`, 109/159 coverage with all eight
  NFR-006 criteria backed, and exact Rust review SR-089.
- Reopened by QUOIN all-lens review SR-141 through SR-148 because the real
  parser's expected exit-1 syntax diagnostic could not pass through the original
  success-only process output.
- Unblocks Task-021.
