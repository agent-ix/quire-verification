---
id: SR-173
title: "Rust review of qualification corpus retention"
type: SpecReview
analysis: code-review
scope: "quire-verification de8379e..971073f; Task-026, contracts/e02-draft-2.schema.json, src/qualification_corpus.rs, src/shared_reference.rs, tests/qualification_corpus.rs, tests/qualification_profile.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-023", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TC-040", type: references }
---

## Summary

The required `/home/peter/dev/agent-skills/rust-review/SKILL.md` review checked
the committed Task-026 implementation against `AGENTS.md`, `rustfmt.toml`,
`deny.toml`, the workspace lints, FR-023, TC-040, and the repository's bare
compiler-checked `ix-trace-rs` convention. Three findings were resolved before
the reviewed revision `971073ffaefd87ccd1ac4ab474eaf9ad7e4c2d2b`; no unresolved
Rust correctness, panic-surface, numeric-boundary, seam, resource-bound, or
test-integrity defect remains.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Resolved: an early replay record proved only an output digest, so a completed run producing the same bytes could be attached to a different generator, seed, invocation, parent, or transformation. The retained run now carries the entry ID and JCS identity of the exact lineage inputs, and TC-040 changes a seed while retaining the old run to prove refusal. | FR-023-AC-3, FR-023-AC-4, `src/qualification_corpus.rs`, `tests/qualification_corpus.rs` | implementation-bug-despite-evidence |
| FND-002 | medium | Resolved: the first mutation-cycle check restarted a tree walk at every mutation, so a valid 100,000-entry chain could require roughly ten billion ordered-map steps. The final implementation uses one three-state iterative traversal and finalizes each node once. | FR-023-AC-3, `src/qualification_corpus.rs` | implementation-bug-despite-evidence |
| FND-003 | medium | Resolved: canonical-envelope deduplication did not reject one immutable artifact identity carrying different digests when opaque proofs had been assembled through separate strict-reader instances. The corpus now compares the complete role-specific artifact identity without its digest, and TC-040 constructs independently validated conflicting proofs that fail only at the corpus boundary. | FR-023-AC-3, `src/qualification_corpus.rs`, `tests/qualification_corpus.rs` | implementation-bug-despite-evidence |

## Review checklist

The public boundary returns the crate's `VerificationError` and adds one stable
`invalid_corpus_manifest` code. Caller-supplied JSON passes the existing 16 MiB,
128-level, 100,000-array-item resource envelope and the authoritative schema
before semantic access. The proof slice has an explicit count bound. Mutation
ancestry is iterative and linear in the number of retained entries. The change
adds no supplied-input panic, recursion, `unsafe`, integer cast, async or blocking
work, lock, test-only production branch, feature bypass, broad lint allowance,
or first-party executable language.

`ValidatedSharedReference` remains opaque and now owns the exact bytes that its
reader already digest-checked. Corpus construction matches JCS reference bytes
and exact roles, rejects duplicate immutable input identities even across reader
instances, and copies only proved input bytes into an immutable deterministic
map. Profile identity, population identity, and the ordered required-class set
must equal the frozen schema-derived profile. Every missing class has exactly one
sorted owner-attributed gap; covered and unknown classes cannot carry gaps.

Generated and mutation records are closed schema objects. Their replay artifacts
are strict-reader-proved `run-artifact` values whose closed payload binds the
entry, JCS lineage identity, and SHA-256 output bytes. Mutation parents must exist,
cannot name self, and form an acyclic graph. The manifest representation is
generated from the authoritative schema, while the schema's public root still
admits exactly the five E02 contracts.

TC-040 executes the public API and schema. Its five tests retain and read exact
bytes, exercise complete and owned-gap outcomes, delete every required dimension,
change identity, class, rights, role, replay output, and lineage independently,
combine conflicting proofs from independent readers, and construct missing,
self-referential, and cyclic parent graphs. TC-039 separately proves that freezing
preserves the owner-authored nonempty guarantee set despite commands and evidence
techniques. Every acceptance test carries a bare `#[trace(...)]` marker.

## Gate results

The repository gates ran from the reviewed checkout with Cargo restricted to two
build jobs. Their terminal results were:

```text
$ cargo fmt --all -- --check
(no output; exit 0)

$ cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
Checking quire-verification v0.1.0
Finished `dev` profile; exit 0

$ cargo test --locked --workspace --all-features
first-party Rust tests: 77 passed; 0 failed; 2 external-evidence lanes ignored
vendored ix-trace-rs tests: 3 passed; 0 failed
doc tests: 0 passed; 0 failed; 4 ignored

$ cargo deny check
advisories ok, bans ok, licenses ok, sources ok
exit 0; existing duplicate-version warnings: hashbrown, syn

$ cargo run --locked --bin validate-fixtures
validated 4 fixtures and 52 public contract instances against e02-draft-2.schema.json

$ quire validate --scope . "spec/**/*.md" --summary
237/237 docs grammar-clean (100%); 0 grammar findings

$ quire validate --scope . "plan/**/*.md" --summary
52/52 docs grammar-clean (100%); 0 grammar findings

$ quire coverage --scope . --json
175/209 matrix rows backed; 82/82 discovered Rust candidates tagged and bound;
TC-040 and FR-023 4/4 backed; their constraint-table rows are backed through TC-039/TC-040
```
