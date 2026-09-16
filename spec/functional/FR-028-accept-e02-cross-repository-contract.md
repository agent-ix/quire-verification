---
id: FR-028
title: "Accept the E02 cross-repository contract"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-007"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-016"
    type: "depends_on"
---
# FR-028: Accept the E02 cross-repository contract

## Description

The verification compatibility slice SHALL accept
`quire-verification/e02-draft-2` at one recorded immutable repository revision
for private cross-repository Rust consumption, while preserving every required
result dimension and every strict-reader refusal.

## Inputs

- One structured E02 draft-2 contract value.
- Retained shared-reference packet values selected by the strict reader.

## Outputs

- An unchanged validated contract value, or a typed refusal before planning,
  matching, or assessment.

## Behavior

- When a private Rust consumer selects an exact dependency-compatible
  `quire-verification` revision, the package SHALL resolve its accepted exact
  Engineering Assurance producer revision alongside the consumer's exact
  `serde` 1.0.228 requirement.
- The dependency-compatible consumer revision SHALL preserve the accepted
  E02 draft-2 schema bytes and strict-reader behavior recorded at
  `d2fc0d8134c776d2f1a17fd249f016b18a6ba124`.
- If the complete consumer dependency graph cannot resolve, then the
  verification compatibility slice SHALL NOT identify that revision as an
  accepted cross-repository consumer revision.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-028-AC-1 | The owner, contract version, immutable accepted revision, retained shared-reference revision, and private-only publication boundary are recorded in the interface decision and contract provenance. | Inspection (TC-046) |
| FR-028-AC-2 | A boundary value with an unknown contract version, unsupported reference version, wrong role, or identity/content mismatch is refused without compatibility inference, fallback, or a replacement result. | Test (TC-002, TC-029, TC-031) |
| FR-028-AC-3 | A result projection that omits any required execution, logical, activation, participation, assumptions, adequacy, provenance, artifact, or oracle-contribution dimension is refused before assessment. | Test (TC-046) |
| FR-028-AC-4 | A minimal private Rust consumer with exact `serde` 1.0.228 resolves and compiles the selected exact `quire-verification` and Engineering Assurance revisions while the accepted E02 schema digest and strict-reader tests remain unchanged. | Test (TC-046) |

## Dependencies

- Uses the exact contract validation in [FR-001](./FR-001-validate-public-contracts.md).
- Preserves independent findings through [FR-007](./FR-007-preserve-result-findings.md).
- Uses the strict shared-reference boundary in [FR-016](./FR-016-read-shared-references-strictly.md).
- Depends on the accepted Engineering Assurance producer boundary and its
  consumer-compatible normal dependency requirements.
