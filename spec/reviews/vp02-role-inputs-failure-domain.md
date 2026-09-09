---
id: SR-103
title: "Failure-domain review of E02 draft-2 role-specific inputs"
type: SpecReview
analysis: failure-domain
scope: "quire-verification@432036c; FR-001, FR-002, FR-015, NFR-002, TC-028, TC-030"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-015", type: reviews }
---

## Summary

Reviewed Task-013 preconditions, identity and role failures, closed-variant
edges, draft transition, preservation, and planner refusal behavior. Every
selected failure has a specified pre-plan refusal or retained non-invocable
manual outcome.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Resolved: a structurally valid but locally unverified envelope now refuses because planner membership requires an opaque output from the strict reader in the same caller context. | FR-015-AC-4, TC-028 |
| FND-002 | medium | Resolved: wrong shared-reference versions and roles, missing fields, cross-variant fields, unknown fields, equal relational identities, and duplicate dispositions all have explicit negative cases. | FR-015-AC-1..4, TC-028 |
| FND-003 | medium | Resolved: the breaking transition explicitly refuses the old contract version instead of inferring a migration or falling back to the former input shape. | FR-001-AC-5, FR-015-CON-1 |
| FND-004 | low | Reference comparison uses JCS equality, so JSON member insertion order cannot turn one validated envelope into an unrecognized or distinct identity. | FR-015-AC-4, FR-008 |

## Failure Boundary

Schema refusal and strict-reference refusal occur before a plan is emitted.
The planner preserves eligible inputs and readiness; it does not retrieve
artifacts, execute native tools, infer reference migration, or turn manual work
into native work.
