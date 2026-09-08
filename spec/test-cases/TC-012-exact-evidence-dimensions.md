---
id: TC-012
title: "Reject every exact evidence-binding mismatch"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-006"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-001"
    type: "verifies"
---
# TC-012: Reject every exact evidence-binding mismatch

## Description

Verify mutation of each required identity and execution dimension prevents a
result from matching an obligation.

## Test Procedure

Run the table-driven evidence test that mutates each of the 14 required
technique, identity, scope, plan, run, producer, and adapter dimensions.

## Expected Results

Every mutation appears in `scopeDifferences`; a nearby result produces missing
status and a named scope-mismatch gap.

