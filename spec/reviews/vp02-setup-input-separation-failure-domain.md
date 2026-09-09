---
id: SR-111
title: "Failure-domain review of setup/property input separation"
type: SpecReview
analysis: failure-domain
scope: "quire-verification@6420b85; FR-013-AC-4 and FR-015"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-013", type: reviews }
---

## Summary

Reviewed missing-input, fabricated-reference, variant-confusion, and downgrade
failures at the setup/verification task boundary. The two task kinds now have
disjoint input obligations.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Resolved: setup work cannot manufacture a review-procedure reference or property-input disposition merely to satisfy the manual variant. It carries its exact prerequisite and no property input. | FR-013-AC-4 |
| FND-002 | medium | Verification tasks still require exactly one of the five closed variants; omitting one or attaching it to the wrong task kind is rejected by the planned schema test. | FR-015-AC-1..4, TC-026, TC-028 |
| FND-003 | low | Coalescing and `requiredBy` ordering remain unchanged, bounded, and covered by TC-026. | FR-013-AC-1, FR-013-AC-4 |
