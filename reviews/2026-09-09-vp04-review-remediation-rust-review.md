---
id: SR-160
title: "Rust review of VP04 independent-review remediation"
type: SpecReview
analysis: code-review
scope: "PR #11 after SR-151; src/report.rs, tests/native_process.rs, tests/verification.rs, tests/vp04_integration.rs and supporting evidence/spec changes"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/Plan-005"
    type: reviews
  - target: "ix://agent-ix/quire-verification/NFR-004"
    type: references
  - target: "ix://agent-ix/quire-verification/NFR-006"
    type: references
---
# Rust Review of VP04 Independent-Review Remediation

## Summary

The exact `agent-skills/rust-review` workflow reviewed the SR-151 remediation
against repository conventions, idiomatic Rust, test discrimination, trace
bindings, panic/unsafe surface, process containment and local gates. All four
review findings are resolved and no new open Rust finding remains.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Resolved: TC-038 now refuses both relative- and absolute-target executable symlinks; removing executable `NO_SYMLINKS` makes the exact test fail at the relative link. | tests/native_process.rs, NFR-006-AC-6..7 | correct-requirement-no-evidence |
| FND-002 | medium | Resolved: a non-ignored Rust unit test reads the committed core, pins its JCS digest, validates retained local artifacts, and recomputes schema, procedure, measurement, reproduction, plan/result/run and assessment bindings. Editing the core makes the exact test fail. | src/report.rs, FR-021-AC-1..5 | correct-requirement-no-evidence |
| FND-003 | low | Resolved: NFR-004, TC-022 and its compiler-bound trace state that repository-owned Rust is the first-party population and vendored symbols remain owned by the dependency matrix. | NFR-004-AC-6, tests/verification.rs | missing-requirement |
| FND-004 | low | Resolved: the finalizer defaults to the ignored repository-local `.vp04-build` directory; candidate evidence documents `VP04_PARSER`, population instructions and the required executable digest. | tests/vp04_integration.rs, evidence/vp04-parser-candidate.md | correct-requirement-no-evidence |

## Rust-specific review

- The new test code is confined to `#[cfg(test)]` or integration-test modules;
  no public API, production error, wire contract, dependency or unsafe surface
  changed.
- Test `expect` and indexing operations consume committed fixtures and
  test-constructed values only. No caller-controlled library path gained a panic.
- The relative symlink probes use per-test temporary directories and no ambient
  home, fixed port, network, wall-clock threshold or shared mutable state.
- The committed-core source inspection is appropriate for the artifact-retention
  property: it executes the real schema, JCS, strict-reference, measurement and
  evidence-assessment code and is mutation-discriminating rather than a text-only
  assertion.
- New acceptance evidence uses bare `ix_trace_rs::trace`; Quire discovers and
  binds every named TC/AC argument.

## Mutation discrimination

| Mutant | Expected result | Observed result |
| --- | --- | --- |
| Remove executable-open `ResolveFlags::NO_SYMLINKS` | TC-038 fails on a relative executable symlink | failed at `tests/native_process.rs:140` because the mutant admitted the link |
| Remove root-open `ResolveFlags::NO_SYMLINKS` | TC-038 fails on a relative root symlink | failed at `tests/native_process.rs:182` because the mutant admitted the linked root |
| Change committed core `reportId` without changing the pinned test digest | TC-037 fails | failed with recomputed `sha256-jcs:8ea964b6...` versus pinned `sha256-jcs:db03d0c8...` |

All mutants were restored before final gates.

## Local gates

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --locked --all-targets --all-features -j 2 -- -D warnings` | pass |
| `cargo test --workspace --all-features --locked -j 2` | 70 passed, 0 failed; 2 named VP04 integration lanes and 2 vendored doc tests ignored |
| `cargo deny check` | advisories, bans, licenses and sources pass; allowed duplicate-version warnings remain |
| `cargo run --locked --bin validate-fixtures` | 4 fixtures and 52 public contract instances validated |
| `quire validate --scope . "spec/**/*.md"` | pass; catalog duplicate notices only |
| `quire coverage --scope . --json` | 165/165 rows backed; 72/72 Rust candidates tagged and bound; 65/65 self-named bound; no status lies |

No GitHub Actions workflow was dispatched or awaited.
