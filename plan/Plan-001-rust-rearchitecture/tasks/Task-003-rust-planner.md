---
id: Task-003
title: "Rust verification planner"
type: Task
status: done
track: B
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-001", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-005", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-009", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-010", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-011", type: verifies }
---
# Task-003: Rust verification planner

## Scope

Implement typed, deterministic, prerequisite-aware plan construction across all
five input kinds.

## Subtasks

- [x] Keep applicability, readiness, and invocation permission distinct.
- [x] Emit explicit prerequisite tasks for missing requirements.
- [x] Enforce fault-safety prerequisites and stale policy handling.
- [x] Reject ambiguous compatible capabilities.

## Deliverables

- Planner module and traced tests for ready, blocked, unsupported, stale, fault, and ambiguous paths.

## Notes

- Selection order must not become an implicit capability tie-breaker.
