---
id: IT-004
title: "Quire-spec production qualification campaign"
type: IT
relationships:
  - target: "ix://agent-ix/quire-verification/FR-022"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-023"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-024"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-025"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-026"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-027"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-007"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-008"
    type: "verifies"
---
# IT-004: Quire-spec production qualification campaign

## Objective

Verify one exact `quire-spec` production profile from owner-authored claim through
representative corpus, independent oracles, discriminating techniques, real caller
integration, performance and reproduction evidence, immutable report sealing, and
external disposition without expanding the reviewed claim.

## Target Integration

The system under test is the Rust `quire-verification` qualification path. External
dependencies are exact accepted Agent A shared-reference and parser/compiler
artifacts, the real production caller, bounded independent oracle tools, Agent C's
versioned Engineering Assurance handoff, and Quoin evidence retention. Each remains
owned by its producer and communicates through a reviewed structured boundary.

## Preconditions

- The owner has selected one production caller, command surface, population,
  environment, semantic guarantee set, resource budget, and exclusions.
- Every required FS02/FS05, parser/compiler, assurance, Quoin, and caller interface
  has an accepted exact revision and compatible consumer case.
- Corpus and oracle artifacts have recorded provenance, rights, independence, and
  retention authorization.
- Local execution limits and the manual-only hosted-workflow policy are active.

## Inputs

- Exact profile, candidate, dependency, corpus, oracle, environment, runner,
  measurement, review-procedure, and production-caller references.
- Required example, property, fuzz, metamorphic, mutation, differential, integration,
  performance, reproduction, Rust-review, and gap-analysis obligations.

## Test Procedure

1. Freeze and validate the production qualification profile.
   - IT-004-SC-01: every claim dimension and exclusion is closed, exact, and JCS-identified.
2. Validate all external interface revisions and corpus artifacts.
   - IT-004-SC-02: every required interface is accepted and every corpus class has exact retained inputs or an explicit blocker.
3. Bind independent oracles and execute the declared evidence techniques.
   - IT-004-SC-03: every result preserves its dependence, budget, stopping rule, outcome, artifacts, and counterexamples.
4. Execute the real caller-to-parser-to-result-to-assurance path.
   - IT-004-SC-04: no test-only transport or copied producer semantics substitutes for a declared production boundary.
5. Collect performance and reproduction evidence.
   - IT-004-SC-05: every owner threshold is evaluated from retained raw observations and every deterministic result replays or records an explained difference.
6. Apply exact Rust review and semantic QUOIN gap analysis.
   - IT-004-SC-06: every in-scope finding is resolved and every requirement/test/code binding reconciles.
7. Seal the complete core and obtain the external owner disposition.
   - IT-004-SC-07: exactly one disposition binds the unchanged core and repeats the exact bounded claim.

## Expected Results

The campaign produces either a final report for one exact accepted, revised, or
rejected profile or an incomplete report naming every unresolved dependency,
evidence gap, disagreement, threshold, review finding, or reproduction difference.
No partial campaign becomes broad qualification.

## Metadata

- Priority: P1
- Target Integration: exact production caller, `quire-spec`, portable verification contracts, Engineering Assurance, and Quoin
- Automation: Rust automation plus exact Rust review, semantic gap analysis, and one external human disposition

## Dependencies

The campaign verifies FR-022 through FR-027 and NFR-007/NFR-008. It consumes exact
external interfaces only after their existing owners accept them.

## Notes

Agents A, C, D, and contract-agent-c may continue their own work concurrently. This
campaign reads their accepted outputs and does not edit their repositories or infer
readiness from an active branch.

## Traceability

The campaign validates [StR-002](../stakeholder/StR-002-production-qualification.md)
under [AP-002](../assurance/AP-002-quire-spec-production-qualification.md).

