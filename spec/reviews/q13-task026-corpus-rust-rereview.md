---
id: SR-175
title: "Rust rereview of qualification corpus remediation"
type: SpecReview
analysis: code-review
scope: "quire-verification ff82a7b..7f9afac; Task-026, contracts/e02-draft-2.schema.json, src/qualification_corpus.rs, tests/qualification_corpus.rs, tests/qualification_profile.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-023", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TC-040", type: references }
---

## Summary

The required `/home/peter/dev/agent-skills/rust-review/SKILL.md` review examined the
independent findings and the complete remediation at
`7f9afac2a701cf519fd11abf6fe900a57785791c`. All five findings are closed. No open Rust
correctness, test-discrimination, panic-surface, resource-bound, trace, or ownership
finding remains in the Task-026 implementation boundary.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Resolved: the original self, missing-parent, and cycle mutations invalidated replay identity first, so they never exercised ancestry validation. The tests now regenerate a valid replay run after each parent edit, accept a re-sealed valid chain, require the parent-specific refusal for self and missing parents, and require the cycle-specific refusal for a two-node cycle. Removing a graph guard now makes the corresponding test accept the invalid graph or return the wrong gate result. | `tests/qualification_corpus.rs:356`, `tests/qualification_corpus.rs:631`, `src/qualification_corpus.rs:288` | correct-requirement-no-evidence |
| FND-002 | medium | Resolved: the schema now models `reference-only` and `unauthorized` decisions only in gap position; Rust reads the disposition, preserves the candidate reference in the frozen manifest, retains no entry bytes, and grants no class coverage. Retention-authorizing dispositions in gap position are refused. | `contracts/e02-draft-2.schema.json:999`, `contracts/e02-draft-2.schema.json:1037`, `contracts/e02-draft-2.schema.json:1085`, `src/qualification_corpus.rs:327`, `tests/qualification_corpus.rs:434` | implementation-bug-despite-evidence |
| FND-003 | medium | Resolved: the Test Execution Summary was manually recounted and the plan now requires the recount after every task that adds a test or changes status until the upstream header contract is corrected. | `spec/tests.md:421`, `plan/Plan-006-quire-spec-production-qualification/plan.md:101` | implementation-bug-despite-evidence |
| FND-004 | medium | Resolved: NFR-008's anti-tamper method is restored to `Mutation Test (TC-045)` in both the metric and acceptance criterion. | `spec/non-functional/NFR-008-reproducible-qualification.md:35`, `spec/non-functional/NFR-008-reproducible-qualification.md:50` | wrong-requirement |
| FND-005 | low | Resolved: Task-026 no longer claims or changes Task-025's TC-039 behavior; the added TC-039 assertions were removed and the task checklist owns only TC-040 plus the manual summary recount. | `plan/Plan-006-quire-spec-production-qualification/tasks/Task-026-retain-qualification-corpus.md:23`, `tests/qualification_profile.rs:349` | wrong-requirement |

## Review checklist

The production module has no `unwrap`, `expect`, `panic`, `unsafe`, unchecked cast,
lint allowance, recursion, async work, blocking IO, lock, test-only branch, or feature
bypass. Caller JSON is schema-validated before semantic access and remains under the
existing 16 MiB, 128-level, and 100,000-array-item bounds. Parent traversal is iterative
and finalizes each retained mutation once.

The rights variants form a closed schema split: corpus entries accept only
`retained-private` or `redistributable`, while a gap candidate accepts only
`reference-only` or `unauthorized`; an absent candidate requires absent rights. The Rust
gap path independently reads that decision and exposes it without retaining candidate
bytes. The public frozen manifest preserves the source reference and complete rights
record.

TC-040 executes the public API and authoritative schema through six bare
`#[trace(...)]` tests. The repaired graph test proves a valid re-sealed chain reaches
success before testing self, absent, and cyclic parent states. The rights test supplies
strict-reader proof bytes for the candidate and still proves that the frozen corpus does
not retain them, so its no-retention assertion does not depend on missing input evidence.

## Gate results

All commands ran locally from `7f9afac` with Cargo limited to two jobs. No GitHub Actions
workflow was dispatched.

```text
cargo fmt --all -- --check
exit 0

cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
exit 0

cargo test --locked --workspace --all-features
first-party Rust tests: 78 passed; 0 failed; 2 named external lanes ignored
vendored ix-trace-rs tests: 3 passed; 0 failed
doc tests: 0 failed; 2 vendored examples ignored

cargo deny check
advisories ok, bans ok, licenses ok, sources ok
existing duplicate-version warnings: hashbrown, syn

cargo run --locked --bin validate-fixtures
validated 4 fixtures and 52 public contract instances against e02-draft-2.schema.json

quire validate --scope . "spec/**/*.md" --summary
238/238 docs grammar-clean; 0 grammar findings

quire validate --scope . "plan/**/*.md" --summary
52/52 docs grammar-clean; 0 grammar findings

quire coverage --scope . --json
175/209 matrix rows backed; 83/83 Rust candidates tagged and bound;
FR-023 4/4 backed; TC-040 backed
```
