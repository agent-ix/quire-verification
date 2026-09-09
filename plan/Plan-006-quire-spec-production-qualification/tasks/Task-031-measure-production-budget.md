---
id: Task-031
title: "Measure the production performance budget"
type: Task
status: not_started
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-030", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/NFR-007", type: references }
---
# Task-031: Measure the Production Performance Budget

## Scope

Implement MP-002 over the exact production chain, preserving setup, hot-path, and
cleanup observations and evaluating each owner threshold independently.

## Subtasks

- [ ] Complete the benchmark portion of TC-044 for missing, exact, and exceeded phase thresholds.
- [ ] Capture five repetitions with exact environment, cache, command, corpus, concurrency, and timer identities.
- [ ] Retain every observation, invalid sample, exclusion, estimator, uncertainty source, and comparison.
- [ ] Enforce at most two Cargo jobs and serialize high-load phases on the shared workstation.

## Deliverables

- Rust measurement path, raw observation set, recomputable summaries, threshold results, and benchmark evidence.

## Notes

- The VP04 baseline supplies no production threshold; only profile-owner values decide this gate.
- This task produces the NFR-007 benchmark evidence; Task-032 is the single completion owner for the combined TC-044 measurement and reproduction gate.
- Unblocks Task-032.
