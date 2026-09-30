---
id: Task-017
title: "Oracle contribution contract"
type: Task
status: done
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-012", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-017", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-033", type: verifies }
---
# Task-017: Oracle Contribution Contract

## Scope

Extend the single authoritative TechniqueResult schema with closed oracle
contributions using the exact independently accepted FS05 roles.

## Subtasks

- [x] Consume the reviewed oracle and observation artifact kinds through Task-012's opaque strict-reader outputs.
- [x] Define closed unique contribution identity, required relation, and exact structured derived-source pairs in the authoritative schema.
- [x] Reject missing, duplicate, wrong-role, conflicting, or unsupported contribution data.

## Deliverables

- Schema-derived Rust oracle contribution types and TC-033 contract cases.

## Notes

- Plan-003 Task-012 and the FS05 packet are complete. No generic-kind fallback
  is allowed.
