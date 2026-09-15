---
id: NFR-009
title: "Deterministic bounded method and ledger accounting"
type: NFR
quality_attribute: reliability
relationships:
  - target: "ix://agent-ix/quire-verification/FR-029"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-031"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-034"
    type: "constrains"
---
# NFR-009: Deterministic bounded method and ledger accounting

## Statement

The complete-V1 method layer SHALL produce byte-identical canonical identities,
node accounting, and qualification-ledger rows for equal exact inputs while
refusing work beyond declared semantic and resource bounds.

## Scope

- Applies to model and relational providers, plan dispatch/accounting, evidence graphs, counterexample lineage, and qualification ledgers.
- Excludes nondeterministic provider observations themselves; their exact order, seeds, environment, and bytes remain bound inputs.

## Rationale

Pre-production replay and qualification cannot be attributed if discovery order, hash-map
iteration, ambient defaults, or exhausted resources silently change identities
or erase unfinished work.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Canonical identity mismatches across 100 set-order permutations | 0 | 0 | Property test (TC-049, TC-051) |
| Unaccounted plan nodes or required ledger vectors | 0 | 0 | Property test (TC-049, TC-051) |
| Operations exceeding an authored semantic or resource bound without a typed non-conclusive result | 0 | 0 | Boundary test (TC-047, TC-048, TC-051) |

## Verification

Property and boundary tests permute set-like inputs, retain ordered inputs, mutate
one identity dimension at a time, and exhaust each declared bound. The checks
compare complete typed outputs rather than display summaries.

## Dependencies

- [FR-029](../functional/FR-029-generate-model-tests.md), [FR-031](../functional/FR-031-execute-linked-method-plans.md), and [FR-034](../functional/FR-034-qualify-complete-v1-ledger.md) define the constrained operations.
