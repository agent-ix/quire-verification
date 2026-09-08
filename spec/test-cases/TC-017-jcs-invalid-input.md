---
id: TC-017
title: "Reject values outside the JCS JSON domain"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-008"
    type: "verifies"
---
# TC-017: Reject values outside the JCS JSON domain

## Description

Verify hashing and structured equality reject inputs that RFC 8785 cannot
canonicalize.

## Test Procedure

Run the byte-ingestion path with malformed JSON and an ill-formed Unicode
escape, attempt to canonicalize a serializable non-finite number, and inspect a
valid digest prefix. Confirm the public Rust value boundary has no representation
for undefined values, sparse arrays, cyclic values, or non-JSON objects.

## Expected Results

Every invalid input raises a descriptive error, while a valid digest matches
`sha256-jcs:` followed by 64 lowercase hexadecimal digits.
