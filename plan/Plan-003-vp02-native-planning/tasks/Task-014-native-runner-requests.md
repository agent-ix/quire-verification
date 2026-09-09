---
id: Task-014
title: "Exact native runner requests"
type: Task
status: done
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-011", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-013", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-005", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-013", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-014", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-015", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-006", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-027", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-030", type: verifies }
---
# Task-014: Exact Native Runner Requests

## Scope

Emit closed runner envelopes only for applicable, ready, qualified native work
with one selected in-limit budget.

## Subtasks

- [x] Define closed positive budget and capability-limit contracts, including the ten-second wall ceiling.
- [x] Copy capability, adapter, permissions, selected allocation, native interface, and role input exactly.
- [x] Make a property budget replace the plan budget as one complete object.
- [x] Turn absent budget into a deterministic setup dependency and excessive budget into a typed value-retaining gap.
- [x] Keep manual, every other non-invocable state, and partially authorized fault work null and unauthorized.
- [x] Preserve Quoin evidence and authored-human sufficiency ownership.

## Deliverables

- Runner-request schema/planner behavior and TC-027/TC-030 evidence.

## Notes

- Tasks 011 and 013 are complete. The implementation is governed by accepted
  QUOIN review run `4a60dcc5-e3fc-4d46-b3f9-7806027dfd19` and SR-120 through
  SR-127 at `adaff03`.
- Completed in `86ebb0b`; refreshed portfolio evidence is `96344bc`. Exact Rust
  review SR-128 found no unresolved defect, all repository gates pass, affected
  FR-005/FR-013/FR-014/FR-015 rows are backed, and Quire binds all 47 Rust test
  candidates.
