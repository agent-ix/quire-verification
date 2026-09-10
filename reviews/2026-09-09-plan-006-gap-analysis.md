---
id: SR-219
title: "Gap analysis — Plan-006 production qualification"
type: SpecReview
analysis: gap-analysis
scope: "plan/Plan-006-quire-spec-production-qualification/, spec/tests.md, src/, tests/, fuzz/"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/Plan-006", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TM-001", type: references }
---
# Gap Analysis of Plan-006 Production Qualification

## Summary

QUOIN gap analysis audited the full Plan-006 bundle, matrix, Rust source, integration
tests, and fuzz targets at `a47d4f7`. The implemented Task-029 portable checkpoint is
specified and traced, but the production-qualification plan is incomplete and its exact
external evidence dependencies remain open.

## Verdict

**FAIL — expected active-plan result.** None of the ten Plan-006 tasks is done, 22 matrix
reference rows remain unbacked, FR-025 is backed for only three of six criteria, and the
accepted execution, differential, native-profile, production-chain, performance,
reproduction, and disposition evidence does not yet exist.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Plan-006 has 0 of 10 tasks done: Tasks 025, 026, 028, and 029 are blocked; Task-027 is in progress; Tasks 030 through 034 are not started. | Plan-006, Task-025..Task-034 |
| FND-002 | high | The matrix has 184 of 211 rows backed. Its 22 wholly unbacked reference rows belong to FR-026, FR-027, NFR-007, NFR-008, TC-043, TC-044, TC-045, and their acceptance criteria. | FR-026, FR-027, NFR-007, NFR-008, TC-043..TC-045 |
| FND-003 | high | FR-025 is 3 of 6 criteria backed. AC-2 lacks the accepted differential method, while AC-3 and AC-4 lack typed execution/result and replay evidence. Task-029 cannot complete TC-042 by target compilation or local mutation diagnostics alone. | FR-025-AC-2..FR-025-AC-4, Task-028, Task-029 |
| FND-004 | high | Engineering Assurance #34 has not published the accepted Rust producer-execution/result interface, blocking property, fuzz, mutation, replay, production-chain, and performance observations. No repository-local runner or evidence store exists. | engineering-assurance#34, Task-028..Task-032 |
| FND-005 | high | Native Quire is the sole formal-clause source, but affirmative evidence exists for none of the ten carried admission rules. The composed profile and temporal bridge remain under redesign, so all ten rules are retained as owned gaps rather than inferred from state-profile work. | quire-specification#10, quire-contract-ir#64, FR-022-AC-5, FR-025-AC-1 |
| FND-006 | high | The accepted Quoin catalog still lacks `differential-testing`, so the Rust planner correctly refuses that family and cannot complete the differential portion of FR-025. | spec-artifacts-process#86, FR-025-AC-2 |
| FND-007 | medium | Bare Quire coverage reports one vendored `ix-trace-rs` symbol as untracked even though the repository module excludes `vendor/**`; the module cannot currently execute alone with its traceability model. | `.ix/modules/quire-verification/manifest.yaml`, `vendor/ix-trace-rs/src/lib.rs` |

## Coverage

- Reconciliation: `quire coverage` with CLI 0.31.0 and engine
  `ca7362d4dacecb96f01d74d1d971327118c25917`.
- Tasks done: 0 / 10.
- Rows backed by a tagged test: 184 / 211; 22 wholly unbacked reference rows; zero
  status lies.
- FR-025 acceptance criteria backed: 3 / 6. AC-1, AC-5, and AC-6 carry direct Rust
  bindings; AC-2 through AC-4 remain incomplete.
- Rust binding census: 107 / 107 discovered candidates tagged and bound. The generated
  underscored property test is bound by its bare attribute rather than its function name.
- Reverse ownership: the Task-029 fuzz wrappers are owned by FR-016, NFR-005, NFR-006,
  NFR-004, and FR-025; the mutation manifest and new discriminating tests are owned by
  FR-025 and their named TC rows. The reviewed slice adds no public production API.
- Stub inspection found no `todo!`, `unimplemented!`, placeholder success, debug output,
  unsafe block, unbounded loop, local runner, source rewriter, transcript parser, or
  evidence store in the Task-029 source slice.
- Semantic review: skipped. The prior all-lens `/spec-review`, `/spec-fuzz`, exact Rust
  review, and eight live mutants supply the targeted semantic checks for this checkpoint.
