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
shape, records the exact contract and shared-reference revisions, and remains
resolvable by the exact `serde` 1.0.228 consumer graph used by Protocol.

## Test Procedure

Inspect the interface decision and retained shared-reference provenance for the
accepted owner, `quire-verification/e02-draft-2`, immutable verification
revision, shared-reference revision, and private-only publication boundary. Run
the Rust contract test with a valid `TechniqueResult`, then remove each required
execution, logical, activation, participation, assumptions, adequacy,
provenance, native-artifact, and oracle-contribution field independently.
Create a temporary Rust consumer that depends on this worktree and exact
`serde` 1.0.228, then run Cargo resolution and compilation offline. Inspect the
resolved Engineering Assurance revision and re-run the E02 schema and strict
reader tests.

## Expected Results

The provenance names the selected immutable revisions and no public-schema
claim. The complete result is accepted. Every lossy projection is rejected as a
schema violation before assessment; no fallback result or reduced finding set is
emitted. The temporary consumer compiles with one `serde` 1.0.228 graph, the
selected Engineering Assurance revision is exact, and the accepted E02 schema
digest and strict-reader results remain unchanged.
