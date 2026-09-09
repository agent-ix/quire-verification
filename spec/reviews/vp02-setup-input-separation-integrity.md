---
id: SR-112
title: "Integrity review of setup/property input separation"
type: SpecReview
analysis: integrity
scope: "quire-verification@6420b85; FR-013-AC-4, FR-015, VerificationPlan task variants"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-013", type: reviews }
  - { target: "ix://agent-ix/quire-verification/FR-015", type: reviews }
---

## Summary

The amendment removes a type and identity ambiguity without changing the
planner's prerequisite semantics. Setup and verification records now have one
valid interpretation each.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Resolved: the `manual-prerequisite` task kind no longer aliases the `manual` property-input kind, preventing a setup action from being mistaken for verification evidence. | FR-013-AC-4, FR-015-AC-3 |
| FND-002 | medium | The exact prerequisite already exists at the setup-task top level; omitting property input avoids a second synchronized copy and retains one identity source. | FR-013-AC-4 |
| FND-003 | low | US-001, FR-013, FR-015, TC-026, and TC-028 retain complete links and non-conflicting responsibilities. | US-001, FR-013, FR-015 |
