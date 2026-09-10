---
id: SR-221
title: "Rust re-review of PR #23 fuzz workspace remediation"
type: SpecReview
analysis: code-review
scope: "dd500cd..b32292b: Cargo workspace and lock, fuzz targets and retained seeds, TC-022/TC-037/TC-042, NFR-004, FR-025 and the Test Matrix"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-025", type: reviews }
  - { target: "ix://agent-ix/quire-verification/NFR-004", type: reviews }
  - { target: "ix://agent-ix/quire-verification/Task-029", type: reviews }
---
# Rust Re-review of PR #23 Fuzz Workspace Remediation

## Summary

Applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` to the PR #23 remediation
range after the independent review at SR-220. All eight PR #23 findings are resolved.
The fuzz package now shares the candidate's exact workspace, dependency lock and lint
policy; every declared source population and retained seed is reached by an executable
test or local gate. No fuzz campaign was run or offered as qualification evidence.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-002 | high | Resolved. Both Test Matrix rows now bind FR-025-AC-6 to TC-042. | `spec/tests.md`; `tests/qualification_technique.rs` | missing-requirement |
| FND-003 | high | Resolved. The root workspace owns `fuzz`; root formatting, lint, test and dependency-policy gates therefore reach it, and the explicit fuzz build gate compiles all targets. TC-022 checks the same closure. | `Cargo.toml`; `.github/workflows/ci.yml`; `tests/verification.rs` | correct-requirement-no-evidence |
| FND-004 | medium | Resolved. The nested fuzz lock and workspace were removed. The fuzz package resolves through the root lock, which contains one pinned `jsonschema` version. | `Cargo.lock`; `fuzz/Cargo.toml`; absence of `fuzz/Cargo.lock` | wrong-requirement |
| FND-005 | medium | Resolved. The fuzz package inherits workspace lints; clippy with denied warnings checks all three target crates without an allowance. | `fuzz/Cargo.toml`; `fuzz/fuzz_targets/*.rs` | correct-requirement-no-evidence |
| FND-006 | medium | Resolved. Retained seed locations and bounded local commands are explicit, and TC-042 compile-time includes and verifies every retained seed. | `fuzz/README.md`; `fuzz/seeds/`; `tests/qualification_technique.rs` | missing-requirement |
| FND-007 | medium | Resolved. NFR-004 consistently names `src/`, `tests/` and `fuzz/`; TC-022 walks that same population. | `spec/non-functional/NFR-004-rust-qualified-path.md`; `tests/verification.rs` | wrong-requirement |
| FND-008 | low | Resolved. The task-binding drift test asserts the exact schema-violation message emitted by the core-binding gate. | `tests/report.rs::tc_037_refuses_core_task_binding_drift` | correct-requirement-no-evidence |
| FND-009 | low | Accepted pre-existing repository debt. Duplicate review identifiers were not introduced or modified by this PR. | SR-220 | missing-requirement |

## Verdict

**PASS for the implemented portable slice.** No open PR #23 Rust, test, matrix or gate
finding remains. External Engineering Assurance publication still prevents these local
fuzz and mutation capabilities from being represented as qualification campaign
evidence, as already recorded by Task-029 and SR-218.

## Rust Boundary Review

- The three fuzz targets are first-party Rust workspace members and inherit
  `unsafe_code = "forbid"`, missing-documentation, Clippy all and Clippy pedantic.
- Root `Cargo.lock` is the only dependency closure. `cargo deny` evaluates that closure,
  including the fuzz package and its libFuzzer dependencies.
- The explicit fuzz build uses cargo-fuzz and the retained Rust targets. No local runner,
  transcript parser, source rewriter, evidence store, shell wrapper or foreign runtime
  was added.
- Each fuzz target retains its bare compiler-checked `ix_trace_rs::trace` annotation.
  The seed-verification test also carries a bare trace annotation.
- The target wrappers add no unsafe block, panic, unwrap, expect, unchecked cast, lint
  allowance, unbounded recursion, network operation or file-system mutation.
- The workflow remains manual `workflow_dispatch`. It was inspected but not dispatched.

## Local Gates

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` with two build jobs | pass, including all fuzz targets |
| `cargo test --locked --workspace --all-features` with two build jobs | 99 first-party and 3 vendored tests passed; 2 VP04 lanes and 2 vendored doc tests ignored |
| `cargo fuzz build -D --sanitizer none` with two build jobs | all 3 targets compile without warnings |
| `cargo deny check` | advisories, bans, licenses and sources pass; configured duplicate-version warnings remain |
| `quire validate` over `spec/`, `plan/`, `reviews/` | 273 / 52 / 48 documents grammar-clean before this artifact |
| `quire coverage --scope .` | 184/211 rows backed; 107/107 Rust candidates bound; zero status lies |
