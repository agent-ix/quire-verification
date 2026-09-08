---
id: TC-013
title: "Refuse ambiguous or malformed evidence identity"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-006"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-001"
    type: "verifies"
---
# TC-013: Refuse ambiguous or malformed evidence identity

## Description

Verify exact duplicate candidates and invalid result identities cannot be
resolved implicitly.

## Test Procedure

Run evidence tests for two exact candidates in reversed identifier order,
duplicate result IDs, and obligations missing technique or result binding.

## Expected Results

Exact candidates yield deterministic ambiguous output, while duplicate IDs and
incomplete obligations are rejected.

