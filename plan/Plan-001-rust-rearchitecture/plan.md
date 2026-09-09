---
id: Plan-001
title: "Rust rearchitecture of the bounded verification path"
type: Plan
status: done
relationships:
  - { target: "ix://agent-ix/quire-verification/StR-001", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-001", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-002", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-003", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-004", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-005", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-006", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-007", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-008", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-009", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-001", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-002", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-003", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-004", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-005", type: references }
---
# Implementation Plan: Rust rearchitecture of the bounded verification path

## Requirements Summary

### Stakeholder Requirements

- [x] **StR-001**: Deliver a Rust core for the qualified verification path.

### Functional Requirements

- [x] **FR-001**: Validate the five public contracts.
- [x] **FR-002**: Derive runtime contract behavior from the schema.
- [x] **FR-003**: Validate every fixture through an explicit handler.
- [x] **FR-004**: Adapt Quoin catalog and advice without changing meaning.
- [x] **FR-005**: Build prerequisite-aware verification plans.
- [x] **FR-006**: Match evidence by exact identity and execution scope.
- [x] **FR-007**: Preserve independent result findings.
- [x] **FR-008**: Canonicalize digest and identity inputs with JCS.
- [x] **FR-009**: Retain bounded portfolio failures as results.

### Non-Functional Requirements

- [x] **NFR-001**: Produce deterministic structured comparison output.
- [x] **NFR-002**: Maintain one authored public-contract source.
- [x] **NFR-003**: Preserve conservative assurance claims.
- [x] **NFR-004**: Implement every executable path in Rust and bind every acceptance-marker argument through Quire.
- [x] **NFR-005**: Enforce bounded JSON ingestion.

## Dependency Graph

- `Task-001 -> Task-002, Task-003, Task-004`: catalog, planning, and evidence need the generated contracts, JCS, errors, and resource bounds.
- `Task-002 -> Task-005`: fixture and baseline binaries need the catalog adapter.
- `Task-002 + Task-003 + Task-004 -> Task-006`: the bounded portfolio composes all three services.
- `Task-001..Task-006 -> Task-007`: the qualified-path gate reviews and validates the complete Rust replacement.

Shared dependencies are the schema-generated wire types, stable error envelope,
JCS functions, and bounded JSON ingestion owned only by Task-001.

Cross-cutting constraints NFR-001 through NFR-005 apply to every downstream task.
The rejected `.mjs` prototype supplies behavioral comparison input but is not an
accepted deliverable.

## Test Plan

### Contract and Canonicalization

- [x] **TC-001**: Accept only the five public contract kinds.
- [x] **TC-002**: Reject contradictory and malformed contracts.
- [x] **TC-003**: Derive runtime definitions from the schema.
- [x] **TC-004**: Reject invalid plan dependency graphs.
- [x] **TC-016**: Canonicalize valid JSON with JCS.
- [x] **TC-017**: Reject values outside the JCS domain.
- [x] **TC-023**: Enforce the JSON resource envelope.

### Fixtures and Quoin Integration

- [x] **TC-005**: Execute the schema against every retained fixture.
- [x] **TC-006**: Reject an unhandled fixture and inspect CI.
- [x] **TC-007**: Preserve complete Quoin method records.
- [x] **TC-008**: Qualify catalog conflicts and advice.
- [x] **TC-021**: Execute the real Quoin catalog baseline.

### Planning and Evidence

- [x] **TC-009**: Build ready tasks across distinct input kinds.
- [x] **TC-010**: Expose unsupported and blocked verification work.
- [x] **TC-011**: Reject an ambiguous compatible capability selection.
- [x] **TC-012**: Reject every exact evidence-binding mismatch.
- [x] **TC-013**: Refuse ambiguous or malformed evidence identity.
- [x] **TC-014**: Distinguish freshness and result states.
- [x] **TC-015**: Retain all findings and ownership fields.

### Portfolio and Qualified-Path Gate

- [x] **TC-018**: Compare catalog parity by technique identifier.
- [x] **TC-019**: Return violated and failed portfolio results.
- [x] **TC-020**: Retain pending human work and bounded claims.
- [x] **TC-022**: Enforce the Rust qualified-path implementation and cross-reader trace binding.

## Completed Work and Open Remediation

### Track A: Critical Path

- **A1 = Task-001** Rust contract core — Hard; exit: generated wire types, runtime schema validation, JCS, stable errors, and resource limits pass TC-001..TC-004, TC-016, TC-017, and TC-023.
- **A2 = Task-004** Rust evidence assessor — Hard; exit: every binding dimension and independent finding passes TC-012..TC-015.
- **A3 = Task-006** Rust bounded portfolio — Hard; exit: parity, permutation, violated, failed, and pending-human outcomes pass TC-018..TC-020.
- **Gate = Task-007** Qualified-path review — Complete; Quoin validation, Rust gates, exact rust-review, and cross-reader trace reconciliation pass with every compiled marker argument bound by Quire.

### Track B: Parallel After Contract Core

- **B1 = Task-002** Rust Quoin adapter — Medium; exit: all catalog fields and adverse diagnostics pass TC-007 and TC-008.
- **B2 = Task-003** Rust planner — Medium; exit: all five input shapes, readiness states, safety prerequisites, and ambiguity pass TC-009..TC-011.

### Track C: Integration

- **C1 = Task-005** Rust fixture and baseline binaries — Medium; exit: all fixtures and the real pinned Quoin output pass TC-005, TC-006, and TC-021.

## Parallel Execution Summary

```text
Task-001 ─┬─ Task-002 ─ Task-005 ─┐
          ├─ Task-003 ────────────┼─ Task-006 ─ Task-007
          └─ Task-004 ────────────┘
```

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
|------|-------|-------------------|------------------------|--------|
| Task-001 | A | FR-001, FR-002, FR-008, NFR-002, NFR-005 | TC-001..TC-004, TC-016, TC-017, TC-023 | done |
| Task-002 | B | FR-004 | TC-007, TC-008 | done |
| Task-003 | B | FR-005 | TC-009..TC-011 | done |
| Task-004 | A | FR-006, FR-007, NFR-001, NFR-003 | TC-012..TC-015 | done |
| Task-005 | C | FR-003 | TC-005, TC-006, TC-021 | done |
| Task-006 | A | FR-009 | TC-018..TC-020 | done |
| Task-007 | Gate | StR-001, NFR-004 | TC-021, TC-022 | done |

## Coordination Rules

The schema and generated-type boundary is single-writer until Task-001 passes.
No downstream task recreates contract structs by hand. The rewrite removes the
JavaScript implementation and offers no compatibility shim or dual runtime.
Retained JSON evidence is regenerated only from the final Rust binary and
must not be edited to manufacture schema conformance.
