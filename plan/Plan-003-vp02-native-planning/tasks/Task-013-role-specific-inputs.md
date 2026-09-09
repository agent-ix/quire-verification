---
id: Task-013
title: "Role-specific verification inputs"
type: Task
status: done
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-012", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-002", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-015", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-002", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-028", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-030", type: verifies }
---
# Task-013: Role-Specific Verification Inputs

## Scope

Replace draft-1 inputs with one draft-2 contract schema containing five closed,
shared-reference-aware variants.

## Subtasks

- [x] Resolve shared artifact envelopes from the retained shared-reference schema without copying their field definitions.
- [x] Generate the draft-2 Rust contract types and expose only opaque strict-reader validation outputs to callers.
- [x] Implement state, relational, trace, fault, and manual variants with keyed exact-JCS validation membership.
- [x] Reject draft-1, missing, aliased-relational, foreign-role, unvalidated, duplicate, cross-variant, and unexpected fields.
- [x] Add the heterogeneous parser plan fixture.

## Deliverables

- Draft-2 schema, regenerated Rust types/fixtures, TC-028 evidence, and the
  heterogeneous fixture consumed by TC-030 under Task-014.

## Notes

- Unblocked by Task-012 at exact Rust review `ce4e03e` / SR-100.
- Specification candidate `432036c` passed owner-selected QUOIN `/spec-review all`
  at `c791101` (SR-102 through SR-109).
- Completed at exact Rust review SR-118 (`70f9d6b`) and gap analysis SR-119
  (`a72d1dc`). FR-002 and NFR-002 are fully backed; FR-015-AC-5 and TC-030
  remain assigned to Task-014's runner-eligibility integration.
