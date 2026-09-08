---
id: FR-006
title: "Match evidence by exact identity and execution scope"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-002"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-008"
    type: "depends_on"
---
# FR-006: Match evidence by exact identity and execution scope

## Description

When evidence is assessed, the assessor SHALL match a result to an obligation only when every required identity, version, binding, provenance, and execution-scope dimension is exactly equal.

## Inputs

- Evidence obligations, technique results, and an assessment time.

## Outputs

- One assessment entry per obligation with exact-match diagnostics.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-006-AC-1 | A difference in any required technique, property, subject, profile, binding, dependency, execution scope, plan, task, run, producer, or adapter dimension prevents a match and names the differing dimension. | Test (TC-012) |
| FR-006-AC-2 | No exact candidate yields missing status while retaining every nearby candidate's scope-mismatch diagnostics. | Test (TC-012) |
| FR-006-AC-3 | More than one exact candidate yields ambiguous status with deterministically ordered candidate result IDs. | Test (TC-013) |
| FR-006-AC-4 | Duplicate result IDs or obligations lacking a technique and task-level result binding are rejected. | Test (TC-013) |
| FR-006-AC-5 | A result beyond its explicit validity or obligation maximum age is classified stale before its logical outcome determines status. | Test (TC-014) |

## Dependencies

- Implements [US-002](../usecase/US-002-assess-evidence-exactly.md).
- Structured equality follows [FR-008](./FR-008-use-jcs.md).
