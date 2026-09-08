---
id: SR-059
title: "Rust review — VP04 bounded native process executor"
type: SpecReview
analysis: code-review
scope: "fa11e0e..be1d45b: Cargo.toml, Cargo.lock, src/contracts.rs, src/lib.rs, src/native_process.rs, tests/native_process.rs, tests/support/native_process_fixture.rs, spec/tests.md, plan/Plan-005-vp04-native-parser-portfolio/"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/Plan-005"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Task-020"
    type: reviews
---

## Summary

Applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` to the exact VP04
Task-020 candidate at `be1d45b`. The review covered repository conventions,
descriptor and process identity, child and descendant lifecycle, bounded stream
capture, deadline behavior, stable typed errors, panic and unsafe surfaces,
checked numeric conversion, public API shape, test substance, compiler-checked
trace markers, and every required executable gate. No open finding remains.

The first review pass found two high-severity containment failures and one
medium-severity test defect. Before acceptance, the requirements returned
through QUOIN `/specify` and the owner-selected all-lens `/spec-review` at
`d3dbf5b` and `fa11e0e`; the implementation now pins an `openat2` descriptor,
uses a dedicated process group, keeps the deadline active until both pipes and
the direct child finish, and observes process state without a fixed post-return
sleep.

## Verdict

**PASS** — the reviewed Task-020 Rust delta has no open Critical, High, Medium,
or Low finding, and every required executable gate exits successfully.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-001 | high | Resolved during review: validating a pathname before `Command::new` allowed replacement between check and launch; admission now uses retained root and executable descriptors with `openat2`, and TC-038 replaces the pathname before proving the admitted bytes execute. | src/native_process.rs:71, tests/native_process.rs:109, NFR-006-AC-7 | wrong-requirement |
| FND-002 | high | Resolved during review: terminating only the direct child could leave descendants alive or block pipe joins past the deadline; execution now creates and monitors a dedicated process group, kills it for every adverse completion, and reaps the direct child. | src/native_process.rs:121, src/native_process.rs:242, tests/native_process.rs:160, NFR-006-AC-3..5 | wrong-requirement |
| FND-003 | medium | Resolved during review: a fixed sleep followed by a liveness assertion was not deterministic evidence; TC-038 now uses returned process identifiers, `/proc` state, marker absence, and the executor's own completion boundary. | tests/native_process.rs:50, tests/native_process.rs:186, NFR-006-AC-3 | correct-requirement-no-evidence |

## Failure-Scenario Review

- Relative, outside-root, absent, non-regular, candidate-symlink, and root-symlink
  paths refuse before launch. A post-admission rename and malicious pathname
  replacement cannot change the executed inode.
- Missing, zero, negative, non-finite, and greater-than-ten-second deadlines
  refuse before launch. A direct child, a descendant retaining both pipes, and
  a descendant closing both pipes each produce a bounded typed failure with no
  live tested process.
- Exactly 16 MiB on each stream succeeds under concurrent output. The first
  excess byte on either stream produces `output_limit_exceeded`, identifies the
  stream/count/limit, terminates the group, and reaps the direct child.
- Nonzero status, signal death, invalid UTF-8 on either stream, malformed JSON,
  and a valid JSON method response remain distinct. Shell metacharacters are
  passed as literal arguments, inherited `HOME` is absent, and each invocation
  receives a different root-contained working directory.
- An execution profile without `openat2` support or `/proc/self/fd` refuses as
  `unsupported_execution_profile`; the reviewed Linux profile has no pathname
  fallback.

## Boundary Review

Production code contains no `unsafe`, panic, unchecked wire or process-ID cast,
async runtime, shared lock, recursion, shell, inherited environment, or
unbounded output collector. The two reader threads are joined after successful
completion or process-group termination, each buffer admits at most 16 MiB plus
one detection byte, and `Instant` owns deadline measurement. Public APIs borrow
paths and requests and return the crate's documented `VerificationError` with
stable catalog codes. The test-only feature compiles a controlled Rust child;
its single narrow `clippy::zombie_processes` allowance documents the exact
adverse descendant condition. All six TC-038 test symbols import and compile
bare `ix_trace_rs::trace` markers covering NFR-006-AC-1 through AC-7.

## Gate Results

- `cargo run --locked --bin validate-fixtures`: PASS, 4 fixtures and 52 public
  contract instances against `e02-draft-1.schema.json`.
- `cargo fmt --all --check`: PASS.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`:
  PASS.
- `cargo test --workspace --all-features --locked`: PASS, 37 first-party tests
  plus 3 vendored `ix-trace-rs` self-tests.
- Fifty repeated executions of the five native-process integration tests: PASS.
- `cargo deny check`: PASS; advisories, bans, licenses, and sources are clean.
  Accepted policy warnings remain for transitive `hashbrown` and `syn` versions.
- `quire validate --scope <repo> "spec/**/*.md" --summary`: PASS, 125/125
  documents grammar-clean.
- `quire validate --scope <repo> "plan/**/*.md" --summary`: PASS, 39/39
  documents grammar-clean.
- Explicit local-module `quire coverage`: NFR-006 is 7/7 backed, TC-038 is
  backed, 37/37 first-party Rust candidates are tagged and bound, no first-party
  symbol is untracked, and 107/157 total rows are backed. Remaining rows belong
  to declared pending or blocked VP02 through VP04 work.
