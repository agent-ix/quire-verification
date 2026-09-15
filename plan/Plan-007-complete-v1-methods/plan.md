---
id: Plan-007
title: "Complete-V1 verification methods and evidence"
type: Plan
status: active
relationships:
  - target: ix://agent-ix/quire-verification/US-005
    type: references
  - target: ix://agent-ix/quire-verification/FR-029
    type: references
  - target: ix://agent-ix/quire-verification/FR-030
    type: references
  - target: ix://agent-ix/quire-verification/FR-031
    type: references
  - target: ix://agent-ix/quire-verification/FR-032
    type: references
  - target: ix://agent-ix/quire-verification/FR-033
    type: references
  - target: ix://agent-ix/quire-verification/FR-034
    type: references
---
# Plan-007: Complete-V1 verification methods and evidence

## Scope

Implement V1-RUN-014 through V1-RUN-019 and V1-EVID-001 through
V1-EVID-006 from accepted QSpec revision `8d0fbad` through Verification
issues #34-#38 and Analyze issue #33. Preserve existing E02/Q13 work and use
shared interfaces rather than duplicating compatibility, execution, retention,
or sufficiency machinery.

This plan is a development DAG. Missing production profiles, production
evidence, performance campaigns, owner dispositions, release approvals, and
deployment ceremony are typed future ledger states and SHALL NOT block provider
implementation, local verification, or development integration.

## Requirements Summary

- **FR-029 / V1-RUN-014:** checked-domain model generation, complete boundary partitions, deterministic seeds, domain-valid shrinking, and exact native replay.
- **FR-030 / V1-RUN-015-016:** explicit execution/trace populations, ordered quantifiers, typed cross-execution expressions, total alignment/correlation, assumptions, and paired witnesses.
- **FR-031 / V1-RUN-017:** external linked plans, typed dependency edges, provider dispatch, failure isolation, and exactly one disposition per node.
- **Analyze #33 / V1-RUN-018-019:** sound hybrid enclosures and bounded canonical synthesis with independent validation.
- **FR-032-FR-034 / V1-EVID-001-006:** typed evidence graph, immutable counterexample lineage, ecosystem/corpus provenance, 176-row ledger, and attributed future decisions.
- **NFR-009:** deterministic bounded identities and complete node/vector accounting.

## Dependency Graph

```text
Task-035 (Verification #34 adoption and self-review)
  -> Task-036 (Verification #35 model generation)
  -> Task-037 (Verification #36 relational/hyperproperty)
  -> Task-038 (Verification #37 linked plan execution)

Task-036 + Task-037 + Task-038
  -> Task-039 (Analyze #33 hybrid/synthesis providers)

Task-036 + Task-037 + Task-038 + Task-039
  -> Task-040 (Verification #38 evidence graph/lineage/176-row ledger)
```

Implementation tasks consume only merged, pinned development interfaces. A
missing provider produces `unsupported`, `incomplete`, `failed`, or `unexecuted`
state at its owning node; it does not stop independent siblings or authorize a
local substitute. Shared-interface contradictions return to QSpec rather than
changing I14/I15/I18-I21 in one repository.

## Provider Contracts

- [interface-001](../../spec/interface/interface-001-verification-method-provider.md) adopts I14/I15 for capability declaration, request preparation, execution, replay, and typed method results.
- [interface-002](../../spec/interface/interface-002-qualification-evidence-authority.md) adopts I18-I21 for ecosystem/corpus identity, graph linkage, immutable lineage, ledger accounting, and attributed decisions.
- Model and relational implementations live in `quire-verification`; hybrid and synthesis live in `quire-analyze` and return the same `MethodResult` shape.
- Semantic verdicts (`proved`, `refuted`, `tested`, `observed`) remain disjoint from non-conclusive/provider dispositions (`inconclusive`, `unsupported`, `incomplete`, `failed`).

## Test Plan

| Capability | Repository test authority | Required methods |
|---|---|---|
| V1-RUN-014 | TC-047 | example/boundary, property, metamorphic determinism, unsatisfiable/refusal, replay |
| V1-RUN-015-016 | TC-048 | example, quantifier-order property, alignment/closure mutations, bounds |
| V1-RUN-017 | TC-049 | graph property, cycle/unsupported/failure mutations, sibling isolation |
| V1-RUN-018 | Analyze #33 local TC | analytic enclosure cases, guard/reset mutations, convergence/error bounds, replay |
| V1-RUN-019 | Analyze #33 local TC | canonical enumeration property, incomplete/exhaustive partition, independent-check mutation |
| V1-EVID-001-006 | TC-050-TC-052 | identity/relation mutations, 176-row/vector property checks, self-evidence, redaction/loss, lineage replay |
| Full development chain | IT-005 | exact cross-provider result intake through graph and ledger; absent production evidence represented as data |

Every Rust implementation ticket adds bare compiler-checked `ix-trace-rs`
markers and runs formatting, lint, tests, exact Rust review, and semantic gap
analysis. Heavy local work uses at most two Cargo jobs. Hosted workflows remain
manual and are not dispatched by this plan.

## Task File Mapping

| Task | Native issue | Owns | Verified by | Status |
|---|---|---|---|---|
| Task-035 | Verification #34 | US-005, FR-029-FR-034, NFR-009, interfaces 001-002, AD-003, AP-003, Plan-007 | self `/spec-review` | done |
| Task-036 | Verification #35 | FR-029, V1-RUN-014 | TC-047, IT-005-SC-01 | not_started |
| Task-037 | Verification #36 | FR-030, V1-RUN-015-016 | TC-048, IT-005-SC-02 | not_started |
| Task-038 | Verification #37 | FR-031, V1-RUN-017 | TC-049, IT-005-SC-03 | not_started |
| Task-039 | Analyze #33 | V1-RUN-018-019; provider side of interface-001 | Analyze-owned tests, IT-005-SC-04 | not_started |
| Task-040 | Verification #38 | FR-032-FR-034, V1-EVID-001-006 | TC-050-TC-052, IT-005-SC-05..07 | not_started |

## Coordination Rules

1. Use one fresh default-branch worktree and one PR per native issue.
2. Finish Task-035 review before method code; this is interface review, not production ceremony.
3. Preserve completed E02 and Q13 behavior; extend shared types or add versioned complete-V1 types without reinterpreting retained evidence.
4. Do not add a local process runner, evidence store, sufficiency engine, shared schema copy, or foreign-language executable path.
5. Production evidence and eventual human decisions remain explicit ledger state and never gate development tasks.
6. Task-040 may integrate only merged/pinned provider interfaces and results; a missing provider remains an accounted node/vector state.
