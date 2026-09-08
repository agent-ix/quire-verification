---
id: MP-001
title: Bounded parser portfolio effort measurement
type: MeasurementPlan
status: proposed
owner: agent-ix-verification-owner
metric: portfolio_phase_elapsed_nanoseconds
definition_version: quire-verification.parser-portfolio-effort-v1
stage: baseline
statistical_design:
  population: valid phase observations from one exact parser-portfolio candidate, fixture pair, dependency closure, environment, and command order
  sampling: five sequential repetitions after one separately observed setup phase, retaining each phase observation in collection order
  repetitions: 5
  estimator: per-phase valid-sample count, minimum, lower median at sorted zero-based index (count minus one) divided by two, and maximum in integer nanoseconds
  error_model: process scheduling, filesystem cache state, clock granularity, build residue, thermal state, and invalid or interrupted executions
  uncertainty: retain every raw observation and report range plus environmental differences and plausible alternative explanations
  decision_rule: treat the baseline as interpretable only when identities and phase boundaries are complete and at least one valid observation exists for every automated phase; otherwise retain the run as incomplete
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

## Population

The population is the valid setup, direct-parse, parse-format-reparse, and
complete automated-run observations from one exact candidate, fixture pair,
dependency closure, environment, and prescribed command order. Build/download
time outside the declared setup phase, other machines, other fixtures, model
linking, evaluation, generated backends, and human review latency are excluded.

## Collection Procedure

Record candidate and environment identity, cache treatment, timer source, phase
boundaries, and command order before collection. Observe setup once, then execute
five sequential repetitions with a monotonic process-local clock. Store every
phase observation as an integer from 0 through 9007199254740991 nanoseconds and
in collection order. Mark interrupted, out-of-domain, identity-mismatched, or
otherwise invalid observations with a reason; never replace one with zero or
silently drop it.

## Interpretation

For each phase, report valid-sample count, minimum, median, and maximum from only
that phase and preserve every raw observation so the summary can be recomputed.
Report environmental differences, invalid runs, uncertainty, and plausible
alternative explanations. Do not infer ROI, delivery time, production throughput,
scale limits, broad effectiveness, certification, or evidence sufficiency.
