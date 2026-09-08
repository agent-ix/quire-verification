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

Verify executable repository code is Rust, unsafe code is forbidden, and every
required Rust quality gate passes.

## Test Procedure

Inspect governed paths for JavaScript or TypeScript executable files, then run
`cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features --
-D warnings`, `cargo test --workspace --all-features`, and `cargo deny check`.
Run `quire coverage --scope . --json` and confirm acceptance-evidence tests use
the imported bare `#[trace(...)]` attribute from `ix-trace-rs`.

## Expected Results

No JavaScript or TypeScript executable source remains, no unsafe code compiles,
all four Rust gates exit successfully, and compiler-checked trace markers back
every claimed matrix row. This procedure remains pending until the Rust
migration is implemented.
