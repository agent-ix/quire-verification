---
id: SR-045
title: "Risk and complexity review of the VP03 evidence-assessment specification"
type: SpecReview
analysis: risk-complexity
scope: "quire-verification@b0d02af; FR-017..FR-018"
review_set: all
---
# Risk and Complexity Review of the VP03 Evidence-Assessment Specification

## Summary

Classified the new requirements by implementation complexity, evidence risk,
and dependency volatility and named a mitigation for each elevated dimension.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | high | FR-017 has high volatility and risk because its exact FS05 roles remain external and correlation mistakes can inflate evidence claims; block implementation and use permutation/reference-mutation tests after amendment. | FR-017, TC-033 |
| FND-002 | medium | FR-018 has medium arithmetic and boundary complexity but low volatility; checked timestamp arithmetic and nanosecond/offset property cases bound the risk. | FR-018, TC-032 |
| FND-003 | low | No concurrency, persistence, production execution, new cryptographic primitive, or performance guarantee enters this increment. | FR-017, FR-018 |

## Classification

| Requirement | Technical Risk | Volatility | Mitigation |
|-------------|----------------|------------|------------|
| FR-017 | High | High | Wait for reviewed FS05 roles; use exact-reference, permutation, derived-lineage, and correlation tests. |
| FR-018 | Medium | Low | Use checked arithmetic and boundary/property tests over timestamps and maximum ages. |

## Top Hazards

1. Treating shared or derived oracle evidence as independent confirmation.
2. Extending evidence life by selecting the later of two applicable bounds.
3. Overflow or textual timestamp comparison changing an expiry result.
