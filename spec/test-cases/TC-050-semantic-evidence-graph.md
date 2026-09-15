---
id: TC-050
title: "Semantic evidence graph integrity"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-032"
    type: "verifies"
---
# TC-050: Semantic evidence graph integrity

## Description

Verify exact typed subject-to-evidence traceability and refusal of stale,
cross-kind, malformed, unsupported, or circular discharge paths.

## Test Procedure

Build a valid graph from one exact claim through subject, implementation,
method/provider, run, result, artifact, evidence, and attributed decision. Then
change each identity independently, introduce unsupported edges and cycles, and
remove, redact, or make replay artifacts inaccessible.

## Expected Results

The valid graph retains every exact identity, scope, relation, provenance, and
limitation. Every invalid mutation refuses only the affected discharge. Cycles
without independent evidence confer no support. Missing or redacted material
narrows explicit claims, and no local evidence store or producer execution is
created.

