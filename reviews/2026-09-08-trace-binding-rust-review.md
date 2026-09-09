---
id: SR-089
title: "Rust review — Quire-readable acceptance bindings"
type: SpecReview
analysis: code-review
scope: "4483bd6..760bbbd: tests/native_process.rs"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/Plan-001"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Task-007"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Plan-005"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Task-020"
    type: reviews
---

## Summary

Applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` to the exact
trace-binding remediation at `760bbbd`. The review covered repository
conventions, `ix-trace-rs` attribute composition, rustfmt stability, Quire
discovery, test placement and behavior, panic/unsafe surfaces, dependency
changes, and the required gates. No open finding remains.

## Verdict

**PASS** — the formatting-only Rust change compiles and executes the same real
native-process test while exposing every named TC/AC argument to Quire.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|----|----------|---------|------|--------------|
| FND-001 | high | Resolved: `tests/native_process.rs:171` previously used one multiline proc-macro attribute. Rust and `ix-trace-rs` accepted it, but Quire discovered only the first-line token, leaving NFR-006-AC-2 and AC-3 false while the plan claimed 8/8. Four rustfmt-stable compiler-checked attributes now expose each criterion; Quire reports all eight NFR-006 ACs backed. | tests/native_process.rs:171, NFR-004-AC-3, NFR-006-AC-2, NFR-006-AC-3, TC-022, TC-038 | implementation-bug-despite-evidence |
| FND-002 | low | Multiple `#[trace]` attributes on the same test compose correctly: Cargo compiles and executes the actual public-API integration test, rustfmt preserves their layout, and Quire binds every argument. No double test execution, replacement stub, or test-only production branch is introduced. | tests/native_process.rs:171, TC-038 | correct-requirement-no-evidence |

## Failure-Scenario Review

- A future multiline marker can still compile, but amended NFR-004-AC-3 and
  TC-022 require the Quire binding census, so compiler success cannot close the
  gate alone.
- Removing any one of the four attributes leaves the corresponding criterion
  false in `quire coverage`; malformed IDs fail `ix-trace-rs` compilation.
- The changed test still invokes `NativeProcessExecutor` through its public API
  and asserts invalid-deadline refusal, no child launch, deadline termination,
  direct-child absence, descendant absence, and marker absence. No assertion or
  behavioral branch was weakened.

## Boundary Review

The delta adds no production code, public API, dependency, feature, unsafe block,
panic, allocation, process, thread, clock wait, wire field, filesystem access,
or external tool invocation. It changes only compiler-checked test metadata.
The existing controlled test panics and process fixture remain outside the delta.

## Gate Results

- `cargo fmt --check`: PASS; the four attributes remain one line each.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`:
  PASS.
- `cargo test --workspace --all-features --locked`: PASS, 38 first-party tests
  plus 3 vendored `ix-trace-rs` self-tests.
- `cargo run --locked --bin validate-fixtures`: PASS, 4 fixtures and 52 public
  contract instances.
- `cargo deny check`: PASS for advisories, bans, licenses, and sources; accepted
  duplicate-version warnings remain for transitive `hashbrown` and `syn`.
- `quire coverage --scope . --json`: 109/159 rows backed, all eight NFR-006
  criteria true, NFR-004-AC-3 true, 41/41 first-party candidates tagged and
  bound, and no first-party untracked symbol.
- Strict Quire specification validation: PASS, 149/149 documents grammar-clean
  with zero grammar findings.
- Quire plan validation before implementation: PASS, 39/39 documents
  grammar-clean.
