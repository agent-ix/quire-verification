---
id: TC-027
title: "Authorize only bounded native runner requests"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-013"
    type: verifies
  - target: "ix://agent-ix/quire-verification/FR-014"
    type: verifies
---
# TC-027: Authorize Only Bounded Native Runner Requests

## Description

Verify ready work receives the complete authored native invocation envelope and
every non-invocable state receives no runner request.

## Test Procedure

Build one ready native task with an in-limit property budget and table-driven
non-applicable, unknown, blocked, unsupported, stale, unqualified, manual,
budget-absent, over-wall-clock, over-parallelism, and incompletely authorized
fault tasks. Verify property budget replacement rather than field merging.
Mutate every copied runner field independently and inspect the emitted schema.

## Expected Results

Only the applicable ready native task has a runner request and invocation
permission; its closed fields exactly match the selected qualified capability,
complete role input, and one selected budget. Missing budget emits an exact
resource-allocation setup dependency. Each exceeded dimension produces a
`budget-exceeded` gap containing requested and limiting values. Fault
authorization requires all three safety bindings. Manual work stays
non-invocable with a null native request. The plan retains Quoin and
authored-human ownership and contains no evidence store, outcome, or sufficiency
field.
