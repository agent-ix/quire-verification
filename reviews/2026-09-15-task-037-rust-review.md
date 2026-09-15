---
id: SR-259
title: "Rust review of Task-037 relational hyperproperty analysis"
type: SpecReview
analysis: code-review
scope: "src/relational_hyperproperty.rs, tests/relational_hyperproperty.rs, tests/props_fr_030.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-030", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TC-048", type: references }
  - { target: "ix://agent-ix/quire-verification/Task-037", type: reviews }
---
# Rust Review of Task-037 Relational Hyperproperty Analysis

## Summary

Reviewed the first-party finite relational provider, its identity and alignment
boundaries, ordered quantification, non-conclusive outcomes, assumptions,
oracle-dependence retention, property tests, and development gates. The review
found and resolved the request-path panic and nondeterministic fallback-identity
risks before completion; no production evidence, disposition, or release state
was used as a precondition.

## Verdict

**PASS for Task-037.** The provider evaluates only finite, closed, exact-trace
inputs; its semantic results and non-conclusions are typed, deterministic, and
covered by executable TC-048 evidence.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Resolved: recursive quantification trusted an earlier validation with `expect`; it now returns the typed invalid-request refusal if the population reference is absent. | `src/relational_hyperproperty.rs::quantify` | implementation-bug-despite-evidence |
| FND-002 | medium | Resolved: non-conclusive identity construction could have emitted a stage-only sentinel after serialization failure. It now has a deterministic SHA-256 fallback binding stage, code, and message. | `src/relational_hyperproperty.rs::non_conclusive` | implementation-bug-despite-evidence |
| FND-003 | low | Resolved: assumption identifiers, attributes, and target populations are now validated before assumptions restrict a trace population. | `src/relational_hyperproperty.rs::prepare_populations` | implementation-bug-despite-evidence |

## Rust Boundary Review

- Exact trace identifiers are a public newtype. Local names are retained only
  for display and never participate in assignment, correlation, or alignment.
  Globally duplicate identities refuse, and each prefix assignment excludes all
  already-bound identities.
- Quantifiers recurse in authored left-to-right order over a finite assignment
  space. The checked multiplication and explicit request bound return
  `requires-bound`; insufficient distinct traces return `incomplete` rather
  than manufacturing a truth value.
- Alignment is selected only by an exact ordered trace-identity pair and must
  be unique and total. Typed values are compared only at mapped positions; an
  exact-event alignment additionally checks equal event identities.
- Environment and declassification assumptions are explicit typed filters
  applied before quantification and included in JCS result identity. Oracle
  dependence is copied verbatim into each terminal outcome and never promoted.
- Production source contains no `unwrap`, `expect`, `panic!`, `todo!`,
  `unimplemented!`, `dbg!`, `unsafe`, ambient IO, clock input, or production
  gating. The only recursion is bounded by the validated finite prefix and
  assignment bound.
- Every new test has a compiler-checked `ix_trace_rs::trace` marker. Examples
  exercise paired witnesses, both assumption kinds, missing closure/alignment,
  ambiguous alignment, bound exhaustion, local-name non-correlation, duplicate
  rejection, and oracle preservation. The property test varies ordered values
  while holding duplicate local names constant.

## Local Gates

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --all-targets --all-features -- -D warnings` | pass |
| `cargo test --test relational_hyperproperty --test props_fr_030` | 6 passed |
| `cargo test --workspace --all-features` | pass; two named VP04 environment lanes and two vendored documentation tests ignored |
| `quire coverage --scope <repo> --json` | FR-030 AC 3/3 and constraints 3/3 backed; zero untracked symbols and status lies |

Native Rust `quoin write . --types SpecReview` remains blocked by
[agent-ix/quoin#526](https://github.com/agent-ix/quoin/issues/526), which reports
the missing `QUOIN_SEMANTIC_ROOT`. Per owner direction, npm Quoin 0.23.1 was
used only to render the authoring contract; native Quire performed validation
and trace reconciliation.
