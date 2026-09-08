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

- FS02 and FS05 identify one accepted private revision and exact fixture digests.
- B's consumer-specific JCS and role findings have an accepted disposition.
- The packet's rights permit private qualification use.

## Inputs

- The exact shared-reference schema bytes and B-relevant positive/adverse vectors.
- Locally supplied referenced artifact bytes and B's declared supported versions/features.

## Test Procedure

1. Verify the selected repository revision and fixture byte digests.
2. Validate every selected positive and adverse vector with B's Rust reader only.
3. Run B's role-composition and JCS-domain mutations.
4. Record accepted/refused counts and stable error categories.

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
- Blocked until the shared owner accepts B's recorded amendment findings.

## Traceability

[TC-031](../test-cases/TC-031-independent-shared-reference-consumer.md) executes this integration boundary.
