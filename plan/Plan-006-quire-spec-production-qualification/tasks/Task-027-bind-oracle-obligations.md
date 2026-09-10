---
id: Task-027
title: "Bind qualification oracle obligations"
type: Task
status: in_progress
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-025", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-026", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-024", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-041", type: verifies }
  - { target: "ix://agent-ix/engineering-assurance/FR-014", type: depends_on }
---
# Task-027: Bind Qualification Oracle Obligations

## Scope

Extend the Rust evidence model so every claimed guarantee has an exact oracle or an
explicit no-oracle gap, with authority, revision, projection, expected relation,
dependence, limitation, and disagreement procedure.

## Subtasks

- [ ] Write TC-041 for independent, shared, derived, self-produced, absent, changed, and disagreeing observations.
- [ ] Reuse the existing oracle-contribution graph and the accepted Rust Engineering
  Assurance producer-execution interface; add no local runner, store, or script.
- [ ] Bind every oracle to corpus inputs and exact tool bytes before execution.
- [ ] Preserve both observations and an unresolved result when an oracle disagrees.
- [ ] Recount the Test Execution Summary by category and status after changing TC-041.

## Deliverables

- Oracle-obligation contracts, bounded adapter integration, dependence findings, and TC-041.

## Notes

- No self-consistency check may be relabeled as independent evidence.
- Engineering Assurance issue `agent-ix/engineering-assurance#11` owns the reusable
  producer-execution seam. Its absence blocks integration without authorizing a substitute.
- Unblocks Task-028.
