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

Verify each missing or exhausted required input remains visible as one setup task
and one verification-task dependency.

## Test Procedure

Build table-driven plans that remove each capability prerequisite, use a stale
policy, and exhaust a required task budget. Include duplicate needs shared by two
tasks and near-miss available prerequisites with the wrong kind or identifier.

## Expected Results

Distinct missing needs produce one deterministic setup task each; all affected
tasks depend on it; duplicates coalesce by exact identity; required work stays in
the plan as blocked; near matches and caller claims cannot satisfy the need.
