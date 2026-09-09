---
id: SR-079
title: "Rust review — observed VP04 process containment"
type: SpecReview
analysis: code-review
scope: "7650708..8d0aa4c: src/contracts.rs, src/native_process.rs and tests/native_process.rs"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/Plan-005"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Task-020"
    type: reviews
---

## Summary

Applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` to the exact Task-020
remediation candidate at `8d0aa4c`. The review covered repository conventions,
process-group signal/reap/observation ordering, cleanup deadline accounting,
stable error precedence and context, pipe-reader lifecycle, immutable fixture
admission, panic and unsafe surfaces, trace tags, and the required Rust gates.
No open finding remains.

## Verdict

**PASS** — every finding was resolved in the reviewed candidate and the local
Rust, fixture, dependency, QUOIN validation, and repeated lifecycle gates pass.
Hosted CI remains required on the pushed revision because it exposed the defect.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|----|----------|---------|------|--------------|
| FND-001 | high | Resolved: successful `kill_process_group` previously meant only that `SIGKILL` was queued, yet the executor returned `process_group_terminated: true` before a descendant had stopped. Cleanup now signals, observes direct-child reap, observes group disappearance within one second, and reports `process_containment_failure` with the original trigger when proof fails. | src/native_process.rs:394, src/native_process.rs:424, src/native_process.rs:542, NFR-006-AC-3..5, NFR-006-AC-8 | implementation-bug-despite-evidence |
| FND-002 | medium | Resolved: each parallel test copied executable bytes immediately before launch, permitting hosted Linux to return `ETXTBSY`. Each case now creates an isolated hard link beside the completed Cargo-built fixture, so no writable fixture handle is introduced. | tests/native_process.rs:28, TC-038 | correct-requirement-no-evidence |
| FND-003 | medium | Resolved during review: the first cleanup implementation concatenated fallback-kill and reap errors into `group_signal_error`. Cleanup now retains separate signal, fallback, reap, and observation contexts and never labels an unobserved group as terminated. | src/native_process.rs:414, src/native_process.rs:542, NFR-006-AC-8 | implementation-bug-despite-evidence |

## Failure-Scenario Review

- Deadline, output-limit, signal, status, read, monitor, and surviving-descendant
  triggers all enter the same bounded cleanup path. A clean path preserves the
  trigger error and reports reaped/terminated true only after observation.
- A signal, fallback kill, direct-child reap, group query, or one-second cleanup
  failure returns `process_containment_failure`; `trigger` retains the original
  stable code and each cleanup phase has its own context key.
- The direct child is polled with `try_wait`, which reaps it without an unbounded
  blocking wait. Process-group existence is polled with the selected rustix
  wrapper and `Instant`; both loops share the exact cleanup deadline.
- Reader threads are joined only after the group is observed absent. A failed
  containment proof cannot be mislabeled as clean termination or a logical
  result.
- The compiled controlled fixture is hard-linked from the completed Cargo
  artifact into a unique same-filesystem root. Descriptor substitution coverage
  remains intact because the test renames only its private link.

## Boundary Review

Production code adds no `unsafe`, panic, shell, inherited environment, async
runtime, shared lock, recursion, unchecked PID conversion, unbounded output, or
unbounded cleanup loop. The public error enum remains exhaustive and the new
stable code round-trips through `from_code`. Test panics remain confined to the
controlled harness. All seven first-party TC-038 symbols compile bare
`ix_trace_rs::trace` attributes; AC-8 is bound by both the real descendant case
and the injected zero-budget observer control.

## Gate Results

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`:
  PASS.
- `cargo test --workspace --all-features --locked`: PASS, 38 first-party tests
  plus 3 vendored `ix-trace-rs` self-tests.
- One hundred consecutive executions of all five concurrent native-process
  integration tests: PASS.
- `cargo run --locked --bin validate-fixtures`: PASS, 4 fixtures and 52 public
  contract instances.
- `cargo deny check`: PASS for advisories, bans, licenses, and sources; accepted
  duplicate-version warnings remain for transitive `hashbrown` and `syn`.
- `quire validate --scope <repo> "spec/**/*.md" --strict --summary`: PASS,
  141/141 documents grammar-clean.
- `quire validate --scope <repo> "plan/**/*.md" --strict --summary`: PASS,
  39/39 documents grammar-clean.
