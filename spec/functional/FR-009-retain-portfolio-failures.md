---
id: FR-009
title: "Retain bounded portfolio failures as results"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-003"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-004"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-002"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-005"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-006"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-007"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-008"
    type: "depends_on"
---
# FR-009: Retain bounded portfolio failures as results

## Description

When the bounded local portfolio executes a catalog check, the portfolio runner SHALL return a schema-valid `TechniqueResult` for a satisfied property, a violated property, or a failed execution.

## Inputs

- Source revision, production time, pinned catalog snapshot, and catalog adapter.

## Outputs

- A bounded portfolio report containing a plan, three results, effort observations, an assessment, and claim limits.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-009-AC-1 | Catalog parity compares complete method records keyed by `techniqueId`, independent of source array order. | Test (TC-018) |
| FR-009-AC-2 | Reversing or rotating unique catalog methods leaves adapted technique output unchanged. | Test (TC-018) |
| FR-009-AC-3 | A parity mismatch returns a completed result with logical outcome violated and an assessed violated obligation. | Test (TC-019) |
| FR-009-AC-4 | An adapter exception returns failed results with logical outcome not-evaluated and assessed failed obligations. | Test (TC-019) |
| FR-009-AC-5 | The manual inspection result remains not-run until a verification owner supplies a disposition. | Test (TC-020) |
| FR-009-AC-6 | The report's claim limit states that no external method, parser, fault, or human review is claimed. | Inspection (TC-020) |

## Dependencies

- Implements [US-003](../usecase/US-003-detect-compatibility-regressions.md).
- Uses the portable adapter behavior in [FR-004](./FR-004-adapt-quoin-catalog.md).
