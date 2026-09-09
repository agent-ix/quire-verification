---
id: Task-025
title: "Freeze the production qualification profile"
type: Task
status: not_started
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-022", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-039", type: verifies }
---
# Task-025: Freeze the Production Qualification Profile

## Scope

Implement the Rust profile contract and planner that bind one exact candidate,
production caller, command surface, population, guarantee set, environment, resource
budget, evidence set, exclusions, and accepted producer revisions under one JCS identity.

## Subtasks

- [ ] Write TC-039 with one-axis omissions, open dimensions, changed identities, and unaccepted-revision cases using bare `ix-trace-rs` markers.
- [ ] Extend the authoritative schema and derive the Rust profile types and validation from it.
- [ ] Inventory the exact E02, FS02, FS05, parser/compiler, caller, Engineering Assurance, and Quoin handoff revisions needed by the selected profile.
- [ ] Return actionable blocked prerequisites for unavailable revisions without copying their definitions.

## Deliverables

- Rust profile contract, schema-derived validator, TC-039, and one frozen profile or exact prerequisite set.

## Notes

- The verification owner supplies the claim dimensions and thresholds; implementation does not invent them.
- Stop this task as `blocked` if any selected producer revision is unavailable or unaccepted.
- Unblocks Task-026.
