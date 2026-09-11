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

- [x] Write the portable TC-041 manifest cases for independent, shared, derived,
  self-produced, absent, changed, and cyclic oracle bindings.
- [x] Derive the closed oracle-manifest representation from E02 and reuse the existing
  oracle-contribution dependence graph.
- [x] Bind every portable oracle declaration to the exact frozen profile, corpus
  projection, executable, procedures, requests, limitations, and strict-reader proofs.
- [x] Submit the exact oracle and comparison requests through the accepted Rust
  Engineering Assurance producer-execution interface from issue 34.
- [x] Consume typed agreeing and disagreeing observations, retaining both exact
  observations and an unresolved result for disagreement.
- [x] Preserve unavailable, refused, failed, timed-out, malformed-response,
  containment-failure, cancelled, and completed producer states without fabricating
  observations.
- [x] Confirm the portable checkpoint adds no local runner, evidence store, fixture
  script, or arbitrary stdout parser.
- [x] Recount the Test Execution Summary by category and status after changing TC-041.
- [x] Recount the Test Execution Summary again by category and overall whenever this
  task changes another test row or status, and record the exact totals before review.

## Deliverables

- Oracle-obligation contracts, bounded adapter integration, dependence findings, and TC-041.

## Notes

- No self-consistency check may be relabeled as independent evidence.
- Merged revision `4a5e1961d353db45aa227f8c6ef964d9f67eae29` contains the
  independently re-reviewed portable manifest and dependence checkpoint. It does not
  contain execution observations or complete TC-041.
- Engineering Assurance issue `agent-ix/engineering-assurance#34` and PR 46 own the
  reusable producer-execution/result seam. TC-041/TC-126 pass locally against exact
  candidate revision `f640f4c1af95e8657bef04207758091fc938658a`; promotion and the
  post-merge pin remain cross-repository gates and do not authorize a substitute.
- Unblocks Task-028.
