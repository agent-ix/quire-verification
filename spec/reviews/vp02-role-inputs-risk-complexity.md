---
id: SR-107
title: "Risk and complexity review of E02 draft-2 role-specific inputs"
type: SpecReview
analysis: risk-complexity
scope: "quire-verification@432036c; Task-013 draft-2 schema and role-input transition"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-015", type: reviews }
---

## Summary

The transition has medium technical risk and low volatility. The main hazards
are external JSON Schema resolution, generated-type behavior, opaque validation
membership, and coordinated fixture/report migration.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | External `$ref` resolution must agree across Typify generation and runtime validation. Mitigation: one exact retained schema, generated types, a runtime registry, and a test that breaks the reference. | FR-002-AC-4, NFR-002-AC-3, TC-028 |
| FND-002 | medium | A broad draft-2 fixture migration could reshape evidence accidentally. Mitigation: retain semantic assertions, validate every fixture, reject draft-1 explicitly, and inspect retained report changes. | FR-001-AC-5, FR-015-CON-1, TC-005, TC-028 |
| FND-003 | medium | Matching embedded envelopes against validated outputs can become quadratic or accept forged values if implemented as an open list. Mitigation: keep the output type opaque and build one keyed JCS membership index per planning call. | FR-015-AC-4, NFR-005 |
| FND-004 | low | The shared-reference representation is pinned to merged revision `8ab058be`; the private E02 draft may change later, but this task contains that volatility in one schema transition. | FR-016-CON-1, AD-001 |

## Risk Register

| Requirement | Technical Risk | Volatility | Mitigation |
|---|---|---|---|
| FR-001 | Low | Low | Exact contract version and negative test |
| FR-002 / NFR-002 | Medium | Low | External-reference generation/runtime tests |
| FR-015 | Medium | Low | Closed schema, opaque reader output, property tests |
| AD-001 | Low | Low | Exact boundary and no-fallback decision |

No concurrency, distributed coordination, new cryptography, or quantified
production performance guarantee enters Task-013.
