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

- [x] Establish the owner-selected Rust cargo-fuzz workspace, run QUOIN `spec-fuzz`,
  retain its method-fit finding, and compile the three grounded targets without claiming
  that source creation or compilation is a completed campaign.
- [x] Freeze bounded fuzz and mutation configuration forms, exact seed and mutant
  identities, stopping rules, replay bindings, and artifact-reference requirements.
- [ ] Write the remaining TC-042 campaign-result, budget, minimization, and replay cases,
  then reconcile the Task-028 property/differential cases into the complete TC-042 gate.
- [ ] Keep an unfinished fuzz budget incomplete even when it finds no counterexample.
- [x] Seed named guard-removal mutants and prove the assigned traced test fails for the intended reason.
- [ ] Retain surviving mutants, crashes, hangs, and minimized inputs as blocking results.
- [ ] Recount the Test Execution Summary by category and overall whenever this task
  changes a test row or status, and record the exact totals before review.

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

- Task-028 remains blocked on `agent-ix/engineering-assurance#34` and the unpromoted
  `agent-ix/spec-artifacts-process#86` catalogue work, so its typed results cannot yet be
  reconciled into the complete TC-042 gate.
- `agent-ix/engineering-assurance#34` has not published the accepted Rust
  producer-execution/result interface needed for fuzz campaigns, mutation observations,
  replay, and retained blocking results.
- Native formal-clause evidence remains gapped against all ten carried admission rules.
  `agent-ix/quire-specification` PR 15 is still a draft composed baseline and
  `agent-ix/quire-contract-ir` PR 65 is a temporal bridge specification; neither is an
  accepted admission-evidence producer.
