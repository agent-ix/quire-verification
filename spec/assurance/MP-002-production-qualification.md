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

**Known limitation (tracked in Linear PLAT-1009): `statistical_design.decision_rule`
in this plan's frontmatter is a placeholder, not this gate's decision rule. Do not
evaluate it as the gate, and do not read a result of it as a pass or fail of this
gate.**

The gate this plan measures is the one NFR-007 states: it passes only when every
claimed phase has an owner threshold, the complete declared sample executes, each
per-phase summary recomputes, and no phase exceeds its own threshold. Each phase is
judged independently against its own owner-declared budget. Those budgets are
`resourceBudgets` entries (`maximumWallClockNanoseconds` per `phase`) in the frozen
qualification profile. The profile author supplies them. Neither this document nor
this repository holds any budget number.

FR-021's `decision_rule` cannot express that gate. Its closed `baseline` vocabulary
(`constant-predictor`, `prior-collection`, `best-seen`) has no kind for "a reference
declared externally, per phase, and resolved by the checker". A single `threshold`
would require inventing one number for every phase. The frontmatter therefore carries
`{ comparator: le, baseline: prior-collection }` only because FR-021 requires a
structured rule when `statistical_design` is present. If a checker evaluated that rule
mechanically, it would test "median duration no worse than the previous collection".
That is a regression check against the last run. It is not a comparison with the
owner's budgets, and it has no reference at all for the first collection. Treat it as
a materially different rule. The only true facts it records are the ones in `le` and
`objective.direction: lower`: the metric is a duration, and lower is better.

The per-phase budget comparison belongs to Task-031 and TC-044, and it is not
implemented yet: Task-031 is `not_started` and TC-044 is Planned. Today,
`src/qualification_profile.rs` only validates that a profile declares uniquely
identified budgets and that every evidence requirement names a declared budget. No
code compares measured durations against those budgets yet. When it is built, it must
read the budgets from the profile, not from this plan's `decision_rule`.

`estimator: median` narrows the previous definition, in which the owner selected the
per-phase summary. It is the per-phase median of valid observations. It is still
computed and compared independently for each phase, so a favorable aggregate cannot
hide one phase over its budget.

PLAT-1009 tracks the engineering-assurance design decision. The options are a new
`baseline` kind for externally declared, per-dimension references, or splitting this
gate into per-phase constituent MeasurementPlans linked by `relationships`. Once it is
resolved, replace this placeholder rule and bump `definition_version`.

