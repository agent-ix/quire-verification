---
id: Task-028
title: "Build property and differential qualification evidence"
type: Task
status: blocked
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-026", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-027", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-025", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-042", type: verifies }
---
# Task-028: Build Property and Differential Qualification Evidence

## Scope

Implement the Rust deterministic case planner, shrinker, comparator, and typed result
adapter for example, property, metamorphic, and differential obligations over the
qualified corpus and classified oracles. Process execution remains in Engineering
Assurance.

## Subtasks

- [x] Run QUOIN `spec-correctness` for property obligations and retain generated Rust
  tests or validated grounding findings; do not create a parallel property generator.
- [x] Add bare `ix-trace-rs` bindings for the Task-028 TC-042 property and relation cases.
- [x] Derive one closed technique manifest from the E02 schema and the exact accepted
  Quoin catalog; block differential configuration until spec-artifacts-process#86 lands.
- [x] Define deterministic seeds, partitions, relations, budgets, stopping rules, case
  identities, and shrinking behavior before execution.
- [x] Require exact native Quire source-profile admission evidence for formal-clause
  obligations, retain every undischarged admission rule as an owned gap, and refuse
  export-only mappings or forbidden foreign runtimes as source authorities/oracles.
- [ ] Submit exact requests only through the accepted Engineering Assurance Rust interface
  from #34 and consume its typed results; add no local process runner or stdout parser.
- [ ] Preserve satisfaction, violation, execution failure, timeout, unsupported,
  unfinished-budget, and minimized-counterexample results separately.
- [ ] Verify every counterexample replays against the exact candidate and applicable
  oracle revisions.
- [ ] Recount the Test Execution Summary by category and overall whenever this task
  changes a test row or status, and record the exact totals before review.

## Deliverables

- Schema-derived technique manifest, Rust case/shrink/comparison logic, typed result
  adapter, replay artifacts, and the property/differential evidence consumed by
  Task-029's complete TC-042 gate.

## Notes

- Execute locally with at most two Cargo jobs; do not start another high-load lane concurrently.
- Quoin owns orchestration/evidence retention; Engineering Assurance owns host/process
  execution and evidence production; this task owns only portable domain semantics.
- Treat a missing common-tool capability as an upstream improvement ticket and blocker,
  never as permission for a repository-local script or private catalogue entry.
- The 2026-09-09 ruling on quire-contract-ir#53 makes native Quire the only source
  profile. OCL 2.4, SysML v2, and FRETish remain export-only mapping profiles whose loss
  reports do not qualify source-clause semantics.
- This task enables FR-025; Task-029 is the single completion owner for FR-025 and TC-042.
- Unblocks Task-029.

## Blocking Dependencies

- `agent-ix/engineering-assurance#34` has not yet published the accepted Rust typed
  producer-execution/result interface needed by the last three subtasks.
- `agent-ix/spec-artifacts-process#86` has a locally reviewed declaration checkpoint and
  an explicit three-task QUOIN plan, but it has not published the real Quoin
  catalog/advice conformance evidence or promoted `differential-testing`. The implemented
  planner therefore continues to refuse that family instead of minting a private method.
- Native formal-clause evidence remains gapped rule by rule until the selected native
  Quire source profile publishes accepted admission observations. Composed-baseline PR 15
  at `d8d984b542fe30f65b6e07f4a07b2d1f81d3962b` maintains one coherent working draft and
  defers exact profile/rule/dependency/source identity freeze to acceptance. Its source,
  observation, and executable/export consumer paths now carry all five independent
  closure axes with discriminating controls, closing B's content gap. Composed acceptance
  remains pending, and temporal bridge PR 65 does not itself establish profile admission.
  The portable contract preserves those admission gaps and does not infer coverage from
  first-party origin.
