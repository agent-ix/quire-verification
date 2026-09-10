---
id: SR-183
title: "Scope and boundary review of Task-027 qualification-oracle specification"
type: SpecReview
analysis: scope-boundary
scope: "quire-verification@d36f146926f6fc2080d85f489da1056c197d4fd3; AD-002, FR-024, Task-027"
review_set: all
---
# Scope and boundary review of Task-027 qualification-oracle specification

## Summary

quire-verification owns portable oracle manifests, exact-reference validation,
dependence assessment, comparison projections, and typed findings. Rust Engineering
Assurance owns host/process execution and evidence production; Quoin owns evidence
retention and sufficiency; FR-027 owns the later human disposition binding.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Resolved: the architecture description previously allocated bounded qualification runners to quire-verification, contradicting the common Engineering Assurance boundary. | AD-002, FR-024-CON-1..2 |
| FND-002 | low | Static inspection is specified to refuse a Task-027 local runner, evidence store, fixture script, or arbitrary stdout parser. | TC-041 Expected Results |
