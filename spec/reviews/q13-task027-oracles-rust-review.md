---
id: SR-186
title: "Rust review of Task-027 portable qualification oracles"
type: SpecReview
analysis: code-review
scope: "quire-verification c60cdc44fc3f398b9eccd0e4d5ad81668727e72c..147dbfe74231edc7a55729f46c36d61b3e969eca; Task-027 portable manifest slice"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-024", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TC-041", type: references }
---
# Rust review of Task-027 portable qualification oracles

## Summary

The required `/home/peter/dev/agent-skills/rust-review/SKILL.md` review checked the
portable Rust implementation against `AGENTS.md`, `rustfmt.toml`, `deny.toml`, the
workspace lints, FR-024, TC-041, and the bare compiler-checked `ix-trace-rs` convention.
Three implementation and test findings are resolved at
`147dbfe74231edc7a55729f46c36d61b3e969eca`. The Engineering Assurance
producer-execution seam remains open, so this review does not establish complete
Task-027 or TC-041 evidence and must be rerun after integration.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Resolved: independent-binding collision detection originally scanned every other binding for every binding, permitting roughly ten billion canonical comparisons at the 100,000-item boundary. The validator now builds three ordered identity-count indexes once and performs logarithmic collision lookups. | `src/qualification_oracle.rs`, FR-024-AC-2 | implementation-bug-despite-evidence |
| FND-002 | medium | Resolved: the first shared, derived, and unknown positive tests asserted only `is_ok()`, so deleting normalized tagged data after validation could pass. The tests now read the frozen schema-derived manifest and compare each retained dependence object and expected-value basis. | `tests/qualification_oracle.rs`, FR-024-AC-2 | correct-requirement-no-evidence |
| FND-003 | medium | Resolved: the initial identity test proved permutation invariance but did not prove that each portable binding dimension participates in identity. TC-041 now changes 21 valid manifest, binding, projection, relation, dependence, procedure, request, limitation, and gap dimensions separately and requires a different JCS identity. | `tests/qualification_oracle.rs`, FR-024-AC-4 | correct-requirement-no-evidence |
| FND-004 | high | Open external seam: current Engineering Assurance ADR-0002/NFR-004 allocate native producer execution outside that package, while accepted FR-024 requires the reusable Rust producer-execution interface there. EA issue #11 records the allocation conflict. Q-v must not add a local substitute; FR-024-AC-3, result reuse dimensions in AC-4, and TC-041 steps 4–5 remain incomplete. | FR-024-CON-1..2, FR-024-AC-3..4, TC-041, `agent-ix/engineering-assurance#11` | correct-requirement-no-evidence |

## Review checklist

The production module contains no `unwrap`, `expect`, `panic`, `unsafe`, unchecked
integer cast, recursion, async or blocking work, lock, test-only production branch,
feature bypass, or lint allowance. Caller JSON passes the existing iterative depth and
array-cardinality envelope and the authoritative schema before semantic access. Binding
and derived-lineage walks are iterative; exact-reference collision checks are bounded by
ordered indexes rather than a quadratic pair scan.

The emitted manifest is the Rust type generated from the authoritative E02 schema and
uses RFC 8785 JCS for reference equality and identity. Every artifact field must match
an opaque strict-reader output with its exact role. Set-like members alone are sorted;
ordered corpus projections retain caller order. The module neither executes a process
nor emits evidence sufficiency, qualification, or human disposition.

Ten integration tests exercise only the public API. Every test carries a bare
`#[trace(...)]` marker. The tests delete every required field, wrong-type and empty
selected fields, exercise every dependence tag, reject cross-tag members, reach the
iterative absent/self/cycle graph gate, prove strict-reader and frozen-identity checks,
read positive frozen results, and ratchet every presently implemented identity
dimension. TC-041 remains Planned because no test fabricates the missing external
execution interface.

The diff contains no workflow change and dispatched no hosted workflow.

## Gate results

All commands ran locally at `147dbfe` with Cargo restricted to two jobs.

```text
cargo fmt --all -- --check
exit 0

cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
exit 0

cargo test --locked --workspace --all-features
first-party Rust tests: 88 passed; 0 failed; 2 named production-evidence lanes ignored
vendored ix-trace-rs tests: 3 passed; 0 failed
doc tests: 0 failed; 2 vendored examples ignored

RUSTDOCFLAGS=-Dwarnings cargo doc --locked --workspace --all-features --no-deps
exit 0

cargo deny check
advisories ok, bans ok, licenses ok, sources ok
existing duplicate-version warnings: hashbrown, syn

cargo run --locked --bin validate-fixtures
validated 4 fixtures and 52 public contract instances against e02-draft-2.schema.json

quire validate --scope . "spec/**/*.md" --summary
248/248 docs grammar-clean; 0 grammar findings

quire validate --scope . "plan/**/*.md" --summary
52/52 docs grammar-clean; 0 grammar findings

quire coverage --scope . --json
180/209 matrix rows mechanically backed; 93/93 Rust candidates tagged and bound;
TC-041 remains Planned and FND-004 records the unimplemented result-integration scope
```
