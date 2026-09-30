---
id: TC-022
title: "Enforce the Rust qualified-path implementation"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/NFR-004"
    type: "verifies"
---
# TC-022: Enforce the Rust qualified-path implementation

## Description

Verify executable repository code is Rust, unsafe code is forbidden, every
required Rust quality gate passes, and maintained toolchain/dependency guidance
describes the current Rust implementation.

## Test Procedure

Inspect governed paths for JavaScript or TypeScript executable files, then run
`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features --
-D warnings`, `cargo test --workspace --all-features`,
`cargo fuzz build -D --sanitizer none`, and `cargo deny check`.
Run `quire coverage --scope . --json` and confirm acceptance-evidence tests use
the imported bare `#[trace(...)]` attribute from `ix-trace-rs`, and confirm each
TC/AC argument in those attributes is discovered and bound. A marker that
compiles but hides one or more continuation-line arguments from the configured
Quire Rust trace reader fails this check. Report the first-party population as
repository-owned Rust under `src/`, `tests/`, and `fuzz/`; classify any symbols discovered
in Cargo-resolved dependencies as dependency-owned and governed by that dependency's matrix
rather than silently including them in the first-party denominator. On a clean CI
runner with no cross-repository credential, resolve the trace crate from its
public upstream repository before running
the Rust gates. Read `AGENTS.md` and `LICENSE-DECISION.md`; require an available
Cargo gate and pointers to `Cargo.toml` and `Cargo.lock`, and reject claims that a removed npm manifest or
JavaScript package remains current implementation evidence.

## Expected Results

No JavaScript or TypeScript executable source remains, no unsafe code compiles,
all five Rust gates exit successfully, and compiler-checked trace markers back
every claimed matrix row and each named acceptance criterion. The coverage claim
states its first-party population and separately identifies vendored dependency
symbols as outside that population. The clean runner obtains the trace
macro source without private dependency authentication. Maintained contributor
and license guidance names the implemented Cargo workflow and Rust dependencies
through `Cargo.toml` and `Cargo.lock`, without stale npm or
JavaScript implementation claims.
