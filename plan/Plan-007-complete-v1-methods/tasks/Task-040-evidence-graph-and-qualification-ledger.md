---
id: Task-040
title: "Implement evidence graph and qualification ledger"
type: Task
status: done
track: D5
priority: P0
relationships:
  - target: ix://agent-ix/quire-verification/Task-036
    type: depends_on
  - target: ix://agent-ix/quire-verification/Task-037
    type: depends_on
  - target: ix://agent-ix/quire-verification/Task-038
    type: depends_on
  - target: ix://agent-ix/quire-verification/Task-039
    type: depends_on
  - target: ix://agent-ix/quire-verification/FR-032
    type: references
  - target: ix://agent-ix/quire-verification/FR-033
    type: references
  - target: ix://agent-ix/quire-verification/FR-034
    type: references
  - target: ix://agent-ix/quire-verification/TC-050
    type: verifies
  - target: ix://agent-ix/quire-verification/TC-051
    type: verifies
  - target: ix://agent-ix/quire-verification/TC-052
    type: verifies
---
# Task-040: Implement evidence graph and qualification ledger

## Scope

Complete Verification #38 and V1-EVID-001-006 after all method provider contracts
are merged. Task-039 is consumed through the shared contract.

## Subtasks

- [x] Implement typed evidence nodes/relations with exact source/model/binding/implementation/method/run provenance.
- [x] Implement immutable counterexample revision and replay lineage.
- [x] Consume exact ecosystem locks and independent conformance corpus records.
- [x] Account all 176 primary rows and applicable independent claim-class vectors.
- [x] Reject self/circular discharge and preserve redaction/loss claim impacts.
- [x] Keep future human decisions separate and missing production evidence non-blocking for development.
- [x] Add TC-050-TC-052 and IT-005-SC-05..07 executable traces, Rust review, and gap analysis.

## Deliverables

Typed evidence graph, counterexample lineage, and complete qualification ledger with no implicit promotion.
