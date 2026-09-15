---
id: FR-029
title: "Generate and shrink complete-V1 model tests"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-005"
    type: "implements"
  - target: "ix://agent-ix/quire-specification/FR-190"
    type: "references"
  - target: "ix://agent-ix/quire-specification/FR-336"
    type: "references"
---
# FR-029: Generate and shrink complete-V1 model tests

## Description

When model-based testing is selected for an exact checked package, the model-test
provider SHALL derive rejection-free generators and a well-founded shrink order
from its checked domains and relationships, using native replay as the semantic
oracle.

## Inputs

- Exact checked package, semantic-subject identity, and checked finite domains.
- Preconditions, cross-field relationships, deterministic seed, case and shrink bounds.
- Exact native-runtime provider and ecosystem-lock identities.

## Outputs

- Deterministic boundary-partitioned cases and one typed method result per run.
- Immutable original, shrink-path, minimal-counterexample, and replay references.
- A typed refusal for an unsatisfiable domain or method failure for invalid replay.

## Behavior

- The provider SHALL derive values from checked source domains rather than Rust implementation types.
- The provider SHALL include empty, singleton, inclusive minimum, inclusive maximum, representable just-outside refusal, and applicable cross-field boundary partitions.
- When domain construction completes, the provider SHALL generate without precondition rejection.
- If the admitted domain is unsatisfiable, then the provider SHALL refuse before execution.
- Every retained shrink SHALL be strictly smaller under the pinned typed order, remain domain-valid, and replay to the same failing clause and disposition.
- A rejected precondition SHALL NOT count as a passing case.
- The same package, profile, seed, and bounds SHALL reproduce the same cases, shrink path, and identities.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-029-CON-1 | Generator, shrinker, replay adapter, and result-construction logic SHALL be first-party Rust. | Language | Inspection (TC-047) |
| FR-029-CON-2 | Native replay SHALL use the accepted provider boundary. | Ownership | Test (TC-047) |
| FR-029-CON-3 | The provider SHALL NOT replace native replay with a local semantic oracle. | Ownership | Test (TC-047) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-029-AC-1 | Generated cases inhabit exact checked domains and cover every applicable empty, singleton, endpoint, just-outside, and cross-field partition. | Test (TC-047) |
| FR-029-AC-2 | Every retained shrink is strictly smaller, domain-valid, and reproduces the same failing clause and disposition through exact native replay. | Test (TC-047) |
| FR-029-AC-3 | Equal inputs reproduce equal case and shrink identities; an unsatisfiable domain, rejected precondition, invalid shrink, changed oracle, or replay disagreement never produces satisfaction. | Test (TC-047) |

## Dependencies

- [interface-001](../interface/interface-001-verification-method-provider.md) owns the common request/result boundary independently of production qualification.
- `ix://agent-ix/quire-specification/FR-190` and `ix://agent-ix/quire-specification/FR-336` are the accepted semantic and corpus contracts.
