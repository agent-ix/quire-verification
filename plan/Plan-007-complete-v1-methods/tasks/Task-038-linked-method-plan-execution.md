---
id: Task-038
title: "Implement linked method plan execution"
type: Task
status: not_started
track: D3
priority: P0
relationships:
  - target: ix://agent-ix/quire-verification/Task-035
    type: depends_on
  - target: ix://agent-ix/quire-verification/FR-031
    type: references
  - target: ix://agent-ix/quire-verification/TC-049
    type: verifies
---
# Task-038: Implement linked method plan execution

## Scope

Complete Verification #37 and V1-RUN-017 behind interface-001.

## Subtasks

- [ ] Implement versioned external plan linking and typed dependency graph validation.
- [ ] Dispatch ready nodes through capability-constrained providers.
- [ ] Isolate failed prerequisites and preserve exactly one disposition per node.
- [ ] Apply only authored aggregation rules without changing source admission.
- [ ] Add TC-049 and IT-005-SC-03 executable traces, Rust review, and gap analysis.

## Deliverables

Deterministic plan coordinator and complete per-node result accounting.

## Development Dependencies

Consume merged stable provider interfaces from the owning B/E lanes when their
nodes are exercised. An unavailable development provider remains an explicit
node disposition and does not require production qualification or block sibling
provider work.
