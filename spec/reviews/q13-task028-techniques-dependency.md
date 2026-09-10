---
id: SR-191
title: "Dependency review of Task-028 qualification techniques"
type: SpecReview
analysis: dependency
scope: "quire-verification@6adbbc2; FR-022..FR-025, NFR-004..NFR-005, Task-025..Task-029"
review_set: all
---
# Dependency review of Task-028 qualification techniques

## Summary

The in-repository dependency graph is acyclic: FR-022/profile and NFR-005 bounds enable
FR-023/corpus, FR-024/oracles, FR-025/techniques, and Task-029 completion in that order.
Three external enablement dependencies are explicit and have no local fallback.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Resolved: evidence-to-budget assignment was a hidden Task-028 prerequisite. FR-022/TC-039 now own it and Task-025 records the remaining implementation subtask before Task-028 consumes a profile. | FR-022, Task-025, Task-028 |
| FND-002 | medium | Open external dependency: Engineering Assurance #34 must publish the accepted Rust producer-execution interface before FR-024 AC-3 or FR-025 result adaptation can complete. | FR-024, FR-025, Task-027..Task-028 |
| FND-003 | medium | Open external dependency: spec-artifacts-process #86 must publish `differential-testing` before a differential configuration can freeze; the planner cannot mint it privately. | FR-025, Task-028 |
| FND-004 | medium | Open external dependency: FS01 and its qualification follow-up must publish the native source-profile identity and admission evidence before any formal-clause configuration contributes. | FR-025, Task-028 |
| FND-005 | high | Resolved: the local consumer contract now has a closed place to bind future native admission and export loss-report artifacts; it does not copy producer semantics or wait for prose interpretation. | FR-022, TC-039 |

## Dependency Order

`FR-022 + NFR-005 → FR-023 → FR-024 → FR-025/Task-028 → Task-029`.
The graph contains no cycle. Portable schema and deterministic-decision work can be
reviewed independently, but each dependent execution result stays incomplete until its
named producer edge is accepted.
