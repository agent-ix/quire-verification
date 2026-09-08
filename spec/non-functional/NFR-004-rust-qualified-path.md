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
---
# NFR-004: Rust implementation for the qualified path

## Statement

The compatibility slice SHALL implement production logic, command-line tools, and automated tests in Rust because the component is eligible for the qualified production verification path.

## Scope

This requirement applies to all executable code in `src/`, `tests/`, and
repository-owned command-line targets. JSON Schema, JSON fixtures, Markdown
specification artifacts, and CI workflow files remain data or configuration.

## Rationale

Qualified evidence processing needs predictable performance, explicit ownership,
typed errors, and a small runtime surface. TypeScript may remain at Quoin's
orchestration boundary, but this component exchanges JSON with it rather than
joining its runtime.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Non-Rust executable source files under `src/`, `tests/`, and repository CLI targets | 0 | 0 | Inspection (TC-022) |
| Unsafe Rust blocks | 0 | 0 | Test (TC-022) |
| Rust quality gates passing | 4 of 4 | 4 of 4 | Test (TC-022) |
| Rust test trace markers compiled through `ix-trace-rs` | Every test that verifies a matrix row | 100% | Coverage inspection (TC-022) |
| Clean-CI resolution of the reviewed `ix-trace-rs` source | Exact recorded upstream revision, with no cross-repository credential | 1 of 1 clean CI runs | Test (TC-022) |

## Verification

The repository gate rejects non-Rust executable source in the governed paths,
forbids unsafe code at the crate root, formats the crate, compiles every target
with Clippy warnings denied, runs the complete test suite, and checks dependency
licenses and advisories with `cargo deny`. The repository records and supplies
the exact reviewed `ix-trace-rs` source needed by those tests so a clean CI
runner does not depend on an unrecorded cross-repository credential.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-004-AC-1 | Governed executable paths contain no JavaScript or TypeScript source files. | Inspection (TC-022) |
| NFR-004-AC-2 | Formatting, Clippy with denied warnings, the complete Rust test suite, and dependency policy checks all exit successfully. | Test (TC-022) |
| NFR-004-AC-3 | Every Rust test used as acceptance evidence imports `ix_trace_rs::trace` and carries a compiler-checked bare `#[trace(...)]` marker naming its TC and verified acceptance criteria. | Inspection (TC-022) |
| NFR-004-AC-4 | A clean CI runner resolves the exact reviewed `ix-trace-rs` source from a repository-contained snapshot with recorded upstream revision and no cross-repository credential. | Test (TC-022) |

## Dependencies

- Constrains the full qualified path defined by [FR-001](../functional/FR-001-validate-public-contracts.md), [FR-005](../functional/FR-005-build-verification-plans.md), [FR-006](../functional/FR-006-match-evidence-exactly.md), and [FR-009](../functional/FR-009-retain-portfolio-failures.md).
