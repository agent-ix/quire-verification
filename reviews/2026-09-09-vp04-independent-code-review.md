---
id: SR-151
title: "Independent code review — VP04 native parser portfolio at 64927b7"
type: SpecReview
analysis: code-review
scope: "PR #11 at 64927b7; src/native_process.rs, src/report.rs, src/parser_portfolio.rs, src/measurement.rs; tests/; evidence/vp04-parser-portfolio-core.json; NFR-006, FR-019..FR-021, IT-003"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/Plan-005"
    type: reviews
  - target: "ix://agent-ix/quire-verification/NFR-006"
    type: references
---

## Summary

Every claim in the PR body that can be checked from outside, checks. The 69
passing tests and 2 ignored lanes reproduce exactly; `cargo deny` is clean;
`validate-fixtures` reports 4 fixtures and 52 contract instances; the trace
census is 71 candidates, 71 tagged, 71 bound with no unbacked rows; the sealed
core digest recomputes byte-for-byte from the committed evidence in an
independent implementation; and all four recorded candidate identities match the
real `quire-spec-language` revision, which is published. The descriptor
containment is genuine — execution goes through `/proc/self/fd/N`, not a
re-opened pathname. Two gaps: the symlink refusal that NFR-006-AC-6/AC-7 rest on
is verified by a case that a symlink-following implementation also refuses, and
the sealed artifact this PR exists to produce is read by no test that runs.

## Verdict

**CONDITIONAL** — no high findings. Two mediums, both closable without changing
behaviour.

## Gates run at `64927b7`

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --locked --all-targets --all-features -j 2 -- -D warnings` | pass |
| `cargo test --workspace --all-features --locked -j 2` | **69 passed, 0 failed**, 2 ignored (the VP04 lanes) + 2 vendored doc-tests — matches the PR body exactly |
| `cargo deny check` | advisories, bans, licenses, sources — all ok |
| `cargo run --locked --bin validate-fixtures` | "validated 4 fixtures and 52 public contract instances" |
| `quire coverage` | `rust: candidates 71, tagged 71, bound 71`; `unbacked_rows: []` |

## Independent verification

| Claim | Check | Result |
| --- | --- | --- |
| Sealed core `sha256-jcs:db03d0c8…56f2` | recomputed the RFC 8785 digest of `core` from the committed bytes in a separate implementation | **exact match** |
| `Cargo.lock` `26c8dc23…` | `git show 2be73504…:Cargo.lock \| sha256sum` in `quire-spec-language` | **match** |
| Fixture `bb7a1876…` | same, `tests/fixtures/parent.native` | **match** |
| Retained copies | `fixtures/vp04/parent.native`, `parent-invalid.native`, `quire-spec-Cargo.lock` | **byte-identical** to the recorded digests |
| Candidate revision | `2be73504…` is an ancestor of `quire-spec-language` `origin/main` | published |
| CI trigger change | `on: workflow_dispatch:` alone, with a new test enforcing it | matches the standing repository policy; **not a finding** |

The digest reproduction is worth one note: the sealed core carries 68 numbers,
all integers well under 2^53 (`elapsedNanoseconds`, `repetition`,
`expectedExitCode`, `denominator`), and the summaries use a **lower median**
rather than a mean. That is what keeps a float out of the canonical form
entirely, so JCS number formatting never becomes load-bearing. Whether or not
that was the reason, it is the right choice and worth keeping deliberately.

## Mutation experiments

Three mutants, each applied to `64927b7` and reverted. The tree was clean before
and after.

| # | Mutant | Full suite |
| --- | --- | --- |
| M1 | Drop `ResolveFlags::NO_SYMLINKS` from the executable `openat2` | **all 69 pass** |
| M2 | Disable the assessment recomputation guard in `validate_core_assessment` | **FAIL** — `tc_037_refuses_an_assessment_that_does_not_recompute` |
| M3 | Disable the `disposition["coreDigest"] != core_digest` binding | **FAIL** — `tc_037_refuses_duplicate_foreign_unsupported_and_invalid_digest_cases` |

M2 and M3 are the report layer's two load-bearing integrity claims, and both are
gated. M1 is FND-001.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | TC-038's symlink case uses an absolute target, which `RESOLVE_BENEATH` refuses on its own, so `RESOLVE_NO_SYMLINKS` is unguarded | tests/native_process.rs:132, src/native_process.rs:106 | correct-requirement-no-evidence |
| FND-002 | medium | The sealed core artifact is read by no test that runs; nothing re-derives its digest or re-validates it | evidence/vp04-parser-portfolio-core.json, tests/vp04_integration.rs:875 | correct-requirement-no-evidence |
| FND-003 | low | `quire coverage` reports one untracked symbol from `vendor/ix-trace-rs`, unmentioned by the "71/71 … no status lies" claim | vendor/ix-trace-rs/src/lib.rs:326 | missing-requirement |
| FND-004 | low | The finalizer lane defaults `VP04_PARSER` to a path under `/tmp`, so the lane is unreproducible for another reviewer without reading the source | tests/vp04_integration.rs:948 | correct-requirement-no-evidence |

## Finding detail

### FND-001 — the symlink refusal is verified by the wrong kind of symlink

`NativeProcessExecutor::admit` opens the executable with
`ResolveFlags::BENEATH | NO_MAGICLINKS | NO_SYMLINKS`. TC-038 puts a symlinked
candidate in its adverse list:

```rust
let link = candidate.root.path().join("linked-candidate");
symlink(&candidate.executable, &link).expect("candidate symlink");
adverse.push(link);
```

`candidate.executable` is an **absolute** path, so the link target is absolute,
and `RESOLVE_BENEATH` refuses an absolute symlink target regardless of
`RESOLVE_NO_SYMLINKS`. The case therefore passes for a reason that has nothing
to do with the flag it is meant to govern.

Measured. Removing `NO_SYMLINKS` from line 106 leaves the entire suite green —
69 passed, 0 failed. A probe with a *relative* target inside the same root then
separates the two implementations cleanly:

```rust
symlink("native-process-fixture", &link)?;      // relative, unlike TC-038
NativeProcessExecutor::admit(root, &link)
```

- at `64927b7`: refused, `InvalidExecutablePath`
- with `NO_SYMLINKS` removed: **ACCEPTED**

So NFR-006-AC-6/AC-7's symlink refusal currently has no gate. Nothing is wrong
with the implementation — it is correct, and the same probe proves it. The fix is
four lines: add a relative-target symlink to TC-038's `adverse` vector alongside
the absolute one. The root-symlink case at line 150 has the same shape and
deserves the same treatment.

### FND-002 — the artifact this PR produces is checked by nothing that runs

`evidence/vp04-parser-portfolio-core.json` is the sealed incomplete core. Its
digest is quoted in the PR body, in `plan/Plan-005-…/log.md`, and in SR-150's
FND-001. It is touched by exactly two things in the repository, and both are
`#[ignore]`d: `vp04_collects_and_retains_automated_core` **writes** it, and
`vp04_binds_external_disposition_to_retained_core` reads it.
`validate-fixtures` covers `fixtures/`, not `evidence/`. `tests/report.rs` and
`tests/parser_portfolio.rs` `include_str!` a different file
(`evidence/local-catalog-portfolio.json`).

Both ignores are honest — one needs the external `quire-spec` binary, the other
needs a disposition that does not exist yet — and the collector is a producer
rather than a test, which is why it cannot run by default. But the consequence
is that the committed seal has no ratchet: an edit to the file, or a change in
`report.rs` that alters what the core would contain, is caught by nothing.

The close needs no external parser and no new machinery, because everything is
already public API. One non-ignored test that reads the committed core,
recomputes `jcs_sha256(core)` against `coreDigest`, and re-runs the assessment
and binding validations would fail on exactly the drift that currently goes
unobserved. I recomputed the digest from the committed bytes in a separate
implementation and it matches today, so such a test starts green.

### FND-003 — one untracked symbol, from a vendored tree

`quire coverage` reports:

```
untracked: vendor/ix-trace-rs/src/lib.rs:326
           tests::the_kind_vocabulary_is_deliberately_not_checked → FR-003-CON-1
```

`FR-003-CON-1` belongs to `ix-trace-rs`, not to this repository, and it arrives
through a vendored dependency with its own matrix. The count in the PR body
("71/71 Rust candidates tagged and bound; no status lies") is accurate for the
first-party population and does not mention this row.

Recorded because the same class is open across the program right now — the
traceability population boundary is what
`agent-ix/engineering-assurance` NFR-005-AC-8 was just written to fix, and the
resolution there is a digest-bound manifest that classifies vendored trees out
explicitly rather than leaving them to be noticed per report. Either state the
population in NFR-004, or exclude `vendor/` in the coverage scope so the number
means what the sentence says.

### FND-004 — a default that points at `/tmp`

`vp04_binds_external_disposition_to_retained_core` resolves the parser as:

```rust
std::env::var_os("VP04_PARSER").map_or_else(
    || PathBuf::from("/tmp/quire-spec-language-agent-b-target/release/quire-spec"),
    PathBuf::from,
)
```

`evidence/vp04-parser-candidate.md` records the same isolated target directory,
and it is correctly presented as an environment fact rather than a promise. The
executable's **identity** is pinned by `CANDIDATE_DIGEST` and re-checked at
`tests/vp04_integration.rs:212`, so this is not a soundness hole — a different
binary at that path fails admission.

It is a reproducibility one. `/tmp` does not survive a reboot and belongs to one
machine, so the documented default cannot be followed by the next reviewer; they
have to read the source to learn that `VP04_PARSER` exists. Naming the variable
and the required digest in `evidence/vp04-parser-candidate.md`'s "Reproduced
checks" section, and defaulting to a path inside the repository's own ignored
build area, costs nothing and makes the lane runnable from the document.

## What is correct

- **The containment is real, not decorative.** `admit` opens the root and the
  root-relative executable with `openat2` under `BENEATH | NO_MAGICLINKS |
  NO_SYMLINKS`, digests the bytes *through the descriptor*, and then executes
  `/proc/self/fd/N` — not a re-opened pathname. TC-038 proves the pin end to end
  by renaming the admitted inode and writing a replacement at the original path;
  the retained descriptor still runs the admitted bytes. That is the correct
  design and the correct test for it.
- **Execution is bounded on every axis it claims:** `env_clear` then an explicit
  environment, `Stdio::null()` stdin, `process_group(0)` with group termination,
  a fresh per-run directory, a 16 MiB output cap, a 10 s deadline ceiling, a
  128 MiB executable admission limit, and a separate process-group cleanup
  budget. `unsafe_code = "forbid"` holds and no `#[allow]` was added.
- **The report layer's integrity guards are gated.** M2 and M3 both fail
  immediately, and `tc_037_refuses_duplicate_foreign_unsupported_and_invalid_digest_cases`
  is one test covering duplicate, foreign, unsupported and malformed-digest
  dispositions rather than four that each prove one spelling.
- **The candidate record is retention, not just a digest.** Unlike the pattern
  recurring elsewhere this week — a revision and a digest recorded with nothing
  kept — `fixtures/vp04/` holds byte-identical copies of the fixture, the
  mutation and the upstream `Cargo.lock`. I verified all three against the real
  upstream revision, and that revision is reachable from its published `main`.
- **The failure is reported rather than hidden.** SR-150 returns `FAIL` because
  IT-003-SC-09 awaits an external disposition, and the PR body says so in its
  first paragraph. A gap analysis that reports FAIL for a real, named, external
  reason is the system working.
- **The CI change is policy, not a lowered gate.** `on: workflow_dispatch:`
  alone is the standing rule for test pipelines in this ecosystem, and the
  commit adds a test that walks `.github/workflows/` and rejects
  `pull_request`, `pull_request_target`, `push`, `schedule`, `workflow_run` and
  `repository_dispatch`. Enforcing the policy in the suite is better than
  relying on it being remembered.
