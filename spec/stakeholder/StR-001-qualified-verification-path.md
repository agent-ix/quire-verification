---
id: StR-001
title: "Qualified verification path uses a Rust core"
type: StR
relationships:
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-002"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-003"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-004"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-005"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-006"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-007"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-008"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-009"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-010"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-011"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-012"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-013"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-014"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-015"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-016"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-017"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-018"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-019"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-020"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-021"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-028"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/NFR-001"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/NFR-002"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/NFR-003"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/NFR-004"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/NFR-005"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/NFR-006"
    type: "satisfied_by"
---
# StR-001: Qualified verification path uses a Rust core

## Stakeholder Need

The Agent IX verification owner SHALL receive a Rust implementation for any component eligible to participate in the qualified production verification path.

## Rationale

This component validates contracts, plans verification work, and attributes
evidence. Its results can influence assurance decisions, and production-scale
use can place its canonicalization and matching work on a performance-sensitive
path. Plain JavaScript is not an acceptable implementation language for that
role. Quoin may retain TypeScript orchestration outside this component and
exchange versioned JSON at the boundary.

## Validation Criteria

| ID | Criteria | Validation |
|----|----------|------------|
| StR-001-VC-1 | All repository-owned executable source, command-line tools, and tests are Rust and pass the required Rust quality gates. | Test (TC-022) |
| StR-001-VC-2 | Evidence with a foreign, stale, ambiguous, failed, incomplete, or otherwise adverse binding is never reported as supported. | Test (TC-012, TC-013, TC-014, TC-015, TC-035, TC-037) |
| StR-001-VC-3 | The Rust implementation preserves the complete versioned contract, fixture, catalog, plan, assessment, and bounded-portfolio behavior. | Demonstration (TC-001..TC-038) |

## Stakeholders

The decision stakeholder is the Agent IX verification owner. Quoin maintainers
and future production consumers are affected stakeholders at the JSON boundary.

## Context and Assumptions

The prior JavaScript implementation was a rejected prototype and supplied
behavioral discovery input only. The accepted retained FS02/FS05 packet fixes
the selected JCS-domain and role-kind amendments for this private Rust boundary;
public schema terms remain unresolved.

## Stakeholder Constraints (Contextual)

The Rust crate must not absorb Quoin's CLI orchestration responsibilities or
create a second evidence store. The current change establishes correctness and
bounded resource behavior before any production performance threshold is set.

## Dependencies

The need depends on the versioned E02 schema packet, the retained accepted
FS02/FS05 reference packet, and the pinned Quoin method-catalog boundary.

## Priority and Risk (Informative)

Priority is P0 because language and evidence-integrity failures disqualify the
component from the intended path. The dominant risk is behavioral drift during
the full rearchitecture.

## Traceability

All FR and NFR artifacts in this bounded slice satisfy a portion of this
stakeholder need; the Rust gate is stated explicitly in NFR-004.
