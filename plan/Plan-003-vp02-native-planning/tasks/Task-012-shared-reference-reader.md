---
id: Task-012
title: "Strict shared-reference reader"
type: Task
status: blocked
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

- [ ] Resolve the shared JCS-domain and trace/fault/manual role findings with Agent A.
- [ ] Reject duplicate raw JSON members and every selected version/identity failure.
- [ ] Verify local byte digests, feature-set semantics, canonical identities, and role kinds.
- [ ] Execute the pinned data-only packet independently.

## Deliverables

- Schema-derived Rust types, strict reader, stable errors, and TC-029/TC-031 evidence.

## Notes

- Blocker: specification PR #8 at `609790e` lacks required B representations.
