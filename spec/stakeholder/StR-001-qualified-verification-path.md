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
| StR-001-VC-2 | Evidence with a foreign, stale, ambiguous, failed, incomplete, or otherwise adverse binding is never reported as supported. | Test (TC-012, TC-013, TC-014, TC-015) |
| StR-001-VC-3 | The Rust implementation preserves the complete versioned contract, fixture, catalog, plan, assessment, and bounded-portfolio behavior. | Demonstration (TC-001..TC-023) |

## Stakeholders

The decision stakeholder is the Agent IX verification owner. Quoin maintainers
and future production consumers are affected stakeholders at the JSON boundary.

## Context and Assumptions

The current JavaScript implementation is a rejected prototype and supplies
behavioral discovery input only. FS02 and FS05 identity decisions remain
provisional and may require a later reviewed migration.

## Stakeholder Constraints (Contextual)

The Rust crate must not absorb Quoin's CLI orchestration responsibilities or
create a second evidence store. The current change establishes correctness and
bounded resource behavior before any production performance threshold is set.

## Dependencies

The need depends on the versioned E02 schema packet and the pinned Quoin method
catalog boundary. Cross-repository identity acceptance remains outside this slice.

## Priority and Risk (Informative)

Priority is P0 because language and evidence-integrity failures disqualify the
component from the intended path. The dominant risk is behavioral drift during
the full rearchitecture.

## Traceability

All FR and NFR artifacts in this bounded slice satisfy a portion of this
stakeholder need; the Rust gate is stated explicitly in NFR-004.

