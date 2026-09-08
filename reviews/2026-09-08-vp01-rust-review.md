---
id: SR-028
title: "Rust review — VP01 registry and policy qualification"
type: SpecReview
analysis: code-review
scope: "origin/main...f678faa: contracts/e02-draft-1.schema.json, src/planner.rs, tests/verification.rs, Cargo.toml, build.rs, deny.toml, AGENTS.md"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/Plan-002"
    type: reviews
---

## Summary

Applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` to the VP01
qualification delta. The review covered repository idioms, typed error and panic
surfaces, schema-generated wire behavior, untrusted JSON handling, deterministic
ordering, ownership and visibility, trace markers, test substance, resource bounds,
dependency policy, and the required executable gates. No open finding remains.

The production delta keeps the authored schema as the only wire-contract source.
Planner normalization clones already bounded and schema-validated property objects,
preserves an authored nonempty classification, inserts `unknown` only when the field
is absent, and collects in deterministic property-id order. It returns the crate's
typed `VerificationError` rather than panicking. No new dependency, public API,
`unsafe`, lint exemption, async work, lock, channel, recursion, wire conversion, or
unbounded ingestion path was added.

## Verdict

**PASS** — the reviewed VP01 Rust delta has no open Critical, High, Medium, or
Low finding, and every required gate exits successfully.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | No findings | - |

## Failure-Scenario Review

- An omitted classification produces `unknown` in the plan output while leaving the
  caller-owned input unchanged; an explicitly empty classification fails schema
  validation instead of being silently normalized.
- Not-applicable and unknown applicability retain ready capability state while refusing
  invocation; an absent capability produces unsupported readiness without changing an
  applicable property into a different applicability state.
- Required, recommended, and optional priorities and their authored explanations survive
  planning unchanged; an ordinal `score` field fails the closed contract.
- TC-024 and TC-025 call the real public schema/planner boundaries. Changing the schema
  minimum, removing planner normalization, coupling readiness to applicability, dropping
  authored reasons, or accepting `score` makes a discriminating assertion fail.

## Boundary Review

The changed input remains within the existing bounded JSON envelope before planner
evaluation. Generated closed wire objects continue to carry unknown-field refusal.
`normalized_inventory` is crate-private and adds no consumer-facing Rust API. Its clone
expresses the required output transformation without mutating borrowed request data.
The `BTreeMap` source preserves deterministic property-id ordering. Panics and `expect`
calls remain confined to tests and CLI process-exit presentation; production library
paths continue to return the stable error envelope and forbid unsafe code.

## Gate Results

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS.
- `cargo test --workspace --all-features --locked`: PASS, 27 first-party tests plus 3 vendored `ix-trace-rs` self-tests.
- `cargo deny check --disable-fetch` with a writable advisory cache: PASS; advisories, bans, licenses, and sources are clean.
- `cargo run --locked --bin validate-fixtures`: PASS, 4 fixtures and 52 public contract instances.
- Pinned Quoin `7072d65` plus spec-artifacts-process `61a20e` baseline: PASS, 33 methods keyed by `techniqueId`.
- `quire coverage --scope <repo> --json --strict`: PASS, 87/87 rows, 27/27 first-party Rust candidates tagged and bound, 25/25 declared Test Cases bound, and no unbacked row, status lie, or untracked symbol.
