---
id: SR-138
title: "Rust review of oracle contribution dependence"
type: SpecReview
analysis: code-review
scope: "quire-verification 8d24eaf..48f635d; Task-017, Task-018, contracts/e02-draft-2.schema.json, src/evidence.rs, src/bounded_portfolio.rs, src/fixtures.rs, tests/verification.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-017", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TC-033", type: references }
---

## Summary

The required `/home/peter/dev/agent-skills/rust-review/SKILL.md` review checked
the committed VP03 implementation against `AGENTS.md`, `rustfmt.toml`,
`deny.toml`, the workspace lints, the accepted FR-017 contract, and the
repository's compiler-checked `ix-trace-rs` convention. Both findings were
resolved at `48f635d`; no unresolved Rust correctness, panic-surface,
numeric-boundary, seam, resource-bound, or test-integrity defect remains.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Resolved: the graph originally represented `(resultId, contributionId)` as a raw pair of strings. A future swapped constructor would have compiled and could have produced a false absent-lineage refusal or wrong correlation member. `ContributionId` now names both fields and is the only graph/lineage identity type. | FR-017-AC-3, FR-017-AC-4, `src/evidence.rs` |
| FND-002 | low | Resolved style note: the public assessor documentation listed only its earlier errors, and the contract README stated that every non-evaluated result retained no contributions although the reviewed schema permits an empty array. Both descriptions now match the implemented boundary. | FR-017-AC-1, FR-017-AC-2, FR-017-AC-3, `src/evidence.rs`, `contracts/README.md` |

## Review checklist

The public assessor returns the crate's stable `VerificationError`, with stable
codes for duplicate contribution IDs and invalid lineage. The change adds no
panic on supplied input, `unsafe`, cast across a wire boundary, async/blocking
work, lock, test-only production branch, feature bypass, broad lint allowance,
or first-party executable language. The generated wire modules retain their
pre-existing localized lint allowances. Contribution and component traversal
is iterative, and public JSON enters through the existing byte, depth, and
array-bounded reader.

The strict reader returns opaque `ValidatedSharedReference` values; the assessor
requires exact RFC 8785 bytes and the exact oracle or observation role before
matching any result. The graph uses deterministic maps and sets, typed member
identities, and selected-result membership. It excludes ambiguous candidates,
deduplicates reused selected results, reports connected bases and unknown
relations without scoring, and retains every selected contribution separately.

TC-033 exercises the real schema, strict reader, assessor, lineage traversal,
selection boundary, and JCS output. It deletes or wrong-types contract fields,
mutates each reference dimension, substitutes the wrong role, repeats validated
values, supplies duplicate/absent/self/cyclic lineage, creates every dependence
basis and unknown reason, permutes every input collection, introduces ambiguous
candidates, and reuses a result. All four test functions carry bare
compiler-checked `#[trace(...)]` attributes. Quire binds all seven FR-017 rows;
the only untracked symbol is the pre-existing vendored `ix-trace-rs`
`FR-003-CON-1` self-test, outside this first-party change.

## Gate results

The repository gates ran from the reviewed checkout with Cargo restricted to
two build jobs. Their terminal results were:

```text
$ cargo fmt --all --check
(no output; exit 0)

$ cargo clippy --workspace --all-targets --all-features --locked -j 2 -- -D warnings
Checking quire-verification v0.1.0
Finished `dev` profile; exit 0

$ cargo test --workspace --all-features --locked -j 2
quire-verification unit tests: 2 passed; 0 failed
native_process integration tests: 5 passed; 0 failed
shared_reference integration tests: 3 passed; 0 failed
verification integration tests: 38 passed; 0 failed
first-party total: 48 passed; 0 failed
vendored ix-trace-rs tests: 3 passed; 0 failed

$ cargo deny check advisories bans licenses sources
advisories ok, bans ok, licenses ok, sources ok
exit 0; existing duplicate-version warnings: hashbrown, syn

$ cargo run --locked -j 2 --bin validate-fixtures
validated 4 fixtures and 52 public contract instances against e02-draft-2.schema.json

$ quire validate --scope . "spec/**/*.md" --summary
190/190 docs grammar-clean (100%); 0 grammar findings
52/126 criteria property-extractable; 1 reviewed metamorphic candidate

$ quire coverage --scope . --json
140/164 matrix rows backed; 51/51 discovered Rust candidates tagged and bound;
FR-017 7/7 backed; no FR-017 untracked symbol or status contradiction

$ check pinned Quoin catalog through check-quoin-baseline
Quoin baseline parity: 33 methods
```
