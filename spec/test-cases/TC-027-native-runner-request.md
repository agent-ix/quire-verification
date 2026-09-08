---
id: TC-027
title: "Authorize only bounded native runner requests"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-014"
    type: verifies
---
# TC-027: Authorize Only Bounded Native Runner Requests

## Description

Verify ready work receives the complete authored native invocation envelope and
every non-invocable state receives no runner request.

## Test Procedure

Build one ready task and table-driven non-applicable, unknown, blocked,
unsupported, stale, unqualified, over-budget, and incompletely authorized fault
tasks. Mutate every copied runner field independently.

## Expected Results

Only the applicable ready task has a runner request and invocation permission;
its fields exactly match the selected qualified capability and plan budget.
Fault authorization requires all three safety bindings. The plan retains Quoin
and authored-human ownership and contains no evidence store or sufficiency score.
