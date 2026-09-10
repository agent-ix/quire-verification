---
id: SR-170
title: "Risk and complexity review of Task-026 qualification corpus specification"
type: SpecReview
analysis: risk-complexity
scope: "quire-verification@e22e31f8813b6dc7d8a5cf923c5c216251d002f6; FR-023, TC-040, Task-026"
review_set: all
---
# Risk and complexity review of Task-026 qualification corpus specification

## Summary

The principal risks are retaining unauthorized bytes, accepting a large but incomplete
population, confusing derived inputs with independent cases, and trusting replay claims
that do not reach exact output bytes. The specified strict proofs, closed denominator,
rights gate, parent DAG, and replay equality address them directly.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | No material risk lacks a named refusal or gap state; actual corpus acquisition remains blocked on owner rights and Task-025 inputs. | FR-023, Task-025, Task-026 |
