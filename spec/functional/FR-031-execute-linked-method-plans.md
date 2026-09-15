---
id: FR-031
title: "Execute linked complete-V1 method plans"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-005"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-005"
    type: "depends_on"
  - target: "ix://agent-ix/quire-specification/FR-192"
    type: "references"
  - target: "ix://agent-ix/quire-specification/FR-332"
    type: "references"
  - target: "ix://agent-ix/quire-specification/FR-333"
    type: "references"
---
# FR-031: Execute linked complete-V1 method plans

## Description

When an authored complete-V1 verification plan is linked, the plan coordinator SHALL resolve its exact subjects, methods, providers, assumptions, domains, bounds, evidence classes, and typed dependency edges before dispatching ready nodes.

## Inputs

- Versioned linked plan, checked package, exact ecosystem lock, and resource policy.
- Capability-constrained provider manifests implementing the method-provider interface.

## Outputs

- One terminal accounted disposition for every plan node.
- Dependency-ordered provider requests and immutable method-result references.
- Typed link, provider, cycle, blocked, unsupported, not-run, or aggregation refusal.

## Behavior

- Plan nodes SHALL bind one exact claim, semantic subject, method profile, provider, assumptions, semantic bounds, work limits, and expected evidence class.
- Dependency edges SHALL be one of `requires`, `validates`, `replays`, or `aggregates`.
- The coordinator SHALL reject a cycle before provider dispatch unless the authored plan names an accepted sound fixed-point rule.
- A failed `requires` edge SHALL block only its descendants.
- Unrelated siblings SHALL continue and retain their results.
- Unsupported methods and unavailable providers SHALL remain per-node dispositions.
- The coordinator SHALL NOT erase other nodes when a method is unsupported or a provider is unavailable.
- An authored aggregation rule SHALL run only after all required inputs have terminal dispositions.
- An authored aggregation rule SHALL NOT relabel a provider outcome or alter source admission.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-031-CON-1 | Plans SHALL select methods without changing clause admission, source meaning, or result strength. | Integrity | Test (TC-049) |
| FR-031-CON-2 | Each plan node SHALL have exactly one terminal disposition, including blocked, unsupported, and not-run. | Completeness | Test (TC-049) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-031-AC-1 | A complete acyclic plan dispatches ready nodes in deterministic dependency order and accounts every requested evidence output. | Test (TC-049) |
| FR-031-AC-2 | A missing reference/provider, unsupported method, or failed prerequisite affects only the owning node and typed descendants; independent sibling results remain unchanged. | Test (TC-049) |
| FR-031-AC-3 | Cycles without an accepted fixed-point rule refuse before execution, and no node or aggregation result can justify itself. | Test (TC-049) |

## Dependencies

- [FR-005](./FR-005-build-verification-plans.md) remains the bounded E02 planning foundation.
- [interface-001](../interface/interface-001-verification-method-provider.md) defines the adopted I14/I15 boundary.
