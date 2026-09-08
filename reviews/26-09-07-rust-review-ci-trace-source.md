---
id: SR-017
title: "Rust review — credential-independent ix-trace-rs source"
type: SpecReview
analysis: code-review
scope: "ee4139f: Cargo.toml, Cargo.lock, tests/verification.rs, vendor/ix-trace-rs/, .ix/modules/quire-verification/manifest.yaml, deny.toml"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/NFR-004"
    type: "reviews"
  - target: "ix://agent-ix/quire-verification/TC-022"
    type: "reviews"
---

## Summary

Applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` and the portable
Rust-style checklist to implementation revision `ee4139f`. The review covered
dependency identity, vendored-source integrity, panic and unsafe surface,
workspace membership, test substance, trace binding, dependency policy, and
the clean-runner failure path. The imported procedural-macro source and license
are byte-identical to upstream revision
`2ce4ebf47f726b9d76388220545cd0abda8a5cfb`; its local manifest contains only
the package metadata needed to build that source.

## Verdict

**PASS** — no open Rust-code finding remains. GitHub Actions run `34196284641`
proved head `a6be6c6` builds and passes every configured gate on a clean runner
without a cross-repository credential.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Resolved: the private Git dependency could not authenticate on a clean GitHub runner, so the first CI gate could never compile. | `Cargo.toml:29`, `vendor/ix-trace-rs/UPSTREAM.md:3` |
| FND-002 | medium | Resolved: the upstream source's self-tests and trace-shaped comments entered this repository's first-party QUOIN census and produced an untracked symbol. | `.ix/modules/quire-verification/manifest.yaml:5`, `tests/verification.rs:661` |
| FND-003 | low | Resolved: the Git-only dependency required a wildcard-version allowance and source exception in dependency policy. | `deny.toml:15` |

## Failure Scenarios and Resolutions

FND-001 occurred on GitHub Actions run `34194264312`: Cargo tried to clone the
private `ix-trace-rs` repository, found no cross-repository credential, and
exited 101 before fixture validation. The dependency is now an exact-version
path dependency. TC-022 checks that the lockfile contains no private Git URL,
the provenance record names the upstream revision, and the compiled source hash
matches the reviewed bytes.

FND-002 occurred when Quire walked the vendored Rust source as if it were
first-party evidence. The repository-local traceability module now excludes
`vendor/**` from the source census. The vendored macro still compiles and its
three upstream unit tests still run under Cargo; only attribution to this
repository's matrix is excluded. Coverage reports one excluded source file,
79/79 targets backed, and 25/25 first-party test symbols tagged and bound.

FND-003 allowed wildcard dependency declarations and one Git origin solely for
the prior trace dependency. The path dependency declares exact version
`=0.1.0`; wildcard declarations, unknown Git sources, and unknown registries are
denied.

## Rust Checklist

The change adds no production API, wire type, untrusted-input parser, async or
blocking path, shared state, integer conversion, unsafe block, or caller-driven
loop. TC-022's source inspection is appropriate because it verifies build,
provenance, lockfile, and coverage configuration rather than runtime behavior.
Each assertion fails on a concrete drift: dependency relocation, version
weakening, private-Git reintroduction, revision change, coverage-scope removal,
or source-byte change.

The vendored source's self-controlled `expect` constructs only Rust compiler
tokens from fixed format strings; caller-supplied trace tokens are rejected as
`compile_error!` output rather than panicking. No local fork of its macro logic
was introduced.

## Gate Results

- `cargo fmt --check`: PASS (exit 0, no output).
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings`: PASS (`Finished dev profile`).
- `cargo test --workspace --all-features --locked --offline`: PASS (`25 passed; 0 failed` project acceptance tests; `3 passed; 0 failed` upstream macro self-tests).
- `cargo deny check`: PASS (`advisories ok, bans ok, licenses ok, sources ok`).
- `cargo run --locked --offline --bin validate-fixtures`: PASS (`validated 4 fixtures and 52 public contract instances against e02-draft-1.schema.json`).
- `quire validate --scope /home/peter/dev/quire-verification-agent-b "spec/**/*.md"`: PASS (exit 0).
- QUOIN coverage with the repository-local source exclusion: PASS (`79/79`, `25/25`, zero unbacked rows, status lies, or untracked symbols; one excluded third-party source file).
- Clean GitHub runner: PASS (run `34196284641`, head `a6be6c6`, 4m34s).
