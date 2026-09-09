---
id: SR-130
title: "Base review of VP03 oracle contributions"
type: SpecReview
analysis: base
scope: "quire-verification@822116c; FR-017, TC-033, TM-001, AD-001"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-017", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TC-033", type: references }
  - { target: "ix://agent-ix/quire-verification/TM-001", type: references }
---

## Summary

The owner-selected `all` review checked the Task-017/Task-018 specification for
identity, schema closure, exact selection, graph behavior, claim limits, and all
six Test Matrix rules. The material ambiguities found at `822116c` were resolved
in the reviewed working tree before implementation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved during review: “exact-matched” allowed ambiguous candidates into the graph although no result was selected for their obligation. Graph membership now begins only after unique selection, and reused selected results contribute each structured member once. | FR-017-AC-4, FR-017-AC-5, TC-033 |
| FND-002 | medium | Resolved during review: contribution field names and dependence-finding member shapes were prose-only and could admit incompatible outputs. The requirement now names every contribution field and closes both finding variants and their structured members. | FR-017-AC-1, FR-017-AC-4 |
| FND-003 | medium | The matrix now exercises all ordering inputs, duplicate validated references, ambiguous candidates, reused selected results, transitive mixed-basis components, and every schema or lineage refusal. | FR-017-AC-2..6, TC-033, TM-001 |
| FND-004 | low | Pre-implementation coverage is honest: Quire reports 132/164 obligations backed, while all seven FR-017 criteria remain planned under TC-033. | FR-017, TC-033, TM-001 |

## Checklist Result

FR-017 has a named user story and prerequisite chain, concrete inputs and
outputs, exact identity keys, closed variants, stable error codes, finite graph
rules, and seven criteria mapped to TC-033. The matrix covers valid variants,
conditional-field errors, cardinality boundaries, lineage cycles, selection
states, and order independence. No unresolved base finding blocks implementation.
