---
id: SR-128
title: "Rust review of bounded native runner requests"
type: SpecReview
analysis: code-review
scope: "quire-verification 86ebb0b..96344bc; Task-014, contracts/e02-draft-2.schema.json, src/planner.rs, src/bounded_portfolio.rs, tests/verification.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-014", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TC-027", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-030", type: references }
---

## Summary

The required `/home/peter/dev/agent-skills/rust-review/SKILL.md` review checked
the Task-014 Rust change against `AGENTS.md`, `rustfmt.toml`, `deny.toml`, the
workspace lints, the reviewed runner-request requirements, and the repository's
compiler-checked `ix-trace-rs` convention. The review found no unresolved Rust
correctness, panic-surface, numeric-boundary, seam, resource-bound, or test-
integrity defect.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Resolved before the reviewed commit: strict Clippy rejected a 118-line boundary-test helper; it was split by input and emitted-record responsibility, after which strict Clippy and the complete suite passed. A future omitted mutation now remains localized to the contract side it exercises. | TC-027, `tests/verification.rs` |

## Review checklist

The public boundary continues to return the crate's stable `VerificationError`
instead of panicking or exposing string errors. The implementation adds no
`unsafe`, casts, blocking/async work, locks, feature-gated bypass, broad lint
allowance, public API, executable language, or unbounded loop. Request numbers
are accepted by the schema only as positive bounded integers before `as_u64`
reads them. Capability-relative comparisons retain both requested and admitted
values when refusing work.

Tests execute the public planner and validator rather than a replacement stub.
TC-027 removes every runner-request field and mutates limits, budgets, gaps, and
resource-allocation prerequisites; TC-030 executes heterogeneous native/manual
planning. Both test functions carry bare compiler-checked `#[trace(...)]`
attributes, and Quire binds all 47 discovered Rust candidates. Assertions read
planner output and fail under missing authorization checks, cross-role input
loss, open-object drift, property-budget precedence reversal, or mistaken manual
invocation.

## Gate results

The repository gates were run from the reviewed checkout with Cargo restricted
to two build jobs. Their terminal results were:

```text
$ cargo fmt --check
(no output; exit 0)

$ cargo clippy --workspace --all-targets --all-features --locked -j 2 -- -D warnings
Checking quire-verification v0.1.0 (/home/peter/dev/quire-verification-agent-b-vp01)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s

$ cargo test --workspace --all-features --locked -j 2
quire-verification unit tests: 2 passed; 0 failed
native_process integration tests: 5 passed; 0 failed
shared_reference integration tests: 3 passed; 0 failed
verification integration tests: 34 passed; 0 failed
first-party total: 44 passed; 0 failed
vendored ix-trace-rs tests: 3 passed; 0 failed

$ cargo deny check
advisories ok, bans ok, licenses ok, sources ok
exit 0; existing duplicate-version warnings: hashbrown, syn

$ cargo run --locked --bin validate-fixtures
validated 4 fixtures and 52 public contract instances against e02-draft-2.schema.json

$ quire validate --scope . "spec/**/*.md" --summary
181/181 docs grammar-clean (100%); 0 grammar finding(s): none
52/124 criteria property-extractable (41%); 0 candidate (metamorphic, needs review)

$ quire coverage --scope . --json
132/162 matrix rows backed; 47/47 Rust candidates tagged and bound; no affected FR-005, FR-013, FR-014, or FR-015 row unbacked
```
