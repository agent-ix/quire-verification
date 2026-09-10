---
id: SR-205
title: "Rust review of Task-028 portable qualification technique planning"
type: SpecReview
analysis: code-review
scope: "4c7b7db: Cargo.toml, contracts/e02-draft-2.schema.json, src/qualification_profile.rs, src/qualification_technique.rs, tests/qualification_profile.rs, tests/qualification_technique.rs, tests/props_fr_025.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-022", type: reviews }
  - { target: "ix://agent-ix/quire-verification/FR-025", type: reviews }
  - { target: "ix://agent-ix/quire-verification/Task-028", type: reviews }
---
# Rust Review of Task-028 Portable Qualification Technique Planning

## Summary

Applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` to revision `4c7b7db`,
including schema-derived wire output, untrusted-value bounds, strict-reference joins,
identity normalization, panic and cast surfaces, test trace placement, mutation
discrimination, dependency policy, and workflow diffs. All implementation findings are
resolved. The portable Rust slice passes; Task-028 remains blocked on its named producer
and catalog dependencies.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Resolved: the first admission draft required ten evidence references and could therefore equate reference presence with accepted native-profile coverage. Each rule now carries a closed accepted result or owned gap, and configurations refuse until the exact gap set is empty. | `contracts/e02-draft-2.schema.json`, `src/qualification_profile.rs`, `src/qualification_technique.rs:528` | wrong-requirement |
| FND-002 | medium | Resolved: the case planner returned only a digest string even though FR-025 promises a typed next-case/shrink request. It now returns a closed schema-derived `QualificationCaseRequest` plus its JCS identity and explicit configuration identity. | `contracts/e02-draft-2.schema.json`, `src/qualification_technique.rs:84`, `src/qualification_technique.rs:207` | correct-requirement-no-evidence |
| FND-003 | medium | Resolved: the first generated property test tagged all of FR-025-AC-2 even though the accepted catalog lacks `differential-testing`. The tag was removed, the row remains honestly unbacked, and SR-196 records the unavailable member. | `tests/props_fr_025.rs`, `reviews/2026-09-09-task-028-spec-correctness.md` | correct-requirement-no-evidence |
| FND-004 | medium | Resolved: initial tests omitted direct one-axis mutations of the four frozen input identities and the absent strict-reader proof set. TC-042 now exercises each and discriminates catalog-source unavailability. | `tests/qualification_technique.rs:209` | correct-requirement-no-evidence |
| FND-005 | medium | Open external boundary: the portable crate receives an opaque dependency-closure reference and cannot itself prove absence of forbidden foreign runtimes. Task-028 remains blocked until Engineering Assurance #34 and boundary-enforcement issue quire-research#64 expose accepted evidence; no local scanner was added. | FR-025-CON-2, Task-028 | correct-requirement-no-evidence |

## Verdict

**PASS for the implemented portable slice.** No open Rust source or test finding remains.
FND-005 is an accurately retained external qualification dependency and prevents Task-028
or production formal-clause evidence from being called complete.

## Rust Boundary Review

- All new production and test logic is Rust. The change adds no Java, JVM, Maven,
  Eclipse, Electron, Node, Python, shell runner, arbitrary stdout parser, evidence store,
  or qualification decision.
- Public wire values are derived from the authoritative JSON Schema. Every closed object
  rejects unknown fields at schema validation, and generated Rust values are used for
  the frozen manifest and case request.
- Public functions return the crate's `VerificationError` with stable catalogued codes.
  The production module contains no `unwrap`, `expect`, `panic!`, `unsafe`, bare `as`
  conversion, lint allowance, filesystem access, process launch, clock dependency, or
  unbounded caller-controlled loop.
- Every caller-controlled array crosses the shared 100,000-item resource-envelope gate;
  case ordinals and seeds additionally use checked canonical `u64` parsing.
- Set-like manifest members and prior observations use deterministic `BTreeMap`/
  `BTreeSet` ordering. Corpus projections and shrink paths retain caller order.
- New tests use bare compiler-checked `ix_trace_rs::trace` attributes. QUOIN binds the
  emitted FR-025-AC-1 property tag and all TC-042 symbols; the blocked AC2 through AC5
  rows remain unbacked.

## Mutation Discrimination

| Mutant | Expected result | Observed result |
| --- | --- | --- |
| Remove the configuration refusal for nonempty native admission gaps | The formal configuration is incorrectly completed | `tc_042_reports_carried_native_admission_gaps_without_assuming_coverage` failed because it received `Complete` |
| Remove canonical sorting of set-like prior observations | Reversing prior observations changes case identity | `tc_042_case_identity_preserves_ordered_shrinks_and_normalizes_prior_sets` failed with distinct JCS digests |

Both mutants were restored before the final gates.

## Local Gates

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --locked --all-targets --all-features -- -D warnings` | pass |
| `cargo test --workspace --all-features --locked -j 2` | 96 first-party tests passed; 3 vendored tests passed; 2 named VP04 lanes and 2 vendored doc tests ignored |
| `cargo deny check` | advisories, bans, licenses, and sources pass; configured duplicate-version warnings remain |
| `cargo run --locked --bin validate-fixtures` | 4 fixtures and 52 public contract instances validated against E02 draft 2 |
| `quire validate --scope <repo> 'spec/**/*.md'` | pass; duplicate installed-module notices only |
| `quire coverage --scope <repo> --json` | 182/210 rows backed; FR-025 is honestly 1/5; 101/101 Rust candidates tagged and bound; zero status lies; the documented bare-command vendored-symbol attribution remains |

No GitHub Actions workflow was dispatched or awaited.
