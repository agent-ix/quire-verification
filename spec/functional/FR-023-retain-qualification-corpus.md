---
id: FR-023
title: "Retain an identified qualification corpus"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-004"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-022"
    type: "depends_on"
---
# FR-023: Retain an identified qualification corpus

## Description

When a qualification profile declares an input population, the corpus builder SHALL
retain a closed manifest of representative, boundary, malformed, and
mutation inputs with exact bytes, classes, provenance, rights, and selection
rationale.

## Inputs

- The frozen qualification profile.
- Candidate corpus artifacts and their provenance and rights records.
- A coverage taxonomy for the declared command and grammar surface.

## Outputs

- One versioned corpus manifest and byte inventory.
- Coverage and exclusion records for every declared input class.

## Behavior

- The builder SHALL distinguish real repository inputs, independently authored
  conformance cases, generated cases, and mutations.
- The builder SHALL reject a missing byte artifact, digest mismatch, duplicate
  identity, unclassified input, absent rights disposition, or unexplained exclusion.
- Generated or mutated inputs SHALL name their generator or parent, seed or
  transformation, and reproducible lineage.
- Corpus counts SHALL NOT be presented as representativeness without a declared
  population and selection rationale.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-023-CON-1 | Private or restricted artifacts SHALL remain referenced or retained only under their recorded rights without promotion by the qualification report. | Rights | Inspection (TC-040) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-023-AC-1 | Every included input has exact bytes, identity, class, provenance, rights, and a population-selection reason. | Test (TC-040) |
| FR-023-AC-2 | The manifest covers every required profile class or records that class as an unresolved gap; an empty class cannot silently disappear from the denominator. | Test (TC-040) |
| FR-023-AC-3 | Missing bytes, changed bytes, duplicate identity, unclassified lineage, missing rights, or unexplained exclusion refuses corpus qualification. | Test (TC-040) |
| FR-023-AC-4 | Generated and mutated cases reproduce from their retained lineage and do not claim independent origin. | Test (TC-040) |

## Dependencies

The corpus is scoped by [FR-022](./FR-022-freeze-qualification-claim.md) and feeds
the oracle and execution requirements.
