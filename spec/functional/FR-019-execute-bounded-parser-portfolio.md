---
id: FR-019
title: "Execute a bounded native parser portfolio"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-001"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/US-002"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/US-003"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-014"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-015"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-016"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-017"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-018"
    type: "depends_on"
---
# FR-019: Execute a bounded native parser portfolio

## Description

When a reviewed parser-portfolio request binds one exact native parser candidate,
the portfolio executor SHALL run direct parse acceptance and parse-format-reparse
stability as separate complementary techniques and retain a separate manual
disposition task over the same admitted subject and execution scope.

## Inputs

- An exact reviewed `quire-spec-language` Rust candidate and `quire-spec` CLI binding.
- A valid native source, one syntax-defect mutation derived from it, and their exact references.
- Expected source SHA-256 digests, clause and import counts, and the exact
  invalid-syntax diagnostic span selected before execution.
- A reviewed plan, policy, capabilities, runner requests, evidence obligations, and freshness limits.
- Exact expected exit code zero for successful parse and format invocations, and
  exact expected exit code one for the declared syntax-refusal invocation.
- An exact verification-owner procedure for the separate manual task.

## Outputs

- An immutable VP04 execution payload containing the exact plan, native
  results, assessment, and native artifact references for a downstream
  Engineering Assurance or Quoin host to retain.

## Behavior

- For the direct technique, the executor SHALL classify a zero-exit parse of the
  valid source as completed and satisfied only when stdout is one exact JSON
  object with `status: parsed`, the requested source identity and revision, the
  SHA-256 byte digest of the admitted source, and the expected clause and import
  counts, while stderr is empty.
- For the metamorphic technique, the executor SHALL format the valid parsed unit,
  retain the first format stdout as an exact native artifact in the fresh run
  directory, parse those exact bytes under a new exact revision, format them a
  second time, and compare the two format byte strings for equality.
- When every metamorphic command completes but the reparse or second-format
  relation differs, the executor SHALL record completed execution with a violated
  logical outcome and retain the exact differing artifacts.
- The executor SHALL decode successful parser stdout and expected diagnostic
  stderr as exactly one bounded JSON value followed only by JSON whitespace.
- If parser JSON is followed by a second value or any non-whitespace bytes, then
  the executor SHALL reject the response as malformed.
- When the seeded syntax mutation produces the expected located refusal, the
  executor SHALL accept only exit status 1, empty stdout, and one exact JSON
  diagnostic on stderr with `status: refused`, `phase: parse`, code
  `invalid_syntax`, the requested source identity and revision, and the declared
  byte, line, and column span.
- When the seeded syntax mutation matches that exact expected diagnostic, the
  executor SHALL record completed execution with a violated logical outcome.
- If the native process cannot start, exits outside the expected method protocol,
  emits output on the wrong stream, emits trailing non-whitespace bytes, returns
  mismatched identity or digest fields, or emits an unreadable result, then the
  executor SHALL record failed execution with a not-evaluated logical outcome.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-019-CON-1 | The portfolio SHALL execute only the reviewed bounded parser profile. | Scope | Inspection |
| FR-019-CON-2 | The portfolio SHALL NOT claim model linking, evaluation, generated backend execution, production fault injection, or general parser effectiveness. | Scope | Inspection |
| FR-019-CON-3 | The portfolio SHALL leave Quoin as evidence and authored-sufficiency owner. | Ownership | Test |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-019-AC-1 | The valid-source baseline contains completed, satisfied, fully participating direct-parse and parse-format-reparse results from the exact native candidate; each parser response has the prescribed exit status, stream, complete payload, source identity, revision, digest, and count or byte-equality relation. | Test (TC-034) |
| FR-019-AC-2 | A single declared syntax mutation yields exit 1 and its exact expected diagnostic object and source span on stderr as a completed, violated result, while wrong-stream, trailing-data, identity, digest, count, process, or protocol failures remain failed and not-evaluated. | Test (TC-034) |
| FR-019-AC-3 | Removing one required runner, binding, tool, or resource prerequisite leaves the affected task blocked, emits actionable setup work, and creates no fabricated execution result. | Test (TC-035) |
| FR-019-AC-4 | Assessing an otherwise exact result after its earliest freshness bound reports stale and retains its logical, participation, assumption, adequacy, provenance, native-artifact, and dependence findings. | Test (TC-035) |
| FR-019-AC-5 | Reused or unknown oracle and observation relationships remain explicit correlation findings; technique count does not imply independence or an aggregate score. | Test (TC-035) |
| FR-019-AC-6 | The manual task remains separate from both native techniques, carries its exact review procedure and allowed dispositions, and cannot be passed to the native runner. | Test (TC-035) |

## Dependencies

- Implements [US-001](../usecase/US-001-plan-portable-verification.md),
  [US-002](../usecase/US-002-assess-evidence-exactly.md), and
  [US-003](../usecase/US-003-detect-compatibility-regressions.md).
- Requires the reviewed native planning and shared-reference behaviors in
  [FR-014](./FR-014-construct-native-runner-requests.md),
  [FR-015](./FR-015-preserve-role-specific-inputs.md), and
  [FR-016](./FR-016-read-shared-references-strictly.md).
- Requires the evidence dependence and freshness behaviors in
  [FR-017](./FR-017-retain-oracle-dependence.md) and
  [FR-018](./FR-018-enforce-evidence-freshness.md).
- Native invocation is constrained by [NFR-006](../non-functional/NFR-006-bound-native-processes.md), and the immutable execution record is assembled into a retained report by [FR-021](./FR-021-bind-portfolio-report.md).
