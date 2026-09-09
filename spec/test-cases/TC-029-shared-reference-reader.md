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

Read the pinned packet through the strict raw-JSON entry point. Exercise all six
role envelopes and the selected semantic reference with their locally supplied
bytes. Apply the upstream structural, semantic, role-substitution, and
exact-reference mutations one at a time across envelope/ref/wire/profile
versions, required features, byte and canonical digests, canonical
domain/version/algorithm, authority, identity, structured revision, kind, and
wire identity/version. Add nested and top-level duplicate decoded member names,
including `"a"` versus `"\u0061"`; changed bytes under a reused immutable
identity; reordered and duplicate required features; null-profile coupling;
unknown drafts; and a retained draft-1 value presented to the new reader.

## Expected Results

Exact supported vectors pass. Duplicate decoded member names and malformed
shapes return `invalid_wire`; unknown envelope/ref/wire/profile versions return
`unsupported_reference_version`; unknown features return
`unknown_required_feature`; unselected canonicalization returns
`unsupported_canonicalization`; validly shaped digest, role, or one-axis
reference mismatches return `identity_mismatch`; and reused immutable identity
with different supplied bytes returns `identity_content_conflict`. Every refusal
occurs before planning or matching. Feature reordering preserves set identity,
duplicate features refuse, semantic/JCS identities never occupy byte-digest
fields, and retained draft-1 evidence is neither rewritten nor accepted by a
fallback.
