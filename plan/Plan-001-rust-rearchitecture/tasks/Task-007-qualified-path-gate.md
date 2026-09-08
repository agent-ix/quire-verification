---
id: Task-007
title: "Qualified-path gate"
type: Task
status: not_started
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

- [ ] Confirm `.mjs`, npm manifests, and JavaScript dependencies were removed by the rewrite.
- [ ] Run format, Clippy with denied warnings, all tests, cargo deny, fixture validation, and Quire validation.
- [ ] Compile `ix-trace-rs` markers and reconcile `quire coverage --json` against the test matrix.
- [ ] Run `/home/peter/dev/agent-skills/rust-review/SKILL.md` and resolve findings.
- [ ] Run Quoin `/gap-analysis` and require a PASS verdict.
- [ ] Update all task, plan, test-matrix, and review statuses from actual evidence.

## Deliverables

- A Rust-only executable tree, green gates, reviewed artifacts, and a mergeable PR.

## Notes

- The merge command runs only after every mandatory profile operation succeeds.
