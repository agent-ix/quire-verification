---
id: FR-007
title: "Preserve independent result findings"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-002"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-006"
    type: "depends_on"
---
# FR-007: Preserve independent result findings

## Description

When one exact result is assessed, the assessor SHALL preserve every applicable execution, activation, observation, participation, assumption, and logical finding independently of the compatibility summary status.

## Inputs

- One exact `TechniqueResult` and its evidence obligation.

## Outputs

- A compatibility status and a deterministic list of independent gaps.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-007-AC-1 | Unsupported, failed, incomplete, untriggered, observation-inadequate, partial, assumption-unmet, violated, and supported cases remain distinguishable. | Test (TC-014) |
| FR-007-AC-2 | Simultaneous partial participation, unmet assumption, and logical violation emit all three gap kinds. | Test (TC-015) |
| FR-007-AC-3 | The assessment denominator and output contain one entry for every inventoried obligation, including obligations without results. | Test (TC-015) |
| FR-007-AC-4 | Provenance and native artifact references remain present in the assessment view. | Test (TC-015) |
| FR-007-AC-5 | The assessment identifies Quoin as evidence owner and records that no sufficiency decision was made. | Test (TC-015) |

## Dependencies

- Implements [US-002](../usecase/US-002-assess-evidence-exactly.md).
