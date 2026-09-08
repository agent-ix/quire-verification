---
id: FR-014
title: "Construct exact native runner requests"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-001"
    type: implements
  - target: "ix://agent-ix/quire-verification/FR-005"
    type: depends_on
  - target: "ix://agent-ix/quire-verification/FR-013"
    type: depends_on
  - target: "ix://agent-ix/quire-verification/FR-015"
    type: depends_on
---
# FR-014: Construct Exact Native Runner Requests

## Description

When a selected verification task is applicable and ready, the planner SHALL
emit an exact native runner request that identifies the qualified capability,
adapter, permissions, resource budget, and native artifact interface needed for
that invocation.

## Inputs

- A selected technique, its exact qualified capability, plan budget, environment permissions, and native artifact interface.

## Outputs

- A runner request for invocable work or an explicit null request for work that is not invocable.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-014-AC-1 | A runner request copies the selected capability ID, adapter ID/version, declared permissions, budget, and native artifact interface without deriving an installed capability from descriptive catalog text. | Test (TC-027) |
| FR-014-AC-2 | A task that is non-applicable, unknown, blocked, unsupported, stale, unqualified, or over budget has a null runner request and cannot be invoked. | Test (TC-027) |
| FR-014-AC-3 | A fault runner request remains null until non-production authorization, injection binding, and an independent observation binding are all exactly present. | Test (TC-027) |
| FR-014-AC-4 | The plan names Quoin as evidence owner and leaves sufficiency to an authored human decision; the runner request contains no evidence store or automated sufficiency decision. | Test (TC-027) |

## Dependencies

- Extends [FR-005](./FR-005-build-verification-plans.md) after the prerequisite DAG in [FR-013](./FR-013-emit-actionable-prerequisite-dag.md) and the role-specific inputs in [FR-015](./FR-015-preserve-role-specific-inputs.md) are known.
