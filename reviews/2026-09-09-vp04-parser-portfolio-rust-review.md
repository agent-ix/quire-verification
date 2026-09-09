---
id: SR-149
title: "Rust review — VP04 native parser portfolio and report core"
type: SpecReview
analysis: code-review
scope: "ef936fd..190e4d6: contracts/e02-draft-2.schema.json, src/native_process.rs, src/parser_portfolio.rs, src/measurement.rs, src/report.rs, tests/native_process.rs, tests/parser_portfolio.rs, tests/measurement.rs, tests/report.rs, tests/vp04_integration.rs"
review_set: subset
relationships:
  - target: "ix://agent-ix/quire-verification/Plan-005"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Task-020"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Task-021"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Task-022"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Task-023"
    type: reviews
  - target: "ix://agent-ix/quire-verification/Task-024"
    type: reviews
---

## Summary

Applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` to the exact VP04
Rust candidate at `190e4d6`. The review covered repository idioms, public error
and ownership boundaries, schema-derived wire types, exact native process and
parser identities, filesystem and process lifecycle, numeric/resource bounds,
panic and unsafe surfaces, report sealing, test substance, ignored integration
lanes, and bare compiler-checked `ix-trace-rs` bindings. No open finding remains.

The review resolved the findings below before accepting the candidate. The
retained automated core remains incomplete by design until one external
verification-owner disposition is supplied; that pending evidence is a Plan-005
qualification gate rather than an open Rust defect.

## Verdict

**PASS** — the reviewed VP04 Rust delta has no open high, medium, or low
finding. Every automated repository gate exits successfully, the exact real
parser collector passes, and the finalizer remains a separately named ignored
lane until external disposition evidence exists.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-001 | high | Resolved: the first integration draft could reuse local-catalog plan identities while reporting the real parser. A caller could therefore attribute unrelated tasks and results to the candidate. IT-003 now builds the parser-specific plan and the report validator cross-binds candidate revision, dependency closure, subject pair, plan, tasks, results, and assessment. | tests/vp04_integration.rs, src/report.rs:74, FR-021-AC-2 | implementation-bug-despite-evidence |
| FND-002 | high | Resolved: the admitted executable bytes were not compared with the candidate digest carried into measurements and the report. A caller could execute binary B while sealing candidate reference A. Admission now hashes the retained executable descriptor and portfolio entry rejects any different selected digest. | src/native_process.rs:117, src/parser_portfolio.rs:260, tests/parser_portfolio.rs, FR-019-AC-1 | implementation-bug-despite-evidence |
| FND-003 | high | Resolved: one ignored integration test originally claimed the complete IT-003 scenario set while producing an incomplete report with no human disposition. The collector and disposition finalizer are now distinct named ignored lanes; only the finalizer can emit the final report. | tests/vp04_integration.rs, evidence/vp04-parser-portfolio-core.json, IT-003 | correct-requirement-no-evidence |
| FND-004 | medium | Resolved: the initial report assembler accepted detached measurement, assessment, task/result, source, run, and bounded-claim fields. Altering those fields could preserve a valid schema and core digest while changing what the report attributed. Runtime validation now recomputes and cross-binds each identity before sealing. | src/report.rs:47, tests/report.rs, FR-021-AC-2..5 | implementation-bug-despite-evidence |
| FND-005 | medium | Resolved: report run validation required each retained run to name a command but did not require each command to have a run. Appending an unexecuted command could enter a sealed core. Command and run ID sets must now be equal and a discriminating test appends an unmatched command. | src/report.rs:404, tests/report.rs, FR-021-AC-2 | implementation-bug-despite-evidence |
| FND-006 | medium | Resolved: the separate manual plan task initially had no retained `TechniqueResult`, making the report unable to prove that it remained not-run. Execution now records one manual result with `not-run`, `not-evaluated`, `not-required`, and `not-started`, and report validation rejects a fabricated manual execution. | src/parser_portfolio.rs:872, src/report.rs:282, tests/parser_portfolio.rs, FR-019-AC-6 | implementation-bug-despite-evidence |
| FND-007 | medium | Resolved: parser expectation and result-context drift could reach native launch before exact role validation. Portfolio preflight now validates diagnostic shape and source binding, unique result identities, plan/task/technique roles, and the direct/adverse property relationship before executing a child. | src/parser_portfolio.rs:284, src/parser_portfolio.rs:390, tests/parser_portfolio.rs, FR-019-AC-1..2 | implementation-bug-despite-evidence |
| FND-008 | medium | Resolved: source admission used metadata-sized allocation and a lossy path representation. A growing input could exceed its intended allocation and a non-UTF-8 path could be rewritten in the exact protocol. Descriptor reads now stop at 16 MiB plus one detection byte, and exact paths must be UTF-8. | src/parser_portfolio.rs:419, src/parser_portfolio.rs:779, tests/parser_portfolio.rs, FR-019-AC-1..2 | implementation-bug-despite-evidence |
| FND-009 | medium | Resolved: multiline trace arguments compiled through `ix-trace-rs` but the configured Quire reader did not discover all FR-020 and FR-021 criteria. Each acceptance ID now has a rustfmt-stable bare trace attribute and coverage proves every named argument is bound. | tests/measurement.rs, tests/report.rs, NFR-004-AC-3 | correct-requirement-no-evidence |
| FND-010 | low | Resolved style note: exact parse validation converted an already validated path with `to_string_lossy`, obscuring the invariant at its comparison site. It now performs an explicit UTF-8 conversion and returns the crate's typed protocol error if the invariant is violated. | src/parser_portfolio.rs:779, FR-019-AC-1 | implementation-bug-despite-evidence |

## Failure-Scenario Review

- Missing, out-of-range, signaled, and mismatched expected statuses retain the
  qualified process-group cleanup behavior. Matching statuses 0, 1, and 255
  return their observed code with complete bounded streams.
- Direct parse, both format passes, formatted reparse, and the expected syntax
  refusal validate exact status, stream, strict single-object JSON, identity,
  revision, digest, counts, diagnostic span, and byte equality. Native failures
  become failed and not-evaluated technique results rather than escaping the
  portfolio as panics.
- Source reads, JSON payloads, native streams, elapsed values, repetitions, and
  child deadlines have finite checked bounds. Production code introduces no
  `unsafe`, panic, unchecked wire cast, async runtime, shared lock, shell, or
  inherited environment.
- Measurement observations retain the fixed 16-slot order, tagged valid and
  invalid variants, exact rejected scalar spelling, and lower-median summaries.
  Invalid samples cannot enter numeric summaries.
- The report core is schema-checked and then semantically cross-bound before
  JCS sealing. Zero dispositions yields `incomplete`; one exact external value
  yields `final`; duplicate, foreign, unsupported, and digest-invalid values
  retain distinct stable errors.

## Gate Results

- `cargo fmt --all -- --check`: `PASS` (exit 0).
- `cargo clippy --locked --all-targets --all-features -j 2 -- -D warnings`:
  `Finished dev profile` (exit 0).
- `cargo test --workspace --all-features --locked -j 2`: `PASS`, 69
  first-party and vendored unit/integration tests passed; the two explicitly
  named VP04 lanes and two vendored doc examples remained ignored.
- `cargo test --locked --all-features --test vp04_integration -j 2
  vp04_collects_and_retains_automated_core -- --ignored --exact --nocapture`:
  `1 passed; 0 failed; 1 filtered out`.
- `cargo run --locked --bin validate-fixtures`: `validated 4 fixtures and 52
  public contract instances against e02-draft-2.schema.json`.
- `cargo run --locked --bin render-fixtures`: exit 0; `git diff --check` is
  clean and retained fixtures did not change.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.
  Configured duplicate-version warnings remain for transitive `hashbrown` and
  `syn`.
- `quire validate --scope <repo> "spec/**/*.md" --summary`: `200/200 docs
  grammar-clean (100%); 0 grammar finding(s): none`.
- `quire validate --scope <repo> "plan/**/*.md" --summary`: `39/39 docs
  grammar-clean (100%); 0 grammar finding(s): none`.
- `quire coverage --scope <repo> --json`: `164/164` rows backed; `71/71` Rust
  candidates tagged and bound; `64/64` self-named candidates bound; no unbacked
  row or status lie. The only untracked symbol is inside the retained
  third-party `vendor/ix-trace-rs` snapshot.
