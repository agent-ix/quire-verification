---
id: FR-011
title: "Emit authored or unknown classification"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-001"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/US-003"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-005"
    type: "depends_on"
---
# FR-011: Emit authored or unknown classification

## Description

When the planner emits its property inventory, the planner SHALL preserve a
non-empty authored classification or emit `unknown` when classification was
omitted.

## Inputs

- Property inventory entries with a non-empty authored classification or no classification.

## Outputs

- Inventory entries whose classification is authored or explicitly `unknown`.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-011-AC-1 | An authored property classification is preserved exactly, and an omitted classification is emitted as `unknown`. | Test (TC-024) |
| FR-011-AC-2 | An empty property classification is rejected rather than emitted as authored or treated as unknown. | Test (TC-024) |

## Dependencies

- Implements [US-001](../usecase/US-001-plan-portable-verification.md) and [US-003](../usecase/US-003-detect-compatibility-regressions.md).
- Depends on [FR-005](./FR-005-build-verification-plans.md).
- VP01 acceptance depends on E02 and a recorded reviewer disposition; this requirement does not close that gate.
