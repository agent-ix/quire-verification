---
id: SR-031
title: "Failure-domain review of the VP02 native-planning specification"
type: SpecReview
analysis: failure-domain
scope: "quire-verification@611f33f; FR-013..FR-016 and TC-026..TC-031"
review_set: all
---
# Failure-Domain Review of the VP02 Native-Planning Specification

## Summary

Reviewed preconditions, partial failures, timeout/resource states, unsafe fault
authorization, malformed inputs, and external shared-reference failures.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | high | A JCS identity cannot be serialized in the proposed shared schema and required trace/fault/manual roles have no exact kind, so a consumer could misclassify a semantic digest as bytes or guess a role. | FR-016-AC-5, IT-002 |
| FND-002 | medium | Resolved: planning no longer claims to observe runtime budget exhaustion; absent allocation blocks planning, while exhaustion remains a later result state. | FR-013-AC-2, TC-026 |
| FND-003 | low | Missing, duplicate, stale, unsupported, wrong-role, digest-conflicting, source-map, and ambiguous cases all have explicit refusal or blocked outcomes; no failure is converted to non-applicable or successful evidence. | FR-013-AC-1..3, FR-014-AC-2..3, FR-016-AC-1..6 |

## Failure Boundaries

The planner may report blocked or unsupported work and may emit a runner request;
it does not execute a tool. A native runner may later report failure, cancellation,
or exhaustion through VP03. Fault invocation requires all three independent safety
bindings. Shared input refuses before planning or result matching.
