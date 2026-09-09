---
id: FR-025
title: "Run discriminating qualification techniques"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-004"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-023"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-024"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-019"
    type: "depends_on"
---
# FR-025: Run discriminating qualification techniques

## Description

For each evidence obligation in a frozen qualification profile, the qualification
runner SHALL execute the selected example, property, fuzz, metamorphic, mutation,
and differential techniques over their declared corpus partitions and retain every
success, violation, failure, timeout, unsupported state, and minimized counterexample.

## Inputs

- Frozen profile, qualified corpus, and oracle obligations.
- Exact Rust harness and external-tool references.
- Deterministic seeds, budgets, mutation operators, and shrinking procedures.

## Outputs

- Exact `TechniqueResult` records and native artifact references.
- Minimized counterexamples and surviving-mutant records.

## Behavior

- Each technique SHALL declare the property, corpus partition, seed, budget,
  stopping rule, oracle, and expected failure interpretation before execution.
- The runner SHALL preserve failure and violation distinctions without erasing
  a failing case because another technique passes.
- A mutation claim SHALL include at least one seeded mutant whose removal of the
  claimed guard makes the assigned test fail for the intended reason.
- Fuzz or generated-input exhaustion without the declared budget completing SHALL
  remain incomplete rather than passing.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-025-CON-1 | First-party harness, generator, mutator, shrinker, assertion, and result-adapter logic SHALL be Rust and traced through `ix-trace-rs`. | Language | Test (TC-042) |
| FR-025-CON-2 | Native candidates and independent tools SHALL run only through NFR-006 containment. | Security | Test (TC-042) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-025-AC-1 | Every required technique retains its exact property, partition, seed, budget, stopping rule, oracle, result, and artifacts. | Test (TC-042) |
| FR-025-AC-2 | A failure, timeout, unsupported state, surviving mutant, or counterexample remains visible and blocks the affected obligation. | Test (TC-042) |
| FR-025-AC-3 | Removing each qualification-critical guard targeted by the mutation plan makes a named traced Rust test fail for the intended reason. | Test (TC-042) |
| FR-025-AC-4 | Minimized counterexamples replay against the exact candidate and reproduce the recorded outcome. | Test (TC-042) |
| FR-025-AC-5 | Unfinished fuzz or generation budgets remain incomplete and cannot be counted as a pass. | Test (TC-042) |

## Dependencies

The runner extends [FR-019](./FR-019-execute-bounded-parser-portfolio.md) over the
profile, corpus, and oracle boundaries established by FR-022 through FR-024.
