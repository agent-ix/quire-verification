---
id: Task-014
title: "Exact native runner requests"
type: Task
status: active
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-011", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-013", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-014", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-027", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-030", type: verifies }
---
# Task-014: Exact Native Runner Requests

## Scope

Emit exact runner envelopes only for applicable, ready, qualified work.

## Subtasks

- [ ] Preserve capability, adapter, permissions, allocation, and native interface fields.
- [ ] Keep every non-invocable and partially authorized fault request null.
- [ ] Preserve Quoin evidence and authored-human sufficiency ownership.

## Deliverables

- Runner-request schema/planner behavior and TC-027/TC-030 evidence.

## Notes

- Blocked by Tasks 011 and 013.
