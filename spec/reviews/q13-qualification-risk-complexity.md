---
id: SR-158
title: "Risk and complexity review of the issue 13 qualification specification"
type: SpecReview
analysis: risk-complexity
scope: "StR-002, FR-022..FR-027, NFR-007..NFR-008"
review_set: all
---
# Risk and complexity review of the issue 13 qualification specification

## Summary

Cross-repository production integration, oracle independence, discriminating
generative evidence, report integrity, and an owner-supplied performance budget are
the principal hazards. The plan must isolate volatile producer gates and implement
the stable identity and corpus work first.

## Risk register

| Req | Tech Risk | Volatility | Drivers | Mitigation |
|-----|-----------|------------|---------|------------|
| StR-002 | Medium | Medium | Owner decision over a bounded claim | Freeze one profile and require a separate disposition. |
| FR-022 | Medium | Medium | Many identity-bearing dimensions | Property-test one-axis mutations and pin exact revisions. |
| FR-023 | Medium | Medium | Corpus selection and rights | Close the denominator and retain lineage and exclusions. |
| FR-024 | High | Medium | Correlated or absent oracles | Classify dependence and preserve disagreement. |
| FR-025 | High | Low | Fuzz, mutation, shrinking, and budgets | Implement bounded Rust harnesses and mutation ratchets. |
| FR-026 | High | High | Four active producer lanes and real caller integration | Use contract cases and block on accepted revisions. |
| FR-027 | High | Medium | Claim integrity and human authority | Seal the core separately and test every invalid disposition. |
| NFR-007 | High | High | Owner thresholds are not selected yet | Keep measurement blocked until profile freeze; retain per-phase data. |
| NFR-008 | High | Medium | Byte availability and environmental drift | Pin artifacts and require independent reproduction before disposition. |

## Top hazards

1. FR-026 interface volatility can invalidate an adapter; compatibility work begins
   only after the owning producer accepts an exact revision.
2. FR-024 correlated agreement can look independent; provenance and dependence are
   first-class result data.
3. NFR-007 has no universal threshold; the profile owner supplies every claimed
   phase budget before measurement begins.
4. FR-027 can overstate narrow evidence; the final claim is byte-bounded by FR-022.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | low | No high-risk item lacks a named mitigation; FR-026 and NFR-007 remain deliberately gated until external revisions and owner thresholds are accepted. | FR-026, NFR-007 |
