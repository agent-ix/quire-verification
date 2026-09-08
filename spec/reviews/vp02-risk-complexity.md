---
id: SR-035
title: "Risk and complexity review of the VP02 native-planning specification"
type: SpecReview
analysis: risk-complexity
scope: "quire-verification@f7a3dcf; FR-013..FR-016"
review_set: all
---
# Risk and Complexity Review of the VP02 Native-Planning Specification

## Summary

Classified the new requirements by implementation complexity, evidence risk, and
dependency uncertainty.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | high | FR-016 is high risk because raw duplicate detection, external schema pinning, local byte resolution, canonical-domain qualification, and role mapping cross trust and ownership boundaries. | FR-016, IT-002 |
| FND-002 | high | Fault runner authorization is safety-sensitive; any partial binding must keep the request null and invocation false. | FR-014-AC-3, TC-027 |
| FND-003 | medium | Prerequisite coalescing and deterministic dependency identifiers have combinatorial cases but remain isolated within the current Rust planner. | FR-013, TC-026 |
| FND-004 | medium | Five closed input variants and a heterogeneous parser fixture require a schema revision and broad fixture refresh after FR-016 unblocks. | FR-015, TC-028, TC-030 |
| FND-005 | medium | Composite task identity must remain injective for arbitrary schema-valid component strings; delimiter-collision cases belong in the prerequisite property test. | FR-013-AC-5, TC-026 |

## Classification

| Requirement | Complexity | Risk | Reason |
|-------------|------------|------|--------|
| FR-013 | Medium | Medium | Deterministic graph/coalescing over existing planner data. |
| FR-014 | Medium | High | Permission and fault-safety authorization boundary. |
| FR-015 | Medium | Medium | Closed discriminated variants and fixture migration. |
| FR-016 | High | High | Cross-repository strict wire, raw JSON, bytes, and canonical identities. |
