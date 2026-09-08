---
id: FR-008
title: "Canonicalize digest and identity inputs with JCS"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-002"
    type: "implements"
---
# FR-008: Canonicalize digest and identity inputs with JCS

## Description

When a JSON value feeds a digest or structured identity comparison, the compatibility slice SHALL canonicalize that value according to RFC 8785 JCS.

## Inputs

- A JSON value used for a digest or identity comparison.

## Outputs

- Canonical UTF-8 JSON, equality, or a `sha256-jcs:` digest.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-008-AC-1 | Objects with identical JSON meaning and different property insertion order canonicalize equally and produce the same digest. | Test (TC-016) |
| FR-008-AC-2 | RFC 8785 number formatting and UTF-16 property ordering match the canonicalization library behavior. | Test (TC-016) |
| FR-008-AC-3 | Malformed JSON bytes, ill-formed Unicode escapes, and attempted serialization of non-finite numbers are rejected before hashing or comparison; Rust types make undefined values, sparse arrays, cyclic values, and non-JSON objects unrepresentable at this boundary. | Test (TC-017) |
| FR-008-AC-4 | Every produced digest carries the `sha256-jcs:` domain prefix. | Test (TC-017) |

## Dependencies

- Implements [US-002](../usecase/US-002-assess-evidence-exactly.md).
