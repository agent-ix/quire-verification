---
id: TC-020
title: "Retain pending human work and bounded claims"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-009"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-003"
    type: "verifies"
---
# TC-020: Retain pending human work and bounded claims

## Description

Verify the local report cannot imply an unperformed human or external run.

## Test Procedure

Run the bounded portfolio and inspect its manual result, assessment, evidence
ownership, sufficiency decision, and claim-limit text.

## Expected Results

The inspection result is not-run and incomplete, no sufficiency decision is
made, and the claim limit excludes external methods, parsers, faults, and human
review.

