---
id: FR-020
title: "Record reproducible portfolio effort"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-003"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-019"
    type: "depends_on"
---
# FR-020: Record reproducible portfolio effort

## Description

When the bounded parser portfolio records execution effort, the report writer SHALL preserve one-time binding and setup observations separately from incremental per-technique and whole-run observations under the exact measured environment.

## Inputs

- The reviewed measurement procedure in [MP-001](../assurance/MP-001-parser-portfolio-effort.md).
- An exact parser-portfolio candidate, environment, commands, fixtures, and run order.
- Monotonic elapsed-time observations and explicit validity outcomes for each phase.

## Outputs

- Raw phase observations, deterministic summaries, invalid-sample reasons, and bounded interpretation text in the retained portfolio report.

## Behavior

- The report writer SHALL label each observation as setup, direct-parse,
  parse-format-reparse, manual-disposition, or complete automated run.
- The report writer SHALL retain every collected observation in collection order
  with elapsed nanoseconds and a valid or invalid status.
- When an observation is invalid, the report writer SHALL retain its reason and
  exclude it from numeric summaries without replacing it with zero.
- For each phase with valid observations, the report writer SHALL compute the
  sample count, minimum, median, and maximum independently from other phases.
- The report writer SHALL compute median as the sorted valid sample at zero-based
  index `(sample_count - 1) / 2`, using integer division.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-020-CON-1 | Elapsed observations SHALL use a monotonic process-local clock and non-negative integer nanoseconds. | Measurement integrity | Test |
| FR-020-CON-2 | The report SHALL NOT convert the bounded observations into ROI, delivery estimates, production throughput, scale limits, or a general performance claim. | Claim boundary | Inspection |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-020-AC-1 | Setup and binding observations are stored in a phase distinct from every incremental method and complete-run observation. | Test (TC-036) |
| FR-020-AC-2 | Five prescribed repetitions retain every raw valid or invalid phase observation in collection order with exact candidate and environment identity. | Test (TC-036) |
| FR-020-AC-3 | Invalid, interrupted, or identity-mismatched observations retain a reason, do not become zero-duration samples, and do not enter numeric summaries. | Test (TC-036) |
| FR-020-AC-4 | Each phase summary reports only its valid sample count, minimum, median, and maximum in integer nanoseconds using the documented deterministic calculation. | Test (TC-036) |
| FR-020-AC-5 | Recomputing summaries from the retained raw observations produces the report values exactly. | Test (TC-036) |
| FR-020-AC-6 | The report states the measurement population, exclusions, clock, command order, cache treatment, environmental differences, uncertainty, and plausible alternative explanations. | Inspection (TC-037) |
| FR-020-AC-7 | The report limits interpretation to this candidate and fixture and makes no ROI, delivery, throughput, scale, broad-effectiveness, certification, or sufficiency claim. | Inspection (TC-037) |

## Dependencies

- Implements [US-003](../usecase/US-003-detect-compatibility-regressions.md).
- Measures the execution defined by [FR-019](./FR-019-execute-bounded-parser-portfolio.md).
- Uses [MP-001](../assurance/MP-001-parser-portfolio-effort.md) as the reproducible observation contract.
