---
id: Task-024
title: "VP04 integration and qualification gate"
type: Task
status: done
track: Gate
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-023", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-019", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-020", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-021", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-006", type: references }
  - { target: "ix://agent-ix/quire-verification/IT-003", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-022", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-023", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-034", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-035", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-036", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-037", type: verifies }
  - { target: "ix://agent-ix/quire-verification/TC-038", type: verifies }
---
# Task-024: VP04 Integration and Qualification Gate

## Scope

Select the exact qualified parser and cleared fixtures, execute IT-003 end to end,
bind the external verification-owner disposition, run every repository gate and
the exact Rust review, and close VP04 with QUOIN gap analysis.

## Subtasks

- [x] Record the exact parser revision, qualification evidence, dependency closure, CLI commands, Linux profile, and fixture rights.
- [x] Execute IT-003-SC-01 through IT-003-SC-09 and retain the complete report; SC-01 through SC-08 are retained in the sealed automated core and SC-09 is retained in the bound external disposition.
- [x] Obtain one external owner disposition bound to the sealed automated core.
- [x] Run formatting, strict Clippy, all Rust tests, dependency policy, fixture validation, integrations, and trace reconciliation.
- [x] Apply `/home/peter/dev/agent-skills/rust-review/SKILL.md` and resolve every in-scope finding.
- [x] Run QUOIN `/gap-analysis` and resolve every in-scope gap before changing this task and Plan-005 to done.

## Deliverables

- Retained VP04 report, exact external disposition, green qualification evidence, Rust review, and QUOIN gap-analysis record.

## Notes

- Task-023 and all transitive implementation gates are complete. The retained
  owner disposition accepts the unchanged sealed core, and final gap analysis
  records no remaining Plan-005 gap.
- The bounded LC01 syntax candidate and fixture entrance criterion is retained in
  `evidence/vp04-parser-candidate.md`; it does not clear the shared wire or
  predecessor gates.
- A successful result establishes only the declared parser-fixture portfolio behavior.
