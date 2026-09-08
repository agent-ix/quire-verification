---
id: TC-004
title: "Reject invalid verification plan dependency graphs"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-002"
    type: "verifies"
---
# TC-004: Reject invalid verification plan dependency graphs

## Description

Verify semantic plan validation rejects duplicate task identifiers, unknown
dependency identifiers, and dependency cycles after schema validation.

## Test Procedure

Construct three otherwise valid plans, inject one graph defect per plan, and
call `validateVerificationPlan` for each.

## Expected Results

Each plan is rejected with a diagnostic naming the duplicate identifier,
unknown dependency, or cycle. This procedure is specified but not yet automated.

