---
id: IT-003
title: "Native parser portfolio execution"
type: IT
relationships:
  - target: "ix://agent-ix/quire-verification/FR-019"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-020"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-021"
    type: "verifies"
---
# IT-003: Native parser portfolio execution

## Objective

Verify the end-to-end boundary from a portable verification plan through the
real Rust `quire-spec` parse and format commands into exact technique results,
conservative assessment, phase-separated measurements, and a report-bound manual
disposition. This demonstrates one bounded domain portfolio without promoting
syntax checks into model or evaluator claims.

## Target Integration

The system under test is the Rust `quire-verification` planner, native runner,
evidence assessor, and report writer. The external producer is the real
`quire-spec-language` Rust `quire-spec` CLI at an exact reviewed revision. The
evidence handoff is a retained versioned report whose owner and sufficiency fields
remain Quoin-owned.

## Preconditions

- VP02 and VP03 have passed their qualification gates at exact revisions.
- The shared-reference packet is accepted for every role used by the portfolio,
  including review procedure and review disposition.
- The `quire-spec-language` candidate has passed its own Rust review and clean CI.
- Exact valid and single-mutation source fixtures have reviewed redistribution terms.
- The verification owner and the report-bound review procedure are identified.
- No network service, production environment, model linker, evaluator, generated backend, or fault injector is required.

## Inputs

- Exact candidate and dependency-closure references for both Rust repositories.
- The reviewed valid native source and its one declared syntax mutation.
- Exact capabilities, policy, plan, runner requests, evidence obligations, freshness bounds, and measurement procedure.
- A fresh temporary execution directory and the prescribed five-repetition order.

## Test Procedure

1. Verify every candidate, fixture, role, environment, and procedure reference before planning.
   - IT-003-SC-01: every reference is supported, exact, digest-valid, and role-correct.
2. Build the portfolio plan with direct-parse, parse-format-reparse, and manual-disposition work.
   - IT-003-SC-02: both automated tasks are ready, the manual task is separately represented, and no input variant is guessed or collapsed.
3. Exercise the native process time, output, argument, environment, path, status, and protocol boundaries.
   - IT-003-SC-03: each boundary produces the NFR-006 result and no shell, orphan process, partial success, or logical claim.
4. Execute the valid source through the real `quire-spec parse` command and the real parse-format-reparse sequence.
   - IT-003-SC-04: both automated baseline results are completed and satisfied with exact native-artifact and observation references.
5. Execute the one declared syntax mutation through the direct-parse technique.
   - IT-003-SC-05: the expected located refusal becomes completed and violated; a crash or malformed protocol becomes failed and not-evaluated.
6. Rerun the plan after removing one required native prerequisite.
   - IT-003-SC-06: the affected task is blocked with actionable setup work and no fabricated result.
7. Assess an otherwise exact retained result at its expiry and one representable instant later.
   - IT-003-SC-07: the result is fresh at expiry, stale afterward, and all independent findings remain present.
8. Collect the five prescribed measurement repetitions and recompute each phase summary from raw observations.
   - IT-003-SC-08: setup is separate, invalid observations are retained and excluded, and every summary recomputes exactly.
9. Seal the automated report core, then bind one verification-owner disposition to that exact core and inspect the final report claims.
   - IT-003-SC-09: the disposition refers to the pre-disposition core digest, exactly one `accept`, `revise`, or `reject` value is present, Quoin still owns sufficiency, and the bounded claim text excludes model, evaluator, production, scale, ROI, and broad-effectiveness conclusions.

## Expected Results

The integration produces a reproducible report with two complementary native
baseline results, the declared adverse result, an explicit missing-prerequisite
case, an exact stale-result case, phase-separated raw measurements, and one
human disposition bound to the sealed automated core. Any unsupported reference, wrong role, missing
prerequisite, unexpected native failure, ambiguity, stale evidence, invalid
measurement, or absent disposition remains visible and cannot be converted into
support.

## Metadata

- Priority: P0
- Target Integration: `agent-ix/quire-spec-language` native parser and Quoin-owned evidence handoff
- Automation: Automated native execution plus one required human disposition

## Dependencies

- Verifies [FR-019](../functional/FR-019-execute-bounded-parser-portfolio.md),
  [FR-020](../functional/FR-020-record-portfolio-effort.md), and
  [FR-021](../functional/FR-021-bind-portfolio-report.md).
- Requires accepted VP02 and VP03 qualification evidence.

## Notes

The seeded mutation demonstrates only the declared syntax defect. A successful
parse or roundtrip is not a model-linking, evaluation, generated-backend,
semantic-correctness, or general parser-effectiveness result.

## Traceability

This integration test is the VP04 domain execution gate and the collection site
for [MP-001](../assurance/MP-001-parser-portfolio-effort.md).
