---
id: SR-187
title: "Gap analysis — Plan-006 Task-027 checkpoint"
type: SpecReview
analysis: gap-analysis
scope: "plan/Plan-006-quire-spec-production-qualification/, spec/tests.md, src/, tests/ at quire-verification@9384d8f"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/Plan-006", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TM-001", type: references }
---
# Gap analysis — Plan-006 Task-027 checkpoint

## Summary

This QUOIN gap analysis checked whole-plan status and then reviewed FR-024 intent,
TC-041, and the portable Rust oracle implementation semantically. The portable manifest
slice has requirement ownership and discriminating tests, while Task-027 and the
production qualification plan remain incomplete on the Engineering Assurance execution
seam and later campaign work.

## Verdict

**FAIL** — Plan-006 has incomplete P0 critical-path tasks, 30 mechanically unbacked
matrix rows, and no implementation or tagged evidence for FR-024-AC-3. This is an honest
checkpoint verdict and does not identify an open defect in the portable manifest slice.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Plan-006 has 0 of 10 tasks done: Task-025 and Task-026 remain blocked on production inputs, Task-027 is in progress, and Tasks 028 through 034 are not started on the P0 critical path. | Task-025..Task-034, Plan-006 |
| FND-002 | high | Quire reports 179 of 209 matrix rows backed. The remaining 30 include FR-024-AC-3 plus post-oracle FR-025 through FR-027, NFR-007/008, TC-043 through TC-045, and IT-004 work. | TM-001, FR-024-AC-3, FR-025..FR-027, NFR-007..NFR-008 |
| FND-003 | high | Semantic review confirms no test or implementation consumes an accepted Engineering Assurance producer result, preserves its typed failure states, or binds agreeing/disagreeing observations. Current EA ADR-0002/NFR-004 also allocate producer execution outside the package; issue `agent-ix/engineering-assurance#11` owns reconciliation. | FR-024-AC-3..4, TC-041 steps 4–5, Task-027 |
| FND-004 | medium | The active traceability model still cannot classify functional-row status because it requests `Status` while the validated table owns `Coverage Status`; status-lie detection is therefore unavailable for that table pending quire-rs#409. | `spec/tests.md`, `.ix/modules/quire-verification/manifest.yaml` |
| FND-005 | low | The sole untracked trace remains the pre-existing `FR-003-CON-1` marker inside vendored `ix-trace-rs`; every one of 93 discovered Rust candidates is otherwise tagged and bound. | `vendor/ix-trace-rs/src/lib.rs::tests::the_kind_vocabulary_is_deliberately_not_checked` |
| FND-006 | low | `Coverage inspection` and `Mutation Test` remain deliberately specific evidence-method terms outside the current Quoin catalogue; replacing them with a weaker catalogue term would change their authored obligations. | NFR-004-M-4..5, NFR-008-M-3, NFR-008-AC-3 |

## Coverage

- Target bundle: `plan/Plan-006-quire-spec-production-qualification/`
- Spec root: `spec/`; matrix: `spec/tests.md` (`TM-001`)
- Identity prefix: `ix://agent-ix/quire-verification`
- Source/test trees: `src/`, `tests/`
- Reconciliation: `quire coverage` 0.31.0 / engine 0.46.0 with local module `quire-verification-local` 0.1.0
- Tasks done: 0 / 10; 2 blocked; 1 in progress; 7 not started
- Rows backed by a tagged test: 179 / 209; FR-024 acceptance criteria: 3 / 4
- Rust binding census: 93 / 93 candidates tagged and bound
- Inventoried Task-027 public behaviors: 8; untraced behaviors: 0
- Source stubs: 0; test stubs: 0
- Status lies reported: 0, subject to FND-004's unavailable functional-row status classification
- Semantic review: ran over FR-024, TC-041, `src/qualification_oracle.rs`, and `tests/qualification_oracle.rs`; axes for AC-1, AC-2, and the portable part of AC-4 agree, while FND-003 records the absent AC-3/result-integration path
- TC-041 remains `Planned`: ten passing portable tests do not complete its Engineering Assurance execution and comparison steps
