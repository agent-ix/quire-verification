---
id: FR-034
title: "Account the complete-V1 qualification ledger"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-005"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-032"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-033"
    type: "depends_on"
  - target: "ix://agent-ix/quire-specification/FR-311"
    type: "references"
  - target: "ix://agent-ix/quire-specification/FR-335"
    type: "references"
  - target: "ix://agent-ix/quire-specification/FR-336"
    type: "references"
  - target: "ix://agent-ix/quire-specification/FR-338"
    type: "references"
---
# FR-034: Account the complete-V1 qualification ledger

## Description

When complete-V1 qualification is evaluated, the qualification-ledger builder SHALL account exactly one primary row for each of the 176 accepted inventory capabilities and every applicable claim-class vector under one exact ecosystem lock.

## Inputs

- Accepted 176-row inventory revision, independent conformance corpus, qualification profile, and ecosystem lock.
- Exact source, package, model/binding, implementation, provider, method, run, result, artifact, and decision records.

## Outputs

- Canonical per-capability ledger and claim-class rollups.
- Planned, unexecuted, passed, failed, unsupported, or waived vector states.
- Scope-limited future qualification disposition or explicit incomplete qualification.

## Behavior

- Every inventory capability SHALL occur exactly once as a primary row.
- Applicable grammar, semantic-kernel, runtime/monitor, analysis, backend, tooling, and consuming-integration vectors SHALL remain independent.
- Every vector SHALL bind exact source, package, implementation, method, corpus, lock, run, and result identities.
- Missing, unexecuted, failed, or unsupported required vectors SHALL block only their containing qualification claim.
- The ledger builder SHALL retain missing, unexecuted, failed, or unsupported vectors as visible state.
- A waiver SHALL require an attributed scope-limited human decision.
- A waiver SHALL NOT count as implementation or a passed vector.
- Eventual complete-V1 candidate acceptance SHALL require every required vector to be passed plus one authenticated attributed decision over the unchanged ledger scope.
- Self-produced or circular evidence SHALL NOT discharge a row.
- A score or favorable majority SHALL NOT promote candidate acceptance.
- Planned, unexecuted, failed, unsupported, or missing production evidence SHALL remain typed ledger state.
- Missing production evidence SHALL NOT block development, local tests, or provider integration.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-034-CON-1 | The accepted inventory revision and all 176 primary identifiers SHALL form a closed denominator. | Completeness | Test (TC-051) |
| FR-034-CON-2 | Human decisions SHALL remain separate immutable records. | Accountability | Test (TC-051) |
| FR-034-CON-3 | The ledger builder SHALL NOT infer a human decision from ledger state. | Accountability | Test (TC-051) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-034-AC-1 | The canonical ledger contains all and only the 176 accepted primary capability IDs exactly once, with every required claim-class vector accounted. | Test (TC-051) |
| FR-034-AC-2 | Removing or duplicating a row/vector, changing an ecosystem pin, using self/circular evidence, or leaving a required vector planned, unexecuted, failed, or unsupported prevents the affected qualification claim. | Test (TC-051) |
| FR-034-AC-3 | Only an authenticated attributed human decision over the unchanged scope can eventually accept, reject, or explicitly limit a candidate; a waiver never becomes implementation or a pass, and absent production evidence does not gate development execution. | Test (TC-051) |

## Dependencies

- [FR-032](./FR-032-link-semantic-evidence.md) and [FR-033](./FR-033-preserve-counterexample-lineage.md) own evidence and replay traceability.
- The final cross-lane integrated execution campaign is owned outside this repository.
