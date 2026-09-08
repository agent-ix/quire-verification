---
id: Task-004
title: "Rust evidence assessor"
type: Task
status: not_started
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-001", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-006", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-007", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-001", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-003", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-012", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-013", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-014", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-015", type: verifies }
---
# Task-004: Rust evidence assessor

## Scope

Implement exact JCS-based result matching, freshness, exhaustive result-state
classification, deterministic gaps, and conservative ownership output.

## Subtasks

- [ ] Compare all 14 identity, version, provenance, and execution dimensions.
- [ ] Reject duplicate result IDs and incomplete obligations.
- [ ] Refuse multiple exact candidates deterministically.
- [ ] Preserve simultaneous adverse findings, native references, and provenance.

## Deliverables

- Evidence module and traced unit/property tests.

## Notes

- Summary status never replaces the complete gap list.

