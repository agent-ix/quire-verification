---
id: IT-002
title: "Independent shared-reference consumer qualification"
type: IT
relationships:
  - target: "ix://agent-ix/quire-verification/FR-016"
    type: verifies
---
# IT-002: Independent Shared-Reference Consumer Qualification

## Objective

Verify the E02 Rust reader consumes the exact accepted shared-reference packet
without executing or importing the producer's validator.

## Target Integration

The system under test is the `quire-verification` bounded Rust reader. The target
is the accepted schema and data-only fixture packet owned by
`agent-ix/quire-specification` FS02/FS05.

## Preconditions

- FS02 and FS05 identify merged private revision
  `agent-ix/quire-specification@8ab058beff03cc76c0f39cc06c87ef98f76e8d78`.
- B's consumer-specific JCS and six role findings are resolved in that revision.
- Private qualification use is accepted; public reusable-artifact terms remain unresolved.

## Inputs

- The exact shared-reference schema bytes and B-relevant positive/adverse vectors.
- Locally supplied referenced artifact bytes and B's declared supported versions/features.

## Test Procedure

1. Verify the retained snapshot provenance, schema digest, and all fourteen fixture byte digests against the selected repository revision.
2. Reject duplicate decoded JSON members before object materialization, then validate every selected positive and adverse vector with B's Rust reader only.
3. Run B's role-composition, immutable-identity, feature-set, exact-reference, and JCS-domain mutations.
4. Verify the non-circular core digest over the validated automated core alone and refuse byte-digest substitution or a foreign core digest.
5. Record accepted/refused counts and stable error categories without importing producer code or fetching a network schema.

## Expected Results

The consumer accepts every supported exact vector, refuses every adverse vector
before planning or result matching, and reports no producer-code dependency or
unmapped B role.

## Metadata

- Priority: P0
- Target Integration: FS02/FS05 shared-reference data packet
- Automation: Rust integration test with pinned data-only fixtures

## Dependencies

- Verifies [FR-016](../functional/FR-016-read-shared-references-strictly.md).
- The external representation dependency is satisfied at `8ab058be`; B's
  independent reader implementation and qualification remain pending.

## Traceability

[TC-031](../test-cases/TC-031-independent-shared-reference-consumer.md) executes this integration boundary.
