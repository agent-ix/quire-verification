---
id: Task-006
title: "Rust bounded portfolio"
type: Task
status: done
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-002", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-003", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-004", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-009", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-018", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-019", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-020", type: verifies }
---
# Task-006: Rust bounded portfolio

## Scope

Compose the adapter, planner, and assessor into a deterministic local portfolio
that retains satisfied, violated, failed, and pending-human results.

## Subtasks

- [x] Key complete field parity by technique identifier.
- [x] Check reverse and rotate permutations.
- [x] Inject comparison violations and adapter errors through a callable seam.
- [x] Preserve claim limits and no-sufficiency ownership.

## Deliverables

- Bounded-portfolio module, CLI path, regenerated retained report, and traced tests.

## Notes

- A check error is data in the returned report, not a panic or lost process output.
