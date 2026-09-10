---
id: SR-207
title: "spec-fuzz — Task-029 qualification surfaces"
type: SpecReview
analysis: evidence
scope: "Cargo.toml, spec/functional/FR-016-read-shared-references-strictly.md, spec/functional/FR-017-retain-oracle-dependence.md, spec/non-functional/NFR-005-bounded-json-ingestion.md, spec/non-functional/NFR-006-bound-native-processes.md, spec/reviews/data/q13-task029-fuzz-advice.json, spec/reviews/data/q13-task029-fuzz-catalog.json"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/FR-016"
    type: reviews
  - target: "ix://agent-ix/quire-verification/FR-017"
    type: reviews
  - target: "ix://agent-ix/quire-verification/NFR-005"
    type: reviews
  - target: "ix://agent-ix/quire-verification/NFR-006"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Task-029"
    type: references
---
# Spec Fuzz for Task-029 Qualification Surfaces

## Summary

The installed QUOIN advisor and active verification catalog selected four obligations
whose top applicable method has `evidenceKind: Fuzz`. The host provides
`cargo-fuzz 0.13.2`, but the repository has no consumer-owned fuzz workspace, manifest,
or target directory. The installed `spec-fuzz` workflow forbids creating that workspace,
so this run wrote no fuzz target and retained one finding for each unserved obligation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-016-AC-3 selects catalog method `fuzzing`, but no repository fuzz harness exists in which a runnable target can be emitted. | FR-016-AC-3, Cargo.toml, quoin#361 |
| FND-002 | high | FR-017-AC-4 selects catalog method `fuzzing`, but no repository fuzz harness exists in which a runnable target can be emitted. | FR-017-AC-4, Cargo.toml, quoin#361 |
| FND-003 | high | NFR-005-AC-1 selects catalog method `fuzzing`, but no repository fuzz harness exists in which a runnable target can be emitted. | NFR-005-AC-1, Cargo.toml, quoin#361 |
| FND-004 | high | NFR-006-AC-5 selects catalog method `fuzzing`, but no repository fuzz harness exists in which a runnable target can be emitted. | NFR-006-AC-5, Cargo.toml, quoin#361 |
| FND-005 | low | FR-017-AC-4 is a dependence-graph relation over already typed inputs rather than an arbitrary byte/text input surface. Its first-ranked fuzz recommendation is unsuitable; the existing property/mutation path remains the applicable evidence. | FR-017-AC-4, `assess_evidence` |

## Selection and Harness Evidence

The selection was computed from `q13-task029-fuzz-advice.json` and
`q13-task029-fuzz-catalog.json`; no local method list was used. The catalog declares
`fuzzing`, `grammar-based-fuzzing`, and `concolic-execution` as Fuzz-kind methods. The
advisor selected `fuzzing` first for exactly FR-016-AC-3, FR-017-AC-4, NFR-005-AC-1,
and NFR-006-AC-5. Manual method-fit review retains FR-016-AC-3, NFR-005-AC-1, and
NFR-006-AC-5 as input, parser, or malformed-response surfaces. FR-017-AC-4 is a typed
dependence-graph relation and remains FND-005 rather than receiving a token target.

`cargo fuzz --version` reports `cargo-fuzz 0.13.2`. Repository inspection finds only the
root crate manifest and the vendored `ix-trace-rs` manifest; it finds no `fuzz/Cargo.toml`
or `fuzz_targets` directory. QUOIN issue 361 records the workflow ambiguity between its
global-tool detection rule and its ban on creating a consumer-owned fuzz workspace.

## Handoff

These targets are **undischarged**. A fuzz target that has never run is a search never
started, and the evidence store will report it as such until a run is recorded.

No target exists yet. Task-029 may establish the owner-authorized standard cargo-fuzz
workspace as a separately specified consumer change, then rerun `spec-fuzz` to ground and
emit the selected targets. A later bounded campaign must retain its exact run result;
creating or compiling targets alone will not close any finding or qualification claim.
