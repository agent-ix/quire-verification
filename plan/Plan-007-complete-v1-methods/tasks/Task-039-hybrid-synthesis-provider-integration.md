---
id: Task-039
title: "Integrate hybrid and synthesis method providers"
type: Task
status: done
track: D4
priority: P0
relationships:
  - target: ix://agent-ix/quire-verification/Task-036
    type: depends_on
  - target: ix://agent-ix/quire-verification/Task-037
    type: depends_on
  - target: ix://agent-ix/quire-verification/Task-038
    type: depends_on
  - target: ix://agent-ix/quire-analyze/issues/33
    type: references
---
# Task-039: Integrate hybrid and synthesis method providers

## Scope

Complete Analyze #33 and V1-RUN-018-019 in `quire-analyze`, then pin and consume
its interface-001-compatible result contract in development integration.

## Subtasks

- [x] Run the Analyze-owned `/specify` and self `/spec-review` before provider code.
- [x] Implement sound mode/flow/guard/reset enclosures with declared numeric error.
- [x] Implement canonical bounded candidate enumeration and independent validation.
- [x] Preserve convergence failure, incomplete search, no-candidate, candidate, proof, and counterexample states.
- [x] Add Analyze-owned tests plus IT-005-SC-04 integration, Rust review, and gap analysis.

## Deliverables

Pinned hybrid and synthesis providers returning the common typed method-result contract.
