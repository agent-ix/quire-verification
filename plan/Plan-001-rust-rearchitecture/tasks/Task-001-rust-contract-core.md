---
id: Task-001
title: "Rust contract core"
type: Task
status: done
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-001", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-002", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-008", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-002", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-005", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-001", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-002", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-003", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-004", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-016", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-017", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-023", type: verifies }
---
# Task-001: Rust contract core

## Scope

Create the Rust workspace, stable error envelope, schema-generated wire types,
runtime draft 2020-12 validation, JCS helpers, iterative plan graph validation,
and bounded JSON ingestion.

## Subtasks

- [x] Configure Rust 2024, forbid unsafe code, workspace lints, and dependency policy.
- [x] Generate wire types from the checked-in schema without a second authored definition.
- [x] Compile the schema once and derive contract vocabularies from it.
- [x] Implement JCS equality and `sha256-jcs` digests.
- [x] Enforce byte, depth, collection, duplicate-ID, unknown-edge, and cycle limits.
- [x] Implement traced tests before downstream modules.

## Deliverables

- `Cargo.toml`, `Cargo.lock`, `deny.toml`, build configuration, contract and canonicalization modules, and traced tests.

## Notes

- No public API returns `String` or `Box<dyn Error>` errors.
- No caller-controlled recursive graph walk or unsafe code is permitted.
