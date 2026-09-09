---
id: SR-102
title: "Base review of E02 draft-2 role-specific inputs"
type: SpecReview
analysis: base
scope: "quire-verification@432036c; FR-001, FR-002, FR-015, NFR-002, AD-001, TC-003, TC-028, TC-030, TM-001"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-015", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TM-001", type: references }
---

## Summary

The owner-selected `all` review checked the Task-013 specification candidate at
`432036c` for identifier shape, requirement quality, link integrity, closed
variants, error and edge cases, and Test Matrix coverage. All 157 specification
documents pass Quire structural validation and all affected criteria have a
concrete planned test.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | Resolved during authoring: the contract requirements and architecture still named draft-1 even though Task-013 promised a breaking draft-2 schema; every authoritative pointer now selects `e02-draft-2.schema.json` and explicitly refuses draft-1 fallback. | FR-001, FR-002, NFR-002, AD-001 |
| FND-002 | high | Resolved during authoring: role-correct structure alone could let a caller fabricate a reference that had never passed identity, digest, feature, and local-byte checks. Planning now requires the corresponding opaque strict-reader output and matches it by exact JCS equality and role. | FR-015-AC-4, FR-016 |
| FND-003 | medium | Resolved during authoring: manual dispositions were described as a set without defining duplicate or order behavior. They are now a duplicate-free nonempty array over three closed values whose authored order is preserved. | FR-015-AC-3, TC-028 |
| FND-004 | low | Coverage is honestly partial before implementation: Quire reports 118/162 rows backed, including 0/5 FR-015 criteria and the new FR-002/NFR-002 schema-reference criteria. | FR-002-AC-4, FR-015-AC-1..5, NFR-002-AC-3 |
| FND-005 | low | The active module structurally requires the matrix heading `Coverage Status` but its traceability declaration looks for `Status`, so status-lie classification is unavailable for that table. The affected rows explicitly remain partial/planned and do not claim completion. | spec/tests.md:43, SR-101-FND-003 |

## Checklist Result

IDs remain unique and well formed. The affected FRs link to US-001, define
inputs and outputs, name the breaking version boundary, and map every criterion
to TC-002, TC-003, TC-004, TC-028, or TC-030. TC-028 covers all five positive
variants plus missing fields, equal relational subjects, wrong and unvalidated
references, duplicate dispositions, cross-variant fields, unknown fields, and
the retired draft-1 version. TC-030 covers the heterogeneous integration case.
No unresolved base-review finding blocks implementation.
