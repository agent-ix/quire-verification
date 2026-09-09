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

When the bounded parser portfolio records execution effort, the VP04 measurement
recorder SHALL preserve one-time binding and setup observations separately from
incremental per-technique and whole-run observations under the exact measured
environment.

## Inputs

- The reviewed measurement procedure in [MP-001](../assurance/MP-001-parser-portfolio-effort.md).
- An exact parser-portfolio candidate, environment, commands, fixtures, and run order.
- Monotonic elapsed-time observations and explicit validity outcomes for each phase.
- One series-level candidate identity and environment identity shared by all 16
  observations without per-observation identity copies.

## Outputs

- Raw phase observations, deterministic summaries, invalid-sample reasons, and
  bounded interpretation text in the VP04 report payload supplied to its external
  retention host.

## Behavior

- The VP04 measurement recorder SHALL label each observation as setup, direct-parse,
  parse-format-reparse, or complete automated run.
- The VP04 measurement recorder SHALL retain every collected observation in collection order
  with elapsed nanoseconds and a valid or invalid status.
- The VP04 measurement recorder SHALL assign each observation a nonempty ID unique within the
  automated report core and one unique `(phase, repetition)` pair.
- The VP04 measurement recorder SHALL assign repetition zero only to the single setup
  observation and repetitions one through five to each automated phase.
- When an observation is invalid, the VP04 measurement recorder SHALL retain its reason and
  exclude it from numeric summaries without replacing it with zero.
- When a duration is outside the admitted integer domain, the VP04 measurement
  recorder SHALL retain its exact JSON scalar spelling in `rejectedElapsed`
  rather than in `elapsedNanoseconds`.
- A valid observation SHALL contain `elapsedNanoseconds` with no invalid-only fields.
- An invalid observation SHALL contain a nonempty reason, omit
  `elapsedNanoseconds`, and optionally contain `rejectedElapsed` when a duration
  value was observed.
- For each phase with valid observations, the VP04 measurement recorder SHALL compute the
  sample count, minimum, median, and maximum independently from other phases.
- The VP04 measurement recorder SHALL compute median as the sorted valid sample at zero-based
  index `(sample_count - 1) / 2`, using integer division.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-020-CON-1 | Elapsed observations SHALL use a monotonic process-local clock and non-negative integer nanoseconds. | Measurement integrity | Test |
| FR-020-CON-2 | The report SHALL NOT convert the bounded observations into ROI, delivery estimates, production throughput, scale limits, or a general performance claim. | Claim boundary | Inspection |
| FR-020-CON-3 | Each serialized elapsed duration SHALL be an integer from 0 through 9007199254740991 nanoseconds. | Interchange integrity | Test |
| FR-020-CON-4 | The VP04 recorder SHALL remain limited to this MeasurementPlan's domain observations, excluding reusable measurement-provider, artifact-store, and evaluation-report-host behavior. | Responsibility | Test |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-020-AC-1 | Setup and binding observations are stored in a phase distinct from every incremental method and complete-run observation. | Test (TC-036) |
| FR-020-AC-2 | The series contains one exact candidate identity and environment identity, exactly one setup observation at repetition zero, and one direct-parse, parse-format-reparse, and complete-run observation for each repetition one through five, retaining all 16 raw valid or invalid observations in collection order. | Test (TC-036) |
| FR-020-AC-3 | Invalid, interrupted, or identity-mismatched observations retain a reason, omit `elapsedNanoseconds`, preserve any observed out-of-domain duration in `rejectedElapsed`, do not become zero-duration samples, and do not enter numeric summaries; valid observations contain no invalid-only fields. | Test (TC-036) |
| FR-020-AC-4 | Each phase summary reports only its valid sample count, minimum, median, and maximum in integer nanoseconds using the documented deterministic calculation. | Test (TC-036) |
| FR-020-AC-5 | Recomputing summaries from the retained raw observations produces the report values exactly. | Test (TC-036) |
| FR-020-AC-6 | The report states the measurement population, exclusions, clock, command order, cache treatment, environmental differences, uncertainty, and plausible alternative explanations. | Test (TC-037) |
| FR-020-AC-7 | The report limits interpretation to this candidate and fixture and makes no ROI, delivery, throughput, scale, broad-effectiveness, certification, or sufficiency claim. | Test (TC-037) |
| FR-020-AC-8 | A negative, fractional, or larger-than-9007199254740991 duration is retained as invalid with its reason and never enters a numeric summary. | Test (TC-036) |
| FR-020-AC-9 | An empty or duplicate observation ID, duplicate or missing phase/repetition pair, repetition outside zero through five, or reordered prescribed series refuses with `invalid_measurement_series`. | Test (TC-036) |

## Dependencies

- Implements [US-003](../usecase/US-003-detect-compatibility-regressions.md).
- Measures the execution defined by [FR-019](./FR-019-execute-bounded-parser-portfolio.md).
- Uses [MP-001](../assurance/MP-001-parser-portfolio-effort.md) as the reproducible observation contract.
- Supplies the effort section of [FR-021](./FR-021-bind-portfolio-report.md)'s immutable automated report core.
