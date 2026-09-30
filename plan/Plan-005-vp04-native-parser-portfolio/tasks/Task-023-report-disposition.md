---
id: Task-023
title: "Report-core and disposition binding"
type: Task
status: done
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-012", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-021", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-022", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-021", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-037", type: verifies }
---
# Task-023: Report-Core and Disposition Binding

## Scope

Derive the internal VP04 report from the authoritative schema, validate and seal
its automated core with `sha256-jcs`, then validate zero or one exact external
review disposition without circular identity or automated sufficiency.

## Subtasks

- [x] Write TC-037 first for schema derivation, complete core identity, JCS digest, disposition values, absence, and every stable refusal.
- [x] Generate the internal report shape beside the five public contracts from the one authoritative schema document.
- [x] Validate the complete core before computing its JCS digest with disposition bytes excluded.
- [x] Verify exact disposition bytes, role, core digest, procedure, reviewer, time, identity, and value.
- [x] Preserve Quoin ownership, `not-made` sufficiency, automated results, measurements, and claim limits.
- [x] Bind every FR-021 criterion through compiler-checked `ix-trace-rs` markers.

## Deliverables

- Schema-derived Rust report types, assembler, disposition verifier, and TC-037 evidence.

## Notes

- Tasks 012, 021, and 022 and the independently accepted JCS and
  review-disposition shared vocabulary supply the completed prerequisites.
