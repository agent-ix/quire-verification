---
id: FR-010
title: "Keep applicability and readiness independent"
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
# FR-010: Keep applicability and readiness independent

## Description

When the planner constructs a verification plan from catalog advice and tool
capabilities, the planner SHALL preserve applicability and readiness as
independent reviewable dimensions.

## Inputs

- Adapted catalog advice with an applicability disposition and reasons.
- Qualified, unavailable, or incomplete capability records.

## Outputs

- Verification tasks with independent applicability and readiness.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-010-AC-1 | Applicable, not-applicable, and unknown selections retain their exact applicability while capability availability changes readiness independently. | Test (TC-024) |
| FR-010-AC-2 | A stale policy or unavailable capability cannot produce an invocable task even when advice reports the technique as applicable. | Test (TC-010) |

## Dependencies

- Implements [US-001](../usecase/US-001-plan-portable-verification.md) and [US-003](../usecase/US-003-detect-compatibility-regressions.md).
- Depends on [FR-004](./FR-004-adapt-quoin-catalog.md) and [FR-005](./FR-005-build-verification-plans.md).
- VP01 acceptance depends on E02 and a recorded reviewer disposition; this requirement does not close that gate.
