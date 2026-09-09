---
id: TC-011
title: "Reject an ambiguous compatible capability selection"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-005"
    type: "verifies"
---
# TC-011: Reject an ambiguous compatible capability selection

## Description

Verify planning cannot choose silently between multiple compatible capabilities.

## Test Procedure

Add a second compatible capability for one selected property, omit
`capabilityId`, and build the verification plan.

## Expected Results

Planning fails with an ambiguity diagnostic through the tagged Rust acceptance
test.
