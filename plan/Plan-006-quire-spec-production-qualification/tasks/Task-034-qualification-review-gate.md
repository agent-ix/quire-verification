---
id: Task-034
title: "Production qualification review gate"
type: Task
status: not_started
track: Gate
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-033", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/StR-002", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-022", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-023", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-024", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-025", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-026", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-027", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-007", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-008", type: references }
  - { target: "ix://agent-ix/quire-verification/IT-004", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-039", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-040", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-041", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-042", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-043", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-044", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-045", type: verifies }
---
# Task-034: Production Qualification Review Gate

## Scope

Execute IT-004, reconcile the complete matrix and trace population, apply the exact
Rust review and semantic gap analysis, resolve every in-scope finding, and present the
sealed report for its required external disposition.

## Subtasks

- [ ] Execute IT-004-SC-01 through IT-004-SC-07 against the exact frozen profile.
- [ ] Run local formatting, denied-warning Clippy, Rust tests, dependency policy, fixture validation, and trace reconciliation with at most two Cargo jobs.
- [ ] Apply `/home/peter/dev/agent-skills/rust-review/SKILL.md` to the exact head and remediate every in-scope finding.
- [ ] Run QUOIN `/gap-analysis` with semantic review and resolve every in-scope requirement, matrix, code, and evidence gap.
- [ ] Verify hosted workflows remain manual-dispatch only; do not trigger CI.
- [ ] Confirm the final owner disposition and claim bind the same unchanged core.

## Deliverables

- Complete IT-004 evidence, local gate transcript, exact Rust review, QUOIN gap-analysis artifact, and final bounded qualification report.

## Notes

- An accepted profile qualifies only the commands, population, environment, budgets, and semantic stages named in that profile.
- Any unresolved dependency or owner decision leaves this task incomplete.
