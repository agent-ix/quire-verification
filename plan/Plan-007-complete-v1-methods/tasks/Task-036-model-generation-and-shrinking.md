---
id: Task-036
title: "Implement model generation and shrinking"
type: Task
status: not_started
track: D1
priority: P0
relationships:
  - target: ix://agent-ix/quire-verification/Task-035
    type: depends_on
  - target: ix://agent-ix/quire-verification/FR-029
    type: references
  - target: ix://agent-ix/quire-verification/TC-047
    type: verifies
---
# Task-036: Implement model generation and shrinking

## Scope

Complete Verification #35 and V1-RUN-014 behind interface-001.

## Subtasks

- [ ] Derive rejection-free generators and boundary partitions from checked domains/relationships.
- [ ] Implement deterministic seeds and well-founded domain-valid shrinking in Rust.
- [ ] Bind native replay and preserve the same failing clause/disposition.
- [ ] Add TC-047 and IT-005-SC-01 executable traces, Rust review, and gap analysis.

## Deliverables

Versioned model-provider implementation and replayable counterexample results.

