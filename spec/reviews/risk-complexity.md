---
id: SR-006
title: "Risk and complexity review of the Rust bounded compatibility specification"
type: SpecReview
analysis: risk-complexity
scope: "spec/functional/*.md, spec/non-functional/*.md, spec/assurance/AD-001-rust-architecture.md"
review_set: all
---
# Risk and complexity review of the Rust bounded compatibility specification

## Summary

The dominant risks are schema-driven Rust generation, exact evidence matching,
behavioral migration drift, and future FS02/FS05 volatility. High items have
contract, property, resource-bound, and mandatory Rust-review mitigations.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | low | Production throughput remains deliberately unmeasured; AD-001 requires a MeasurementPlan and benchmark threshold before measured hot-path placement. | AD-001, NFR-004 |

## Risk Register

| Req | Tech Risk | Volatility | Drivers | Mitigation |
|-----|-----------|------------|---------|------------|
| FR-001 | High | Medium | Draft 2020-12 conditionals and code generation | Runtime schema validation plus contract mutation tests |
| FR-002 | High | Medium | Single-source generation without type drift | Build-time generation and schema-derived vocabulary tests |
| FR-003 | Low | Low | Filesystem enumeration | Explicit handler census and isolated temp-directory test |
| FR-004 | Medium | High | External Quoin catalog evolution | Real pinned integration test and first-wins diagnostics |
| FR-005 | Medium | Medium | Task DAG and safety prerequisites | Iterative cycle checks and complete readiness tests |
| FR-006 | High | High | Provisional FS02/FS05 identity seam | Dimension-mutation property test and refusal on ambiguity |
| FR-007 | Medium | Medium | Multi-dimensional state classification | Exhaustive enums and simultaneous-failure tests |
| FR-008 | High | Low | Cryptographic digest preimage canonicalization | RFC 8785 crate plus published-vector tests |
| FR-009 | Medium | Medium | Failure retention across composed services | Injected trait seam and violated/failed result tests |
| NFR-001 | Medium | Low | Deterministic ordering | Ordered maps, stable sorting, property tests |
| NFR-002 | High | Medium | Generated types and runtime schema can diverge | One schema source and source inspection |
| NFR-003 | High | Low | False assurance claim | Explicit adverse states and claim-limit tests |
| NFR-004 | High | Low | Full language rearchitecture | TDD plan, Rust gates, exact rust-review skill |
| NFR-005 | High | Low | Memory or stack exhaustion | Preparse byte cap and iterative structural limits |

## Top Hazards

1. FR-006 can attribute foreign evidence if any identity dimension is omitted.
2. FR-001/FR-002 can recreate two authorities if generated Rust types drift from the schema.
3. NFR-004 can appear complete while JavaScript remains in a command path.
4. NFR-005 can fail under adversarial nesting or collection cardinality.

## Failure-Domain Gaps

No open failure-domain requirement gaps remain; see SR-002.

