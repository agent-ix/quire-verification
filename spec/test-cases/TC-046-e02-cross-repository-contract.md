---
id: TC-046
title: "Accept the E02 cross-repository contract without result loss"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-028"
    type: verifies
---
# TC-046: Accept the E02 Cross-Repository Contract Without Result Loss

## Description

Verify the accepted private E02 draft-2 boundary preserves its complete result
shape.

## Test Procedure

Inspect the interface decision for the accepted owner,
`quire-verification/e02-draft-2`, and private-only publication boundary. Run
the Rust contract test with a valid `TechniqueResult`, then remove each required
execution, logical, activation, participation, assumptions, adequacy,
provenance, native-artifact, and oracle-contribution field independently.

## Expected Results

The interface decision makes no public-schema claim. The complete result is accepted. Every lossy projection is rejected as a
schema violation before assessment; no fallback result or reduced finding set is
emitted.
