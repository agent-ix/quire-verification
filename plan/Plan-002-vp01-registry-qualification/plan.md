---
id: Plan-002
title: "VP01 registry and policy qualification"
type: Plan
status: done
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-010", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-011", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-012", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-004", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-005", type: references }
---
# Implementation Plan: VP01 Registry and Policy Qualification

## Requirements Summary

### Functional Requirements

- [x] **FR-010**: Keep applicability and readiness independent.
- [x] **FR-011**: Emit authored or unknown classification.
- [x] **FR-012**: Preserve authored prioritization.

### Non-Functional Requirements

- [x] **NFR-004**: Keep the qualified executable path in Rust.
- [x] **NFR-005**: Enforce bounded JSON ingestion.

## Dependency Graph

- `Plan-001/Task-001 + Plan-001/Task-003 -> Task-008`
  Reason: independent applicability/readiness and classification normalization use the
  existing schema-generated contract boundary and Rust planner.
- `Plan-001/Task-003 -> Task-009`
  Reason: authored priority and explanation preservation extend the existing planner.
- `Task-008 + Task-009 -> Task-010`
  Reason: the qualification gate reviews the complete VP01 behavior and evidence.

### Shared dependencies

The authored E02 draft schema remains the single contract source for both VP01 tasks.
The existing Rust planner remains the single implementation boundary; no parallel
selection model or non-Rust helper is introduced.

### Cross-cutting constraints

- `NFR-004` applies to the contract, planner, test, fixture, and evidence paths.
- `NFR-005` applies at the existing bounded JSON boundary before planner evaluation.

## Test Plan

### Selection dimensions and classification

- [x] **TC-010**: Preserve applicability while capability absence produces unsupported readiness.
- [x] **TC-024**: Exercise applicable, not-applicable, and unknown values independently of readiness; preserve authored classification, default omission to unknown, and reject empty classification.

### Authored policy

- [x] **TC-025**: Preserve required, recommended, and optional priorities with authored explanations; reject an ordinal score field.

### Qualification gate

- [x] **TC-022**: Confirm that all first-party executable and assertion paths remain Rust and pass the required repository gates.

## Completed Work

### Track A: Critical path

- **A1 = Task-008** Selection dimensions and classification — Medium; exit: applicability, readiness, and classification remain independent under all specified cases.
- **Gate = Task-010** VP01 qualification gate — Hard; pass: exact Rust review and QUOIN gap analysis find no unresolved high-severity issue.

### Track B: Parallel after the existing planner

- **B1 = Task-009** Authored policy preservation — Medium; exit: authored priority and reasons survive planning without synthesized scoring.

## Parallel Execution Summary

```text
Plan-001/Task-001 + Plan-001/Task-003 -> Task-008 --+
Plan-001/Task-003 ---------------------> Task-009 --+-> Task-010
```

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
|------|-------|-------------------|------------------------|--------|
| Task-008 | A | FR-010, FR-011, NFR-005 | TC-010, TC-024 | done |
| Task-009 | B | FR-012 | TC-025 | done |
| Task-010 | Gate | NFR-004 | TC-022 | done |

## Coordination Rules

The schema and planner remain single-writer surfaces. This plan qualifies only the
VP01 registry and selection-policy increment; shared FS02/FS05 artifact-reference
adoption, E02 publication, native parser portfolios, and human disposition remain
outside this implementation slice. The bundle records a process correction after
implementation and does not claim that planning preceded the code.
