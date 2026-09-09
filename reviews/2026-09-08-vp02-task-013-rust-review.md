---
id: SR-118
title: "Rust review — VP02 draft-2 role inputs"
type: SpecReview
analysis: code-review
scope: "ee4157e..e66f554: build.rs, contracts/e02-draft-2.schema.json, fixtures/, evidence/, src/contracts.rs, src/evidence.rs, src/fixtures.rs, src/planner.rs, src/shared_reference.rs, src/bounded_portfolio.rs, tests/verification.rs, spec/tests.md"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/Plan-003"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Task-013"
    type: reviews
---

## Summary

Applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` to the exact
Task-013 candidate at `e66f554`. The review covered repository conventions,
single-source schema generation, external shared-schema resolution, strict
reader opacity, RFC 8785 identity membership, role allocation, closed variant
decoding, evidence identity migration, typed failures, panic and unsafe
surfaces, resource bounds, retained evidence consistency, test substance,
compiled trace markers, and every required executable gate. No open finding
remains.

## Verdict

**PASS** — the reviewed Task-013 Rust delta has no open Critical, High, Medium,
or Low finding, and every required executable gate exits successfully.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-001 | high | Resolved before the final candidate: evidence assessment still selected candidates through the former flat `propertyRef.identity`, so every migrated draft-2 result failed before classification. Candidate selection now reads `propertyRef.value.identity`, while exact scope comparison still uses JCS over the complete envelopes. | `src/evidence.rs:42`, `src/evidence.rs:47`, TC-012..TC-015, TC-032 | implementation-bug-despite-evidence |
| FND-002 | medium | Resolved before the final candidate: draft-2 generated types made the unboxed `PublicContract` variants differ enough for strict Clippy to reject the boundary. All five generated variants now use the same boxed representation. | `src/contracts.rs:280`, TC-001, TC-003 | implementation-bug-despite-evidence |
| FND-003 | medium | Resolved during review: the first TC-028 refusal assertions accepted any error, so a regression could fail for an unrelated reason and still pass. Every schema/version/role/duplicate refusal now asserts `schema_violation`, unvalidated exact-JCS membership asserts `identity_mismatch`, and equal relational identities assert `invalid_input`. | `tests/verification.rs:524`, `tests/verification.rs:549`, FR-015-AC-1..4 | correct-requirement-no-evidence |
| FND-004 | medium | Resolved before the final candidate: the hand-migrated retained local report kept pre-DAG task identifiers and a fabricated setup-task input, so its plan and results no longer described the executable Rust output. The report was regenerated from the Rust runner; only measured timing remains run-specific. | `evidence/local-catalog-portfolio.json`, `src/bounded_portfolio.rs:204`, FR-013-AC-4 | implementation-bug-despite-evidence |
| FND-005 | medium | Resolved at the coverage gate: Rustfmt expanded one long trace attribute across lines that Quire could not bind, leaving the six new acceptance criteria unbacked. Three rustfmt-stable compiled attributes now bind TC-028, FR-002-AC-4, FR-015-AC-1 through AC-4, and NFR-002-AC-3. | `tests/verification.rs:474`, TC-028 | correct-requirement-no-evidence |
| FND-006 | low | Resolved before the final candidate: the role-input test and evidence assessor exceeded the repository's denied `clippy::too_many_lines` threshold. Bounded helpers now isolate the separate checks without a lint allowance. | `tests/verification.rs:478`, `src/evidence.rs:120` | implementation-bug-despite-evidence |

## Failure-Scenario Review

- E02 draft-2 is the only contract schema selected by build-time generation,
  runtime validation, fixtures, and maintained guidance. A former draft-1
  contract version is refused before planning and has no compatibility path.
- Runtime validators register the exact retained shared-reference draft-2
  schema under its declared URN. Build-time generation localizes that URN only
  for Typify, then imports the retained schema definitions; the E02 schema does
  not contain an independently authored `ArtifactRef` field set.
- The strict reader's accepted value is opaque outside its defining module.
  Planning indexes only version-2 artifact outputs by canonical envelope bytes
  and selected role; missing, altered, foreign-role, or unvalidated envelopes
  cannot enter a plan.
- State, trace, fault-model, oracle, and review-procedure references keep their
  separate roles. Relational subjects must differ. Every authored field is
  retained unchanged on each verification task.
- Each variant is closed and every present field is required. Deletion,
  cross-variant insertion, an unknown member, duplicate manual disposition,
  wrong shared-reference version, wrong artifact-reference version, and wrong
  artifact role return their asserted stable refusal class.
- Setup tasks carry only the prerequisite and sorted `requiredBy` identities;
  they no longer manufacture property inputs or property/technique identities.
  Manual verification retains its selected manual runner and authored
  disposition order.
- Fixture and local-portfolio helpers execute the strict reader with explicit
  artifact expectations and local bytes. They do not expose a public shortcut
  that can manufacture `ValidatedSharedReference` values.

## Boundary Review

Production code contains no new `unsafe`, request-derived panic, unchecked
numeric cast, async work, channel, shell, subprocess, network call, or
unbounded recursion. Public fallible APIs return the crate's documented
`VerificationError` and stable error codes. Runtime schema and definition
validators are cached; generated contract values are boxed at the public enum
boundary. The only source-inspection assertions cover properties with no
runtime behavioral substitute: absence of a second schema definition and the
build-time import of retained definitions. All behavioral input and planning
assertions execute through public Rust APIs and compiled `ix_trace_rs::trace`
markers.

## Gate Results

- `cargo run --locked -j 2 --bin validate-fixtures`: PASS, 4 fixtures and 52
  public contract instances against `e02-draft-2.schema.json`.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --locked --all-targets --all-features -j 2 -- -D warnings`:
  PASS on exact candidate `e66f554`.
- `cargo test --workspace --all-features --locked -j 2`: PASS on exact
  candidate, 42 first-party tests plus 3 vendored `ix-trace-rs` self-tests.
- `cargo deny check`: PASS; advisories, bans, licenses, and sources are clean.
  Accepted policy warnings remain for transitive `hashbrown` and `syn`
  versions.
- `quire validate --scope . "spec/**/*.md" "plan/**/*.md" "reviews/*.md"`:
  PASS before this review artifact.
- `quire coverage --scope . --json`: FR-002 is 4/4 backed, FR-015 is 4/5,
  NFR-002 is 3/3, TC-028 is backed, all 45 first-party Rust candidates are
  tagged and bound, no status lie exists, and 125/162 total matrix rows are
  backed. The unbacked FR-015-AC-5 belongs to pending TC-030 and Task-014.
