---
id: Task-011
title: "Actionable prerequisite DAG"
type: Task
status: in_progress
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-003", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-013", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-026", type: verifies }
---
# Task-011: Actionable Prerequisite DAG

## Scope

Extend the Rust planner to coalesce missing prerequisite identities, retain each
required task, and connect each affected task to its deterministic setup task.

## Subtasks

- [ ] Test every missing prerequisite kind, shared duplicate need, near match, stale policy, and absent resource allocation.
- [ ] Generate exactly one setup task per exact missing identity.
- [ ] Preserve required blocked work and exact dependency identifiers.
- [ ] Bind TC-026 and FR-013 criteria with `ix-trace-rs`.

## Deliverables

- Rust planner behavior and TC-026 evidence.

## Notes

- Runtime exhaustion remains VP03 result behavior.
