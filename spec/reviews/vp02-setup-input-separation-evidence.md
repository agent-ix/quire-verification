---
id: SR-114
title: "Evidence review of setup/property input separation"
type: SpecReview
analysis: evidence
scope: "quire-verification@6420b85; QUOIN advice for FR-013-AC-4 and FR-015-AC-1..5"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/TM-001", type: references }
---

## Summary

The deterministic QUOIN advisor ran over 145 obligations. FR-013-AC-4 and all
five FR-015 criteria match catalog-declared test methods; none is mismatched,
uncatalogued, or inconclusive.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | FR-013-AC-4 retains the advised property/metamorphic Test class through TC-026, which varies missing prerequisites and verifies exact ordered setup records without property input. | FR-013-AC-4, TC-026 |
| FND-002 | low | FR-015 retains its prior compatible Test methods and TC-028/TC-030 evidence plan; narrowing the subject from all tasks to verification tasks does not weaken a criterion. | FR-015-AC-1..5, TC-028, TC-030 |
| FND-003 | low | Corpus-wide advisor residue is unchanged and outside this amendment: six pre-existing method mismatches, one pre-existing uncatalogued method, and zero inconclusive obligations. | spec/reviews/data/vp02-setup-input-separation-advice.json |
