---
id: SR-166
title: "Failure-domain review of Task-026 qualification corpus specification"
type: SpecReview
analysis: failure-domain
scope: "quire-verification@e22e31f8813b6dc7d8a5cf923c5c216251d002f6; FR-023, TC-040"
review_set: all
---
# Failure-domain review of Task-026 qualification corpus specification

## Summary

The corpus boundary covers identity confusion, byte drift, missing population classes,
unauthorized retention, forged independence, replay substitution, missing parents,
self-parenting, cycles, and unexplained exclusions.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | Resolved: a caller-supplied byte copy could previously masquerade as replay; derived cases now require a strict-reader-proved run artifact and byte-equal retained output. | FR-023-AC-3..4, TC-040 |
| FND-002 | low | No unstated in-scope corpus failure domain remains; execution containment stays owned by NFR-006 and later technique tasks. | FR-023, NFR-006, Task-028 |
