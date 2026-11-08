---
id: IT-005
title: "Complete-V1 method and evidence development chain"
type: IT
relationships:
  - target: "ix://agent-ix/quire-verification/FR-029"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-030"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-031"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-032"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-033"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-034"
    type: "verifies"
---
# IT-005: Complete-V1 method and evidence development chain

## Objective

Verify the development integration from an accepted checked package and linked
plan through independent method providers, typed results, evidence graph,
counterexample lineage, and all 176 qualification-ledger primary rows without
requiring production evidence or a release disposition.

## Target Integration

The system under test is `quire-verification`. External dependencies are the
accepted checked-package/native-runtime interfaces, `quire-analyze` method
provider interface, accepted 176-row inventory/corpus shapes, and Quoin evidence
intake boundary. Tests use local fixtures or explicit unavailable states;
they do not deploy or invoke a production system.

## Preconditions

Each unavailable provider or future production artifact is declared
as an explicit input state rather than a test skip or task blocker.

## Inputs

- Exact checked packages, domains, traces, alignments, plans, provider manifests, and bounds.
- Model, relational, hybrid, and synthesis method-result fixtures with positive and adverse outcomes.
- Accepted 176-row inventory/corpus fixture, ecosystem lock, evidence graph, lineage, and decision variants.

## Test Procedure

1. Execute model boundary generation, unsatisfiable-domain refusal, shrinking, and exact native replay.
   - IT-005-SC-01: every case and retained shrink satisfies FR-029, and no rejected precondition becomes a pass.
2. Execute relational and finite-trace hyperproperties with reversed quantifier prefixes and mutated alignments.
   - IT-005-SC-02: exact trace identities and order are preserved; missing closure/alignment is non-conclusive.
3. Dispatch an acyclic plan, then inject a missing provider, failed prerequisite, unsupported method, and cycle.
   - IT-005-SC-03: every node has exactly one disposition and independent siblings remain unchanged.
4. Consume valid and malformed hybrid/synthesis results through the same method-provider boundary.
   - IT-005-SC-04: heuristic candidates, failed convergence, incomplete search, and provider failure cannot become proof.
5. Build evidence graphs and counterexample revisions, then inject stale, cross-kind, circular, redacted, and verdict-changing variants.
   - IT-005-SC-05: invalid discharge is refused without deleting immutable evidence or sibling results.
6. Account the exact accepted inventory and mutate one row, vector, ecosystem pin, evidence path, waiver, and decision at a time.
   - IT-005-SC-06: all 176 rows remain explicit; only the affected future qualification claim changes.
7. Repeat development execution with production evidence and final disposition absent.
   - IT-005-SC-07: local provider tests and integration complete while the ledger reports those future artifacts as planned or unexecuted.

## Expected Results

All seven success criteria pass. Semantic verdicts, provider execution states,
evidence limitations, and eventual decision state remain separate. The test
produces no production-readiness, deployment, release, or certification claim.

## Metadata

- Priority: P0
- Target Integration: QSpec I04/I10/I13-I15/I18-I21 development boundaries
- Automation: Automated Rust tests with fixtures

## Dependencies

The hybrid/synthesis provider fixture is owned by `ix://agent-ix/quire-analyze/issues/33`.
The final cross-lane integrated execution campaign is outside this repository.

## Notes

Missing future production evidence is intentionally tested as data. It is not a
precondition for the implementation or execution of this integration test.

## Traceability

This test covers [FR-029](../functional/FR-029-generate-model-tests.md) through
[FR-034](../functional/FR-034-qualify-complete-v1-ledger.md) and adopts QSpec
TC-212 through TC-216 and TC-230 through TC-232 at the repository boundary.

