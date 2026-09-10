---
id: Task-029
title: "Add fuzz and mutation discrimination"
type: Task
status: blocked
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-028", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-025", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-042", type: verifies }
---
# Task-029: Add Fuzz and Mutation Discrimination

## Scope

Add bounded Rust fuzzing and seeded mutation analysis for qualification-critical
parsing, identity, oracle, containment, result, and claim guards.

## Subtasks

- [ ] Write the remaining TC-042 fuzz, mutation, budget, and replay cases first, then reconcile the Task-028 property/differential cases into the complete TC-042 gate.
- [x] Define corpus seeds, fuzz targets, budgets, stopping rules, artifact retention, and minimization.
- [ ] Keep an unfinished fuzz budget incomplete even when it finds no counterexample.
- [x] Seed named guard-removal mutants and prove the assigned traced test fails for the intended reason.
- [ ] Retain surviving mutants, crashes, hangs, and minimized inputs as blocking results.

## Deliverables

- Rust fuzz targets, mutation plan and ratchets, retained counterexamples, and the complete TC-042 result gate for every FR-025 technique family.

## Notes

- Mutation runs are local, serial, and scoped; they do not trigger hosted CI.
- This task is the single completion owner for FR-025 and TC-042 after Task-028.
- Unblocks Task-030.
- The 2026-09-09 owner ruling leaves every native Quire admission rule gapped until
  affirmative evidence exists. No state-only, temporal, or export mapping work is
  treated as coverage of the composed native profile.

## Blocking Dependencies

- Task-028 remains blocked on `agent-ix/engineering-assurance#34` and
  `agent-ix/spec-artifacts-process#86`, so its typed results cannot yet be reconciled
  into the complete TC-042 gate.
- `agent-ix/engineering-assurance#34` has not published the accepted Rust
  producer-execution/result interface needed for fuzz campaigns, mutation observations,
  replay, and retained blocking results.
- Native formal-clause evidence remains gapped against all ten carried admission rules
  while `agent-ix/quire-specification#10` and `agent-ix/quire-contract-ir#64` redesign the
  composed profile and temporal bridge.
