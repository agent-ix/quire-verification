---
id: TC-029
title: "Refuse malformed or unsupported shared references"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-016"
    type: verifies
---
# TC-029: Refuse Malformed or Unsupported Shared References

## Description

Verify an independent Rust reader accepts only exact supported shared byte and
semantic references before they enter a verification plan or result join.

## Test Procedure

Read bounded raw JSON positive vectors for every B-used reference role, then
mutate one axis at a time: duplicate member name, envelope/ref/wire/profile
version, required feature, byte digest, canonical domain/version/digest,
structured revision, kind, source correspondence, and migration selection.
Reorder required features and separately duplicate a decoded feature.

## Expected Results

Exact supported vectors pass. Every adverse mutation returns its stable refusal
before planning or matching. Feature reordering preserves set identity, a
duplicate feature refuses, and semantic/JCS identities never occupy byte-digest
fields. Retained draft-1 evidence is not rewritten or accepted by fallback.
