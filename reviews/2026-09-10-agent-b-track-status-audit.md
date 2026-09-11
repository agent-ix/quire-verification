---
id: SR-233
title: "Agent B track status audit — Plan-006 Tasks 025-034 and the paused native-Quire bridge"
type: SpecReview
analysis: code-review
scope: "quire-verification at bcbe960 on agent-b/plan006-ea34-consumer (unpushed), origin/main at e937f15, all eight agent-b worktrees, Plan-006 Task-025..Task-034, and the external gates engineering-assurance#34/PR46, spec-artifacts-process#86, quire-specification#10, quire-contract-ir#64"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/Plan-006", type: reviews }
  - { target: "ix://agent-ix/quire-verification/FR-024", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TC-041", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-031", type: references }
---
# Agent B Track Status Audit

## Summary

Independent audit of the whole Agent B track: `origin/main` at `e937f15` (PRs #14
through #30 merged) and the active `agent-b/plan006-ea34-consumer` branch at `bcbe960`,
which is eight commits ahead of main, unpushed, and has no pull request. Every claim was
checked against git history, GitHub issue/PR state, the spec matrix, `quire coverage`,
and the six CI gates run locally. Plan-006 remains 0 of 10 tasks done. Task-027 is
genuinely in progress with a working Engineering Assurance consumer, but the branch
fails the test gate on a dependency-induced RFC 8785 regression, and the spec matrix
already publishes the affected rows as passing.

## Verdict

**FAIL** — the candidate branch is red at `cargo test --workspace --all-features
--locked` (FND-001) and its test matrix simultaneously reports zero failures
(FND-002). Neither is a merge-ready state. The underlying provider defect is correctly
escalated and is not Agent B's to fix.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The `engineering-assurance` git dependency enables `serde_json/arbitrary_precision`, and Cargo feature unification breaks the repository's retained RFC 8785 canonical vectors; `tc_031` fails at `bcbe960` in the default and all-features builds alike. | tests/shared_reference.rs:757, Cargo.toml:51, TC-031 |
| FND-002 | high | `spec/tests.md` publishes FR-024, FR-024-CON-1..2 and TC-041 as passing and reports `Failed 0` in the Test Execution Summary, while TC-031 is still carried as `✅ Implemented` and its test is failing in the same tree. | spec/tests.md:79, spec/tests.md:103, spec/tests.md:142, spec/tests.md:152, spec/tests.md:426-432 |
| FND-003 | medium | Eight `#[trace]` tags in the new execution test bind to nothing: `TC-126` (five sites) is an Engineering Assurance identifier with no local declaration, and `FR-024-CON-1`/`FR-024-CON-2` (three sites) match no declared row. `quire coverage` reports all eight as unresolved. | tests/qualification_oracle_execution.rs:689, :768, :832, :877, :908, :944 |
| FND-004 | medium | The pinned toolchain moved from 1.94.0 to 1.98.1 across a new `rust-toolchain.toml`, `Cargo.toml` `rust-version`, and both CI steps, with no rationale recorded in the plan log, the spec, or repository guidance, and no statement of what required it. | rust-toolchain.toml:2, Cargo.toml:5, .github/workflows/ci.yml:15-16 |
| FND-005 | low | `build.rs` recovers the Engineering Assurance revision by string-splitting `Cargo.toml` on `rev = "`. A table-form dependency, a reordered key, or a second `rev =` on the line fails the build with "revision is missing" rather than naming the real cause. | build.rs:63-82 |
| FND-006 | low | Eight commits carrying the only copy of the consumer implementation, its schema, and its reviews exist solely in one local worktree: the branch is unpushed and has no pull request. | agent-b/plan006-ea34-consumer @ bcbe960 |
| FND-007 | low | `SR-009`, `SR-010` and `SR-160` remain duplicated across `reviews/` and `spec/reviews/`. Unchanged pre-existing debt, carried forward from SR-220 FND-009; the nine new `SR-224..SR-232` ids are unique. | reviews/, spec/reviews/ |

## Gates Reproduced at bcbe960

All six CI steps were run locally in
`/home/peter/dev/worktrees/quire-verification-agent-b-ea34-consumer`. No hosted CI was
dispatched.

| Gate | Result |
| --- | --- |
| `cargo run --locked --bin validate-fixtures` | pass — 4 fixtures, 52 public contract instances |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | pass |
| `cargo test --workspace --all-features --locked` | **FAIL** — 107 passed, 1 failed (`tc_031`), 4 ignored |
| `cargo fuzz build -D --sanitizer none` | pass — all three targets |
| `cargo deny check` | pass — advisories, bans, licenses, sources ok |

`quire validate` is clean. `quire coverage` reports 185 of 211 rows backed (88%) and a
113/113 Rust binding census, with the eight unresolved tags of FND-003 reported
separately.

Two of these gates were failing when this audit began and were repaired by the
implementing agent while it ran: `cargo deny check sources` rejected the new git source
until `deny.toml` gained a named `allow-git` entry, and clippy reported three
`manual_let_else` errors in the new fixture. Both are confirmed repaired at `bcbe960`.

## Task Status — Plan-006

Tasks done: **0 of 10**. No task status in the plan overstates its evidence.

| Task | Status | Determination |
| --- | --- | --- |
| Task-025 freeze qualification profile | blocked | Correct. Awaits owner-supplied exact executable, dependency, caller, environment and accepted-interface inputs. Issue #15 open. |
| Task-026 retain qualification corpus | blocked | Correct. Depends on Task-025; PR #19 landed the Rust boundary only, not a production corpus. |
| Task-027 bind oracle obligations | in_progress | Correct and the only active lane. Implementation exists and its own tests pass; blocked from completion by FND-001. Issue #20 open. |
| Task-028 property and differential evidence | blocked | Correct. Depends on Task-027 plus `spec-artifacts-process#86`, which still has not published `differential-testing`. Issue #21 open. |
| Task-029 fuzz and mutation discrimination | blocked | Correct. PRs #22/#23 landed the portable checkpoint and local mutation diagnostics; promotion to accepted evidence depends on Task-028. |
| Task-030 execute production chain | not_started | Correct. Depends on Task-029. |
| Task-031 measure production budget | not_started | Correct. Depends on Task-030. |
| Task-032 reproduce artifacts and results | not_started | Correct. Depends on Task-031. |
| Task-033 seal disposition report | not_started | Correct. Depends on Task-032. |
| Task-034 qualification review gate | not_started | Correct. Depends on Task-033. |

The 26 unbacked matrix rows are exactly the downstream scope: FR-026, FR-027, NFR-007,
NFR-008, TC-043, TC-044, TC-045 and their acceptance criteria. SR-219's FAIL verdict
stands unchanged and must not be revised on the strength of the Task-027 work.

## External Dependencies and Owners

| Gate | Owner | State |
| --- | --- | --- |
| `engineering-assurance#34` / PR #46 | Agent C | PR open, unreviewed, head `f640f4c`, diverged from EA main (4 ahead, 1 behind). Carries the FND-001 regression; blocker returned at `issuecomment-5629327354`. |
| `spec-artifacts-process#86` | owner | Open. `differential-testing` still absent from the accepted catalog, so FR-025-AC-2 stays unreachable. |
| `quire-specification#10` | owner | **Closed 2026-09-11T00:35Z.** The native-language redesign is unblocked by FCD PR #97 `baseline-1-2.md` at `d6ac501`. |
| `quire-contract-ir#64` | Agent B | Open. Temporal lowering to the TL representation and evaluator; this is the paused bridge work, and its stated blocker has now cleared. |

Ten native Quire admission rules remain owned gaps with no affirmative evidence. The
production profile and corpus remain externally blocked, and no production
qualification is claimed anywhere in the reviewed tree.

## Consumer Boundary Assessment

The Task-027 implementation is sound where it is reachable. The consumer accepts one
bounded, strictly parsed, schema-derived response from Engineering Assurance's sealed
output descriptor, re-decodes completed observations from the retained bytes rather than
trusting a caller-held value, and refuses interface, request, result and retained
observation drift. Its sixteen tests across `qualification_oracle` and
`qualification_oracle_execution` all pass. The new module adds no `unwrap`, `expect`,
`panic`, `unsafe`, unchecked cast, lint allowance, or test-only production branch, and
every public item is documented.

The implementing agent's own review SR-232 records six findings, five resolved and one
open provider regression. Every claim checked in this audit reproduced. The Test
Execution Summary recount to Integration 11/9/2 and Overall 45/40/5 is arithmetically
correct, which closes the stale-matrix class that recurred through PRs #14, #16, #19 and
#22 — the remaining matrix problem is status honesty (FND-002), not arithmetic.

## Next Executable Work

1. Correct the FND-002 matrix rows so no row claims a pass while its test is red, and
   state the TC-031 regression in the matrix rather than only in the plan log.
2. Resolve the FND-003 tags: declare `TC-126` locally or drop it, and bind the
   `FR-024-CON` tags to declared rows.
3. Push `agent-b/plan006-ea34-consumer` and open a draft PR so the eight commits and the
   blocking dependency stop living on one machine (FND-006).
4. Wait on Engineering Assurance PR #46 to remove `serde_json/arbitrary_precision` from
   the `producer-execution` closure, then repin to the merged revision and re-run the
   complete suite. Task-027 cannot close before this.
5. With `quire-specification#10` now closed, `quire-contract-ir#64` is the one
   Agent B lane that is unblocked and not waiting on another owner.
