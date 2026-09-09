---
id: Task-016
title: "Conservative evidence freshness"
type: Task
status: done
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-018", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-032", type: verifies }
---
# Task-016: Conservative Evidence Freshness

## Scope

Extend the authoritative E02 contract and Rust evidence assessor so every
obligation requires a bounded maximum age, the earliest expiry controls, and
invalid temporal relationships refuse before classification.

## Subtasks

- [x] Add the reviewed freshness shape and stable temporal-order error to the single schema-derived contract path.
- [x] Write TC-032 first with earliest-bound, exact-boundary, offset-equivalence, invalid-order, malformed, missing, boundary, and overflow cases.
- [x] Implement checked Rust instant arithmetic and preserve all findings for stale results.
- [x] Bind TC-032 and all FR-018 criteria with `ix-trace-rs`.

## Deliverables

- Schema-derived freshness contract, Rust assessment behavior, fixtures, and TC-032 evidence.

## Notes

- This task is independent of the unresolved FS05 roles.
