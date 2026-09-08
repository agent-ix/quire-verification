---
id: TC-026
title: "Retain every actionable prerequisite"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-013"
    type: verifies
---
# TC-026: Retain Every Actionable Prerequisite

## Description

Verify each missing required input or allocation remains visible as one setup task
and one verification-task dependency.

## Test Procedure

Build table-driven plans that remove each capability prerequisite, use a stale
policy, and omit a capability-declared resource allocation. Include one missing
need shared by two verification tasks and near-miss available prerequisites with
the wrong kind or identifier. Include distinct prerequisite and selection tuples
whose components would collide under delimiter joining.

## Expected Results

Distinct missing needs produce one deterministic plan-level setup task each; all
affected tasks depend on it; duplicates coalesce by exact identity; `requiredBy`
lists the affected verification-task IDs in sorted order; no fake property or
technique ID is attached. Required work stays blocked and near matches or caller
claims cannot satisfy the need. Composite task identifiers remain distinct when a
component contains the delimiter.
Runtime budget exhaustion is outside this planning test and remains a result state.
