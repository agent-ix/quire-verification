---
id: FR-001
title: "Validate the five public contracts"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-001"
    type: "implements"
---
# FR-001: Validate the five public contracts

## Description

When a public contract value enters the compatibility slice, the contract validator SHALL accept it only when it satisfies exactly one of the five definitions in `contracts/e02-draft-2.schema.json`.

## Inputs

- A JSON value presented as a public contract.

## Outputs

- The unchanged validated value, or a validation error with schema-path detail.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-001-AC-1 | Given one valid instance of each public contract kind, each instance is accepted unchanged. | Test (TC-001) |
| FR-001-AC-2 | Given an internal inventory object that is not one of the five public contracts, root contract validation rejects it. | Test (TC-001) |
| FR-001-AC-3 | Given a result whose execution did not complete but whose logical outcome claims satisfied or violated, validation rejects it. | Test (TC-002) |
| FR-001-AC-4 | Given a plan task with a missing required field, an unknown field, or a wrongly typed field, validation rejects it. | Test (TC-002) |
| FR-001-AC-5 | Given a structured boundary value whose `contractVersion` is not `quire-verification/e02-draft-2`, including the former draft-1 value, validation rejects it without compatibility inference or fallback. | Test (TC-002) |

## Dependencies

- Driven by [US-001](../usecase/US-001-plan-portable-verification.md).
- Shared reference shapes come from the exact independently accepted packet governed by [FR-016](./FR-016-read-shared-references-strictly.md).
