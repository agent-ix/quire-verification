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
  - target: "ix://agent-ix/quire-verification/NFR-006"
    type: depends_on
---
# FR-014: Construct Exact Native Runner Requests

## Description

When a selected verification task is applicable and ready for a qualified
`native-producer` capability, the planner SHALL emit an exact closed runner
request that identifies the capability, adapter, permissions, selected finite
resource budget, native artifact interface, and role-specific input needed for
that invocation. The planner SHALL emit no runner request for any other task.

## Inputs

- A selected technique and its exact capability, whose runner kind is either
  `native-producer` or `manual`.
- Optional property-level and plan-level budgets. A property budget, when
  present, replaces the plan budget for that task; the two objects are never
  merged or guessed.
- A native capability's required `resourceLimits`, containing positive integer
  `maxWallClockSeconds` and `maxParallelism` ceilings, plus its exact adapter,
  environment permissions, and native artifact interface. The wall-clock
  ceiling is at most ten seconds.

## Outputs

- A closed native runner request for invocable work or an explicit null request
  for work that is not invocable through the native boundary.
- A deterministic missing `resource-allocation` prerequisite when no budget is
  selected, or a `budget-exceeded` gap when either selected dimension exceeds
  the capability ceiling.

The missing-budget prerequisite is exactly
`{"kind":"resource-allocation","id":"budget:<capabilityId>","required":true}`.
The exceeded-budget gap is exactly
`{"kind":"budget-exceeded","requested":<selected-budget>,"limits":<selected-resourceLimits>}`.
Each budget and limit object is closed and contains only its two positive
integer dimensions: `wallClockSeconds` plus `parallelism` for a budget, and
`maxWallClockSeconds` plus `maxParallelism` for limits.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-014-AC-1 | An applicable ready task with one qualified `native-producer` capability and an in-limit budget receives a closed runner request whose capability ID, adapter ID/version, permissions, selected budget, native artifact interface, and complete role-specific input are exact JCS copies of the selected records; descriptive catalog text cannot create a capability or field. | Test (TC-027) |
| FR-014-AC-2 | A non-applicable, unknown, blocked, unsupported, stale, unqualified, manual, budget-absent, or over-limit task has a null runner request and `invocationAllowed: false`; absent budget adds one deterministic resource-allocation setup dependency, while an over-limit budget retains both requested and limiting values in a `budget-exceeded` gap. | Test (TC-027) |
| FR-014-AC-3 | A fault runner request remains null until non-production authorization, injection binding, and an independent observation binding are all exactly present. | Test (TC-027) |
| FR-014-AC-4 | The plan names Quoin as evidence owner and leaves sufficiency to an authored human decision; the closed runner request contains no evidence store, logical outcome, or automated sufficiency decision, and its selected wall-clock budget is at most NFR-006's ten-second execution ceiling. | Test (TC-027) |

## Dependencies

- Extends [FR-005](./FR-005-build-verification-plans.md) after the prerequisite DAG in [FR-013](./FR-013-emit-actionable-prerequisite-dag.md) and the role-specific inputs in [FR-015](./FR-015-preserve-role-specific-inputs.md) are known.
