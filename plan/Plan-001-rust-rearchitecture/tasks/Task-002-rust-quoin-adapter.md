---
id: Task-002
title: "Rust Quoin adapter"
type: Task
status: not_started
track: B
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-001", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-004", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-007", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-008", type: verifies }
---
# Task-002: Rust Quoin adapter

## Scope

Implement strict Quoin catalog and advice deserialization plus portable
adaptation with first-wins diagnostics and explicit qualification.

## Subtasks

- [ ] Model the external JSON boundary with unknown-field refusal where the producer contract is closed.
- [ ] Preserve every required method field and stable technique identifier.
- [ ] Retain duplicate and unreadable diagnostics without qualifying the snapshot.
- [ ] Map positive and inconclusive advice without inventing non-applicability.

## Deliverables

- Catalog/advice adapter module and traced tests.

## Notes

- Quoin remains an external TypeScript producer reached through JSON.

