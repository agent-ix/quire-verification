---
id: SR-131
title: "Failure-domain review of VP03 oracle contributions"
type: SpecReview
analysis: failure-domain
scope: "quire-verification@822116c; FR-017, TC-033, AD-001"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-017", type: reviews }
---

## Summary

The review checked strict-reader failure behavior, contribution identity,
lineage cycles, disconnected and ambiguous graph nodes, and traversal bounds.
Every entry boundary now either produces a closed deterministic finding or a
stable refusal before assessment.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved during review: ambiguous exact candidates could influence top-level dependence findings without appearing in any selected assessment entry. They are now excluded from the selected-result graph. | FR-017-AC-4, TC-033 |
| FND-002 | medium | Derived sources resolve against every supplied contribution before matching; absent, self, duplicate, and cyclic lineage fail before any assessment. Valid sources outside the selected graph remain explicitly unknown. | FR-017-AC-3, FR-017-AC-5 |
| FND-003 | medium | Structured `(resultId, contributionId)` pairs are the identity key. Iterative lineage and component traversal under NFR-005 bounds prevents delimiter collision, recursion exhaustion, and nontermination on adversarial cycles. | FR-017-CON-4, AD-001 |
| FND-004 | low | The strict reader is the only trust boundary; duplicate opaque outputs are idempotent, wrong roles or changed envelopes refuse, and no callback or user-supplied executable logic enters this assessment. | FR-017-AC-2 |
