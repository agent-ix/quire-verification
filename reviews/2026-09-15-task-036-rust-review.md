---
id: SR-257
title: "Rust review of Task-036 model generation and shrinking"
type: SpecReview
analysis: code-review
scope: "src/contracts.rs, src/model_testing.rs, tests/model_testing.rs, tests/props_fr_029.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-029", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TC-047", type: references }
  - { target: "ix://agent-ix/quire-verification/Task-036", type: reviews }
---
# Rust Review of Task-036 Model Generation and Shrinking

## Summary

Reviewed the first-party Rust model provider, checked-domain construction,
deterministic generation, native replay seam, shrink lineage, typed outcomes,
resource bounds, tests, and unchanged workflow gates. All source and test
findings were resolved; no production evidence or release state was used as a
development precondition.

## Verdict

**PASS for Task-036.** The implementation has no open Rust source or test
finding and the complete local Rust gate passes.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Resolved: a collection with one element value and an enormous minimum length could perform work beyond the construction bound. Domain depth, structural cardinality, field count, predicate count, collection length, Cartesian size, case count, and shrink count are now explicitly bounded. | `src/model_testing.rs::CheckedDomain::values`, `validate_request` | implementation-bug-despite-evidence |
| FND-002 | high | Resolved: the initial shrink result labeled the smallest retained case as minimal even when the shrink work bound was exhausted. It now records `complete: false` and omits `minimal_counterexample` unless every finite candidate was examined. | `src/model_testing.rs::ShrinkResult`, `shrink_counterexample` | implementation-bug-despite-evidence |
| FND-003 | medium | Resolved: model construction initially introduced a second public string-coded error type. Public fallible helpers now use the crate `VerificationError` and four catalogued stable model error variants; terminal refusal/failure discriminants are closed enums. | `src/contracts.rs::VerificationErrorCode`, `src/model_testing.rs::ModelRefusalCode` | implementation-bug-despite-evidence |
| FND-004 | medium | Resolved: the first result shape omitted an explicit contract version and retained-shrink replay references. Request/result versions, exact runtime/profile/subject/lock identities, original observation references, and one replay reference per retained shrink are now identity-bound. | `src/model_testing.rs::ModelTestRequest`, `ModelTestReport`, `ReplayRecord` | implementation-bug-despite-evidence |

## Rust Boundary Review

- All added implementation and executable test logic is Rust with AGPL-3.0-or-later SPDX headers. No foreign executable path, process runner, evidence store, unsafe block, filesystem access, network access, ambient environment input, clock dependency, or production-only gate was added.
- Generator inputs come from closed checked-domain enums, not Rust implementation-type introspection. Domain construction enumerates the bounded finite Cartesian set once and execution selects only admitted cases; just-outside values remain non-executable refusal probes.
- Native semantics cross the injected `NativeReplay` trait. Rejected preconditions and provider failures return typed method failures, never satisfaction. Retained shrinks must be strictly smaller, admitted, and reproduce the exact clause/disposition.
- Public request/result types serialize through compiler-checked structs/enums. Identity construction uses RFC 8785 JCS through the crate contract helper and binds the request version, package, profile, subject, runtime, ecosystem lock, seed, case bound, shrink bound, generated order, observations, and counterexample lineage.
- Production source contains no `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`, `dbg!`, bare `as` cast, lint allowance, or unchecked caller-controlled arithmetic.
- Every new test has a bare compiler-checked `ix_trace_rs::trace` attribute. Quire 0.32.0 reports FR-029 acceptance criteria 3/3 backed, zero untracked symbols, and zero status lies.

## Local Gates

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | pass |
| `cargo test --workspace --all-features --locked` | 117 tests passed; two named VP04 environment lanes and two vendored documentation tests ignored |
| `cargo run --locked --bin validate-fixtures` | 4 fixtures and 52 public contract instances validated |
| `cargo fuzz build -D --sanitizer none` | pass |
| `cargo deny check` | advisories, bans, licenses, and sources pass; configured duplicate-version warnings remain |
| `quire coverage --scope <repo> --json` | FR-029 3/3 backed; TC-047 and IT-005 bound; zero untracked symbols and status lies |

The native Rust `quoin write` authoring-pack command exposed agent-ix/quoin#526.
The owner-authorized npm Quoin 0.23.1 fallback rendered the SpecReview contract;
the review itself was validated by native Quire.
