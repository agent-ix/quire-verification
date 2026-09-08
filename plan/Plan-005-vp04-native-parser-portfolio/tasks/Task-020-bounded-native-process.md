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
process/protocol failures, group termination, and direct-child reaping.

## Subtasks

- [ ] Write TC-038 first with controlled Rust child fixtures for every exact and adverse process boundary.
- [ ] Pin the root-relative executable with Linux `openat2` beneath/no-magiclink/no-symlink resolution and launch only its retained descriptor-backed path.
- [ ] Launch a dedicated process group without a shell or inherited environment and drain both streams concurrently.
- [ ] Enforce the accepted deadline and per-stream byte caps, then terminate the group, reap the direct child, and prove no live group member remains on every exceeded bound.
- [ ] Prove pathname replacement cannot substitute admitted bytes and a descendant retaining both pipes cannot escape the deadline.
- [ ] Return distinct typed failures for timeout, stream excess, signal, status, invalid UTF-8, and malformed protocol.
- [ ] Bind TC-038 and every NFR-006 criterion through compiler-checked `ix-trace-rs` markers.

## Deliverables

- Rust descriptor/process-group adapter, controlled child fixture binary, TC-038 tests, and trace bindings.

## Notes

- This task can use controlled children before selecting the exact parser candidate.
- No shell, network service, production invocation, or cross-platform claim is in scope.
- Unblocks Task-021.
