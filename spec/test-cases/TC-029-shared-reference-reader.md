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
Exercise valid and malformed retained version-1 source maps over controlled
original/body bytes, and zero, one, and conflicting exact migration mappings.
Present version-1 artifact/semantic envelopes and version-2 source-map/migration
envelopes to prove kind/version allocation cannot fall through to another decoder.

## Expected Results

Exact supported vectors pass. Duplicate decoded member names and malformed
shapes return `invalid_wire`; unknown envelope/ref/wire/profile versions return
`unsupported_reference_version`; unknown features return
`unknown_required_feature`; unselected canonicalization returns
`unsupported_canonicalization`; validly shaped digest, role, or one-axis
reference mismatches return `identity_mismatch`; and reused immutable identity
with a conflicting digest in one input set returns `identity_content_conflict`.
An unavailable selected revision returns `stale`, malformed segment coverage or
byte correspondence returns `malformed_source_map`, and multiple applicable
mappings return `ambiguous_equivalence`. Every refusal occurs before planning or
matching. One exact accepted mapping preserves only its stated byte-artifact
correspondence. Feature reordering preserves set identity, duplicate features
refuse, semantic/JCS identities never occupy byte-digest fields, and historical
artifact/semantic evidence is neither rewritten nor accepted by a fallback.
