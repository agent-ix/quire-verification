---
id: SR-232
title: "Rust review of Task-027 Engineering Assurance consumer"
type: SpecReview
analysis: code-review
scope: "quire-verification c029225d14c01bfe0076ff18de21501950f63412..1b5a5c4c667cbdab4e4ec66b2fc1c2e2872d0bfd; Task-027 EA consumer slice"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-024", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TC-041", type: references }
  - { target: "ix://agent-ix/engineering-assurance/FR-014", type: references }
---
# Rust review of Task-027 Engineering Assurance consumer

## Summary

The exact `/home/peter/dev/agent-skills/rust-review/SKILL.md` review checked the new
Rust consumer, schema-derived records, fixture, tests, dependency boundary, and local
gates against the repository idioms. Five q-v findings are resolved at
`1b5a5c4c667cbdab4e4ec66b2fc1c2e2872d0bfd`. One high downstream dependency finding
remains open in Engineering Assurance PR 46, so this checkpoint is not ready to merge.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Resolved: result validation rejected an EA `executable_identity` refusal because it required every observed digest to match. That suppressed the valid typed finding. Exact observed executable identity is now required only before a completed observation contributes; TC-041 separately reaches and retains the refusal. | `src/qualification_oracle_execution.rs`, `tests/qualification_oracle_execution.rs`, FR-024-AC-3 | implementation-bug-despite-evidence |
| FND-002 | high | Resolved: a caller could mutate the public typed observation after EA decoded it, re-seal the result reference, and contribute the changed observation. The consumer now re-decodes the sealed EA output descriptor and compares exact JCS bytes; the discriminating test supplies a valid alternate observation and proof that would otherwise pass. | `src/qualification_oracle_execution.rs`, `tests/qualification_oracle_execution.rs`, FR-024-AC-3..4 | implementation-bug-despite-evidence |
| FND-003 | high | Resolved: the first schema modeled four unit `ExecutionFailure` reasons as strings, while EA serializes the closed enum as tagged objects. A real failed execution could not produce a schema-derived record. The schema now mirrors the EA wire form and the fixture reaches `exit_code: 7` after retaining its output. | `contracts/e02-draft-2.schema.json`, TC-041 step 5 | implementation-bug-despite-evidence |
| FND-004 | medium | Resolved: an eight-argument public record API and one oversized mutation test required or invited lint allowances. The API now takes a documented borrowed evidence context, mutation cases are separate traced tests, and the new module has no lint allowance. | `src/qualification_oracle_execution.rs`, `tests/qualification_oracle_execution.rs` | implementation-bug-despite-evidence |
| FND-005 | medium | Resolved: the exact Git dependency initially failed both the unknown-source and wildcard dependency policies. The manifest now pins version and revision, and `deny.toml` admits only the named Agent IX provider source while retaining `unknown-git = "deny"`. | `Cargo.toml`, `deny.toml` | implementation-bug-despite-evidence |
| FND-006 | high | Open provider regression: EA's minimal `producer-execution` feature enables `serde_json/arbitrary_precision`. Cargo feature unification changes q-v `Value` number serialization, so retained RFC 8785 vectors fail in TC-031 even though the focused consumer lane passes. The exact feature edge and reproduction are posted to Engineering Assurance PR 46 in `issuecomment-5629327354`. | `engineering-assurance@f640f4c`, `tests/shared_reference.rs:757`, TC-031, TC-126 | correct-requirement-no-evidence |

## Review checklist

The production module contains no `unwrap`, `expect`, `panic`, `unsafe`, unchecked
integer cast, recursion, async work, lock, test-only production branch, or lint allowance.
All public items are documented. The caller-owned adapter accepts only one bounded,
strictly parsed, schema-derived response from EA's sealed output descriptor. Completed
observations are re-decoded from those retained bytes. Non-completed states carry no
observation, and the comparison and disagreement records cannot issue FR-027's
qualification disposition.

The Rust-only fixture is feature-contained and exercises the real EA executor; it does
not bypass or reproduce supervision. All six integration tests use bare
`ix_trace_rs::trace` markers. The tests reach nine state cases including two distinct
refusal reasons, agreeing and disagreeing comparisons, exact and foreign disagreement
procedures, forbidden disposition injection, and independent interface, request, result,
and retained-observation mutations.

The workflow remains manual `workflow_dispatch` only. No hosted CI ran.

## Gate results

```text
cargo fmt --all -- --check
exit 0

cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
exit 0

cargo test --features qualification-oracle-test-fixture \
  --test qualification_oracle_execution -- --test-threads=1
6 passed; 0 failed

cargo run --locked --bin validate-fixtures
validated 4 fixtures and 52 public contract instances against e02-draft-2.schema.json

cargo deny check
advisories ok, bans ok, licenses ok, sources ok
duplicate-version warnings remain advisory

quire validate --scope . "spec/**/*.md" "plan/**/*.md"
exit 0; only pre-existing duplicate catalog notices

cargo test --workspace --all-features --locked -- --test-threads=1
FAIL: TC-031 canonical vector mismatch after EA enables serde_json/arbitrary_precision
```
