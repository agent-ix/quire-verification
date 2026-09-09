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

- Bounded UTF-8 JSON containing either an `ix.shared-reference/2-draft`
  artifact/semantic envelope or an `ix.shared-reference/1-draft`
  source-map/migration envelope from the exact
  `agent-ix/quire-specification@8ab058beff03cc76c0f39cc06c87ef98f76e8d78`
  private data packet.
- Locally supplied artifact bytes, supported versions/features, and qualified canonical derivations.

## Outputs

- A validated role-specific reference or a stable refusal that does not enter planning or result selection.
- Stable refusal codes: `unsupported_contract_version`, `unsupported_reference_version`, `unknown_required_feature`, `invalid_wire`, `identity_mismatch`, `identity_content_conflict`, `stale`, `malformed_source_map`, `unsupported_canonicalization`, and `ambiguous_equivalence`.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-016-AC-1 | A byte reference is accepted only when its version, kind, authority, identity, structured revision, exact byte digest, and wire identity/version are supported and match locally supplied bytes. | Test (TC-029) |
| FR-016-AC-2 | A semantic reference is accepted only when its byte artifact, language, semantic-profile definition digest, duplicate-free required-feature set, and selected canonical identity are supported and internally consistent; a null semantic profile requires null language and canonical identity plus an empty feature set. | Test (TC-029) |
| FR-016-AC-3 | Before materializing a JSON object, duplicate decoded member names, including escape-equivalent spellings, refuse as `invalid_wire`; unknown versions or required features, wrong role kinds, digest mismatch, stale references, identity-content conflict, malformed source correspondence, and ambiguous equivalence refuse with their corresponding stable code before planning or result matching. | Test (TC-029) |
| FR-016-AC-4 | Reordered required features compare as the same set, while every other collection follows its owning contract's declared ordering or identity-key rule. | Test (TC-029) |
| FR-016-AC-5 | The reader accepts only the selected `quire.verification.jcs` / `rfc8785-v1` / `sha256` canonical identity and exact `trace`, `fault-model`, `review-procedure`, `oracle`, `observation`, and `review-disposition` roles without placing `sha256-jcs:` semantic digests in `sha256:` byte-digest fields. | Test (TC-029, TC-031) |
| FR-016-AC-6 | Version 2 accepts only artifact/semantic envelopes with `ix.artifact-ref/3-draft`; retained version 1 accepts only source-map/migration envelopes under its exact schema. Draft-1 artifact/semantic evidence is not reinterpreted as current, no migration is inferred, and no fallback reader treats an unknown kind or version as supported. | Test (TC-029) |
| FR-016-AC-7 | The independent consumer verifies a repository-retained digest manifest for the exact amendment schema, all fourteen amendment fixture files, and the retained version-1 schema; executes every upstream and B-authored positive/adverse case without producer code or network schema retrieval; and reports snapshot drift before reference interpretation. | Test (TC-031) |

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-016-CON-1 | The reader SHALL consume only the exact independently reviewed private shared-reference packet merged at `agent-ix/quire-specification@8ab058beff03cc76c0f39cc06c87ef98f76e8d78`. | Compatibility | Integration Test (IT-002) |
| FR-016-CON-2 | The retained packet snapshot SHALL stay data-only, byte-identical to the selected upstream files, provenance-recorded, and excluded from public promotion until its reusable-artifact terms are selected. | Rights / containment | Inspection (TC-031) |

## Dependencies

- Contract validation and JCS behavior come from [FR-001](./FR-001-validate-public-contracts.md) and [FR-008](./FR-008-use-jcs.md).
- The FS02/FS05 representation dependency is satisfied by merged specification
  revision `8ab058beff03cc76c0f39cc06c87ef98f76e8d78`. Independent B adoption and
  qualification are implemented and pass TC-029 and TC-031.
