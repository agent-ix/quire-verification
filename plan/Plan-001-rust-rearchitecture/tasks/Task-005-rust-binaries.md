---
id: Task-005
title: "Rust fixture and baseline binaries"
type: Task
status: done
track: C
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-001", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-002", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-003", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-005", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-006", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-021", type: verifies }
---
# Task-005: Rust fixture and baseline binaries

## Scope

Replace every JavaScript helper with bounded Rust binaries for fixture
validation, fixture rendering, local portfolio output, and Quoin baseline input.

## Subtasks

- [x] Discover every fixture and fail on an unhandled JSON file.
- [x] Validate whole fixture wrappers and every embedded public contract.
- [x] Compare real Quoin standard-input output by technique identifier.
- [x] Emit deterministic JSON without unbounded reads.

## Deliverables

- Rust binaries, traced integration tests, and updated operator documentation.

## Notes

- IT-001 uses real Quoin and real filesystem module loading.
