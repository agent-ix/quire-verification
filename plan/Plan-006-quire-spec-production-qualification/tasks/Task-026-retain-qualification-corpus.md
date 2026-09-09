---
id: Task-026
title: "Retain the qualification corpus"
type: Task
status: not_started
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

- [ ] Write TC-040 properties for complete classes, empty classes, digest drift, duplicate identities, missing rights, and broken lineage.
- [ ] Define the closed corpus manifest in the authoritative schema and derive Rust types.
- [ ] Inventory representative, boundary, malformed, generated, and mutated inputs without importing artifacts whose rights do not permit retention.
- [ ] Preserve an explicit gap for every unpopulated required class.

## Deliverables

- Qualified corpus manifest, retained byte inventory, lineage records, gaps, and TC-040.

## Notes

- Corpus size never substitutes for a population and selection rationale.
- Unblocks Task-027.
