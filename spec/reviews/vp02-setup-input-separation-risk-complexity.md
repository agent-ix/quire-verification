---
id: SR-115
title: "Risk and complexity review of setup/property input separation"
type: SpecReview
analysis: risk-complexity
scope: "quire-verification@6420b85; setup and verification task input shapes"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-013", type: reviews }
---

## Summary

The amendment has low technical risk and low volatility. It removes a schema
union ambiguity and requires coordinated schema, generated-type, planner, and
fixture changes already contained in Task-013.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | Generated plan-task input becomes optional at the structural type level, so runtime conditionals must require it for verification tasks and forbid it for setup tasks. Mitigation: delete/insert discriminating cases in TC-026 and TC-028. | FR-013-AC-4, FR-015-AC-4 |
| FND-002 | low | Existing fixtures and retained reports contain setup `input` objects and require a mechanical draft-2 refresh; exact semantic assertions prevent silent reshaping. | TC-005, TC-026 |
| FND-003 | low | No concurrency, external service, cryptographic primitive, or policy volatility is added. | FR-013, FR-015 |
