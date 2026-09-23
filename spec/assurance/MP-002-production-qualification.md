---
id: MP-002
title: Quire-spec production qualification measurement
type: MeasurementPlan
status: proposed
owner: agent-ix-verification-owner
metric: quire_spec_profile_phase_duration
definition_version: quire-verification.production-qualification-v2
stage: gate
objective:
  direction: lower
statistical_design:
  population: every owner-declared setup, hot-path execution, and cleanup phase for the exact candidate, caller, corpus partition, environment, cache policy, and concurrency profile
  sampling: deterministic corpus strata plus retained seeded generative cases executed in the declared command order, with setup and cleanup measured separately from the hot path
  repetitions: 5
  estimator: median
  error_model: corpus selection, process scheduling, cache state, filesystem state, clock resolution, thermal state, tool variance, and environment drift
  uncertainty: report every observation, exclusion, invalid sample, distribution summary, threshold comparison, and plausible environmental explanation
  decision_rule:
    comparator: le
    baseline: prior-collection
relationships:
  - target: ix://agent-ix/quire-verification/NFR-007
    type: measures
  - target: ix://agent-ix/quire-verification/NFR-008
    type: references
  - target: ix://agent-ix/quire-verification/IT-004
    type: references
---
# Quire-spec production qualification measurement

## Decision Use

The measurements decide whether one exact production profile satisfies its
owner-supplied phase budgets. They do not create a universal `quire-spec` SLA,
compare developer productivity, or establish general scale.

## Population

The population contains every setup, hot-path execution, and cleanup phase in the
frozen profile across its declared deterministic corpus strata and retained seeded
generative cases. Other commands, machines, profiles, unselected inputs, network
conditions, and semantic stages are excluded unless the owner adds them before
profile freeze.

## Collection Procedure

Record exact candidate, caller, dependency closure, environment, corpus partition,
seed, command order, cache policy, concurrency, phase boundaries, timer, and owner
threshold. Execute five repetitions with at most two Cargo jobs and serialize
high-load phases. Retain every raw observation and mark interrupted, mismatched, or
otherwise invalid samples with a reason instead of dropping or replacing them.

## Interpretation

Compute the declared per-phase estimator from valid observations and retain the
complete distribution, threshold comparison, invalid samples, uncertainty sources,
and plausible environmental explanations. A passing aggregate cannot mask a phase
violation. A missing threshold, incomplete population, unrecomputable summary, or
unexplained drift leaves the gate incomplete.

## Decision Rule

`statistical_design.decision_rule` is the mechanical rule FR-021 requires. The gate's
true reference values are the per-phase budgets declared in the frozen qualification
profile (`resourceBudgets` in `qualification_profile.rs`, owned by the profile author,
not this document) -- FR-021's closed `baseline` vocabulary (`constant-predictor`,
`prior-collection`, `best-seen`) has no kind for "a reference value supplied by an
external, owner-authored profile," so `decision_rule` here uses `baseline:
prior-collection` as the closest available shape: a reference the checker resolves at
evaluation time rather than one written into this frontmatter. **This is an
approximation, not the intended gate.** The actual per-phase pass/fail behavior --
"every claimed phase has an owner threshold and no phase exceeds it," independently
per phase -- still lives in NFR-007 and is implemented directly against the
qualification profile's `resourceBudgets` by Task-031/TC-044, not read out of this
plan's `decision_rule`. Comparator `le` and `objective.direction: lower` record only
that the metric is a duration where less is better. Follow-up: PLAT-956 should decide
whether FR-021 needs an additional `baseline` kind for externally-declared,
per-dimension thresholds, or whether a gate composed of independently-thresholded
phases belongs in per-phase constituent MeasurementPlans linked by `relationships`
instead of one plan's single `decision_rule`.

