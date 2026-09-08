---
id: NFR-002
title: "Contract source maintainability"
type: NFR
quality_attribute: maintainability
relationships:
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-002"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-003"
    type: "constrains"
---
# NFR-002: Contract source maintainability

## Statement

The compatibility slice SHALL maintain one authoritative definition of each public contract in `contracts/e02-draft-1.schema.json`.

## Scope

This constraint applies to runtime validators, exported vocabulary constants,
fixture validation, and CI.

## Rationale

Hand-synchronized schema and runtime definitions can diverge while unit fixtures
continue to exercise only one of them.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Independently authored public-contract definition sets | 1 | 1 | Inspection (TC-003) |
| Discovered JSON fixtures without an explicit schema handler | 0 | 0 | Test (TC-005, TC-006) |

## Verification

Static inspection confirms that runtime definitions are read and compiled from
the schema packet. Fixture validation fails when any direct fixture lacks a
handler, and CI executes fixture validation separately.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-002-AC-1 | Runtime contract validators and exported vocabularies derive from the single versioned JSON Schema packet. | Inspection (TC-003) |
| NFR-002-AC-2 | Every discovered direct-child JSON fixture executes a registered schema-validation handler. | Test (TC-005, TC-006) |

## Dependencies

- Constrains [FR-001](../functional/FR-001-validate-public-contracts.md), [FR-002](../functional/FR-002-derive-runtime-from-schema.md), and [FR-003](../functional/FR-003-validate-all-fixtures.md).
