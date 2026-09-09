---
id: SR-119
title: "Gap analysis — Plan-003 after draft-2 role inputs"
type: SpecReview
analysis: gap-analysis
scope: "plan/Plan-003-vp02-native-planning/, spec/tests.md, spec/functional/FR-002-derive-runtime-from-schema.md, spec/functional/FR-013-emit-actionable-prerequisite-dag.md, spec/functional/FR-014-construct-native-runner-requests.md, spec/functional/FR-015-preserve-role-specific-inputs.md, spec/functional/FR-016-read-shared-references-strictly.md, spec/non-functional/NFR-002-single-contract-source.md, contracts/, src/, tests/ at 70f9d6b"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/Plan-003"
    type: reviews
  - target: "ix://agent-ix/quire-verification/TM-001"
    type: references
  - target: "ix://agent-ix/quire-verification/Task-013"
    type: reviews
---

## Summary

Applied QUOIN `/gap-analysis`, including intent-to-test-to-code semantic review,
after exact Rust review SR-118 at `70f9d6b`. Task-013's implemented boundary is
complete and traced: FR-002 and NFR-002 are fully backed, while FR-015 is 4/5
because its runner-eligibility integration criterion deliberately remains in
Task-014. Plan-003 remains incomplete until Tasks 014 and 015 close.

## Verdict

**FAIL** — Plan-003 still has three task records outside `done` and five
unbacked FR-014/FR-015 acceptance-criterion rows; this plan-level verdict finds
no missing Task-013 implementation or test.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Task-013 remains `active` at the audited revision even though its reviewed code and TC-028 evidence pass; the gap workflow is read-only and cannot reconcile that task state. | Task-013, SR-118 |
| FND-002 | high | Task-014 and Task-015 remain blocked on their declared predecessor chain, so only two of five Plan-003 task records are `done` at the audited revision. | Task-014, Task-015 |
| FND-003 | high | Exact native runner construction and the heterogeneous runner-eligibility integration remain unbacked: FR-014 is 0/4 and FR-015-AC-5 remains open under TC-027/TC-030. | FR-014-AC-1..4, FR-015-AC-5, TC-027, TC-030 |
| FND-004 | medium | The active traceability declaration expects a `Status` column, while the validated TestMatrix schema requires the authored `Coverage Status` header; Quire therefore cannot classify complete-but-unbacked rows in the functional coverage table. | spec/tests.md:43, spec-artifacts-process manifest:traceability.functional-coverage |
| FND-005 | low | Quire reports one untracked constraint tag inside the byte-identical vendored `ix-trace-rs` self-test. The module excludes `vendor/**` from first-party source ownership, and the finding is outside Task-013. | vendor/ix-trace-rs/src/lib.rs::tests::the_kind_vocabulary_is_deliberately_not_checked |

## Coverage

- Target: `plan/Plan-003-vp02-native-planning/`; specification root `spec/`;
  matrix `spec/tests.md` (`TM-001`); identity prefix
  `ix://agent-ix/quire-verification`; implementation in `contracts/` and `src/`;
  tests in `tests/`.
- Reconciliation: `quire coverage` 0.31.0 / engine `ca7362d4` at `70f9d6b`.
- Tasks done: 2 / 5 at the audited revision. Task-011 and Task-012 are done;
  Task-013 is active and Tasks 014–015 are blocked. Plan checkboxes agree with
  those authored task states.
- Rows backed by a tagged test: 125 / 162 repository-wide. Within the affected
  requirements, FR-002 is 4/4, FR-013 is 5/5, FR-015 is 4/5, FR-016 is 7/7,
  and NFR-002 is 3/3. TC-028 is backed and TC-030 remains pending.
- Evidence symbols: all 45 first-party Rust candidates are tagged and bound.
  No status lie is reported, subject to FND-004's module/header limitation.
  The only untracked symbol is the excluded vendored constraint tag in FND-005.
- Reverse ownership: draft-2 generation and validators map to FR-002/NFR-002;
  opaque exact-JCS membership and the five role variants map to FR-015/FR-016;
  removal of fabricated setup inputs maps to FR-013; evidence-envelope identity
  selection maps to FR-006; and local fixture/report handling maps to FR-003 and
  FR-009. No changed public or internal behavior lacks an owning requirement.
- Stub scan: no `todo!`, `unimplemented!`, TODO/FIXME/XXX marker, `dbg!`, hollow
  source body, assertion-free Task-013 test, ignored test, or new unsafe block
  was found.
- Semantic review: ran over FR-002-AC-4, FR-013-AC-4, FR-015-AC-1 through
  AC-4, and NFR-002-AC-3. TC-028 executes generated/runtime schema resolution,
  opaque-reader membership, each accepted role, exact input retention, every
  required-field deletion, both version layers, wrong roles, absent validation,
  equal relational identities, duplicate dispositions, cross-variant and
  unknown fields, and draft-1 refusal. TC-026 executes the setup-task separation.
  The tests reach public Rust APIs and agree with the reviewed requirements.
- Execution evidence: exact Rust review SR-118 records passing fixture,
  formatting, denied-warning Clippy, complete Rust test, dependency-policy, and
  Quire validation gates for candidate `e66f554`; the review-only commit changes
  no executable source.
