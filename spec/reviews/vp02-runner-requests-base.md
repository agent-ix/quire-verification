---
id: SR-120
title: "Base review of bounded native runner requests"
type: SpecReview
analysis: base
scope: "quire-verification@c683b5b; FR-005, FR-013, FR-014, FR-015, NFR-006, AD-001, TC-009, TC-027, TC-030, TM-001"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-014", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TM-001", type: references }
---

## Summary

The owner-selected `all` review checked the Task-014 specification candidate at
`c683b5b` for requirement quality, link integrity, boundary behavior, and the
six Test Matrix coverage rules. Quire validates all 173 specification documents,
and every affected criterion names a concrete test case.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved during review: FR-005 previously called every applicable ready qualified task invocable, contradicting the manual-work boundary in FR-014 and FR-015. FR-005 now limits native invocation to qualified `native-producer` work with an in-limit budget. | FR-005-AC-2, FR-014-AC-2, FR-015-AC-3 |
| FND-002 | medium | Resolved during authoring: the prior runner request had no closed field contract or declared resource ceilings. FR-014 now defines exact copied fields, positive two-dimensional budgets and limits, and the NFR-006 wall-clock ceiling. | FR-014-AC-1, FR-014-AC-4, NFR-006 |
| FND-003 | medium | Missing and excessive budgets now have distinct deterministic behavior: absence creates one actionable resource-allocation setup dependency, while excess creates a value-retaining gap and never authorizes invocation. | FR-013-AC-2, FR-014-AC-2 |
| FND-004 | low | Coverage is honestly partial before implementation: Quire reports 125/162 rows backed, including 0/4 FR-014 criteria and 4/5 FR-015 criteria. TC-027 and TC-030 remain pending. | FR-014, FR-015-AC-5, TC-027, TC-030 |
| FND-005 | low | The installed traceability module still expects a `Status` heading while the TestMatrix schema requires `Coverage Status`; the affected rows remain explicitly pending and no status lie is claimed. | TM-001, SR-119-FND-003 |

## Checklist Result

IDs are unique and well formed. The changed FRs link to US-001 and their actual
prerequisites, define inputs and outputs, distinguish every refusal state, and
map every criterion to TC-009, TC-026, TC-027, TC-028, or TC-030. The tests cover
both budget dimensions, absent budget, property-over-plan precedence, manual
work, every non-invocable planning state, incomplete fault authorization,
unknown output fields, and the heterogeneous integration case. No unresolved
base finding blocks implementation.
