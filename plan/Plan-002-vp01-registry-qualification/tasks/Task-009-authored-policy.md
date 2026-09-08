---
id: Task-009
title: "Authored policy preservation"
type: Task
status: done
track: B
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-003", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-012", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-025", type: verifies }
---
# Task-009: Authored Policy Preservation

## Scope

Qualify the existing Rust planner's policy behavior for authored priority and
explanation fields without adding ordinal scoring.

## Subtasks

- [x] Verify required, recommended, and optional priorities survive plan construction.
- [x] Verify authored reasons survive plan construction.
- [x] Verify no score field is synthesized and an authored score is rejected.
- [x] Exercise the cases through compiler-checked `ix-trace-rs` tags.

## Deliverables

- TC-025 policy qualification evidence and the corresponding contract refusal path.

## Notes

- The planner applies authored policy; it does not own verification sufficiency or human disposition.
