---
id: FR-015
title: "Preserve role-specific verification inputs"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-001"
    type: implements
  - target: "ix://agent-ix/quire-verification/FR-005"
    type: depends_on
---
# FR-015: Preserve Role-Specific Verification Inputs

## Description

When a planning request carries state, relational, trace, fault, or manual work,
the contract reader and planner SHALL preserve the selected variant's complete
role-specific input without coercing it into a unary predicate shape.

## Inputs

- One closed input variant selected by its `kind` discriminator.

## Outputs

- The same validated role-specific input in the inventory, verification task, and runner request boundary where applicable.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-015-AC-1 | State input requires an exact snapshot reference and property expression; relational input requires distinct left and right subject references plus the relation. | Test (TC-028) |
| FR-015-AC-2 | Trace input requires an exact trace reference, clock profile, and completion statement; fault input requires an exact fault-model reference, requested site, and mitigation oracle. | Test (TC-028) |
| FR-015-AC-3 | Manual input requires an exact review-procedure reference and at least one allowed disposition, and it always remains human work. | Test (TC-028) |
| FR-015-AC-4 | Missing, cross-variant, or unexpected semantic fields fail the closed selected input variant instead of being ignored or guessed. | Test (TC-028) |
| FR-015-AC-5 | One parser-portfolio request retains complementary state/property, relational/differential, and manual-review tasks with separate prerequisites and runner eligibility. | Test (TC-030) |

## Dependencies

- Refines the variant-preservation behavior in [FR-005](./FR-005-build-verification-plans.md).
- Exact reference envelopes are governed by [FR-016](./FR-016-read-shared-references-strictly.md).
