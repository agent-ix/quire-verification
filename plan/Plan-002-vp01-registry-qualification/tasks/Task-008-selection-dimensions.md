---
id: Task-008
title: "Selection dimensions and classification"
type: Task
status: done
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-001", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-003", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-010", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-011", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-005", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-010", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-024", type: verifies }
---
# Task-008: Selection Dimensions and Classification

## Scope

Extend the existing Rust schema and planner so applicability, readiness, and
classification remain independent, with an explicit unknown classification when
the author omits it.

## Subtasks

- [x] Preserve applicable, not-applicable, and unknown values independently of capability readiness.
- [x] Preserve a nonempty authored classification and default an omitted classification to `unknown`.
- [x] Reject an explicitly empty classification at the authored contract boundary.
- [x] Exercise the cases through compiler-checked `ix-trace-rs` tags.

## Deliverables

- Updated authored schema, generated runtime behavior, Rust planner normalization, and TC-010/TC-024 evidence.

## Notes

- Classification normalization happens only on the plan output copy; the caller's input is not mutated.
