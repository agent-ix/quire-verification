---
id: TC-001
title: "Accept only the five public contract kinds at the schema root"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "verifies"
---
# TC-001: Accept only the five public contract kinds at the schema root

## Description

Verify root validation accepts one valid instance of each public contract kind
and refuses an internal evidence-obligation object.

## Test Procedure

Run the traced Rust root-schema test using instances produced from the retained
catalog, portfolio, and result fixtures.

## Expected Results

All five public kinds return unchanged, while the internal object fails the root
`oneOf` validation.
