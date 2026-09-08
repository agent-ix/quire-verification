---
id: FR-005
title: "Build prerequisite-aware verification plans"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-001"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-002"
    type: "depends_on"
---
# FR-005: Build prerequisite-aware verification plans

## Description

When a planning request selects techniques for inventoried properties, the planner SHALL emit a validated task graph that keeps applicability, readiness, prerequisites, and invocation permission distinct.

## Inputs

- A plan identifier, evaluation time, policy, properties, selections, capabilities, available prerequisites, and optional budget.

## Outputs

- A validated `VerificationPlan` with verification and prerequisite tasks.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-005-AC-1 | State, relational, trace, fault, and manual properties retain their distinct required input shapes. | Test (TC-009) |
| FR-005-AC-2 | An applicable selection with a qualified compatible capability and all required prerequisites emits a ready invocable verification task. | Test (TC-009) |
| FR-005-AC-3 | Missing capability support emits an unsupported non-invocable verification task with an actionable gap. | Test (TC-010) |
| FR-005-AC-4 | Missing bindings, tools, permissions, qualification, reviewer assignment, or policy freshness emit blocked work and explicit prerequisite tasks. | Test (TC-010) |
| FR-005-AC-5 | A fault task lacks invocation permission until non-production authorization, injection binding, and independent observation binding are present. | Test (TC-010) |
| FR-005-AC-6 | Multiple compatible capabilities without an explicit capability selection fail planning as ambiguous. | Test (TC-010) |

## Dependencies

- Implements [US-001](../usecase/US-001-plan-portable-verification.md).
- Plan validation is governed by [FR-001](./FR-001-validate-public-contracts.md) and [FR-002](./FR-002-derive-runtime-from-schema.md).
