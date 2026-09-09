---
id: Task-027
title: "Bind qualification oracle obligations"
type: Task
status: not_started
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-025", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-026", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-024", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-041", type: verifies }
---
# Task-027: Bind Qualification Oracle Obligations

## Scope

Extend the Rust evidence model so every claimed guarantee has an exact oracle or an
explicit no-oracle gap, with authority, revision, projection, expected relation,
dependence, limitation, and disagreement procedure.

## Subtasks

- [ ] Write TC-041 for independent, shared, derived, self-produced, absent, changed, and disagreeing observations.
- [ ] Reuse the existing oracle-contribution graph and bounded native-process executor.
- [ ] Bind every oracle to corpus inputs and exact tool bytes before execution.
- [ ] Preserve both observations and an unresolved result when an oracle disagrees.

## Deliverables

- Oracle-obligation contracts, bounded adapter integration, dependence findings, and TC-041.

## Notes

- No self-consistency check may be relabeled as independent evidence.
- Unblocks Task-028.
