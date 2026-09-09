---
id: SR-152
title: "Independent re-review — VP04 review remediation at a05253b"
type: SpecReview
analysis: code-review
scope: "PR #11 at a05253b; SR-151 FND-001..FND-004; src/native_process.rs, src/report.rs; tests/native_process.rs, tests/verification.rs; NFR-004-AC-6, TC-022, TC-037, TC-038; evidence/vp04-parser-portfolio-core.json"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/NFR-006"
    type: references
  - target: "ix://agent-ix/quire-verification/NFR-004"
    type: references
---

## Summary

All four SR-151 findings are closed, and the two mediums are closed in the form
that matters: the mutations that found them now fail. Dropping
`RESOLVE_NO_SYMLINKS` from either the executable open or the root open fails
TC-038 at the exact new relative-target case. The committed VP04 core is
ratcheted by a default unit test that pins the JCS digest as a literal, so even
a tampered core re-sealed to stay internally consistent is refused. The seal
itself is unchanged — the remediation did not disturb the artifact it now
protects. No new findings.

## Verdict

**PASS**

## Gates run at `a05253b`, re-confirmed at `7e443cc`

The branch advanced to `7e443cc` ("docs: remove redundant remediation review
cycle") while this review was in progress. That commit deletes eight
`spec/reviews/vp04-review-remediation-*.md` lens artifacts and touches no
source, test, fixture, evidence or requirement file, so every result below
stands; the suite was re-run at `7e443cc` and remains green.


| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --locked --all-targets --all-features -j 2 -- -D warnings` | pass |
| `cargo test --workspace --all-features --locked -j 2` | **70 passed, 0 failed**, 2 ignored — 69 previously plus the new ratchet |
| `cargo deny check` | advisories, bans, licenses, sources ok |
| `cargo run --locked --bin validate-fixtures` | 4 fixtures, 52 contract instances |
| `quire validate` over `spec/` and `reviews/` | clean |
| Sealed core digest, recomputed independently | `sha256-jcs:db03d0c8…56f2` — **unchanged and matching** |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings | reviews/2026-09-09-vp04-independent-code-review.md |

## Mutation evidence for the closures

Every mutant below was applied to `a05253b` and reverted; the tree was clean
before and after.

| # | Mutant | Suite at `64927b7` | Suite at `a05253b` |
| --- | --- | --- | --- |
| M1a | Drop `NO_SYMLINKS` from the **executable** `openat2` | all 69 pass | **FAIL** — `tc_038` at `tests/native_process.rs:140` |
| M1b | Drop `NO_SYMLINKS` from the **root** `openat2` | not previously tested | **FAIL** — `tc_038` at `tests/native_process.rs:182` |
| M4 | Tamper inside the sealed core, then re-seal `coreDigest` so the file stays self-consistent | no test reads the file | **FAIL** — `tc_037_ratchets_…` at `src/report.rs:596` |
| M5 | Change one byte-run in the retained subject fixture `fixtures/vp04/parent.native` | no test reads the file | **FAIL** — `tc_037_ratchets_…` at `src/report.rs:562` |

M1b is the stronger half of the user's claim and it holds: the remediation
guards *both* resolve flags, not only the one SR-151 probed. The new relative
root-symlink case builds its link as `../<candidate-root-name>` inside a sibling
temporary directory, which is a relative target that `RESOLVE_BENEATH` does not
refuse on its own — so it discriminates for the same reason the executable case
now does.

M4 is the strongest form of the ratchet test. The constant

```rust
const CORE_DIGEST: &str = "sha256-jcs:db03d0c8…56f2";
```

is a literal in the test, and the test asserts both `jcs_sha256(core) ==
CORE_DIGEST` and `retained["coreDigest"] == CORE_DIGEST`. A forger who edits the
core and recomputes the embedded digest to match still fails. That is a real
ratchet rather than an internal-consistency check.

M5 confirms the ratchet reaches past the JSON: `validate_artifact` re-derives
each artifact reference from the committed fixture bytes, so the core is bound
to the retained inputs and not merely to itself.

## Closure detail

### FND-001 — both symlink guards are now gated

`tests/native_process.rs:134-136` adds a relative-target executable symlink to
the existing `adverse` vector, and `:160-186` adds a relative-target root
symlink resolved through `..`. Both mutants fail at those exact lines. The
implementation is unchanged, which is correct — it was already right; only the
evidence was missing.

### FND-002 — the committed core has a ratchet

`src/report.rs` gains a `#[cfg(test)] mod tests` carrying
`tc_037_ratchets_the_committed_vp04_core_and_recomputes_its_assessment`, traced
to FR-021-AC-1/AC-2/AC-5. It runs in the default suite — no external parser, no
`#[ignore]` — and it does more than the finding asked for:

- validates `Vp04PortfolioReport` and `Vp04AutomatedReportCore` against their
  schema definitions;
- pins the JCS digest as a literal and checks it two ways;
- re-derives the `subjectRef`, `mutationRef`, `dependencyClosureRef` and
  `environmentRef` artifact references from the retained bytes;
- pins the candidate digest and the upstream revision
  `2be73504edff1224d6396da15edca3775905d37b`;
- re-validates the review-procedure binding and the four oracle/observation
  contributions;
- re-runs `validate_measurement_bindings`, `validate_reproduction_bindings`,
  `validate_core_bindings`, `validate_core_assessment` and
  `validate_measurement_series`.

The two `#[ignore]`d lanes stay ignored, which remains right: one is a producer
that needs the external parser, the other needs a disposition that does not
exist. What changed is that their output is no longer unguarded.

### FND-003 — the population boundary is stated normatively

NFR-004 gains a Scope paragraph placing `vendor/` outside the first-party
population, NFR-004-AC-6 requires coverage claims to say so, a measurement row
targets zero undisclosed vendored symbols, TC-022's procedure and expected
result carry the classification, and `tc_022` asserts the exact sentences in
both documents through `include_str!`. The trace attribute was split so
`NFR-004-AC-6` is its own compiler-checked argument.

### FND-004 — the finalizer default is repository-local

`VP04_PARSER` now defaults to `root().join(".vp04-build/release/quire-spec")`,
`.vp04-build/` is gitignored, and `evidence/vp04-parser-candidate.md` gains a
"Disposition finalizer input" section giving the exact
`cargo build --locked --release --no-default-features --target-dir …` command and
restating that admission rechecks SHA-256
`c6bd7561…79e5`, so a different binary at either path is refused. The lane is now
runnable from the document.

## One observation, not a finding

The repository already contains
`.ix/modules/quire-verification/manifest.yaml`, whose declared purpose is
"Repository-local traceability scope for vendored third-party Rust" and which
carries `source_exclude: - "vendor/**"`. It cannot be used on its own —
`quire coverage --scope . --module .ix/modules/quire-verification` reports "no
module in scope declares a `traceability:` model" because that module declares
the exclusion but not the model — so the documented invocation in `README.md:70`
remains the bare `quire coverage --scope . --json`, which loads the installed
process module and therefore still surfaces the vendored `ix-trace-rs` symbol.

FND-003 asked for the population to be stated or excluded, and stating it is a
complete answer to the finding. Recorded only because a repeatable `--module`
invocation naming both modules would make the boundary mechanical as well as
declared, and the module that would do it is already written.

## What is confirmed unchanged

- The sealed core digest still recomputes to
  `sha256-jcs:db03d0c81141e06135fa68c84125253ed28b0e92e961abb5b0c44c8a7abd56f2`
  in an independent implementation. The remediation added a guard around the
  artifact without moving it.
- `cargo deny`, `validate-fixtures`, and the full spec/review `quire validate`
  remain clean, and no `#[allow]`, `unsafe`, or weakened flag was introduced.
- SR-150's `FAIL` still stands for its stated reason: Task-024 and IT-003-SC-09
  await the external verification-owner disposition. Nothing in this review
  changes that, and it should not be read as a defect.
