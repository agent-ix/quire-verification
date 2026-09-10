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
- [ ] Recount the Test Execution Summary by category and overall whenever this task
  changes a test row or status, and record the exact totals before review.

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
| E02 portable contracts | `agent-ix/quire-verification@4a5e1961d353db45aa227f8c6ef964d9f67eae29` | accepted portable checkpoint | Bind this merged and independently re-reviewed contract revision, or an explicitly reviewed successor, in the selected profile. |
| FS02/FS05 shared references and composed profile | `agent-ix/quire-specification@8ab058beff03cc76c0f39cc06c87ef98f76e8d78`; draft PR 15 at `a6c73093d6a6399f67a7047d96859c5f1736fc62` | redesign in review | A has published corrected edition/state/package successors. Proposed protocol candidate `quire.protocol.finite-global/v1` revision `1-draft.4` pins the integrated FR-061/FR-062 bytes but still selects the earlier edition closure; A records its corrected protocol successor as pending. F-owned FR-116/TC-146 still collapse the independent decision-scope, surrounding-execution, and complete-global-conformance progress/closure axes, and the composed full review has not run. Wait for the shared baseline to be reviewed and accepted, then bind the exact role and profile versions used by the executable projection. Proposed candidates remain inventory evidence, not qualification admission. |
| parser/compiler and direct CLI caller | `agent-ix/quire-spec-language@3bc1b3890f72f91873022ac807d74d047349baa8` | missing selected-v1 executable projection | After the composed profile is accepted, retain its selected release binary, dependency closure, command invocation, and environment reference. The historical state-profile candidate does not establish the composed product boundary. |
| Engineering Assurance | `agent-ix/engineering-assurance@c47e176e5ef5d133c448171e25640b76d653a077`; issue 34 open with no producer PR | provisional | Consume Agent C's accepted Rust producer-execution/result revision after issue 34's specification, implementation, cross-repository contract test, and reviews land. |
| Quoin evidence ownership | `agent-ix/quoin@7072d65db7e236e819d99be42e094ba548e9496a` | accepted containment candidate | Bind the exact versioned evidence interface used by the selected profile without expanding Quoin. |

The E02 merge closes one former prerequisite but does not freeze the profile. The
composed native source profile, its selected executable projection and caller, and the
Engineering Assurance producer interface remain unaccepted. The inventory retains those
owned prerequisites and emits no frozen qualification profile. Missing executable,
dependency-closure, caller, and environment references prevent candidate construction;
after those intrinsic references exist, every remaining unaccepted interface is returned
as a typed planner blocker.
