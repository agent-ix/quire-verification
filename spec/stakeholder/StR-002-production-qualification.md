---
id: StR-002
title: "Verification owners need bounded production qualification"
type: StR
relationships:
  - target: "ix://agent-ix/quire-verification/FR-022"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-023"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-024"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-025"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-026"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/FR-027"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/NFR-007"
    type: "satisfied_by"
  - target: "ix://agent-ix/quire-verification/NFR-008"
    type: "satisfied_by"
---
# StR-002: Verification owners need bounded production qualification

## Stakeholder Need

Verification owners require the qualification system to admit a `quire-spec` candidate
to a production path only through affirmative evidence for one explicit input
population, command surface, environment, resource budget, and claim boundary.

## Rationale

The existing VP04 result proves a narrow syntax fixture pair. Treating that result
as broad parser or production qualification would hide untested grammar, malformed
inputs, oracle disagreement, resource behavior, integration failures, and
environmental differences. A bounded qualification makes those limits reviewable
and preserves a path to expanding the evidence later.

## Validation Criteria

| ID | Criteria | Validation |
|----|----------|------------|
| StR-002-VC-1 | The owner can inspect one immutable qualification report that names every admitted and excluded command, input class, environment, guarantee, resource limit, oracle, and dependency. | Demonstration (IT-004) |
| StR-002-VC-2 | Evidence gaps, oracle disagreements, unmeasured budgets, and unsupported environments prevent an acceptance disposition rather than disappearing behind a passing aggregate. | Demonstration (IT-004) |
| StR-002-VC-3 | The final decision is separately authored and bound to the exact evidence core without expanding its claims. | Demonstration (TC-045) |

## Stakeholders

The verification owner decides whether the evidence is sufficient for the stated
boundary. Parser maintainers, production integrators, Quoin evidence owners, and
Engineering Assurance maintainers provide or consume identified artifacts without
inheriting the verification owner's decision authority.

## Context and Assumptions

The current candidate is Rust. FS02 and FS05 identities, Agent A parser/compiler outputs,
and Agent C assurance-chain interfaces may advance independently. Qualification
uses only exact reviewed revisions and does not modify those producers.

## Stakeholder Constraints (Contextual)

First-party qualified and hot-path code remains Rust, Rust acceptance evidence uses
bare compiler-checked `ix-trace-rs` markers, GitHub workflows remain manual-dispatch
only, and local qualification uses bounded concurrency on the shared workstation.

## Dependencies

The need depends on an owner-selected production boundary and reviewed exact
producer contracts. FR-022 through FR-027 define its functional path; NFR-007 and
NFR-008 constrain the performance evidence and reproducibility required for disposition.

## Priority and Risk (Informative)

Priority is P1 after the current Rust groundwork. The principal risk is false
qualification from evidence that is internally consistent but too narrow,
correlated, stale, or detached from the production path.

## Notes (Informative)

Qualification can proceed in bounded profiles. A later profile may expand the
population without rewriting the identity or decision attached to an earlier one.

## Traceability

The user story [US-004](../usecase/US-004-qualify-quire-spec.md) refines this need.
