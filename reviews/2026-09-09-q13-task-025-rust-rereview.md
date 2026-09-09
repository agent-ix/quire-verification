---
id: SR-164
title: "Rust re-review — Plan-006 Task-025 review remediation"
type: SpecReview
analysis: code-review
scope: "quire-verification@c77079ab6119ede4dadcb530884b5b42a34b7eaa; build.rs, contracts/e02-draft-2.schema.json, src/qualification_profile.rs, tests/qualification_profile.rs, contracts/README.md, spec/functional/FR-022-freeze-qualification-claim.md, spec/tests.md, plan/Plan-006-quire-spec-production-qualification/tasks/Task-025-freeze-qualification-profile.md"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/Task-025", type: reviews }
  - { target: "ix://agent-ix/quire-verification/FR-022", type: references }
  - { target: "ix://agent-ix/quire-verification/TC-039", type: references }
---

## Summary

Re-applied `/home/peter/dev/agent-skills/rust-review/SKILL.md` after the independent
review of PR 16. The rereview covered the exact accepted-interface role gate, schema and
runtime agreement, owner-supplied budgets, strict-reader proofs, blocker boundaries,
claim non-inference, generated Rust, trace tags, and every required local gate. All five
review findings are resolved at the reviewed revision; Task-025 remains externally
blocked and does not claim a frozen production profile.

## Verdict

**PASS** — no open Critical, High, Medium, or Low finding remains in the reviewed
Task-025 implementation slice.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Resolved: accepted interface validation derived the expected artifact kind from the same reference it checked, so the role check could not fail | `src/qualification_profile.rs:203`, `contracts/e02-draft-2.schema.json:825`, `tests/qualification_profile.rs:322` |
| FND-002 | medium | Resolved: the schema imposed an unowned ten-second wall-clock ceiling instead of retaining the verification owner's positive phase budget | `contracts/e02-draft-2.schema.json:800`, `tests/qualification_profile.rs:417` |
| FND-003 | medium | Resolved: the Test Matrix execution totals did not include the passed TC-039 row | `spec/tests.md:420` |
| FND-004 | low | Resolved: documentation implied typed blockers covered absent intrinsic candidate identity references, although those references must exist before a profile candidate can be evaluated | `spec/functional/FR-022-freeze-qualification-claim.md:27`, `plan/Plan-006-quire-spec-production-qualification/tasks/Task-025-freeze-qualification-profile.md:32` |
| FND-005 | low | Resolved: TC-039 did not directly discriminate command or technique presence from an authored semantic guarantee, and the library's planned consumer was unstated | `tests/qualification_profile.rs:383`, `plan/Plan-006-quire-spec-production-qualification/tasks/Task-025-freeze-qualification-profile.md:41` |

## Failure Scenarios and Resolutions

An accepted `parser-compiler` interface previously passed with a `source` reference
because both sides of the role comparison came from that same reference. The schema now
constrains the role-to-kind relation and the Rust gate independently maps the semantic
role to the expected strict-reader artifact kind. TC-039 changes each of the six accepted
interface kinds and observes refusal.

An owner-authored phase budget above ten seconds previously failed schema validation
without an owning requirement. The schema now requires only a positive wall-clock
budget; the explicit two-worker and stream-size bounds remain. TC-039 freezes and reads
back a twenty-second owner budget, so merely accepting an input without preserving it
cannot satisfy the test.

TC-039 now removes the guarantee while retaining a command, then while retaining an
evidence technique, and observes refusal. FR-022 and Task-025 explicitly distinguish
mandatory identity references from the six external-interface acceptance states. The
module remains a tested library boundary until Task-026 can consume it after the stated
external prerequisites resolve.

## Boundary and Integrity Review

The public E02 root remains exactly five contract kinds. The internal profile definition
and generated Rust remain schema-derived. Portable contracts, shared references,
Engineering Assurance, and Quoin evidence map to retained `source` artifacts;
parser/compiler maps to `executable-projection`; production caller maps to `invocation`.
Every non-null envelope must still match exact canonical bytes and the role returned by
the strict FS02/FS05 reader.

No new unsafe block, panic on caller input, unchecked numeric conversion, IO, async
work, lock, recursion, unbounded retry, test-only production branch, lint suppression,
dependency, or workflow trigger was introduced. Test panics remain confined to invalid
test-fixture construction and impossible asserted outcomes.

The `functional-coverage` status diagnostic remains an upstream module-model issue:
`agent-ix/quire-rs#409` already tracks the additive per-reference `status_column` needed
to reconcile the structurally required `Coverage Status` header with the global `Status`
selector. This repository cannot correct that declaration without changing the owning
Filament module, which is outside Task-025 and explicitly excluded from this lane.

## Gate Results

- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` with `CARGO_BUILD_JOBS=2`: PASS.
- `cargo test --workspace --all-features --locked --no-fail-fast` with `CARGO_BUILD_JOBS=2`: PASS, 75 tests; 2 named VP04 lanes ignored.
- `cargo deny check --disable-fetch`: PASS; advisories, bans, licenses, and sources are clean; two existing duplicate-version warnings remain allowed by policy.
- `cargo run --locked --bin validate-fixtures`: PASS, 4 fixtures and 52 public contract instances.
- `quire validate --scope <repo> "spec/**/*.md" --summary`: PASS, 229/229 documents grammar-clean.
- `quire validate --scope <repo> "plan/**/*.md" --summary`: PASS, 52/52 documents grammar-clean.
- `quire coverage --scope <repo> --format human`: 170/209 rows backed; FR-022 is 4/4; all 77 read first-party and vendored Rust symbols are tagged and bound. TC-039 is carried by five bare `ix-trace-rs` tests.
