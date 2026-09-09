---
id: Plan-004
title: "VP03 evidence assessment"
type: Plan
status: done
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-017", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-018", type: references }
---
# Implementation Plan: VP03 Evidence Assessment

## Requirements Summary

### Functional Requirements

- [x] **FR-017**: Retain oracle dependence without aggregation.
- [x] **FR-018**: Enforce every evidence freshness bound.

## Dependency Graph

- `FR-001 + FR-006 + FR-007 -> Task-016`: conservative freshness extends the existing validated evidence assessor.
- `Plan-003/Task-012 + accepted FS05 packet -> Task-017`: oracle fields use the implemented strict shared-reference reader and exact oracle/observation roles.
- `Task-017 -> Task-018`: correlation assessment needs the reviewed oracle contribution contract.
- `Task-016 + Task-018 -> Task-019`: qualification covers the complete VP03 increment.

### Shared dependencies

Plan-003 Task-012 is complete and remains the only shared-reference reader. Task-017 extends the
single schema-derived TechniqueResult definition; it does not create another
wire or evidence store.

### Cross-cutting constraints

NFR-001, NFR-003, NFR-004, and NFR-005 apply to every task: deterministic JCS
ordering, conservative claims, a Rust-only qualified path, and bounded JSON
ingestion remain mandatory.

### The seams

Task-016 changed `src/evidence.rs` and the authoritative E02 schema freshness
shape. Task-017 adds the schema-derived contribution variants; Task-018 consumes
them in the same module through iterative lineage and selected-result component
traversal.

## Test Plan

### Freshness

- [x] **TC-032**: Apply the earliest result/obligation expiry, exact instant boundaries, offset equivalence, invalid order, and checked overflow.

### Oracle dependence

- [x] **TC-033**: Retain independent, shared, derived, and unknown oracle relationships under permutation and exact-reference mutation.

### Qualified-path gates

- [x] **TC-022**: Enforce the Rust-only qualified path.
- [x] **TC-023**: Enforce the bounded JSON envelope.

## Remaining Work

### Track A: Critical Path (serial)

- **A1 = Task-016** Conservative freshness — Medium; exit: every applicable bound participates and invalid temporal input refuses deterministically.
- **A2 = Task-017** Oracle contribution contract — Medium; exit: closed, exact role-qualified lineage validates from the single E02 schema.
- **A3 = Task-018** Oracle correlation assessment — Medium; exit: selected-result shared, derived, and unknown dependence remains visible without aggregate scoring.
- **Gate = Task-019** VP03 qualification — Hard; pass: Rust review and QUOIN gap analysis have no unresolved in-scope finding.

## Parallel Execution Summary

```text
Implemented evidence core -> Task-016 ------------------------+
Plan-003/Task-012 + FS05 -> Task-017 -> Task-018 -------------+-> Task-019
```

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
|------|-------|-------------------|------------------------|--------|
| Task-016 | A | FR-018 | TC-032 | done |
| Task-017 | A | FR-017 | TC-033 | done |
| Task-018 | A | FR-017 | TC-033 | done |
| Task-019 | Gate | FR-017, FR-018 | TC-032, TC-033 | done |

## Coordination Rules

Task-017 proceeds from accepted QUOIN review workflow
`c2e99bbd-6105-4282-b8ec-4eb1f6161647`; Task-018 starts only after its schema
contract passes. B does not edit the upstream FS05 schema, C's Engineering
Assurance implementation, Quoin, or Filament. Any changed behavior returns to
`/specify` and `/spec-review` before implementation.
