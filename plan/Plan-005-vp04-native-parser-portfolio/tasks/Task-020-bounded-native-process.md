---
id: Task-020
title: "Bounded native process executor"
type: Task
status: not_started
track: P
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/NFR-006", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-038", type: verifies }
---
# Task-020: Bounded Native Process Executor

## Scope

Add the Linux Rust child-process boundary for exact executables, direct arguments,
fresh working directories, declared environments, finite deadlines, concurrent
bounded output capture, typed process/protocol failures, termination, and reaping.

## Subtasks

- [ ] Write TC-038 first with controlled Rust child fixtures for every exact and adverse process boundary.
- [ ] Validate executable-root, regular-file, and no-symbolic-link constraints before launch.
- [ ] Launch without a shell or inherited environment and drain both streams concurrently.
- [ ] Enforce the accepted deadline and per-stream byte caps, then terminate and reap on every exceeded bound.
- [ ] Return distinct typed failures for timeout, stream excess, signal, status, invalid UTF-8, and malformed protocol.
- [ ] Bind TC-038 and every NFR-006 criterion through compiler-checked `ix-trace-rs` markers.

## Deliverables

- Rust process adapter, controlled child fixture binary, TC-038 tests, and trace bindings.

## Notes

- This task can use controlled children before selecting the exact parser candidate.
- No shell, network service, production invocation, or cross-platform claim is in scope.
- Unblocks Task-021.
