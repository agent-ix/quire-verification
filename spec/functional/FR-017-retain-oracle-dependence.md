---
id: FR-017
title: "Retain oracle dependence without aggregation"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-002"
    type: implements
  - target: "ix://agent-ix/quire-verification/FR-006"
    type: depends_on
  - target: "ix://agent-ix/quire-verification/FR-007"
    type: depends_on
  - target: "ix://agent-ix/quire-verification/FR-016"
    type: depends_on
  - target: "ix://agent-ix/quire-specification/FS05"
    type: depends_on
---
# FR-017: Retain Oracle Dependence Without Aggregation

## Description

When a technique result claims a logical outcome, the evidence assessor SHALL
retain the exact oracle contributions and their declared dependence relationships
without converting multiple contributions into a global assurance score.

## Inputs

- Evidence obligations and exact-matched technique results.
- Oracle contributions carrying exact oracle and observation references.
- Authored dependence relationships among oracle contributions.

## Outputs

- One non-authoritative assessment entry per obligation.
- Deterministically ordered oracle-dependence findings across the assessment.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-017-AC-1 | A completed result with logical outcome `satisfied` or `violated` carries at least one oracle contribution identifying a unique nonempty contribution ID, an exact oracle reference, an exact observation reference, and relation `independent`, `shared`, `derived`, or `unknown`; a non-evaluated result may carry no contribution. | Test (TC-033) |
| FR-017-AC-2 | Distinct technique or result identifiers do not establish independence; contributions sharing an oracle, observation, or declared dependence group emit one `correlated-oracle` finding with sorted result and contribution identifiers. | Test (TC-033) |
| FR-017-AC-3 | Relation `unknown` emits `oracle-dependence-unknown` and makes no independence claim; an evaluated contribution with an absent relation or duplicate contribution ID is rejected before assessment. | Test (TC-033) |
| FR-017-AC-4 | A `derived` contribution identifies its exact source contribution and cannot be reported as an independent confirmation of that source. | Test (TC-033) |
| FR-017-AC-5 | The assessment preserves each obligation and contribution separately, emits no numeric method score or aggregate assurance level, identifies Quoin as evidence owner, and records sufficiency as `not-made`. | Test (TC-033) |

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-017-CON-1 | The assessor SHALL use the exact independently reviewed FS05 reference roles selected for oracle and observation artifacts. | Compatibility | Test (TC-033) |
| FR-017-CON-2 | The assessor SHALL label independence only as the declared relationship supported by the supplied exact references. | Claim boundary | Inspection (TC-033) |
| FR-017-CON-3 | The assessor SHALL NOT claim undisclosed causal independence. | Claim boundary | Inspection (TC-033) |

## Dependencies

- Exact obligation matching and independent result findings come from [FR-006](./FR-006-match-evidence-exactly.md) and [FR-007](./FR-007-preserve-result-findings.md).
- Exact shared-reference consumption comes from [FR-016](./FR-016-read-shared-references-strictly.md).
- Contract and implementation acceptance remain blocked until `ix://agent-ix/quire-specification/FS05` represents the selected oracle and observation roles.
