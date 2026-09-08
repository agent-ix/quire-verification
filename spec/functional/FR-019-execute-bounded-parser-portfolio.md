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
- A reviewed plan, policy, capabilities, runner requests, evidence obligations, and freshness limits.
- An exact verification-owner procedure for the separate manual task.

## Outputs

- An immutable automated execution record containing the exact plan, native
  results, assessment, and native artifact references.

## Behavior

- For the direct technique, the executor SHALL classify a zero-exit parse of the
  valid source as completed and satisfied.
- For the metamorphic technique, the executor SHALL format the valid parsed unit,
  parse the formatted bytes, and compare the second formatted output with the
  first output under the reviewed relation.
- When every metamorphic command completes but the reparse or second-format
  relation differs, the executor SHALL record completed execution with a violated
  logical outcome and retain the exact differing artifacts.
- When the seeded syntax mutation produces the expected located refusal, the
  executor SHALL record completed execution with a violated logical outcome.
- If the native process cannot start, exits outside the expected method protocol,
  or emits an unreadable result, then the executor SHALL record failed execution
  with a not-evaluated logical outcome.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-019-CON-1 | The portfolio SHALL execute only the reviewed bounded parser profile. | Scope | Inspection |
| FR-019-CON-2 | The portfolio SHALL NOT claim model linking, evaluation, generated backend execution, production fault injection, or general parser effectiveness. | Scope | Inspection |
| FR-019-CON-3 | The portfolio SHALL leave Quoin as evidence and authored-sufficiency owner. | Ownership | Test |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-019-AC-1 | The valid-source baseline contains completed, satisfied, fully participating direct-parse and parse-format-reparse results from the exact native candidate. | Test (TC-034) |
| FR-019-AC-2 | A single declared syntax mutation yields its exact expected diagnostic code and source location as a completed, violated result, while an unexpected process or protocol failure remains failed and not-evaluated. | Test (TC-034) |
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
