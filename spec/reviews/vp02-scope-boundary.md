---
id: SR-036
title: "Scope-boundary review of the VP02 native-planning specification"
type: SpecReview
analysis: scope-boundary
scope: "quire-verification@611f33f; VP02 ownership and external seams"
review_set: all
---
# Scope-Boundary Review of the VP02 Native-Planning Specification

## Summary

Reviewed component, data, execution, evidence, producer, and human-decision
ownership against the Agent B track and current Agent A/C assignments.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | high | Agent A must amend or disposition the shared JCS domain and role kinds; B must not fork or edit A's schema from this repository. | FR-016-AC-5, specification PR #8 |
| FND-002 | low | B owns the five portable verification contracts, prerequisite/runner semantics, strict consumer, and exact result selection. | FR-013..FR-016 |
| FND-003 | low | Native producers execute tools and retain native artifacts; Quoin retains evidence intake; the authored human retains sufficiency and manual disposition. | FR-014-AC-4, TC-027, TC-030 |
| FND-004 | low | Agent C's Engineering Assurance Rust implementation, Quoin catalog relocation, Filament changes, public promotion, and production fault execution remain outside this scope. | spec/spec.md |

## Boundary Decision

Only FR-013 is dependency-ready. B may prepare tests and code for that planner
behavior after review; FR-014 through FR-016 and IT-002 wait for the shared owner.
