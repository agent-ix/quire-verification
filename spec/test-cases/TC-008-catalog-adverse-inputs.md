---
id: TC-008
title: "Qualify catalog conflicts and inconclusive advice"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-004"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-003"
    type: "verifies"
---
# TC-008: Qualify catalog conflicts and inconclusive advice

## Description

Verify source defects and inconclusive advice remain explicit.

## Test Procedure

Run the catalog-adapter tests with duplicate and unreadable diagnostics, then
adapt positive and empty inconclusive advice cases.

## Expected Results

The diagnostic view remains available but unqualified, positive advice is
applicable, and empty inconclusive advice remains unknown.

