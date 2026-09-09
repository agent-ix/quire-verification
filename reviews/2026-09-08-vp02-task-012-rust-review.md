---
id: SR-100
title: "Rust review — VP02 strict shared-reference reader"
type: SpecReview
analysis: code-review
scope: "3dbe798..90ef4ac: Cargo.toml, Cargo.lock, build.rs, contracts/shared-reference*, fixtures/shared-reference-2-draft/, src/contracts.rs, src/shared_reference.rs, tests/shared_reference.rs, spec/tests.md"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/Plan-003"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Task-012"
    type: reviews
---

## Summary

Applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` to the exact Task-012
candidate at `90ef4ac`. The review covered repository conventions, strict raw
JSON intake, schema generation and runtime validation, version allocation,
authority and identity selection, JCS and byte-digest separation, feature-set
semantics, source maps, explicit migrations, state rollback, local snapshot
containment, typed errors, panic and unsafe surfaces, resource bounds, compiled
trace markers, test substance, and the required executable gates. No open
finding remains.

## Verdict

**PASS** — the reviewed Task-012 Rust delta has no open Critical, High, Medium,
or Low finding, and every required executable gate exits successfully.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-001 | high | Resolved during implementation review: Typify unified the two canonical-identity digest alternatives as a 64-character newtype, so the valid 75-character `sha256-jcs:` vector passed the runtime schema but failed generated deserialization. Constructive generation now relaxes only that conflicting branch constraint while the complete retained schema remains the runtime validator. | `build.rs:65`, `build.rs:84`, `tests/shared_reference.rs:154`, FR-016-AC-2 | implementation-bug-despite-evidence |
| FND-002 | high | Resolved during review: a bare integer beyond `u64`/`i64` could reach Serde's floating-point visitor and lose exact input identity before JCS. Strict intake now rejects such integer tokens before materialization; the adverse value `18446744073709551616` is compiled into TC-029. | `src/shared_reference.rs:487`, `src/shared_reference.rs:1040`, `tests/shared_reference.rs:187`, FR-016-AC-3 | implementation-bug-despite-evidence |
| FND-003 | medium | Resolved during review: an input that failed after artifact observation could poison the reader's immutable-identity state and make a later exact input report a false conflict. Each read and migration selection now rolls observed identities back on refusal, and TC-029 executes that sequence. | `src/shared_reference.rs:184`, `src/shared_reference.rs:240`, `tests/shared_reference.rs:280`, FR-016-AC-3 | implementation-bug-despite-evidence |
| FND-004 | medium | Resolved during review: manifest verification followed listed paths but did not refuse unlisted fixture files or paths resolving outside the selected root. It now requires regular files under the canonical root and exactly fourteen listed fixture entries; TC-031 adds an unlisted file and observes `invalid_wire`. | `src/shared_reference.rs:505`, `src/shared_reference.rs:556`, `tests/shared_reference.rs:908`, FR-016-AC-7 | correct-requirement-no-evidence |
| FND-005 | medium | Resolved during review: canonicalization preflight could classify an incomplete foreign-domain object as unsupported before the closed schema classified its malformed shape. Preflight now assigns `unsupported_canonicalization` only when all four selection fields are strings; incomplete shapes reach schema refusal as `invalid_wire`. | `src/shared_reference.rs:794`, `tests/shared_reference.rs:341`, FR-016-AC-3 | implementation-bug-despite-evidence |
| FND-006 | medium | Resolved at the coverage gate: one Rustfmt-wrapped trace attribute compiled through `ix-trace-rs` but left FR-016-AC-1, AC-2, and AC-4 unbound in Quire. Rustfmt-stable single-line attributes now bind all seven criteria, and the non-minted `IT-002` tag was removed. | `tests/shared_reference.rs:136`, FR-016-AC-1..7 | correct-requirement-no-evidence |
| FND-007 | low | Resolved during review: three long acceptance procedures failed strict Clippy and obscured individual failure scenarios. They are now small traced entry points over bounded helpers, with no lint allowance. | `tests/shared_reference.rs:145`, `tests/shared_reference.rs:513`, `tests/shared_reference.rs:719` | implementation-bug-despite-evidence |

## Failure-Scenario Review

- Exact version-2 artifact/semantic records and retained version-1
  source-map/migration records are the only allocations. Unknown or crossed
  versions cannot select another decoder, and historical artifact/semantic
  records remain refused.
- Duplicate decoded member names, including escape-equivalent keys at the root
  or in a nested object, refuse before their containing map is created. The
  16 MiB, 128-level, and 100,000-item ceilings remain explicit.
- All six verification roles and the five shared exact-reference roles are
  checked against local bytes and selected authority, identity, revision,
  digest, and wire values. Pairwise nearby roles and every declared one-axis
  mutation refuse without weakening the accepted role.
- Required features compare as a decoded `BTreeSet`; reordering succeeds,
  duplicates fail schema validation, and unsupported values remain distinct
  from a supported but inexact selection. Language and semantic-profile
  versions use explicit local support registries.
- Raw SHA-256 artifact identity and `sha256-jcs:` semantic identity remain
  separate. The selected JCS tuple is exact, a missing qualified derivation is
  stale, malformed prefixes are invalid wire, and a valid wrong digest is an
  identity mismatch.
- Source-map spans are bounded and UTF-8 aligned. Ordered positive-length
  segments cover every body byte exactly once, do not overlap in the original,
  and bind equal byte slices under only the three declared layout deletions.
- Migration selection accepts one exact forward mapping, rejects zero, and
  reports multiple applicable records as ambiguous. Old/new artifact identity
  must remain equal apart from the selected reference encoding; reverse and
  transitive joins are absent.
- Snapshot verification checks both schemas and every one of the fourteen
  upstream JSON fixtures against the retained manifest before interpretation.
  It neither imports the producer crate nor uses a network schema source.

## Boundary Review

Production code contains no `unsafe`, request-derived panic, unchecked numeric
cast, recursion, async work, lock, channel, shell, subprocess, or network call.
All public fallible APIs return the crate's `VerificationError` and stable error
catalog. Raw input is borrowed; accepted values and local expectation data are
owned. Identity-map updates are transactional across a complete reference.
Runtime schema compilation is cached without network retrieval. Tests use only
the public crate boundary, temporary directories, retained data, and bare
compiler-checked `ix_trace_rs::trace` markers.

## Gate Results

- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets --all-features --locked -j 2 -- -D warnings`:
  PASS on exact reviewed head.
- `cargo test --workspace --all-features --locked -j 2`: PASS, 41 first-party
  tests plus 3 vendored `ix-trace-rs` self-tests. The trace-only review fix then
  passed the exact `cargo test --test shared_reference --locked -j 2` target.
- `cargo run --locked -j 2 --bin validate-fixtures`: PASS, 4 fixtures and 52
  public contract instances.
- `cargo run --locked -j 2 --bin render-fixtures`: PASS; the repository remained clean.
- `cargo deny check`: PASS; advisories, bans, licenses, and sources are clean.
  Accepted policy warnings remain for transitive `hashbrown` and `syn` versions.
- `quire validate --scope . "spec/**/*.md" --summary`: PASS, 157/157 documents
  grammar-clean before this review artifact.
- `quire validate --scope . "plan/**/*.md" --summary`: PASS, 39/39 documents grammar-clean.
- `quire coverage --scope . --json`: FR-016 is 7/7 backed; TC-029 and TC-031
  are backed; 44/44 reported Rust candidates are tagged and bound; no
  first-party symbol is untracked; no status lie exists; 118/160 total rows are
  backed. The remaining 42 rows belong to declared pending Plan-003 through
  Plan-005 work.
