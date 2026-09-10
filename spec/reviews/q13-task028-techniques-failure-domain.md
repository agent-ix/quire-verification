---
id: SR-189
title: "Failure-domain review of Task-028 qualification techniques"
type: SpecReview
analysis: failure-domain
scope: "quire-verification@6adbbc2; FR-022, FR-025, TC-039, TC-042"
review_set: all
---
# Failure-domain review of Task-028 qualification techniques

## Summary

The review covered extension failure, identity collisions, deterministic evaluation,
finite traversal, replay, and producer-state preservation. Local manifest semantics are
closed; producer execution and formal-clause admission remain named external gaps.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Resolved: unknown catalog methods, cross-family fields, duplicate configuration/gap/mutant identities, wrong reference roles, and conflicting obligation coverage now refuse rather than selecting a fallback. | FR-025-AC-1, TC-042 steps 1..2 |
| FND-002 | high | Resolved: seeds and case/shrink/execution counts were positive but unbounded. Canonical u64 seeds, 100,000-operation ceilings, NFR-005 arrays/documents, and one-at-a-time deterministic identities bound the work. | FR-025, NFR-005, TC-042 step 2 |
| FND-003 | medium | Open external dependency: exact unavailable/refused/failed/timeout/malformed/containment states cannot be adapted or discriminated until Engineering Assurance #34 publishes its versioned Rust result contract. | FR-025-AC-3, TC-042 step 5 |
| FND-004 | medium | Open producer evidence: first-party origin is not profile admission. Every undischarged numeric, presence, containment, collection, call/result, authority, instant, reference-design, or temporal rule remains an owner-attributed gap. | FR-025, TC-042 steps 6..7 |
| FND-005 | high | Resolved: claim scope no longer depends on parsing owner prose. The closed authority tag determines whether non-formal exclusion, ten native admission observations, or an export mapping/loss report is required. | FR-022-AC-5, TC-039 |
