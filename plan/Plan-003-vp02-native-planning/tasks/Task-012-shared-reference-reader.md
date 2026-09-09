---
id: Task-012
title: "Strict shared-reference reader"
type: Task
status: active
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-016", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-029", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-031", type: verifies }
---
# Task-012: Strict Shared-Reference Reader

## Scope

Implement the independent bounded Rust reader for the accepted FS02/FS05 packet.

## Subtasks

- [x] Resolve the shared JCS-domain and six exact role findings with Agent A at merged revision `8ab058be`.
- [ ] Retain the exact data-only schemas/fixtures with a provenance and digest manifest and derive the Rust wire types from those schemas.
- [ ] Reject duplicate raw JSON members and every selected version/identity failure.
- [ ] Verify local byte digests, feature-set semantics, canonical identities, and role kinds.
- [ ] Validate retained version-1 source-map/migration behavior without cross-version fallback.
- [ ] Execute the pinned data-only packet independently.

## Deliverables

- Schema-derived Rust types, strict reader, stable errors, and TC-029/TC-031 evidence.

## Notes

- External representation blocker resolved by
  `agent-ix/quire-specification@8ab058beff03cc76c0f39cc06c87ef98f76e8d78`.
- Reviewed adoption revision: `quire-verification@9fb29ff`; implementation starts
  only after this task becomes active in the plan.
