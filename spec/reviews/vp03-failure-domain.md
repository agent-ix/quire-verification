---
id: SR-041
title: "Failure-domain review of the VP03 evidence-assessment specification"
type: SpecReview
analysis: failure-domain
scope: "quire-verification@b0d02af; FR-017..FR-018 and TC-032..TC-033"
review_set: all
---
# Failure-Domain Review of the VP03 Evidence-Assessment Specification

## Summary

Reviewed identity collision, correlated evidence, missing lineage, temporal
ordering, timestamp equivalence, arithmetic overflow, and stale-result masking.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | high | Oracle and observation references cannot be assigned an exact shared kind at the current FS05 proposal, so accepting them would require a local role guess or generic-kind downgrade. | FR-017-CON-1, TC-033 |
| FND-002 | medium | Resolved: duplicate contribution IDs, absent relations, derived-source ambiguity, and identifier-only independence are explicit adverse cases. | FR-017-AC-1..4, TC-033 |
| FND-003 | medium | Resolved: the effective expiry uses both limits, comparisons operate on instants rather than timestamp text, and invalid order or overflow refuses before classification. | FR-018-AC-1..4, TC-032 |
| FND-004 | low | A stale result retains all independent findings and native references; temporal precedence cannot erase execution, logical, activation, participation, assumption, or adequacy failures. | FR-018-AC-5, TC-032 |

## Failure Boundaries

The assessor consumes bounded data and emits a non-authoritative view. It does
not execute an oracle, infer undisclosed causal independence, average evidence,
persist into Quoin, or make a sufficiency decision.
