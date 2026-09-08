---
id: TC-015
title: "Retain all findings and assessment ownership fields"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-007"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-001"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-003"
    type: "verifies"
---
# TC-015: Retain all findings and assessment ownership fields

## Description

Verify a compatibility status does not erase concurrent findings or ownership
boundaries.

## Test Procedure

Run evidence tests for simultaneous partial participation, unmet assumption,
and logical violation, then inspect denominator accounting, provenance, native
artifact references, evidence owner, and sufficiency decision.

## Expected Results

All three findings are emitted in deterministic order, every obligation is
counted, native references remain intact, evidence ownership is Quoin, and no
sufficiency decision is made.

