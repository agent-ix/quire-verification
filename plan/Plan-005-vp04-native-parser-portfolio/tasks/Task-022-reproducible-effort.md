---
id: Task-022
title: "Reproducible effort observations"
type: Task
status: done
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-021", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-020", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-036", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-037", type: verifies }
---
# Task-022: Reproducible Effort Observations

## Scope

Consume MP-001 to retain the prescribed 16 observation slots, exact identities,
invalid reasons, integer domains, deterministic per-phase summaries, measurement
context, uncertainty, and bounded interpretation.

## Subtasks

- [x] Write TC-036 first for all series-shape, identity, range, validity, order, and lower-median boundaries.
- [x] Derive the closed measurement shape from the authoritative E02 schema document.
- [x] Preserve raw observations in order and recompute summaries from valid samples only.
- [x] Add the TC-037 interpretation and claim-limit assertions owned by FR-020.
- [x] Bind every FR-020 criterion through compiler-checked `ix-trace-rs` markers.

## Deliverables

- Rust measurement types, validation, summary behavior, and TC-036/TC-037 evidence.

## Notes

- Task-021 fixes the measured phases and exact identities consumed here.
- This task consumes Agent C's reusable MeasurementPlan artifact and does not create a competing assurance framework.
- Unblocks Task-023.
