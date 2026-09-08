---
id: SR-009
title: "Rust review — Plan-001 verification rearchitecture"
type: SpecReview
analysis: code-review
scope: "Cargo.toml, build.rs, src/**/*.rs, tests/**/*.rs, contracts/e02-draft-1.schema.json, .github/workflows/ci.yml"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/Plan-001"
    type: "reviews"
---

## Summary

Applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` and its required
portable Rust-style checklist to the complete Rust replacement. The review
covered error and panic surfaces, generated wire contracts, untrusted JSON,
resource bounds, deterministic ordering, identity types, public visibility,
the injected adapter seam, trace markers, test substance, dependencies, and CI.
Four code-quality and traceability findings were found and resolved during the
review; no open finding remains.

## Verdict

**PASS** — the remediated implementation has no open Critical, High, Medium, or
Low finding, and every required Rust gate exits successfully.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | Resolved: catalog JCS sort errors were converted to an empty key, allowing malformed values to fall back to source order | `src/catalog.rs:112` |
| FND-002 | medium | Resolved: a multiline trace marker compiled but left three FR-007 criteria invisible to QUOIN's static reconciler | `tests/verification.rs:466` |
| FND-003 | low | Resolved: planner and portfolio orchestration used broad `too_many_lines` exemptions that obscured their separate validation and result-building phases | `src/planner.rs:27`, `src/bounded_portfolio.rs:51` |
| FND-004 | low | Resolved: technique identifiers used an ad-hoc parser and exposed an indexing helper beyond the crate | `src/catalog.rs:26`, `src/catalog.rs:216` |

## Failure Scenarios and Resolutions

FND-001 was reachable if a future adapter diagnostic contained a value outside
the JCS serializer's domain. Every such failure became the same empty sort key,
so stable sort preserved producer order and violated deterministic error
handling. The adapter now canonicalizes every key fallibly and propagates the
typed `canonicalization_failed` error.

FND-002 was visible in `quire coverage`: `FR-007-AC-3` through `FR-007-AC-5`
were unbacked even though they appeared inside a compiler-checked attribute.
The marker was split into short bare `#[trace(...)]` attributes, the test was
strengthened to measure denominator and missing-obligation preservation, and
the reconciler now reports 78/78 targets backed.

FND-003 made future failure-path edits harder to assess because input
validation, request construction, execution observation, and result assembly
were embedded in two functions under lint exemptions. Those phases are now
small fallible helpers, and the exemptions were removed.

FND-004 allowed two technique identifiers to be confused at call sites and
added a utility to the external API only to support tests. `TechniqueId` now
implements `FromStr`, remains crate-private, and the public API contains only
the adapter behavior consumers need.

## Boundary Review

The five public wire contracts are generated from the authored JSON Schema and
validated against the unmodified draft 2020-12 schema. Generated closed objects
carry `serde(deny_unknown_fields)`. The schema's documented open extension
objects remain open intentionally at the provisional Quoin JSON seam. No
caller-controlled recursion, unsafe block, async worker, lock, channel, network
loop, or persistence conversion exists. Public library functions return the
stable `VerificationError` envelope; binary-local `Box<dyn Error>` values do
not cross the crate API.

The adapter-error seam executes the real portfolio and substitutes only the
catalog conversion function. Its adverse tests prove that a comparison
mismatch becomes a violated `TechniqueResult` and an adapter error becomes a
failed `TechniqueResult`; neither test replaces planning, result validation, or
evidence assessment.

## Gate Results

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS.
- `cargo test --workspace --all-features --locked`: PASS, 25 tests.
- `cargo deny check`: PASS; advisories, bans, licenses, and sources are clean.
- `cargo run --locked --bin validate-fixtures`: PASS, 4 fixtures and 52 public contract instances.
- Pinned Quoin `7072d65` plus spec-artifacts-process `61a20e` baseline: PASS, 33 methods keyed by `techniqueId`.
- `quire coverage --scope <repo> --json`: PASS, 78/78 targets, 25/25 Rust test symbols tagged and bound, no unbacked row, status lie, or untracked symbol.
