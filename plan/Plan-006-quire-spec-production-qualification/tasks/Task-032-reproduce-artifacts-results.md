---
id: Task-032
title: "Reproduce qualification artifacts and results"
type: Task
status: not_started
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-029", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-030", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/Task-031", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/NFR-008", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-044", type: verifies }
---
# Task-032: Reproduce Qualification Artifacts and Results

## Scope

Perform an independent local reproduction from exact source, lockfile, binary,
profile, corpus, oracle, environment, seed, command, observation, and result identities.

## Subtasks

- [ ] Write the TC-044 reproduction, exact-byte, and explained-difference cases first, then reconcile Task-031's benchmark cases into the complete TC-044 gate.
- [ ] Resolve every required identity to retained or authorized retrievable bytes and verify its digest.
- [ ] Rebuild the exact candidate and replay deterministic cases and minimized counterexamples.
- [ ] Repeat MP-002 without sharing mutable build or evidence state with the original run.
- [ ] Preserve stochastic or environmental differences with explanations and prevent exploratory environments from expanding the profile.

## Deliverables

- Independent reproduction procedure, byte-resolution record, replay results, difference report, and the complete TC-044 measurement and reproduction gate.

## Notes

- Schedule this task after other active agent builds finish if shared-machine noise would invalidate the profile.
- This task is the single completion owner for TC-044 after Task-031.
- Unblocks Task-033.
