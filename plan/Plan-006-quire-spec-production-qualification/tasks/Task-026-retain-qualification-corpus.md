---
id: Task-026
title: "Retain the qualification corpus"
type: Task
status: blocked
track: A
priority: P0
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-025", type: depends_on }
  - { target: "ix://agent-ix/quire-verification/FR-023", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-040", type: verifies }
---
# Task-026: Retain the Qualification Corpus

## Scope

Implement the Rust corpus manifest and validator for the frozen population, preserving
exact bytes, classes, provenance, rights, selection rationale, exclusions, and
reproducible generator or mutation lineage.

## Subtasks

- [x] Write TC-040 properties first for profile/population drift, complete and empty
  classes, digest and replay drift, duplicate identities, missing retention rights,
  unknown classes, and missing, self-referential, or cyclic lineage.
- [x] Define the closed corpus manifest in the authoritative schema and derive Rust types.
- [ ] Inventory representative, boundary, malformed, generated, and mutated inputs without importing artifacts whose rights do not permit retention.
- [x] Require strict-reader proofs for retained bytes and exact replay evidence for every
  generated or mutated input.
- [x] Preserve an explicit gap for every unpopulated required class.
- [x] Extend TC-039/TC-040 to prove commands and catalog techniques do not augment the
  frozen profile's nonempty authored guarantee set.

## Deliverables

- Qualified corpus manifest, retained byte inventory, lineage records, gaps, and TC-040.

## Notes

- Corpus size never substitutes for a population and selection rationale.
- Unblocks Task-027.
- The schema-derived Rust boundary and TC-040 are complete. Production corpus inventory
  remains blocked on Task-025's owner-supplied exact executable, dependency, caller,
  environment, and accepted interface inputs; synthetic tests are not presented as a
  production corpus.
