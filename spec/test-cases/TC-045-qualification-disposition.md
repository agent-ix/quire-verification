---
id: TC-045
title: "Seal and bind the qualification disposition"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-027"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-008"
    type: "verifies"
---
# TC-045: Seal and bind the qualification disposition

## Description

Verify that the final core reaches through the report to every retained source
artifact, remains incomplete for every unresolved obligation, and accepts one exact
external disposition without circular identity or claim expansion.

## Test Procedure

Assemble the complete core, recompute its JCS digest independently, and bind each of
`accept`, `revise`, and `reject` separately. Delete every required evidence class and
substitute missing, duplicate, foreign, stale, unsupported, or claim-expanding
dispositions. Tamper a source artifact and recompute all internal report digests.

## Expected Results

The core digest is non-circular and every source binding recomputes. Any unresolved
input leaves an explicit incomplete report. One exact disposition finalizes the same
core and claim. Every adverse disposition refuses without changing the core, and a
self-consistently resealed report still fails when retained source bytes change.

