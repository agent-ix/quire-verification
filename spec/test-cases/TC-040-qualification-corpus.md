---
id: TC-040
title: "Qualify corpus population and lineage"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-023"
    type: "verifies"
---
# TC-040: Qualify corpus population and lineage

## Description

Verify that the qualification corpus covers or explicitly gaps every profile class
and retains exact bytes, provenance, rights, selection rationale, and reproducible
generation or mutation lineage.

## Test Procedure

Validate representative real, boundary, malformed, generated, and mutated inputs.
Remove each class, byte artifact, provenance field, rights decision, selection reason,
generator seed, and parent relation independently. Change bytes under a retained
digest, duplicate an identity, and attempt to label a derived case independent.

## Expected Results

Only the complete corpus qualifies. Empty required classes remain gaps. Missing or
changed bytes, duplicate identities, unclassified lineage, missing rights, and
unexplained exclusions refuse qualification; generated and mutated cases replay and
retain their derived status.

