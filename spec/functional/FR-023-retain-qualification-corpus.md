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
- Exact bounded replay-run reference and output bytes for each generated or mutated candidate.

## Outputs

- One versioned corpus manifest and byte inventory.
- Coverage and exclusion records for every declared input class.
- Reproducible lineage records bound to exact retained derived bytes.

## Behavior

- The builder SHALL distinguish real repository inputs, independently authored
  conformance cases, generated cases, and mutations.
- The manifest SHALL bind the frozen profile identity, population identity, and exact
  required-class set without adding or removing a profile class.
- Every included byte artifact SHALL match an exact strict-reader proof produced from
  its locally supplied bytes.
- The builder SHALL reject a missing byte artifact, digest mismatch, duplicate
  identity, unclassified input, absent rights disposition, or unexplained exclusion.
- An included artifact's rights disposition SHALL explicitly authorize local retention;
  a reference-only or unauthorized artifact remains an owned class gap.
- Generated inputs SHALL name an exact generator reference, seed, invocation, and
  strict-reader-proved `run-artifact` whose retained output equals their retained bytes.
- Mutated inputs SHALL name an existing parent entry, seed, transformation, and replay
  `run-artifact` whose retained output equals their retained bytes.
- Mutation parent relationships SHALL form an acyclic graph with no self-parent.
- Corpus counts SHALL NOT be presented as representativeness without a declared
  population and selection rationale.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-023-CON-1 | Private or restricted artifacts SHALL remain referenced or retained only under their recorded rights without promotion by the qualification report. | Rights | Inspection (TC-040) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-023-AC-1 | Every included input has strict-reader-proved exact bytes, identity, class, provenance, retention authorization, and a population-selection reason under the exact frozen profile and population identities. | Test (TC-040) |
| FR-023-AC-2 | The manifest covers every required profile class with at least one retained input or exactly one unresolved owned gap; an empty class cannot silently disappear from the denominator. | Test (TC-040) |
| FR-023-AC-3 | A missing or changed byte artifact, duplicate identity, unknown class, missing retention authorization, unexplained exclusion, invalid parent graph, missing replay-run proof, or replay mismatch refuses corpus qualification. | Test (TC-040) |
| FR-023-AC-4 | Every generated and mutated case has a strict-reader-proved replay run whose retained output reproduces the case byte-for-byte from its generator or parent lineage, and the case remains classified as derived. | Test (TC-040) |

## Dependencies

The corpus is scoped by [FR-022](./FR-022-freeze-qualification-claim.md) and feeds
the oracle and execution requirements.
