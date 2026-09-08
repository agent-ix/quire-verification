---
id: TC-019
title: "Return violated and failed portfolio results"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-009"
    type: "verifies"
---
# TC-019: Return violated and failed portfolio results

## Description

Verify check failures remain inside the bounded portfolio report.

## Test Procedure

Run the portfolio once with an adapter that corrupts a field and once with an
adapter that throws an injected exception.

## Expected Results

The mismatch produces a completed violated result, the exception produces
failed not-evaluated results, and both reports include assessed obligations.

