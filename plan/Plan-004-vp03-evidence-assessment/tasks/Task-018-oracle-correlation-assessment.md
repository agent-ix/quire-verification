---
id: Task-018
title: "Oracle correlation assessment"
type: Task
status: done
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-017", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-017", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-033", type: verifies }
---
# Task-018: Oracle Correlation Assessment

## Scope

Extend the Rust assessment view to retain independent, shared, derived, and
unknown oracle relationships deterministically without numeric or assurance
aggregation.

## Subtasks

- [x] Detect shared oracle, observation, and declared dependence identities across uniquely selected results.
- [x] Retain derived source pairs and unknown relationships as explicit findings.
- [x] Prove obligation, result, contribution, and validated-reference input order cannot change the assessment.
- [x] Preserve Quoin evidence ownership and `not-made` sufficiency.

## Deliverables

- Rust dependence analysis and complete TC-033 behavioral evidence.

## Notes

- This task consumes Task-017's schema-derived types, traverses graphs iteratively,
  and does not persist evidence.
