---
id: SR-038
title: "Rust review — VP02 actionable prerequisite DAG"
type: SpecReview
analysis: code-review
scope: "98d8059..250811e: contracts/e02-draft-1.schema.json, src/planner.rs, src/bounded_portfolio.rs, tests/verification.rs"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/Plan-003"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Task-011"
    type: reviews
---

## Summary

Applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` to the exact VP02
Task-011 candidate at `250811e`. The review covered repository conventions,
schema-generated wire behavior, deterministic task identity and ordering, typed
errors, panic and unsafe surfaces, resource bounds, trace markers, test substance,
and the required executable gates. No open finding remains.

The pre-candidate review found that colon-joined component values could alias
distinct prerequisite or verification task identities. The requirement returned
through QUOIN `/specify` and owner-selected `/spec-review all`; the reviewed
implementation now compares prerequisite identities as typed tuples, emits
length-prefixed composite task IDs, shares verification-ID construction with the
bounded portfolio, and tests delimiter-bearing collision pairs. A second gate
iteration split the `ix-trace-rs` tags into single-line attributes after Quire
showed that a Rustfmt-wrapped attribute did not bind the five acceptance criteria.

## Verdict

**PASS** — the reviewed Task-011 Rust delta has no open Critical, High, Medium,
or Low finding, and every required executable gate exits successfully.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | No findings | - |

## Failure-Scenario Review

- Missing capability prerequisites, a capability-declared resource allocation,
  forced policy freshness, and near matches remain visible as blocked setup work.
- Two verification tasks that need the same prerequisite share one setup node;
  its `requiredBy` values are sorted and deduplicated, and both task dependencies
  resolve through graph validation.
- Distinct `(kind, id)` and `(propertyId, techniqueId)` tuples containing colons
  produce distinct task identifiers. Reintroducing delimiter joining makes TC-026
  fail or produces a duplicate-task error.
- A setup task cannot borrow property or technique identity, omit its exact
  prerequisite, or omit its nonempty `requiredBy` list without schema refusal.
- Conflicting full prerequisite definitions under one exact tuple are compared
  with RFC 8785 JCS and return the crate's stable `invalid_input` envelope.

## Boundary Review

The public planner continues to borrow caller values and return the crate's typed
`VerificationError`; no request-derived panic, unsafe block, async work, lock,
channel, recursion, numeric cast, dependency, or public Rust API was added. The
schema remains the only wire-contract definition and generates optional
variant-specific fields directly. `BTreeMap`, `BTreeSet`, explicit sorting, and
length-prefixed UTF-8 byte lengths make plan output deterministic and injective.
Test `expect` calls remain confined to acceptance code.

## Gate Results

- `cargo run --locked --bin validate-fixtures`: PASS, 4 fixtures and 52 public
  contract instances.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`:
  PASS.
- `cargo test --workspace --all-features --locked`: PASS, 28 first-party tests
  plus 3 vendored `ix-trace-rs` self-tests.
- `cargo deny check`: PASS; advisories, bans, licenses, and sources are clean.
  Accepted policy warnings remain for transitive `hashbrown` and `syn` versions.
- `quire coverage --scope <repo> --json`: FR-013 5/5 backed; 31/31 Rust
  candidates tagged and bound; 93/113 total rows backed. The 20-row remainder is
  the declared FR-014 through FR-016 and TC-027 through TC-031 implementation
  scope, not Task-011 evidence.
