---
id: MP-002
title: Quire-spec production qualification measurement
type: MeasurementPlan
status: proposed
owner: agent-ix-verification-owner
metric: quire_spec_profile_phase_duration
definition_version: quire-verification.production-qualification-v1
stage: gate
statistical_design:
  population: every owner-declared setup, hot-path execution, and cleanup phase for the exact candidate, caller, corpus partition, environment, cache policy, and concurrency profile
  sampling: deterministic corpus strata plus retained seeded generative cases executed in the declared command order, with setup and cleanup measured separately from the hot path
  repetitions: 5
  estimator: owner-selected per-phase distribution summary with raw observations retained and threshold evaluation performed independently for each phase
  error_model: corpus selection, process scheduling, cache state, filesystem state, clock resolution, thermal state, tool variance, and environment drift
  uncertainty: report every observation, exclusion, invalid sample, distribution summary, threshold comparison, and plausible environmental explanation
  decision_rule: the performance gate passes only when every claimed phase has an owner threshold, the complete declared sample executes, each summary recomputes, and no phase exceeds its threshold
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

