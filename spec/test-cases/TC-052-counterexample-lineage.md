---
id: TC-052
title: "Immutable counterexample lineage and replay"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-033"
    type: "verifies"
---
# TC-052: Immutable counterexample lineage and replay

## Description

Verify original, transformed, shrunk, translated, normalized, redacted, and
replayed counterexample revisions remain immutable and claim-relative.

## Test Procedure

Create a valid original and ordered child lineage with exact typed values/traces,
operations, correspondences, providers, and replay outcomes. Attempt to overwrite
a parent, create a cycle, change domain validity or failing disposition, omit a
correspondence, redact replay-required fields, and replay against a changed
subject/provider revision.

## Expected Results

Valid children preserve immutable parentage and exact replay provenance. Invalid
or verdict-changing transformations are refused without deleting ancestors.
Redacted, inaccessible, changed-revision, and lost-material revisions remain
visible as limited non-replayable evidence and cannot substitute for a qualified
counterexample.

