---
id: SR-261
title: "Rust review of Task-040 evidence graph and qualification ledger"
type: SpecReview
analysis: code-review
scope: "src/evidence_ledger.rs, tests/evidence_ledger.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-032", type: reviews }
  - { target: "ix://agent-ix/quire-verification/FR-033", type: reviews }
  - { target: "ix://agent-ix/quire-verification/FR-034", type: reviews }
  - { target: "ix://agent-ix/quire-verification/Task-040", type: reviews }
---
# Rust Review of Task-040 Evidence Graph and Qualification Ledger

## Summary

Reviewed the typed intake, claim-local graph discharge, counterexample lineage,
and 176-row ledger implementation against FR-032 through FR-034. The review
found and resolved unscoped-cycle refusal, unverified caller-supplied digests,
and missing unavailable-replay limitations before this artifact was written.

## Verdict

**PASS for Task-040's reviewed Rust subset.** The reviewed source and focused
executable tests pass, and the subsequent linked-plan cleanup restored the
crate-wide strict Clippy gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Resolved: evidence intake now computes SHA-256 from supplied producer bytes rather than trusting two caller-declared digest strings. | `src/evidence_ledger.rs::evidence_digest`, TC-050 |
| FND-002 | low | Resolved: only a cycle upstream of the requested claim refuses that discharge; an unrelated graph cycle leaves sibling evidence usable. | `src/evidence_ledger.rs::has_cycle_reaching_claim`, TC-050 |
| FND-003 | low | Resolved: an unavailable replay must retain a claim-specific limitation and cannot substitute for replay-qualified evidence. | `src/evidence_ledger.rs::validate_lineage`, TC-052 |
| FND-004 | low | Resolved: linked-plan public-contract documentation and small helper extraction restore crate-wide strict Clippy under `-D warnings`. | `src/linked_method_plan.rs`, `abbedfd` |

## Coverage

- Public identities distinguish source, package, model, binding, requirement,
  subject, assumptions, implementation, method, provider/run/result, artifact,
  corpus, lock, and decision kinds. Equal text does not erase identity kind.
- Intake validates SHA-256 over bytes without running a producer. Graph edges
  are append-only inputs, require scope/provenance, retain limitations, and
  reject only the affected circular discharge.
- Lineage requires ordered immutable parents, exact subject/claim/provider/run,
  correspondence, domain consistency, and retained non-replayable evidence.
- Ledger accounting uses a closed 176-row denominator, exact lock binding,
  independent per-row claim-class vectors, and separately attributed decisions.
  Planned/unexecuted production material remains explicit incomplete state and
  does not stop local development execution.
- `cargo fmt --all --check`: pass.
- `cargo test --test evidence_ledger -q`: 5 passed; all new tests carry
  compiler-checked TC/FR/IT trace attributes.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: pass.

Native `quoin write` failed with QM012 transport (tracked in
[quoin#535](https://github.com/agent-ix/quoin/issues/535)); npm Quoin 0.23.1
rendered the authoring contract. No production evidence or disposition was
required for this review.
