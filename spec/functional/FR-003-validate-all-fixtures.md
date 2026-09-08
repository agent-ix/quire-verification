---
id: FR-003
title: "Validate every fixture through an explicit handler"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-003"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-002"
    type: "depends_on"
---
# FR-003: Validate every fixture through an explicit handler

## Description

When fixture validation runs, the fixture validator SHALL execute an explicit schema-validation handler for every JSON file discovered directly under `fixtures/`.

## Inputs

- All direct-child JSON files under `fixtures/`.

## Outputs

- A per-file contract count, or a failing diagnostic.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-003-AC-1 | The four retained JSON fixtures and every embedded or adapted public contract instance pass the authoritative schema. | Test (TC-005) |
| FR-003-AC-2 | A newly discovered JSON fixture without a registered handler fails validation and names the file. | Test (TC-006) |
| FR-003-AC-3 | Fixture validation executes as a distinct CI step before the unit-test suite. | Inspection (TC-006) |

## Dependencies

- Implements [US-003](../usecase/US-003-detect-compatibility-regressions.md).
- Uses [FR-001](./FR-001-validate-public-contracts.md).
