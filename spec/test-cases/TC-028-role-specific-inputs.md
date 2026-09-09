---
id: TC-028
title: "Reject cross-variant verification inputs"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-002"
    type: verifies
  - target: "ix://agent-ix/quire-verification/FR-015"
    type: verifies
  - target: "ix://agent-ix/quire-verification/NFR-002"
    type: verifies
---
# TC-028: Reject Cross-Variant Verification Inputs

## Description

Verify the five verification input variants retain their own required roles and
reject missing, foreign, or unexpected fields.

## Test Procedure

Validate one positive state, relational, trace, fault, and manual draft-2 input.
Feed each reference-bearing envelope through the strict reader and supply its
opaque validated output to planning. Assert that the E02 input definition
contains external references to the retained shared schema and no local copy of
the shared artifact field set. Delete every required field; make relational
subjects equal; replace each reference with a wrong-version, wrong-role, or
unvalidated envelope; duplicate a manual disposition; add a field from another
variant; add an unknown field; and submit a former draft-1 contract version.

## Expected Results

Every complete role-correct variant passes and round-trips unchanged through
planning. Every deletion, equal relational identity, wrong or unvalidated
reference, duplicate disposition, cross-variant field, unexpected field, or
draft-1 version fails with a stable schema or shared-reference refusal before a
plan is emitted. No E02 copy of the shared artifact-reference field set exists.
