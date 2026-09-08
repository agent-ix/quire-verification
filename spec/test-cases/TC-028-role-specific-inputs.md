---
id: TC-028
title: "Reject cross-variant verification inputs"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-015"
    type: verifies
---
# TC-028: Reject Cross-Variant Verification Inputs

## Description

Verify the five verification input variants retain their own required roles and
reject missing, foreign, or unexpected fields.

## Test Procedure

Validate one positive state, relational, trace, fault, and manual input. For each
variant, delete every required field, substitute a reference with the wrong role
kind, add a semantic field from another variant, and add an unknown top-level
field.

## Expected Results

Every complete role-correct variant passes and round-trips through planning.
Every deletion, wrong role, cross-variant field, or unexpected field fails with
a stable schema or shared-reference refusal.
