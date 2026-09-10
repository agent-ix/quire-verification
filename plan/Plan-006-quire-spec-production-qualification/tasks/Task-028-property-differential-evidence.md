---
id: Task-028
title: "Build property and differential qualification evidence"
type: Task
status: not_started
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-026", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-027", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-025", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-042", type: references }
---
# Task-028: Build Property and Differential Qualification Evidence

## Scope

Implement the Rust generators and runners for example, property, metamorphic, and
differential obligations over the qualified corpus and classified oracles.

## Subtasks

- [ ] Add bare `ix-trace-rs` bindings for the TC-042 property and relation cases.
- [ ] Define deterministic seeds, partitions, relations, budgets, stopping rules, and shrinking behavior before execution.
- [ ] Preserve pass, violation, failure, timeout, unsupported, and minimized-counterexample results separately.
- [ ] Verify every counterexample replays against the exact candidate and oracle revisions.

## Deliverables

- Rust generators, relation runners, replay artifacts, and the property/differential evidence consumed by Task-029's complete TC-042 gate.

## Notes

- Execute locally with at most two Cargo jobs; do not start another high-load lane concurrently.
- This task enables FR-025; Task-029 is the single completion owner for FR-025 and TC-042.
- Unblocks Task-029.
