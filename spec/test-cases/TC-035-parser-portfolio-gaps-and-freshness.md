---
id: TC-035
title: "Retain parser portfolio gaps and stale evidence"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-019"
    type: "verifies"
---
# TC-035: Retain parser portfolio gaps and stale evidence

## Description

Verify that absent native prerequisites, expired results, and correlated or
unknown observations remain distinct findings in the parser portfolio.

## Test Procedure

Remove each required runner, binding, tool, and resource prerequisite in turn;
assess an otherwise exact result at and after its earliest expiry; then reuse an
observation across techniques and replace a declared relationship with unknown.

## Expected Results

Each missing prerequisite blocks only affected work, emits actionable setup, and
produces no invented result. The result is fresh at the exact expiry and stale
one representable instant later without losing any independent finding. Shared
and unknown dependence remain explicit and never create an independence or
aggregate-sufficiency claim.
