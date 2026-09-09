---
id: Task-033
title: "Seal and disposition the qualification report"
type: Task
status: not_started
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-032", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-027", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-045", type: verifies }
---
# Task-033: Seal and Disposition the Qualification Report

## Scope

Extend the Rust report assembler to recompute every source binding, seal a core that
excludes disposition bytes, and bind exactly one external `accept`, `revise`, or
`reject` decision without expanding the profile.

## Subtasks

- [ ] Write TC-045 for changed source bytes, internally resealed tampering, unresolved gaps, and every invalid disposition form.
- [ ] Recompute profile, corpus, oracle, technique, production-chain, measurement, review, and reproduction identities before sealing.
- [ ] Keep the report incomplete while any evidence, interface, threshold, finding, or gap is unresolved.
- [ ] Obtain one external verification-owner disposition over the unchanged core and exact bounded claim.

## Deliverables

- Schema-derived report extension, sealed evidence core, disposition record or explicit incomplete report, and TC-045.

## Notes

- `revise` and `reject` remain final human decisions; neither is rewritten as an automated failure.
- Unblocks Task-034.
