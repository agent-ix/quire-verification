---
id: SR-042
title: "Integrity review of the VP03 evidence-assessment specification"
type: SpecReview
analysis: integrity
scope: "quire-verification@b0d02af; VP03 identity and claim boundaries"
review_set: all
---
# Integrity Review of the VP03 Evidence-Assessment Specification

## Summary

Reviewed contribution identity, exact-reference trust, freshness arithmetic,
claim language, evidence ownership, and traceability from US-002 to tests.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | high | FR-017 cannot be implemented without a reviewed exact mapping for oracle and observation artifacts; a generic local reference would fork FS05 identity. | FR-017-CON-1 |
| FND-002 | medium | Resolved: contribution identity is unique within a result, cross-result lineage uses `(resultId, contributionId)`, and input order cannot define output order. | FR-017-AC-1..4, TC-033 |
| FND-003 | medium | The word `independent` describes only the supplied declared relationship; FR-017-CON-2 and CON-3 prevent promotion to an undisclosed causal-independence claim. | FR-017-CON-2, FR-017-CON-3 |
| FND-004 | low | US-002 traces to both new FRs, every new criterion maps to TC-032 or TC-033, and no new NFR scope or second evidence owner is implied. | US-002, TM-001 |

## Traceability Result

`StR-001 → US-002 → FR-017/FR-018 → TC-033/TC-032` is complete. Existing
FR-006, FR-007, and FR-016 supply exact matching, independent finding
preservation, and strict shared-reference prerequisites without duplicated rules.
