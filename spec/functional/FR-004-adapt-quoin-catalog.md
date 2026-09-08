---
id: FR-004
title: "Adapt Quoin catalog and advice without changing meaning"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-003"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-002"
    type: "depends_on"
---
# FR-004: Adapt Quoin catalog and advice without changing meaning

## Description

When a pinned Quoin catalog or advice export is adapted, the catalog adapter SHALL preserve authored method identity and meaning while exposing source conflicts, unreadable roots, and inconclusive applicability.

## Inputs

- A pinned Quoin `MethodCatalog` export or advice record.

## Outputs

- A portable technique registry or portable selection advice.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-004-AC-1 | Every unique method preserves its identifier, name, class, definition, evidence kind, applicability, tooling hints, and source module. | Test (TC-007) |
| FR-004-AC-2 | Duplicate definitions or unreadable roots preserve the diagnostic first-wins view but mark the adapted registry unqualified. | Test (TC-008) |
| FR-004-AC-3 | Positive advice maps to applicable selections with reasons, while empty inconclusive advice maps to unknown residual applicability. | Test (TC-008) |

## Dependencies

- Implements [US-003](../usecase/US-003-detect-compatibility-regressions.md).
- The real producer boundary is verified by [IT-001](../integration/IT-001-real-quoin-catalog-parity.md).
