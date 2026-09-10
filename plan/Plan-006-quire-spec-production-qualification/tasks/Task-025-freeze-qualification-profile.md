---
id: Task-025
title: "Freeze the production qualification profile"
type: Task
status: blocked
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

- [x] Write TC-039 with one-axis omissions, open dimensions, changed identities, and unaccepted-revision cases using bare `ix-trace-rs` markers.
- [x] Extend the authoritative schema and derive the Rust profile types and validation from it.
- [x] Inventory the exact E02, FS02, FS05, parser/compiler, caller, Engineering Assurance, and Quoin handoff revisions needed by the selected profile.
- [x] Return actionable blocked prerequisites for unavailable external interfaces in a
  structurally complete candidate without copying their definitions.
- [x] Bind each evidence obligation to one exact declared resource budget and reject an
  absent or foreign assignment before Task-028 consumes the profile.
- [x] Add the closed non-formal/native-formal-clause/export-mapping authority union and
  reject incomplete native admission or mapping loss-report bindings.

## Deliverables

- Rust profile contract, schema-derived validator, TC-039, and one frozen profile or exact prerequisite set.

## Notes

- The verification owner supplies the claim dimensions and thresholds; implementation does not invent them.
- The verification owner is the action owner for recording those selected values in issue 15; no implementation task may infer them from VP04 or a schema ceiling.
- The profile candidate is an identity-bearing value, so its candidate,
  dependency-closure, caller, and environment references are mandatory. Absence of
  those intrinsic references prevents construction and remains an owner-recorded task
  prerequisite. Typed planner blockers cover the six external interface acceptance
  seams after those intrinsic inputs exist.
- Task-025 exposes the profile gate as a library API. Task-026 is its first planned
  in-crate consumer and remains blocked by this task; no qualification execution path
  is wired early.
- Stop this task as `blocked` if any selected producer revision is unavailable or unaccepted.
- Unblocks Task-026.

## External Revision Inventory

The initial profile is bounded to the direct `quire-spec` CLI `parse` and `format`
commands, syntax and formatting guarantees, and the declared retained population. Model
binding, type checking, evaluation, backend projection, and other environments remain
excluded.

| Role | Observed revision | State | Exact prerequisite |
|---|---|---|---|
| E02 portable contracts | `agent-ix/quire-verification@a57a88d9b259d7c48e95d375da0a8b01b3be7c0d` | provisional | Merge and accept the reviewed qualification-profile contract; the main revision does not contain it. |
| FS02/FS05 shared references | `agent-ix/quire-specification@8ab058beff03cc76c0f39cc06c87ef98f76e8d78` | accepted input candidate | Bind the exact role/profile versions actually used when the executable projection is created. |
| parser/compiler and direct CLI caller | `agent-ix/quire-spec-language@3bc1b3890f72f91873022ac807d74d047349baa8` | missing executable projection | Retain an accepted release binary, dependency closure, command invocation, and environment reference for this revision. |
| Engineering Assurance | `agent-ix/engineering-assurance@c47e176e5ef5d133c448171e25640b76d653a077` | provisional | Consume Agent C's accepted Rust-native handoff revision after its interface specification and review land. |
| Quoin evidence ownership | `agent-ix/quoin@7072d65db7e236e819d99be42e094ba548e9496a` | accepted containment candidate | Bind the exact versioned evidence interface used by the selected profile without expanding Quoin. |

Until the three non-accepted states are resolved, the inventory retains their owning
prerequisites and no frozen qualification profile is emitted. The missing executable,
dependency closure, caller, and environment prevent candidate construction; once those
intrinsic references exist, each remaining unaccepted external interface is returned as
a typed planner blocker.
