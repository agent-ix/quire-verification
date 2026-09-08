---
id: TC-007
title: "Preserve complete Quoin method records"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-004"
    type: "verifies"
---
# TC-007: Preserve complete Quoin method records

## Description

Verify every pinned Quoin method retains all portable descriptive fields.

## Test Procedure

Run the catalog-adapter field-preservation test over all 33 retained methods.

## Expected Results

Every adapted technique equals its source method for identifier, name, class,
definition, evidence kind, applicability, tooling hints, and source module.

