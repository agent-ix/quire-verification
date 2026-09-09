---
id: SR-050
title: "Base review of the VP04 native parser portfolio specification"
type: SpecReview
analysis: base
scope: "quire-verification@d3dbf5b; FR-019..FR-021, NFR-006, IT-003, MP-001, TC-034..TC-038 and amended traceability"
review_set: all
---
# Base Review of the VP04 Native Parser Portfolio Specification

## Summary

The owner-selected `all` review checked the VP04 candidate at `d3dbf5b` for
identifier format, atomicity, link integrity, terminology, negative controls,
and the six Test Matrix rules. Quire reports 125 of 125 documents grammar-clean.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | medium | Resolved during review: the native child boundary now has direct-argv, clean-environment, descriptor-pinned path, dedicated-process-group, deadline, concurrent-output, status, protocol, termination, and reap requirements with exact limits. | NFR-006, TC-038 |
| FND-002 | medium | Resolved during review: the automated report core is sealed before a separately supplied human disposition refers to it, avoiding a circular digest and adding exact foreign/duplicate refusal behavior. | FR-021, TC-037 |
| FND-003 | medium | Resolved during review: measurement observations now have a JSON-safe integer domain, unique IDs and phase/repetition pairs, exact 16-slot cardinality, deterministic lower-median rule, and invalid-series refusal. | FR-020, MP-001, TC-036 |
| FND-004 | high | Open dependency gate: VP04 requires accepted VP02 and VP03 behavior, and the shared contract still lacks B's JCS domain plus exact trace, fault-model, review-procedure, oracle, observation, and review-disposition roles. | FR-016, FR-017, FR-019, FR-021, IT-003 |
| FND-005 | low | Coverage at the reviewed specification candidate is honestly 99/157. A direct binding audit finds all 26 VP04 acceptance criteria and TC-034 through TC-038 still without Rust evidence. | FR-019..FR-021, NFR-006, TC-034..TC-038 |
| FND-006 | high | Resolved during re-review: pathname checking followed by pathname execution admitted a substitution race, and direct-child-only termination admitted descendants. The selected Linux profile now requires `openat2` descriptor admission and whole-process-group containment before Task-020 can complete. | NFR-006-AC-3..7, TC-038 |

## Checklist Result

IDs are unique and sequential. Each new FR links to its driving story, has typed
inputs and outputs, names errors and dependencies, and maps every criterion to a
detailed Test Case or IT-003. The matrix covers method combinations, numeric and
process boundaries, typed errors, plan/evidence/report transitions, identity
collisions, pipe deadlock, stale evidence, foreign disposition, and claim limits.
Every implementation test in this scope must use compiler-checked `ix-trace-rs`
markers under NFR-004. FND-004 requires the implementation plan to preserve the
dependency gates and prevents implementation from being reported as a qualified
VP04 result before those gates clear.
