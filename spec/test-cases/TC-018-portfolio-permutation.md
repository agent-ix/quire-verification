---
id: TC-018
title: "Compare catalog parity by technique identifier"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-009"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-001"
    type: "verifies"
---
# TC-018: Compare catalog parity by technique identifier

## Description

Verify catalog parity and metamorphic comparison are independent of source array
position.

## Test Procedure

Run the bounded portfolio with the retained order, reversed methods, and the
runner's reversed and rotated metamorphic variants.

## Expected Results

All unique methods are compared by `techniqueId`; parity and permutation results
complete with satisfied logical outcomes.

