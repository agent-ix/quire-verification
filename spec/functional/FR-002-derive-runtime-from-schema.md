---
id: FR-002
title: "Derive runtime contract behavior from the schema"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-001"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "depends_on"
---
# FR-002: Derive runtime contract behavior from the schema

## Description

When the contract module loads, the runtime SHALL compile its public validators and derive its exported contract constants from `contracts/e02-draft-2.schema.json`.

## Inputs

- The E02 draft-2 schema and exact retained shared-reference draft-2 schema.

## Outputs

- Compiled validators and frozen exported constants.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-002-AC-1 | The exported contract version equals the schema contract-version constant. | Test (TC-003) |
| FR-002-AC-2 | The exported input, applicability, readiness, execution-status, and logical-outcome vocabularies equal their schema definitions. | Test (TC-003) |
| FR-002-AC-3 | Duplicate plan task IDs, unknown dependency IDs, and dependency cycles are rejected by explicit cross-record validation. | Test (TC-004) |
| FR-002-AC-4 | Generated E02 input types resolve their shared artifact envelopes from the retained shared-reference schema; no independently authored E02 copy of the shared artifact-reference field set exists. | Test (TC-028) |

## Dependencies

- Implements [US-001](../usecase/US-001-plan-portable-verification.md).
- Conforms to [FR-001](./FR-001-validate-public-contracts.md).
