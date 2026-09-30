---
id: NFR-004
title: "Rust implementation for the qualified path"
type: NFR
quality_attribute: performance_efficiency
relationships:
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-005"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-006"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-009"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-025"
    type: "constrains"
---
# NFR-004: Rust implementation for the qualified path

## Statement

The compatibility slice SHALL implement production logic, command-line tools, and automated tests in Rust because the component is eligible for the qualified production verification path.

When maintained repository guidance describes the implemented toolchain or
dependency inventory, that guidance SHALL name the current Rust commands,
identify `Cargo.toml` and `Cargo.lock` as the authoritative package inventory
without presenting removed JavaScript manifests or packages as current implementation
evidence.

## Scope

This requirement applies to all executable code in `src/`, `tests/`, `fuzz/`, and
repository-owned command-line targets. JSON Schema, JSON fixtures, fuzz seed inputs, Markdown
specification artifacts, and CI workflow files remain data or configuration.
Dependency sources resolved by Cargo remain outside this repository's
first-party traceability population because their owning dependency retains its
own requirements matrix; coverage reports SHALL identify that population
boundary instead of presenting dependency symbols as unmentioned first-party gaps.

## Rationale

Qualified evidence processing needs predictable performance, explicit ownership,
typed errors, and a small runtime surface. TypeScript may remain at Quoin's
orchestration boundary, but this component exchanges JSON with it rather than
joining its runtime.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Non-Rust executable source files under `src/`, `tests/`, `fuzz/`, and repository CLI targets | 0 | 0 | Inspection (TC-022) |
| Unsafe Rust blocks | 0 | 0 | Test (TC-022) |
| Rust quality gates passing | 5 of 5 | 5 of 5 | Test (TC-022) |
| Rust test trace markers compiled through `ix-trace-rs` and bound by Quire | Every test and every named TC/AC argument that verifies a matrix row | 100% | Coverage inspection (TC-022) |
| Undisclosed vendored symbols in the reported first-party trace population | 0 | 0 | Coverage inspection (TC-022) |
| Clean-CI resolution of the reviewed `ix-trace-rs` source | Exact recorded upstream revision, with no cross-repository credential | 1 of 1 clean CI runs | Test (TC-022) |
| False current toolchain or dependency claims in maintained repository guidance | 0 | 0 | Test (TC-022) |

## Verification

The repository gate rejects non-Rust executable source in the governed paths,
forbids unsafe code at the crate root, formats the crate, compiles every target
with Clippy warnings denied, runs the complete test suite, and checks dependency
licenses and advisories with `cargo deny`. The same workspace lock and lints govern
the fuzz targets, and a separate cargo-fuzz build compiles their instrumented entry
points. The workspace pins `ix-trace-rs` to an exact reviewed revision of its public
upstream repository, so a clean CI runner resolves it without any
cross-repository credential.
TC-022 also reads the maintained contributor guidance and license decision,
requires the current Cargo gate and authoritative Rust inventory/rights
pointers, and rejects stale claims that removed npm manifests or JavaScript
packages remain current.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-004-AC-1 | Governed executable paths, including every qualification fuzz target, contain no JavaScript or TypeScript source files. | Inspection (TC-022) |
| NFR-004-AC-2 | Formatting, Clippy with denied warnings, the complete Rust test suite, the cargo-fuzz target build, and dependency policy checks all exit successfully. | Test (TC-022) |
| NFR-004-AC-3 | Every Rust test or fuzz target used as acceptance evidence imports `ix_trace_rs::trace` and carries a compiler-checked bare `#[trace(...)]` marker whose TC and acceptance-criterion arguments are each discovered and bound by `quire coverage --scope . --json`; compiler acceptance alone is insufficient. | Inspection (TC-022) |
| NFR-004-AC-4 | A clean CI runner resolves the `ix-trace-rs` source from its public upstream repository with no cross-repository credential. | Test (TC-022) |
| NFR-004-AC-5 | `AGENTS.md` names the available Cargo build/test gate, and `LICENSE-DECISION.md` identifies `Cargo.toml`/`Cargo.lock` as the current dependencies without claiming that a removed npm manifest or JavaScript dependency is current. | Test (TC-022) |
| NFR-004-AC-6 | Trace coverage claims state that the first-party population covers repository-owned Rust under `src/`, `tests/`, and `fuzz/` while dependency sources remain governed by their owning dependency's matrix. | Inspection (TC-022) |

## Dependencies

- Constrains the full qualified path defined by [FR-001](../functional/FR-001-validate-public-contracts.md), [FR-005](../functional/FR-005-build-verification-plans.md), [FR-006](../functional/FR-006-match-evidence-exactly.md), [FR-009](../functional/FR-009-retain-portfolio-failures.md), and [FR-025](../functional/FR-025-run-qualification-techniques.md).
