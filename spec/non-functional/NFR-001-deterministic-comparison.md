---
id: NFR-001
title: "Deterministic structured comparison"
type: NFR
quality_attribute: reliability
relationships:
  - target: "ix://agent-ix/quire-verification/FR-006"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-008"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-009"
    type: "constrains"
---
# NFR-001: Deterministic structured comparison

## Statement

The compatibility slice SHALL produce identical canonical bytes, digests, ordered diagnostics, and portfolio verdicts for inputs with identical JSON meaning under every tested property-order and catalog-order permutation.

## Scope

This constraint applies to contract identity comparison, result-gap ordering,
catalog parity, and the bounded local portfolio.

## Rationale

Digest or verdict changes caused only by insertion order make retained evidence
irreproducible and can hide an index-coupled parity defect.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Tested semantic-equivalence permutations with divergent output | 0 | 0 | Test (TC-016, TC-018) |
| Required diagnostics with nondeterministic ordering | 0 | 0 | Test (TC-012, TC-013, TC-015) |

## Verification

Property and example-based tests permute JSON property order and catalog method
order, then compare canonical values, digests, adapted records, and diagnostics.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-001-AC-1 | Every tested JSON property-order and catalog-order permutation produces identical canonical or adapted output. | Test (TC-016, TC-018) |
| NFR-001-AC-2 | Exact-match candidates and independent result gaps appear in deterministic identifier or canonical order. | Test (TC-012, TC-013, TC-015) |

## Dependencies

- Constrains [FR-006](../functional/FR-006-match-evidence-exactly.md), [FR-008](../functional/FR-008-use-jcs.md), and [FR-009](../functional/FR-009-retain-portfolio-failures.md).
