---
id: SR-222
title: "Independent re-review — quire-verification PR #22 and PR #23 remediation"
type: SpecReview
analysis: code-review
scope: "PR #22 (6d6f3fe) and PR #23 (12d1da1): remediation of SR-220 FND-001..FND-009 across spec/tests.md, Cargo workspace and lock, fuzz/, tests/verification.rs, tests/report.rs, tests/qualification_technique.rs, NFR-004, TC-022"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-025", type: reviews }
  - { target: "ix://agent-ix/quire-verification/NFR-004", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TM-001", type: references }
---
# Independent Re-review of PR #22 and PR #23 Remediation

## Summary

Re-reviewed the SR-220 findings against PR #22 head `6d6f3fe` and PR #23 head
`12d1da1`. All three blocking findings and all four mediums are resolved at the
source, not merely asserted; the one low is resolved and the remaining low is
unchanged pre-existing repository debt. Every gate was re-run locally, including
the newly added fifth gate. The gap posture is unchanged: this remediation closes
review findings and does not advance Plan-006 completion.

## Verdict

**PASS** — no open finding from SR-220 remains, and no new finding was introduced
by the remediation range.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings | reviews/2026-09-09-q13-task-027-029-independent-code-review.md |

## Remediation Verification

| SR-220 ID | Was | Verified at | How it was checked independently |
| --- | --- | --- | --- |
| FND-001 | high | `6d6f3fe` | Recounted all 45 Test Case Summary rows by type and status from the authored table. Unit 17/17/0, Integration 11/8/3, Property 13/11/2, Static 2/2/0, Benchmark 2/1/1, Overall 45/39/6. The published summary now states exactly those values, including Property 85% and Overall 87%. The Task-027 recount subtask is ticked. |
| FND-002 | high | `b32292b` | `spec/tests.md:79` reads `FR-025-AC-1..6` and `:152` reads `FR-025-AC-1..FR-025-AC-6`. `quire coverage` reports FR-025 at 3/6, so AC-6 is now a matrix-reportable criterion rather than an untracked one. |
| FND-003 | high | `b32292b` | `cargo metadata --no-deps` now lists `quire-verification-fuzz` as a workspace member; before the remediation it listed only two packages. `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` checks `quire-verification-fuzz`, and `cargo fuzz build -D --sanitizer none` compiles all three targets. `cargo fmt --all -- --check` replaces the single-package form. TC-022 now walks `fuzz` and asserts the workspace membership, the absence of `fuzz/Cargo.lock`, the inherited lints, the five-gate metric row, and the fuzz build step in the workflow. |
| FND-004 | medium | `b32292b` | `fuzz/Cargo.lock` is deleted. Root `Cargo.lock` contains exactly one `jsonschema` entry at `0.55.0`, and the fuzz build compiled `jsonschema v0.55.0` — the same closure as the candidate. `cargo deny check` passes over that single closure. |
| FND-005 | medium | `b32292b` | `fuzz/Cargo.toml` declares `[lints] workspace = true`. No `allow` appears anywhere under `fuzz/`, and clippy with denied warnings passes on the fuzz crate without an allowance. |
| FND-006 | medium | `b32292b` | `fuzz/README.md` documents the explicit seed-directory invocation, which is how cargo-fuzz consumes a directory that is not `fuzz/corpus/`. `tc_042_freezes_fuzz_inputs_and_refuses_unretained_seeds` compile-time includes all three retained seeds and asserts the README carries each target's exact seed path, so the guidance cannot drift silently. |
| FND-007 | medium | `b32292b` | NFR-004-AC-6 and the TC-022 procedure now both say `src/`, `tests/`, and `fuzz/`, matching the widened Scope. TC-022 asserts the new wording. |
| FND-008 | low | `b32292b` | `tc_037_refuses_core_task_binding_drift` now asserts the exact message `schema_violation: report core task IDs differ from its plan` in addition to the shared code, so it no longer passes on an unrelated schema refusal. |
| FND-009 | low | unchanged | `SR-009`, `SR-010`, and `SR-160` remain duplicated across `reviews/` and `spec/reviews/`. Pre-existing; not introduced or worsened. `SR-220`, `SR-221`, and this artifact are unique. |

## Gates Reproduced at `12d1da1`

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | clean |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | clean, checking `quire-verification-fuzz` |
| `cargo test --workspace --all-features --locked` | 99 first-party and 3 vendored passed; 2 VP04 lanes and 2 vendored doc tests ignored; 0 failed |
| `cargo fuzz build -D --sanitizer none` | all 3 targets compiled on pinned stable `1.94.1` |
| `cargo deny check` | advisories, bans, licenses, sources all ok |
| `quire validate` over `spec/`, `plan/`, `reviews/` | clean |
| `quire coverage --scope .` | 184/211 rows backed; 107/107 Rust candidates tagged and bound; zero status lies |

## Gap Posture

Unchanged by this remediation, and correctly so:

- Plan-006 remains 0 of 10 tasks done.
- 184 of 211 matrix rows backed; the same 22 unbacked reference rows belong to
  FR-026, FR-027, NFR-007, NFR-008, TC-043, TC-044, and TC-045.
- FR-025 remains 3 of 6 criteria backed. AC-2 still lacks the accepted differential
  method; AC-3 and AC-4 still lack typed execution/result and replay evidence.
- All ten native Quire admission rules remain owned gaps with no affirmative evidence.
- SR-219's FAIL for Plan-006 remains correct and should not be revised on the strength
  of this remediation. Compiling a fuzz target under a gate is not campaign evidence.

## Merge Sequence

Merge PR #22 first, then PR #23. Do not delete `agent-b/q13-task027-oracles` until
PR #23 has been retargeted onto `main` and the retarget is confirmed; deleting the
base branch of an open stacked PR closes that PR.
