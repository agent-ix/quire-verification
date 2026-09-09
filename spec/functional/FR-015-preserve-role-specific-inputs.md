---
id: FR-015
title: "Preserve role-specific verification inputs"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-001"
    type: implements
  - target: "ix://agent-ix/quire-verification/FR-005"
    type: depends_on
  - target: "ix://agent-ix/quire-verification/FR-016"
    type: depends_on
---
# FR-015: Preserve Role-Specific Verification Inputs

## Description

When a draft-2 property inventory carries state, relational, trace, fault, or
manual work, the contract reader and planner SHALL preserve the selected
variant's complete role-specific input on its verification task without
coercing it into a unary predicate shape.

## Inputs

- One closed property-input variant selected by its `kind` discriminator.
- For every reference-bearing field, a complete
  `ix.shared-reference/2-draft` artifact envelope and the corresponding opaque
  validated-reference value returned by the strict reader in the caller's
  validation context.

## Outputs

- The same validated role-specific input in the inventory and verification
  task, and in the runner request boundary where applicable, or a stable refusal
  before a plan is emitted.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-015-AC-1 | State input requires a validated artifact envelope whose value kind is `snapshot` plus a nonempty property expression; relational input requires distinct nonempty left and right subject identities plus a nonempty relation. | Test (TC-028) |
| FR-015-AC-2 | Trace input requires a validated artifact envelope whose value kind is `trace`, a nonempty clock profile, and a nonempty completion statement; fault input requires validated `fault-model` and `oracle` artifact envelopes plus a nonempty requested injection site. | Test (TC-028) |
| FR-015-AC-3 | Manual property input requires a validated artifact envelope whose value kind is `review-procedure` and a duplicate-free nonempty array drawn only from `accept`, `revise`, and `reject`; the planner preserves the authored array order, emits a null native runner request, and keeps `invocationAllowed` false even when its human prerequisites are present. | Test (TC-028, TC-030) |
| FR-015-AC-4 | The planner matches embedded envelopes to opaque strict-reader outputs by exact JCS equality and role; a missing field, equal relational subject identities, wrong reference version or role, envelope absent from those outputs, cross-variant field, duplicate disposition, or unexpected field refuses before plan emission instead of being ignored or guessed. | Test (TC-028) |
| FR-015-AC-5 | One parser-portfolio request retains complementary state/property, relational/differential, and manual-review tasks with separate prerequisites and runner eligibility. | Test (TC-030) |

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-015-CON-1 | Draft-1 E02 inputs are not migrated, reinterpreted, or accepted by the draft-2 planner. | Compatibility | Test (TC-028) |

## Dependencies

- Refines the variant-preservation behavior in [FR-005](./FR-005-build-verification-plans.md).
- Exact reference envelopes are governed by [FR-016](./FR-016-read-shared-references-strictly.md).
