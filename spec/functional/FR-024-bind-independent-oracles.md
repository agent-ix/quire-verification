---
id: FR-024
title: "Bind independent qualification oracles"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-004"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-022"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-023"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-017"
    type: "depends_on"
---
# FR-024: Bind independent qualification oracles

## Description

For every qualification guarantee, the evidence planner SHALL bind each selected
oracle to its authority, implementation, revision, input projection, expected
relation, independence class, limitations, and disagreement procedure.

## Inputs

- The frozen profile and qualified corpus manifest.
- Exact oracle implementation and procedure references.
- Declared relations between candidate and oracle observations.

## Outputs

- Closed oracle obligations and contribution records.
- Explicit no-oracle or disagreement findings where independence is unavailable.

## Behavior

- The planner SHALL distinguish independent, shared, derived, and unknown oracle
  relationships using [FR-017](./FR-017-retain-oracle-dependence.md).
- A candidate-generated expected value SHALL NOT be classified as independent.
- Oracle disagreement SHALL retain both observations and remain unresolved
  until the declared procedure attributes or bounds the difference.
- If no qualified oracle exists for a required guarantee, then the planner SHALL
  report a gap rather than treating a self-consistency check as equivalent evidence.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-024-CON-1 | Oracle execution SHALL use the same bounded Rust native-process boundary as other external tools. | Security | Test (TC-041) |
| FR-024-CON-2 | The repository SHALL store references and observations, not a duplicate Quoin evidence store. | Ownership | Inspection (TC-041) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-024-AC-1 | Every required guarantee names at least one exact oracle or an explicit no-oracle gap with limitations. | Test (TC-041) |
| FR-024-AC-2 | Self-produced, shared-source, derived, and independent observations retain their correct dependence classes. | Test (TC-041) |
| FR-024-AC-3 | Agreement supports only the declared relation; disagreement retains both exact observations and blocks acceptance until dispositioned. | Test (TC-041) |
| FR-024-AC-4 | Changed oracle identity, projection, input, expected relation, or limitation refuses contribution reuse. | Test (TC-041) |

## Dependencies

Oracle obligations consume [FR-022](./FR-022-freeze-qualification-claim.md),
[FR-023](./FR-023-retain-qualification-corpus.md), and the existing dependence
model in [FR-017](./FR-017-retain-oracle-dependence.md).
