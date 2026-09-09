---
id: Task-030
title: "Execute the real production chain"
type: Task
status: not_started
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-025", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-029", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-026", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-043", type: verifies }
---
# Task-030: Execute the Real Production Chain

## Scope

Execute the selected real caller through the exact `quire-spec` candidate, portable
result contract, and Engineering Assurance handoff, preserving all observations and
typed seam failures.

## Subtasks

- [ ] Write TC-043 for the exact chain and every one-seam test-only, stale, changed, missing, or copied substitution.
- [ ] Verify interface versions and artifact bytes before invoking each boundary.
- [ ] Exercise the production caller rather than a test adapter and retain the semantic stages actually reached.
- [ ] Attribute every blocked or failed seam to its existing owner without editing the producer repository.

## Deliverables

- Real-chain Rust integration, compatibility cases, exact run artifacts, typed seam results, and TC-043.

## Notes

- Re-run compatibility cases against the accepted producer revisions immediately before evidence capture.
- Unblocks Task-031.
