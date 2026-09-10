---
id: SR-196
title: "spec-correctness — quire-verification FR-025 — 5 criteria"
type: SpecReview
analysis: spec-correctness
scope: "spec/functional/FR-025-run-qualification-techniques.md, src/qualification_technique.rs, tests/props_fr_025.rs"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/FR-025"
    type: reviews
  - target: "ix://agent-ix/quire-verification/TC-042"
    type: references
---
# Spec Correctness for FR-025

## Summary

QUOIN classified all five FR-025 acceptance criteria. One extractable criterion is now
grounded in a runnable Rust `proptest` test; four remain explicit implementation findings
owned by an unpublished catalog method, the accepted Engineering Assurance seam, or
Task-029.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Example, property, and metamorphic case identities are implemented, but the complete universal criterion cannot be grounded until spec-artifacts-process#86 publishes `differential-testing`; the row remains unbacked. | FR-025-AC-2, spec-artifacts-process#86 |
| FND-002 | high | No Task-028 typed result adapter exists until Engineering Assurance #34 publishes the accepted Rust producer-execution result interface, so the ten-state outcome criterion cannot yet be grounded. | FR-025-AC-3, engineering-assurance#34 |
| FND-003 | high | Exact minimized-counterexample replay requires the same accepted Engineering Assurance interface; no local runner or substitute replay adapter was generated. | FR-025-AC-4, engineering-assurance#34 |
| FND-004 | high | Seeded guard-removal mutants, fuzz crashes and hangs, survivor handling, and unfinished fuzz budgets are explicitly owned by Task-029 and have no Task-028 implementation symbol. | FR-025-AC-5, Task-029 |

## Census

emitted 1 — extractable and grounded

emitted with finding 0

not settled 4 — dependency unavailable 1, unimplemented 3

already covered 0

witnesses 0

## Grounding

| Row | Domain | Oracle | Implementation boundary | Generated test |
| --- | --- | --- | --- | --- |
| FR-025-AC-1 | Evidence obligations and closed configurations at `spec/functional/FR-025-run-qualification-techniques.md:191` | Exact coverage, binding, refusal, and set normalization at `src/qualification_technique.rs:100` | `qualify_technique_manifest` | `prop_fr_025_ac_1_normalizes_sets_and_refuses_one_axis_binding_changes` |

The generated tests use the repository's Rust integration-test harness, `proptest`
generator library, and bare compiler-checked `ix-trace-rs` attributes. They contain the
required `row_id`, classification, extraction, and origin provenance lines and are not
ignored.

## Second Pass

The four-record residue was read in full. FR-025-AC-2 is universal and has implemented
example, property, and metamorphic witnesses, but its differential member is deliberately
unavailable. FR-025-AC-3 is a concrete outcome-set example; FR-025-AC-4 is a universal
replay property; and FR-025-AC-5 is a universal mutation and fuzz obligation. Each has a
clear future oracle, but no complete accepted implementation symbol exists in this task's
boundary. Generating a private catalog method, local execution adapter, replay runner, or
mutation/fuzz substitute would violate FR-025's ownership constraints, so no property
test was written for those rows.
