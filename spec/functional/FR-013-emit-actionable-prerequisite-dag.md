---
id: FR-013
title: "Emit an actionable prerequisite DAG"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-001"
    type: implements
  - target: "ix://agent-ix/quire-verification/FR-005"
    type: depends_on
---
# FR-013: Emit an Actionable Prerequisite DAG

## Description

When required verification inputs or capabilities are unavailable, the planner SHALL retain each need as an identified prerequisite task and dependency rather than converting the selected verification work to non-applicable or omitting it.

## Inputs

- Selected verification work, required capability prerequisites, available prerequisites, policy freshness, and an optional resource budget.

## Outputs

- Verification tasks with explicit dependency identifiers and setup tasks for unmet prerequisites.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-013-AC-1 | Each distinct missing required prerequisite emits exactly one plan-level setup task, and each affected verification task depends on its setup-task identifier. | Test (TC-026) |
| FR-013-AC-2 | Required verification work remains present and non-invocable when a capability-declared resource allocation is absent. | Test (TC-026) |
| FR-013-AC-3 | A prerequisite is satisfied only by an exact kind-and-identifier match; caller input cannot mark an expired selection policy current. | Test (TC-026) |
| FR-013-AC-4 | A setup task carries its exact prerequisite and a sorted nonempty `requiredBy` list of verification-task identifiers; it does not invent a property or technique identity for plan-level work. | Test (TC-026) |
| FR-013-AC-5 | Every composite setup-task and verification-task identifier injectively encodes its component values, including values containing the identifier delimiter. | Test (TC-026) |

## Dependencies

- Extends the base planner behavior in [FR-005](./FR-005-build-verification-plans.md).
- Implements the integrator need in [US-001](../usecase/US-001-plan-portable-verification.md).
