---
id: TC-010
title: "Expose unsupported and blocked verification work"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-005"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-010"
    type: "verifies"
---
# TC-010: Expose unsupported and blocked verification work

## Description

Verify absent capabilities, stale policies, qualification gaps, and fault-safety
requirements prevent invocation without changing applicability.

## Test Procedure

Run planner tests for absent capability, independent applicability/readiness,
stale policy, and the retained fault property with missing safety prerequisites.

## Expected Results

Unsupported and blocked tasks remain non-invocable, and each missing prerequisite
is represented by explicit plan data and setup work.
