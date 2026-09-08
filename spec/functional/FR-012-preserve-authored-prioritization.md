---
id: FR-012
title: "Preserve authored prioritization"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-001"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/US-003"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-004"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-005"
    type: "depends_on"
---
# FR-012: Preserve authored prioritization

## Description

When a policy and selection provide priority and explanation data, the planner SHALL preserve the authored values without deriving an ordinal score.

## Inputs

- An authored selection policy with rule priority and explanation.
- A selection with authored priority and applicability reasons.

## Outputs

- Policy validation and task data that retain authored priority and reasons.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-012-AC-1 | Required, recommended, and optional policy priorities and non-empty explanations validate, and task priority and reasons equal their authored selection values. | Test (TC-025) |
| FR-012-AC-2 | A selection-policy rule containing a `score` is rejected instead of being treated as an authored priority. | Test (TC-025) |

## Dependencies

- Implements [US-001](../usecase/US-001-plan-portable-verification.md) and [US-003](../usecase/US-003-detect-compatibility-regressions.md).
- Depends on [FR-004](./FR-004-adapt-quoin-catalog.md) and [FR-005](./FR-005-build-verification-plans.md).
- VP01 acceptance depends on E02 and a recorded reviewer disposition; this requirement does not close that gate.
