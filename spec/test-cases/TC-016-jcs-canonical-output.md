---
id: TC-016
title: "Canonicalize valid JSON with RFC 8785 JCS"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-008"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-001"
    type: "verifies"
---
# TC-016: Canonicalize valid JSON with RFC 8785 JCS

## Description

Verify canonical number formatting, UTF-16 property ordering, insertion-order
independence, and array-order significance.

## Test Procedure

Run the valid-domain JCS tests with RFC 8785 numeric examples, reordered object
properties, and arrays in opposite orders.

## Expected Results

Canonical strings match the RFC example, equivalent objects compare equally and
hash equally, and arrays with different order remain unequal.

