---
id: SR-220
title: "Code review — quire-verification PR #22 and PR #23 qualification checkpoints"
type: SpecReview
analysis: code-review
scope: "PR #22 (8a9840e) and PR #23 (6743dfc): src/qualification_oracle.rs, src/qualification_technique.rs, src/qualification_profile.rs, src/contracts.rs, tests/, fuzz/, spec/, plan/Plan-006"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-024", type: reviews }
  - { target: "ix://agent-ix/quire-verification/FR-025", type: reviews }
  - { target: "ix://agent-ix/quire-verification/NFR-004", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TM-001", type: references }
---
# Independent Code Review — Task-027/028 Oracles and Task-029 Fuzz/Mutation

## Summary

Independent review of the stacked pair PR #22 (`8a9840e`) and PR #23 (`6743dfc`) against
the Rust review lane, the repository's own AGENTS.md language and gate policy, and the
authored Test Matrix. The Rust implementation and its tests are strong and the reported
counts reproduce exactly; three defects are in the assurance envelope around them — a
stale published summary, a requirement criterion missing from the matrix, and a
requirement scope widened to `fuzz/` that no test and no gate reaches.

## Verdict

**FAIL** — three high findings. All three are small, local edits; none requires new
design or an external unblock. Neither PR is clear to merge as it stands.

## Gates reproduced at `6743dfc`

| Gate | Command | Result |
| --- | --- | --- |
| Formatting | `cargo fmt --all -- --check` | pass |
| Lint | `cargo clippy --locked --all-targets --all-features -- -D warnings` | pass, no warnings |
| Tests | `cargo test --workspace --all-features --locked -j 2` | 99 first-party + 3 vendored passed; 2 VP04 lanes and 2 vendored doc tests ignored |
| Dependency policy | `cargo deny check` | advisories, bans, licenses, sources ok |
| Quire | `quire validate` over `spec/`, `plan/`, `reviews/` | pass; 273 / 52 / 47 documents |
| Coverage | `quire coverage --scope .` | 184/211 rows backed, 107/107 Rust candidates bound, **zero status lies** |
| Fuzz build | `cargo +nightly fuzz build` | all 3 targets compile, no warnings |
| Mutation spot-check | `cargo mutants 27.1.0`, `replace validate_core_bindings -> … with Ok(())`, assigned test only | 1 caught |
| Mutation spot-check | `cargo mutants 27.1.0`, `replace validate_claim_authority -> … with Ok(())`, assigned test only | 1 caught |

Every count published in both PR bodies reproduces exactly (182/210 at `8a9840e`,
184/211 at `6743dfc`). The two mutation claims spot-checked are true. `SR-219`'s FAIL
verdict is correct and honestly argued.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | PR #22 changed TC-039 from `✅ Passed` to `🚧 In progress` without recounting the Test Execution Summary: Property still reads 12 passed / 1 pending (actual 11 / 2) and Overall still reads 40 / 5 / 89% (actual 39 / 6 / 87%). | spec/tests.md:149, spec/tests.md:426-431, Task-027 checklist item at plan/Plan-006-quire-spec-production-qualification/tasks/Task-027-bind-oracle-obligations.md:30 | correct-requirement-no-evidence |
| FND-002 | high | PR #23 adds FR-025-AC-6 and a test that traces it, but leaves both matrix rows at `FR-025-AC-1..5`, so the authored matrix under-declares the requirement it claims to cover and can never report AC-6 unbacked. | spec/functional/FR-025-run-qualification-techniques.md:238, spec/tests.md:79, spec/tests.md:152, tests/qualification_technique.rs:224 | missing-requirement |
| FND-003 | high | PR #23 widens NFR-004's scope, metric row, AC-1 and AC-3 to include `fuzz/`, but TC-022 still walks only `src`, `tests`, `test`, `scripts`, and `fuzz/` is a separate Cargo workspace that no root gate and no CI step reaches. NFR-004 still reports 6/6 (100%). | spec/non-functional/NFR-004-rust-qualified-path.md:33, tests/verification.rs:2500, tests/verification.rs:2544-2552, fuzz/Cargo.toml:10, .github/workflows/ci.yml | correct-requirement-no-evidence |
| FND-004 | medium | `fuzz/Cargo.lock` resolves a different closure than the candidate under test (jsonschema 0.55.1 vs the root lock's 0.55.0, plus chrono and tempfile drift), and `cargo deny check` never sees it, so the fuzz targets exercise an unpinned, unpoliced build of the library. | fuzz/Cargo.lock, Cargo.lock, deny.toml, AGENTS.md dependency-provenance clause | wrong-requirement |
| FND-005 | medium | The fuzz crate declares its own `[workspace]` and no `[lints]`, so `unsafe_code = "forbid"`, `missing_docs` and `clippy::all`/`pedantic` from the root `[workspace.lints]` do not apply to it. It is clean today only because nothing has been added yet. | fuzz/Cargo.toml:10, Cargo.toml `[workspace.lints]` | correct-requirement-no-evidence |
| FND-006 | medium | Committed seeds live in `fuzz/seeds/<target>/`, not cargo-fuzz's default `fuzz/corpus/<target>/` (which `fuzz/.gitignore` ignores), and no README command or CI step passes the seed path, so the retained seeds are inert for any operator who does not already know. | fuzz/seeds/, fuzz/.gitignore, README.md:23-28 | missing-requirement |
| FND-007 | medium | NFR-004's Scope, metric row, AC-1 and AC-3 now name `fuzz/` while AC-6 still restricts the first-party population to `src/` and `tests/`, and TC-022 asserts that exact two-directory wording — the requirement states two different populations for itself. | spec/non-functional/NFR-004-rust-qualified-path.md:33, :82, tests/verification.rs:2535 | wrong-requirement |
| FND-008 | low | `tc_037_refuses_core_task_binding_drift` asserts only `SchemaViolation`, which `invalid_core` and genuine JSON-Schema refusals both return, so the assertion cannot say which gate refused. The PR #19 round-2 standard for exactly this situation was to assert on the refusal message. Discrimination is real today — verified by a live mutant — but is not pinned by the test. | tests/report.rs:611-630, src/report.rs:505 | correct-requirement-no-evidence |
| FND-009 | low | `SR-009`, `SR-010` and `SR-160` are each used twice across `reviews/` and `spec/reviews/`. Pre-existing, not introduced here; `quire validate` does not see it because the two trees validate separately. | reviews/26-09-07-rust-review.md, spec/reviews/ci-trace-source-base.md, reviews/2026-09-09-vp04-review-remediation-rust-review.md, spec/reviews/q13-qualification-ears-conformance.md | missing-requirement |

## What was checked and found sound

- **Oracle independence gate.** `validate_binding_dependence` refuses `independent` on
  candidate-identical executables, intersecting ancestry, any oracle/executable/comparison
  identity repeated across bindings, and non-independent expected-value origins. This is
  faithful to FR-024's "shared exact reference … SHALL NOT be classified as independent"
  and is exercised on every axis by `tc_041_refuses_false_independence`.
- **Derived-graph traversal.** The iterative visit-state walk correctly rejects
  self-derivation, absent sources, and cycles, and `tc_041_reaches_and_discriminates_the_derived_graph_gate`
  asserts on the refusal *message*, proving each case reaches the graph gate rather than
  an earlier one.
- **Native admission closure.** The schema pins `admissionResults` to exactly ten entries,
  one per rule, via `minItems`/`maxItems`/`uniqueItems` plus ten `contains` clauses with
  `minContains`/`maxContains` of 1 — all supported by the pinned `jsonschema` 0.55 under
  the declared 2020-12 dialect. `validate_configurations` then refuses any configuration
  whose guarantee carries an admission gap, and `validate_coverage` requires each gap to
  reproduce the frozen rule set exactly. A reference alone genuinely cannot admit.
- **Fixture seam.** `proofs()` mints a strict-reader proof per envelope from the envelope's
  own fields, but the expected role in both validators comes from a hardcoded field→role
  table, so the role check is not tautological; `tc_042_...references_and_catalog_methods`
  exercises the mismatch.
- **Boundary honesty.** No local runner, source rewriter, transcript parser or evidence
  store; no `todo!`, `unimplemented!`, `dbg!`, `unsafe`, or new `#[allow]`; typed
  `QualificationTechniqueUnavailable` is used for the unpublished `differential-testing`
  method rather than a private mint.

## Required before merge

1. Recount `spec/tests.md` Test Execution Summary (FND-001) and tick Task-027's own
   recount item, or leave it open with the reason.
2. Extend `spec/tests.md:79` and `:152` to `FR-025-AC-1..6` (FND-002).
3. Resolve FND-003 one way or the other — either narrow NFR-004 back off `fuzz/` and say
   why, or extend TC-022's walk, add a `fuzz` build/lint/deny step to
   `.github/workflows/ci.yml`, and update AC-6 (FND-007) and README's command list
   (FND-006) with it. A 100% NFR-004 coverage row that no gate backs is the
   `EC-027` failure mode this repository already named for itself.

FND-004 through FND-009 are recorded for disposition and do not block the merge on their
own; FND-004 should be answered before any fuzz observation is offered as qualification
evidence.

## Merge sequence

`#22` then `#23`. `#23` is based on `agent-b/q13-task027-oracles`; do not delete that
branch until `#23` is merged or retargeted, or GitHub will close `#23`.
