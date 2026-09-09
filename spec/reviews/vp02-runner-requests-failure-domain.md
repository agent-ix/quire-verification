---
id: SR-121
title: "Failure-domain review of bounded native runner requests"
type: SpecReview
analysis: failure-domain
scope: "quire-verification@c683b5b; FR-005, FR-013, FR-014, FR-015, NFR-006, TC-027, TC-030"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-014", type: reviews }
---

## Summary

The runner boundary now refuses absent authority, ambiguous capability choice,
missing or excessive resources, manual work, and incomplete fault authorization
without creating a partially invocable request. The affected planner graph has
stable identities and finite execution dimensions.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved during authoring: an absent budget previously had no defined failure behavior and could flow into an open request. It now produces an exact capability-keyed resource-allocation prerequisite and a null request. | FR-013-AC-2, FR-014-AC-2 |
| FND-002 | high | Resolved during authoring: capability-relative over-budget behavior was undefined. Either exceeded dimension now blocks invocation and retains exact requested and limiting objects in one typed gap. | FR-014-AC-2, TC-027 |
| FND-003 | high | Resolved during review: ready manual work could inherit the generic invocation rule. Manual tasks now remain ready human work while carrying `runnerRequest: null` and `invocationAllowed: false`. | FR-005-AC-2, FR-014-AC-2, FR-015-AC-3 |
| FND-004 | medium | Capability ambiguity, stale policy, incompatible input/profile, unqualified capability, and incomplete fault authorization already have strict fail-closed behavior; TC-027 composes these states with the runner boundary. | FR-005-AC-3..6, FR-014-AC-2..3 |
| FND-005 | low | No new callback, recursive traversal, or user-supplied executable logic is introduced. Composite task identifiers and exact prerequisite identities remain governed by FR-013. | FR-013-AC-3..5 |
