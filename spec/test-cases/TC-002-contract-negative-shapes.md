---
id: TC-002
title: "Reject contradictory results and malformed planner tasks"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "verifies"
---
# TC-002: Reject contradictory results and malformed planner tasks

## Description

Verify schema conditionals and closed planner-task shapes reject silent drift.

## Test Procedure

Run the contract tests that combine failed execution with a verdict, delete each
required emitted task field, rename one field, and replace a prerequisite array
with a string. Replace the boundary `contractVersion` with an unknown version.

## Expected Results

Every contradictory, malformed, or unknown-version instance fails validation
with a diagnostic that identifies the relevant field, version, or conditional.
