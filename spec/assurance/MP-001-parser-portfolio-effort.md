---
id: MP-001
title: Bounded parser portfolio effort measurement
type: MeasurementPlan
status: proposed
owner: agent-ix-verification-owner
metric: portfolio_phase_elapsed_nanoseconds
definition_version: quire-verification.parser-portfolio-effort-v2
stage: baseline
statistical_design:
  population: valid phase observations from one exact parser-portfolio candidate, fixture pair, dependency closure, environment, and command order
  sampling: one setup observation at repetition zero followed by direct-parse, parse-format-reparse, and complete-run observations for each of five sequential repetitions, retaining all 16 slots in collection order
  repetitions: 5
  estimator: median
  error_model: process scheduling, filesystem cache state, clock granularity, build residue, thermal state, and invalid or interrupted executions
  uncertainty: retain every raw observation and report range plus environmental differences and plausible alternative explanations
  decision_rule:
    comparator: ge
    threshold: 0
relationships:
  - target: ix://agent-ix/quire-verification/FR-020
    type: measures
  - target: ix://agent-ix/quire-verification/IT-003
    type: references
---

# Bounded parser portfolio effort measurement

## Decision Use

The observations establish a reproducible setup-versus-incremental baseline for
the exact VP04 candidate and identify phases that deserve later investigation.
They do not approve the candidate or establish a performance threshold.
This repository implements only the VP04 observation producer and summary
calculation. Engineering Assurance retains ownership of reusable measurement
providers and generic evaluation-report orchestration.

## Population

The population is the 16 setup, direct-parse, parse-format-reparse, and complete
automated-run observation slots from one exact candidate, fixture pair,
dependency closure, environment, and prescribed command order. Build/download
time outside the declared setup phase, other machines, other fixtures, model
linking, evaluation, generated backends, and human review latency are excluded.

## Collection Procedure

Record candidate and environment identity, cache treatment, timer source, phase
boundaries, and command order before collection. Observe setup as repetition
zero, then execute the three automated phases for repetitions one through five
with a monotonic process-local clock. Give every slot a nonempty unique
observation ID and unique phase/repetition pair. Store every phase observation as
an integer from 0 through 9007199254740991 nanoseconds and in collection order.
Mark interrupted, out-of-domain, identity-mismatched, or
otherwise invalid observations with a reason; never replace one with zero or
silently drop it.

The setup phase validates the selected references and admits the exact executable
descriptor, valid fixture, mutation fixture, and execution profile. Each
direct-parse phase invokes the valid source once. Each parse-format-reparse phase
formats the valid source, parses the exact formatted bytes under the prescribed
formatted revision, and formats those bytes again. Each complete-run phase runs
the direct and metamorphic baselines, the seeded syntax-refusal case, and their
portfolio assessment in that order. The five repetitions are sequential in
increasing repetition order and use the host's naturally warmed filesystem
cache; no cache is cleared or pre-warmed outside the recorded setup.

## Interpretation

For each phase, report valid-sample count, minimum, median, and maximum from only
that phase and preserve every raw observation so the summary can be recomputed.
Report environmental differences, invalid runs, uncertainty, and plausible
alternative explanations. Do not infer ROI, delivery time, production throughput,
scale limits, broad effectiveness, certification, or evidence sufficiency.

## Decision Rule

`statistical_design.decision_rule` is the mechanical rule FR-021 requires whenever
`statistical_design` is present. This plan is a baseline measurement, not a gate: it
establishes a reproducible reference and "does not approve the candidate or establish
a performance threshold" (see Decision Use above), so its rule is intentionally
trivial rather than judging the metric -- `{ comparator: ge, threshold: 0 }` holds for
any valid non-negative elapsed-nanosecond estimate. The actual interpretive bar is
completeness, not magnitude: identities and phase boundaries complete, and at least
one valid observation for every automated phase, as this section's opening paragraphs
describe; that is a population/collection concern, not something FR-021's closed
`decision_rule` vocabulary is meant to express. A later gate-stage plan that compares
against this baseline does so via its own `decision_rule` with
`baseline: prior-collection`.
