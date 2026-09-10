---
id: SR-218
title: "Rust review of Task-029 portable fuzz and mutation checkpoint"
type: SpecReview
analysis: code-review
scope: "8c1c859..a47d4f7: fuzz/, Task-029 plan/spec amendments, mutation manifest and discriminating Rust tests"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-025", type: reviews }
  - { target: "ix://agent-ix/quire-verification/NFR-004", type: reviews }
  - { target: "ix://agent-ix/quire-verification/Task-029", type: reviews }
---
# Rust Review of Task-029 Portable Fuzz and Mutation Checkpoint

## Summary

Applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` to the Task-029 Rust
fuzz-target and mutation-discrimination slice at `a47d4f7`. All implementation and test
findings are resolved. The portable slice passes its local gates; Task-029 remains blocked
on the accepted Engineering Assurance producer interface and makes no campaign or native
profile qualification claim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Resolved: the standard spec-fuzz macro-adjacent documentation emitted `unused_doc_comments`; each target now places provenance on a named Rust wrapper carrying the bare trace attribute, with no lint allowance. | `fuzz/fuzz_targets/*.rs`, quoin#362 |
| FND-002 | high | Resolved: exclusive-limit mutations survived the former NFR-005 and FR-016 tests because only one-over-limit rejection was asserted. The tests now accept an exactly 16 MiB valid document and kill both `>` to `>=` mutants. | `tests/verification.rs::tc_023_enforces_json_resource_limits`, `tests/shared_reference.rs::tc_029_accepts_exact_v2_roles_and_refuses_each_selected_failure` |
| FND-003 | high | Resolved: claim-authority and report-binding bypass mutants survived cases refused by earlier schema or identity gates. TC-039 now omits only the exact source-profile proof, and TC-037 now changes a schema-valid task binding; each assigned test kills its bypass mutant. | `tests/qualification_profile.rs`, `tests/report.rs` |
| FND-004 | high | Resolved: the native admission fixture marked nine carried rules accepted without affirmative evidence. It now carries owned gaps for all ten rules, including temporal and runtime-dependency boundaries, while the native-language redesign remains open. | `tests/support/qualification_technique.rs`, `tests/qualification_technique.rs`, quire-specification#10, quire-contract-ir#64 |
| FND-005 | high | Open external boundary: target compilation and local cargo-mutants discrimination cannot become qualification campaign evidence until Engineering Assurance #34 publishes the accepted typed Rust execution/result interface. No local substitute was added. | FR-025-CON-2, Task-029, engineering-assurance#34 |

## Verdict

**PASS for the implemented portable slice.** No open Rust code or test finding remains.
FND-005 accurately prevents Task-029, fuzz campaigns, mutation observations, and native
formal-clause qualification from being called complete.

## Rust Boundary Review

- The executable fuzz additions are Rust and call three already-reviewed bounded public
  APIs directly. They perform no file, network, process, assertion, panic, unchecked
  result unwrap, or evidence-retention work.
- The native-response target admits only valid Unicode because `NativeProcessOutput`
  exposes stdout as a `String`; arbitrary byte decoding remains the native executor's
  separate boundary.
- Every fuzz wrapper and new test carries a bare compiler-checked `ix_trace_rs::trace`
  marker whose identifiers resolve through Quire coverage.
- The mutation plan uses the authoritative schema-derived technique manifest and the
  accepted `mutation-testing` catalog identifier. It binds eight closed fault-model and
  trace references, exact expected failures, and a deterministic eight-case bound.
- The mutation work uses cargo-mutants itself. It adds no mutator, source rewriter, test
  runner, transcript parser, evidence store, workflow, shell code, or foreign runtime.
- The reviewed target/test slice adds no public API, unsafe block, lint allowance,
  unchecked integer cast, recursion, async work, clock assertion, ambient environment
  dependency, or unbounded caller-controlled loop.
- The workflow diff is empty. No GitHub Actions workflow was dispatched.

## Local Gates

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| `cargo test --locked --workspace --all-features` with two build jobs | 99 first-party tests and 3 vendored tests passed; 2 named VP04 lanes and 2 vendored doc tests ignored |
| `RUSTDOCFLAGS=-Dwarnings cargo doc --locked --workspace --all-features --no-deps` | pass |
| `cargo deny check` | advisories, bans, licenses, and sources pass; configured duplicate-version warnings remain |
| `cargo run --locked --bin validate-fixtures` | 4 fixtures and 52 public contract instances validated against E02 draft 2 |
| `cargo fuzz build -D --sanitizer none --target-dir <worktree-local>` | all 3 targets compile without warnings; no campaign executed |
| Eight serial one-mutant/one-test cargo-mutants runs | 8 killed, 0 survived, 0 timed out, 0 unviable |
| `quire validate --scope <repo> 'spec/**/*.md' --summary` | 273/273 documents grammar-clean before this review artifact |
| `quire validate --scope <repo> 'plan/**/*.md' --summary` | 52/52 documents grammar-clean |
| `quire validate --scope <repo> 'reviews/**/*.md' --summary` | 45/45 documents grammar-clean before this review artifact |
| `quire coverage --scope <repo> --json` | 184/211 rows backed; 107/107 Rust candidates tagged and bound; zero status lies |
