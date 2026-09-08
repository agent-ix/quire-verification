---
id: FR-016
title: "Read shared references strictly"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-001"
    type: implements
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: depends_on
  - target: "ix://agent-ix/quire-verification/FR-008"
    type: depends_on
  - target: "ix://agent-ix/quire-specification/FS02"
    type: depends_on
  - target: "ix://agent-ix/quire-specification/FS05"
    type: depends_on
---
# FR-016: Read Shared References Strictly

## Description

When an E02 draft-2 contract carries a shared byte or semantic reference, the
Rust reader SHALL validate the exact selected shared-reference version and role
before using the reference in planning or result identity comparison.

## Inputs

- Bounded UTF-8 JSON containing a selected shared `ArtifactRef` or `SemanticRef`.
- Locally supplied artifact bytes, supported versions/features, and qualified canonical derivations.

## Outputs

- A validated role-specific reference or a stable refusal that does not enter planning or result selection.
- Stable refusal codes: `unsupported_contract_version`, `unsupported_reference_version`, `unknown_required_feature`, `invalid_wire`, `identity_mismatch`, `identity_content_conflict`, `malformed_source_map`, `unsupported_canonicalization`, and `ambiguous_equivalence`.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-016-AC-1 | A byte reference is accepted only when its version, kind, authority, identity, structured revision, exact byte digest, and wire identity/version are supported and match locally supplied bytes. | Test (TC-029) |
| FR-016-AC-2 | A semantic reference is accepted only when its byte artifact, language, semantic-profile definition digest, duplicate-free required-feature set, and selected canonical identity are supported and internally consistent. | Test (TC-029) |
| FR-016-AC-3 | Duplicate member names, unknown versions or required features, wrong role kinds, digest mismatch, identity-content conflict, malformed source correspondence, and ambiguous equivalence refuse with the corresponding stable code before planning or result matching. | Test (TC-029) |
| FR-016-AC-4 | Reordered required features compare as the same set, while every other collection follows its owning contract's declared ordering or identity-key rule. | Test (TC-029) |
| FR-016-AC-5 | The shared schema represents B's named JCS canonical-identity domain and exact trace, fault-model, and review-procedure roles without placing semantic digests in byte-digest fields. | Test (TC-029, TC-031) |
| FR-016-AC-6 | A breaking draft-1 to draft-2 transition does not reinterpret or rewrite retained draft-1 evidence, and no fallback reader treats an unknown draft as current. | Test (TC-029) |

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-016-CON-1 | The reader SHALL consume the exact independently reviewed shared-reference schema revision selected by VP02. | Compatibility | Integration Test (IT-002) |

## Dependencies

- Contract validation and JCS behavior come from [FR-001](./FR-001-validate-public-contracts.md) and [FR-008](./FR-008-use-jcs.md).
- Acceptance is blocked on `ix://agent-ix/quire-specification/FS02` and `ix://agent-ix/quire-specification/FS05`; PR #8 head `609790e` remains conditional pending B's recorded amendment findings.
