---
id: SR-209
title: "Failure-domain review of Task-029 fuzz and mutation evidence"
type: SpecReview
analysis: failure-domain
scope: "16922e1: FR-025, NFR-004, TC-042, Task-029"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-025", type: reviews }
  - { target: "ix://agent-ix/quire-verification/Task-029", type: reviews }
---
# Failure-Domain Review of Task-029 Fuzz and Mutation Evidence

## Summary

The amended behavior distinguishes absent harnesses, unsuitable and ungrounded targets,
unexecuted source, unfinished campaigns, crash/hang counterexamples, failed replay, and
five mutation outcomes. Exact identities and finite limits bound every planned join.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: a generated or compiled target can no longer be mistaken for an executed campaign; the obligation stays unfinished until a typed producer result binds all campaign identities and reaches its stopping rule. | FR-025 behavior, FR-025-AC-6 |
| FND-002 | high | Resolved: crash and hang minimization cannot discard the original counterexample or claim a violation from a non-reproducing smaller input. | FR-025 behavior, TC-042 step 9 |
| FND-003 | high | Resolved: a mutant is not killed by any failing test. It requires the assigned test and declared failure reason; unrelated failure, crash, timeout, survival, and unfinished budget remain separate. | FR-025-AC-5, TC-042 step 10 |
| FND-004 | medium | Open producer failure domain: cancellation, descendant cleanup, malformed results, and producer containment are specified at the accepted Engineering Assurance boundary and cannot be demonstrated by a repository-local substitute. | FR-025-CON-2, engineering-assurance#34 |

