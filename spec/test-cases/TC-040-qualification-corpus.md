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
generator seed, invocation, replay-run reference, replay output, and parent relation independently. Change
the profile identity, population identity, required-class set, retained bytes, replay
bytes, and artifact kind one axis at a time. Duplicate an identity; add an unknown
class; create missing, self, and cyclic mutation parents; deny retention; and attempt to
label a derived case independent.

## Expected Results

Only the complete corpus qualifies. Empty required classes remain gaps. A reference-only
or unauthorized candidate remains referenced in its owned gap, contributes no retained
bytes, and does not cover its class. Missing or
changed bytes, duplicate identities, unclassified lineage, missing rights, and
unexplained exclusions refuse qualification. Generated and mutated cases replay to the
exact retained bytes and retain their derived status. An unpopulated or unauthorized
required class produces one owned gap and no qualified-complete outcome.
