---
id: Task-007
title: "Qualified-path gate"
type: Task
status: in_progress
track: Gate
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-001", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-002", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-003", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-004", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-005", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-006", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/StR-001", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-004", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-021", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-022", type: verifies }
---
# Task-007: Qualified-path gate

## Scope

Remove rejected executable languages, run all Rust and Quoin gates, perform the
exact requested Rust review and gap analysis, and reconcile plan and matrix status.

## Subtasks

- [x] Confirm `.mjs`, npm manifests, and JavaScript dependencies were removed by the rewrite.
- [x] Run format, Clippy with denied warnings, all tests, cargo deny, fixture validation, and Quire validation.
- [ ] Compile `ix-trace-rs` markers and reconcile every marker argument through `quire coverage --scope . --json` against the test matrix.
- [x] Run `/home/peter/dev/agent-skills/rust-review/SKILL.md` and resolve findings.
- [x] Prepare completed task and matrix state for the independent Quoin `/gap-analysis` gate.
- [x] Update all task and test-matrix statuses from actual evidence.

## Deliverables

- A Rust-only executable tree, green gates, reviewed artifacts, and a candidate
  ready for the independent gap-analysis and merge gates.

## Notes

- The merge command runs only after every mandatory profile operation succeeds.
- Reopened after the full-plan coverage audit showed that one multiline compiled
  marker left NFR-006-AC-2 and NFR-006-AC-3 unbound in Quire. Reviewed amendment
  SR-081 through SR-088 governs the remediation.
