---
id: SR-178
title: "Failure-domain review of Task-027 qualification-oracle specification"
type: SpecReview
analysis: failure-domain
scope: "quire-verification@d36f146926f6fc2080d85f489da1056c197d4fd3; FR-017, FR-024, TC-041"
review_set: all
---
# Failure-domain review of Task-027 qualification-oracle specification

## Summary

The reviewed slice covers identity confusion, incomplete guarantee coverage, invalid
dependence topology, correlated evidence, producer containment failures, disagreement,
and reuse after any identity-bearing input changes.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Resolved: an unequal identifier or caller declaration could previously masquerade as causal independence; independent bindings now require positive exact evidence while shared exact references override the declaration. | FR-024-AC-2, TC-041 step 3 |
| FND-002 | medium | Resolved: derived binding ancestry now refuses absent sources, self-reference, and cycles through an iterative bounded traversal. | FR-024-AC-2, TC-041 step 3 |
| FND-003 | low | Typed unavailable, refused, failed, timed-out, malformed-response, and containment-failure producer states remain findings and cannot mint observations. | FR-024-AC-3, TC-041 step 5 |
