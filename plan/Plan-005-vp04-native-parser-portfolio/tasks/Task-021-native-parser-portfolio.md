---
id: Task-021
title: "Native parser portfolio execution"
type: Task
status: blocked
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-015", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-019", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-020", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-019", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-034", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-035", type: verifies }
---
# Task-021: Native Parser Portfolio Execution

## Scope

Compose accepted VP02 planning and VP03 assessment with Task-020 to run direct
parse and parse-format-reparse techniques, retain the declared syntax violation,
and keep blocked, stale, correlated, manual, and infrastructure states distinct.

## Subtasks

- [ ] Extend the authoritative schema-derived contract for the reviewed portfolio records without adding a public contract.
- [ ] Write TC-034 and TC-035 before implementation, using controlled failures and the exact native parser protocol.
- [ ] Execute baseline and adverse automated tasks and retain exact native artifacts and observations.
- [ ] Preserve missing-prerequisite, freshness, correlation, and manual-task boundaries without fabricated results or aggregate scoring.
- [ ] Bind every FR-019 criterion through compiler-checked `ix-trace-rs` markers.

## Deliverables

- Rust portfolio executor and TC-034/TC-035 behavioral evidence.

## Notes

- Plan-003, Plan-004, the shared wire, and the exact qualified parser/fixture
  entrance criteria are complete; only Task-020's expected-status extension remains.
- Unblocks Task-022 and Task-023.
