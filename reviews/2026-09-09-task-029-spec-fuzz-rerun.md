---
id: SR-216
title: "spec-fuzz rerun — Task-029 grounded Rust targets"
type: SpecReview
analysis: evidence
scope: "8c1c859 plus fuzz/Cargo.toml, fuzz/fuzz_targets/, fuzz/seeds/, spec/reviews/data/q13-task029-fuzz-advice.json, spec/reviews/data/q13-task029-fuzz-catalog.json"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-016", type: reviews }
  - { target: "ix://agent-ix/quire-verification/FR-017", type: reviews }
  - { target: "ix://agent-ix/quire-verification/NFR-005", type: reviews }
  - { target: "ix://agent-ix/quire-verification/NFR-006", type: reviews }
  - { target: "ix://agent-ix/quire-verification/Task-029", type: references }
---
# Spec Fuzz Rerun for Task-029 Grounded Rust Targets

## Summary

After the owner-established consumer harness, the installed QUOIN workflow grounded and
emitted three of four selected Fuzz-kind obligations. All three targets compile locally in
the standard Rust fuzz harness with two Cargo jobs and no warnings. One selected
dependence-graph criterion remains a method-fit finding and received no target.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-017-AC-4 specifies a typed dependence-graph relation rather than an arbitrary byte/text input surface. Its first-ranked fuzz recommendation is unsuitable; the existing property and mutation path remains applicable. | FR-017-AC-4, `assess_evidence` |

## Emitted Targets

| Obligation | Grounded public entry point | Target |
| --- | --- | --- |
| FR-016-AC-3 | `shared_reference::parse_strict_bounded_json` | `fuzz_fr_016_ac_3_strict_shared_reference` |
| NFR-005-AC-1 | `contracts::parse_bounded_json` | `fuzz_nfr_005_ac_1_bounded_json` |
| NFR-006-AC-5 | `native_process::parse_native_json_output` | `fuzz_nfr_006_ac_5_native_response` |

Each target calls only the grounded Rust function, drops its fallible result, and performs
no file, network, process, correctness-assertion, panic, or unchecked result-unwrapping
work. Each carries the required `Trace:` and `spec-fuzz:` provenance plus a bare
compiler-checked `ix-trace-rs` marker on its named wrapper. Quoin issue 362 records why the
warning-producing macro doc-comment form was replaced by a named wrapper with a real Rust
source span.

The standard harness manifest names all three target binaries. Retained initial seeds live
under `fuzz/seeds/`; mutable campaign corpus, artifacts, coverage, and build products remain
ignored. `cargo fuzz build -D --sanitizer none --target-dir <worktree-local>` compiled all
three targets; this build performed no fuzz search.

## Handoff

These targets are **undischarged**. A fuzz target that has never run is a search never
started, and the evidence store will report it as such until a run is recorded.

The accepted Engineering Assurance Rust producer interface remains unavailable. No local
runner or substitute evidence record was added, and no target or seed is presented as a
completed campaign.
