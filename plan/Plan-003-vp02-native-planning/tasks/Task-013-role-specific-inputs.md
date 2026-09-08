---
id: Task-013
title: "Role-specific verification inputs"
type: Task
status: blocked
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-012", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-015", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-028", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-030", type: verifies }
---
# Task-013: Role-Specific Verification Inputs

## Scope

Replace open input objects with five closed, shared-reference-aware variants.

## Subtasks

- [ ] Implement state, relational, trace, fault, and manual variants.
- [ ] Reject missing, foreign-role, cross-variant, and unexpected fields.
- [ ] Add the heterogeneous parser plan fixture.

## Deliverables

- Draft-2 input schema, regenerated types/fixtures, and TC-028/TC-030 evidence.

## Notes

- Blocked by Task-012.
