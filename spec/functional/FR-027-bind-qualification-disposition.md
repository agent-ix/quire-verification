---
id: FR-027
title: "Bind the production qualification disposition"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-004"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-021"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-026"
    type: "depends_on"
---
# FR-027: Bind the production qualification disposition

## Description

After all required evidence and measurements are retained, the qualification
assembler SHALL seal one immutable core and bind exactly one external
verification-owner `accept`, `revise`, or `reject` disposition to that core and
its exact qualification profile.

## Inputs

- Frozen profile, corpus, oracle, technique, production-chain, measurement, review, and gap-analysis artifacts.
- One external disposition following the selected review procedure.

## Outputs

- A final qualification report or an explicit incomplete report.

## Behavior

- When disposition is requested, the assembler SHALL recompute all profile, corpus, oracle,
  result, measurement, review, and production-chain bindings and seal them without
  disposition bytes in the digest domain.
- The assembler SHALL remain incomplete while any required obligation, owner
  threshold, external interface, review finding, or gap is unresolved.
- The assembler SHALL admit `accept` only when all required evidence passes and the
  disposition repeats the exact core digest, profile identity, owner, procedure,
  time, and bounded claim text.
- `revise` and `reject` SHALL remain final attributed decisions without being
  rewritten as execution failure or logical violation.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-027-CON-1 | Quoin remains evidence owner and the human owner remains the sole author of sufficiency and qualification decisions. | Ownership | Inspection (TC-045) |
| FR-027-CON-2 | The final claim SHALL NOT exceed the frozen profile or infer certification, every-platform support, or untested semantic stages. | Claims | Test (TC-045) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-027-AC-1 | The sealed core recomputes every retained identity and excludes disposition bytes from its own JCS digest. | Test (TC-045) |
| FR-027-AC-2 | Any unresolved evidence, measurement, interface, review, or gap keeps the report incomplete and identifies the blocker. | Test (TC-045) |
| FR-027-AC-3 | One exact external disposition produces a final report whose claim is byte-for-byte bounded by the frozen profile. | Test (TC-045) |
| FR-027-AC-4 | Missing, duplicate, foreign, unsupported, stale, or claim-expanding dispositions are refused without changing the core. | Test (TC-045) |
| FR-027-AC-5 | `revise` and `reject` remain attributed final decisions and cannot be reported as an automated result. | Test (TC-045) |

## Dependencies

This requirement extends [FR-021](./FR-021-bind-portfolio-report.md) only after the
real production chain in [FR-026](./FR-026-execute-production-chain.md) completes.
