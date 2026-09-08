---
id: Task-011
title: "Actionable prerequisite DAG"
type: Task
status: done
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-003", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-013", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-026", type: verifies }
---
# Task-011: Actionable Prerequisite DAG

## Scope

Extend the Rust planner to coalesce missing prerequisite identities into
plan-level setup work, retain each required task, and connect each affected task
through a sorted `requiredBy` relation.

## Subtasks

- [x] Test every missing prerequisite kind, shared duplicate need, near match, stale policy, absent resource allocation, and delimiter collision.
- [x] Generate exactly one plan-level setup task per exact missing identity without fake property or technique identity.
- [x] Preserve required blocked work and injective dependency identifiers.
- [x] Bind TC-026 and every FR-013 criterion with `ix-trace-rs`.

## Deliverables

- Rust planner behavior and TC-026 evidence.

## Notes

- Runtime exhaustion remains VP03 result behavior.
