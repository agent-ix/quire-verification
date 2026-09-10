---
id: SR-171
title: "Scope and boundary review of Task-026 qualification corpus specification"
type: SpecReview
analysis: scope-boundary
scope: "quire-verification@e22e31f8813b6dc7d8a5cf923c5c216251d002f6; FR-023, AD-002, Task-026"
review_set: all
---
# Scope and boundary review of Task-026 qualification corpus specification

## Summary

quire-verification owns the portable corpus manifest, exact-byte validation, coverage
denominator, lineage graph, and gap semantics. Artifact owners retain rights authority;
generator execution remains behind exact run-artifact references; Quoin retains evidence
orchestration and storage ownership.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | No local producer definition, rights decision, generator implementation, or evidence store is introduced by the corpus contract. | FR-023, AD-002, Task-026 |
