---
id: SR-125
title: "Risk and complexity review of bounded native runner requests"
type: SpecReview
analysis: risk-complexity
scope: "quire-verification@c683b5b; FR-005, FR-013, FR-014, FR-015, NFR-006, AD-001"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-014", type: reviews }
  - { target: "ix://agent-ix/quire-verification/NFR-006", type: references }
---

## Summary

FR-014 has high technical risk and low volatility because it authorizes a
security-sensitive native execution boundary under hard resource limits.
The other affected planning requirements have medium or low technical risk and
low volatility; their mitigations are closed schemas and discriminating tests.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The top hazard is authorizing a native request from incomplete, manual, stale, unsupported, or over-limit state. The mitigation is one fail-closed predicate exercised across the full TC-027 refusal table. | FR-014-AC-2..3, TC-027 |
| FND-002 | high | Exact copies cross a security and identity boundary. Closed request validation plus per-field mutation tests mitigate field invention, omission, and confusion between selected values and descriptive catalog text. | FR-014-AC-1, TC-027 |
| FND-003 | medium | Property-over-plan precedence can churn if represented as a field merge. The spec removes that volatility by requiring whole-object replacement and a test with distinguishable complete budgets. | FR-014 Inputs, TC-027 |
| FND-004 | medium | Resource arithmetic has two independent dimensions and a hard ten-second wall ceiling. Minimum, maximum, and one-over boundaries plus safe integer schema limits are required implementation checks. | FR-014-AC-2, FR-014-AC-4, NFR-006 |
| FND-005 | low | External contract volatility is contained to Quoin-authored catalog records and the shared portable envelope; this task adds no dependency on Engineering Assurance internals. | AD-001, AP-001 |

## Risk Register

| Requirement | Technical Risk | Volatility | Mitigation |
| --- | --- | --- | --- |
| FR-005 | medium | low | Reconciled native/manual rule and existing ambiguity tests |
| FR-013 | medium | low | Stable prerequisite identity and deduplicated setup tasks |
| FR-014 | high | low | Closed schema, fail-closed predicate, boundary tables |
| FR-015 | medium | low | Exact JCS role-input copies and heterogeneous integration test |
| NFR-006 | high | low | Ten-second ceiling and existing contained-process tests |
