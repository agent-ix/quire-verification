---
id: SR-214
title: "Scope-boundary review of Task-029 fuzz and mutation evidence"
type: SpecReview
analysis: scope-boundary
scope: "16922e1: AD-002, FR-025, NFR-004, TC-042, Task-029"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-025", type: reviews }
  - { target: "ix://agent-ix/quire-verification/Task-029", type: reviews }
---
# Scope-Boundary Review of Task-029 Fuzz and Mutation Evidence

## Summary

The repository owns Rust targets, seed bindings, portable manifests, exact result joins,
and domain assertions. QUOIN owns target generation and evidence retention; accepted
catalog producers own fuzz/mutation mechanics; Engineering Assurance owns bounded
execution; the verification owner alone decides sufficiency and qualification.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: Task-029 cannot add a local fuzzer driver, source rewriter, test runner, transcript parser, or evidence store. The boundary admits only consumer target code and portable result logic. | FR-025-CON-5, TC-042 step 11 |
| FND-002 | medium | QUOIN `spec-fuzz` owns selection and target emission but does not execute campaigns or convert source presence into evidence. | FR-025 behavior, SR-207 |
| FND-003 | medium | Engineering Assurance #34 is assumed until its Rust interface and real consumer case are accepted. No provisional shape is copied into q-v. | FR-025-CON-2, engineering-assurance#34 |
| FND-004 | high | The native Quire language remains the sole source profile. Fuzz or mutation success cannot stand in for any of the ten carried admission rules, and export-only foreign-runtime tools remain excluded from the qualification dependency closure. | FR-025 native admission behavior, quire-contract-ir#53 ruling |

## Responsibility Allocation

| Component | Responsibility | State |
| --- | --- | --- |
| quire-verification | Rust targets, exact corpus/manifest binding, deterministic portable results | guaranteed locally after implementation |
| QUOIN | Catalog advice, spec-derived target generation, evidence retention and sufficiency | exact installed workflow; #361 gap retained |
| Engineering Assurance | Bounded fuzz/mutation producer execution and typed results | assumed until #34 acceptance |
| catalog-declared producer | Input search, minimization, mutation generation and test execution | assumed through Engineering Assurance |
| verification owner | Harness authorization and final disposition | external owner action |

