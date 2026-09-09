---
id: SR-122
title: "Integrity review of bounded native runner requests"
type: SpecReview
analysis: integrity
scope: "quire-verification@c683b5b; FR-005, FR-013, FR-014, FR-015, NFR-006, AD-001, TC-027, TC-030"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-014", type: reviews }
---

## Summary

The affected requirements form one consistent planning boundary: applicability,
readiness, runner eligibility, and human ownership are separate observable
values. Each new obligation has explicit verification and an owning stakeholder
path through US-001 and StR-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved during review: the parent ready-task criterion conflicted with the manual specialization. The revised FR-005 criterion and Test Matrix now agree with FR-014 and FR-015. | FR-005-AC-2, FR-014-AC-2, FR-015-AC-3, TM-001 |
| FND-002 | medium | The property-budget precedence rule is singular and deterministic: a complete property budget replaces the complete plan budget, so no field-level merge or hidden default remains. | FR-014, TC-027 |
| FND-003 | medium | Runner construction and runner refusal are split into independently testable criteria; fault authorization and ownership constraints remain separate criteria with concrete observations. | FR-014-AC-1..4 |
| FND-004 | low | The specification introduces no duplicate budget object: selected budgets and capability limits have distinct names, dimensions, roles, and validation boundaries. | FR-014 Inputs, FR-014 Outputs |
| FND-005 | low | All affected criteria have verification links; their source evidence is intentionally pending until Task-014 implementation, so the partial matrix state is consistent. | TC-009, TC-026, TC-027, TC-028, TC-030 |
