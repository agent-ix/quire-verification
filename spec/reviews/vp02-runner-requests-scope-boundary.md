---
id: SR-126
title: "Scope-boundary review of bounded native runner requests"
type: SpecReview
analysis: scope-boundary
scope: "quire-verification@c683b5b; FR-005, FR-013, FR-014, FR-015, NFR-006, AD-001, AP-001"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-014", type: reviews }
---

## Summary

The local planner owns validation and construction of a closed native request;
it does not run Quoin, manufacture capabilities, make evidence claims, replace
human disposition, or absorb Engineering Assurance internals. Native execution
remains a separate contained adapter responsibility governed by NFR-006.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved during authoring: an open request could have absorbed evidence or sufficiency fields. FR-014 now closes the transport value and expressly excludes evidence storage, outcome, and automated disposition. | FR-014-AC-1, FR-014-AC-4 |
| FND-002 | high | Manual verification is owned by authored human work and never crosses the native invocation seam, even when its prerequisites make it ready. | FR-014-AC-2, FR-015-AC-3 |
| FND-003 | medium | Quoin is an external catalog and evidence owner whose portable records are consumed as validated inputs; this component neither reimplements nor invokes Quoin's TypeScript internals. | FR-014-AC-4, AD-001 |
| FND-004 | medium | Engineering Assurance remains owned by Agent C under its Rust transition. This task shares only the portable E02 contract surface and makes no decision about that subsystem's internal seam. | AD-001, AP-001 |
| FND-005 | low | Responsibility allocation is exact: contract/schema validation is infrastructure, planner eligibility is core logic, NFR-006 process containment is cross-cutting execution control, and human sufficiency is external. | FR-005, FR-014, NFR-006 |
