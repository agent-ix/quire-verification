---
id: SR-106
title: "Evidence review of E02 draft-2 role-specific inputs"
type: SpecReview
analysis: evidence
scope: "quire-verification@432036c; QUOIN advice for FR-002-AC-4, FR-015-AC-1..5, NFR-002-AC-3"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-015", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TM-001", type: references }
---

## Summary

Ran the deterministic QUOIN advisor over 145 obligations and retained its JSON
output. All seven Task-013 criteria use the catalog-declared `Test` class and
match at least one recommendation; none is inconclusive or uncatalogued.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | Resolved during review: FR-002-AC-4 and NFR-002-AC-3 were authored as inspection even though the advisor selected test methods. They now require a discriminating TC-028 test that detects copied or missing shared-schema references. | FR-002-AC-4, NFR-002-AC-3, TC-028 |
| FND-002 | low | FR-015-AC-1, AC-2, and AC-4 match unit and specification-by-example recommendations; TC-028 additionally uses property permutations for the closed variants and adverse fields. | FR-015-AC-1, FR-015-AC-2, FR-015-AC-4 |
| FND-003 | low | FR-015-AC-3 matches property and negative-abuse methods for its invariant and refusal behavior; FR-015-AC-5 matches test-class recommendations and is discharged by the real heterogeneous integration in TC-030. | FR-015-AC-3, FR-015-AC-5 |

## Advisor Evidence

- Command: `quoin advise --json` with the exact Quire 0.31.0 native binary.
- Retained result: `spec/reviews/data/vp02-role-inputs-advice.json`.
- Corpus result: 145 obligations, 6 pre-existing mismatches outside this
  candidate, 1 pre-existing uncatalogued method outside this candidate, and 0
  inconclusive obligations.
- Scoped result: 7/7 affected obligations have a recommendation compatible with
  their authored `Test` class; 0 scoped mismatches.
