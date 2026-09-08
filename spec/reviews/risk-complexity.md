---
id: SR-006
title: "Risk and complexity review of the VP01 qualification specification"
type: SpecReview
analysis: risk-complexity
scope: "quire-verification@1d30fed; StR-001, FR-001..FR-012, NFR-001..NFR-005, AD-001"
review_set: all
---
# Risk and complexity review of the VP01 qualification specification

## Summary

The dominant risks are schema-derived Rust behavior, evidence identity,
behavioral migration drift, and future FS02/FS05 volatility. The new VP01 work
is low technical risk and isolated behind direct contract and planner tests.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | low | Production throughput remains deliberately unmeasured; AD-001 requires a MeasurementPlan and threshold before measured hot-path placement. | AD-001, NFR-004 |
| FND-002 | low | The VP01 increment adds no new dependency or executable boundary; its main risk is silently treating omitted classification as authored, covered by FR-011 and TC-024. | FR-011, TC-024 |

## Risk Register

| Req | Tech Risk | Volatility | Drivers | Mitigation |
|-----|-----------|------------|---------|------------|
| StR-001 | High | Low | Qualified-path language and evidence-integrity mandate | Rust-only CI, exact rust-review, coverage census |
| FR-001 | High | Medium | Draft 2020-12 conditionals and code generation | Runtime validation and contract mutation tests |
| FR-002 | High | Medium | Single-source generation without type drift | Build-time generation and vocabulary tests |
| FR-003 | Low | Low | Filesystem enumeration | Explicit handler census and isolated refusal test |
| FR-004 | Medium | High | External Quoin catalog evolution | Pinned real integration test and conflict diagnostics |
| FR-005 | Medium | Medium | Task DAG and safety prerequisites | Cycle checks and complete readiness tests |
| FR-006 | High | High | Provisional FS02/FS05 identity seam | Dimension-mutation tests and ambiguity refusal |
| FR-007 | Medium | Medium | Multi-dimensional result classification | Exhaustive states and simultaneous-failure tests |
| FR-008 | High | Low | Digest preimage canonicalization | RFC 8785 crate and published vectors |
| FR-009 | Medium | Medium | Failure retention across composed services | Injected trait seam and failed-result tests |
| FR-010 | Low | Low | Cross-product of two explicit enums | Table-driven permutation test |
| FR-011 | Low | Medium | Missing versus authored classification | Schema constraint and normalization test |
| FR-012 | Low | Medium | Drift from authored policy/advice | Exact-value tests and score refusal |
| NFR-001 | Medium | Low | Deterministic ordering | Ordered maps and property tests |
| NFR-002 | High | Medium | Generated/runtime contract divergence | One schema source and source inspection |
| NFR-003 | High | Low | False assurance claim | Adverse states and claim-limit tests |
| NFR-004 | High | Low | Qualified-path language compliance | Rust gates, exact rust-review, ix-trace-rs |
| NFR-005 | High | Low | Memory or stack exhaustion | Preparse byte cap and iterative limits |

## Top Hazards

1. FR-006 can attribute foreign evidence if an identity dimension is omitted.
2. FR-001/FR-002 can recreate two authorities if generated Rust drifts from the schema.
3. NFR-004 can appear complete while a disallowed executable remains in a governed path.
4. FR-004 can drift from the Quoin-owned catalog without the pinned integration check.
5. FR-011 can turn missing classification into a misleading authored risk class.

## Failure-Domain Gaps

No open failure-domain requirement gap remains; see SR-002.
