---
id: SR-048
title: "Rust review — VP03 conservative evidence freshness"
type: SpecReview
analysis: code-review
scope: "07fd52f..6961a09: contracts/e02-draft-1.schema.json, src/contracts.rs, src/evidence.rs, tests/verification.rs"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/Plan-004"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Task-016"
    type: reviews
---

## Summary

Applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` to the exact VP03
Task-016 candidate at `6961a09`. The review covered repository conventions,
schema-derived wire behavior, checked timestamp arithmetic, offset and boundary
semantics, typed errors, panic and unsafe surfaces, trace markers, test substance,
and the required executable gates. No open finding remains.

The first review pass found that `freshness` had been made mandatory on the base
planning `PropertyInventoryItem`, widening VP02 input requirements beyond the
reviewed VP03 behavior. Before acceptance, the schema was narrowed so only the
`EvidenceObligation` subtype requires freshness; TC-032 proves a planning-only
property may omit it while an evidence obligation may not.

## Verdict

**PASS** — the reviewed Task-016 Rust delta has no open Critical, High, Medium,
or Low finding, and every required executable gate exits successfully.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | Resolved during review: requiring freshness on every `PropertyInventoryItem` changed planning inputs outside FR-018; the requirement now attaches only to `EvidenceObligation`, with a discriminating planning/evidence test. | contracts/e02-draft-1.schema.json, TC-032 |

## Failure-Scenario Review

- Either absolute result validity or obligation maximum age can be the earlier
  bound; exact expiry is fresh and one nanosecond later is stale.
- Equivalent instants expressed with different RFC 3339 offsets classify alike.
- Production after assessment and validity before production return
  `invalid_temporal_order` with deterministic context.
- Missing, negative, or larger-than-safe-integer maximum ages fail schema
  validation; a schema-valid duration or expiry outside RFC 3339 range returns
  `invalid_timestamp` without overflow or panic.
- Stale precedence retains logical, activation, participation, assumption,
  adequacy, provenance, native-artifact, and independent-gap content.

## Boundary Review

The public assessor continues to borrow caller values and return the crate's
typed `VerificationError`; no request-derived panic, unsafe block, async work,
lock, channel, recursion, unchecked numeric cast, dependency, or unrelated
public API was added. `TimeDelta::try_seconds`, `checked_add_signed`, and the
RFC 3339 year check bound arithmetic before comparison. Test `expect` calls
remain confined to acceptance code. Three TC-032 tests compile their trace tags
through `ix-trace-rs` and reach only the public assessment/planning APIs.

## Gate Results

- `cargo run --locked --bin validate-fixtures`: PASS, 4 fixtures and 52 public
  contract instances.
- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`:
  PASS.
- `cargo test --workspace --all-features --locked`: PASS, 31 first-party tests
  plus 3 vendored `ix-trace-rs` self-tests.
- `cargo deny check`: PASS; advisories, bans, licenses, and sources are clean.
  Accepted policy warnings remain for transitive `hashbrown` and `syn` versions.
- Explicit local-module `quire coverage`: FR-018 is 5/5 backed; 31/31
  first-party Rust candidates are tagged and bound; zero untracked symbols;
  99/125 total rows backed. The remaining rows belong to declared blocked or
  pending VP02/VP03 work.
