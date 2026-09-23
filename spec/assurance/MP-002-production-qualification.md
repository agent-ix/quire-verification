---
id: MP-002
title: Quire-spec production qualification measurement
type: MeasurementPlan
status: proposed
owner: agent-ix-verification-owner
metric: quire_spec_profile_phase_duration
definition_version: quire-verification.production-qualification-v3
stage: gate
ground_truth_kind: mechanical
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
    baseline: external-reference
protected_apparatus:
  - contracts/e02-draft-2.schema.json
  - src/qualification_profile.rs
  - tests/qualification_profile.rs
negative_controls:
  - kind: apparatus-edit
    description: >-
      the QualificationProfile/PerformanceObservation/PerformanceSummary shapes in
      contracts/e02-draft-2.schema.json and the parsing and cross-reference checks in
      src/qualification_profile.rs are protected, so editing either to loosen what a
      profile or an observation must declare changes the recorded digests
  - kind: suppressed-observation
    description: >-
      ensure_known_guarantees_and_budgets refuses to freeze a profile whose
      evidenceRequirements name a budgetId absent from resourceBudgets, so a phase
      cannot be dropped from measurement by deleting its budget while an evidence
      requirement still names it -- the profile fails to freeze instead of silently
      measuring fewer phases
  - kind: stale-evidence
    description: >-
      candidateRef, dependencyClosureRef, productionCallerRef, and environmentRef are
      bound and validated before freeze, and the frozen profile's identity is an
      RFC 8785 digest over that exact bound value, so a result collected against a
      different candidate, dependency closure, caller, or environment carries a
      different profile identity rather than presenting as current
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

The gate this plan measures is the one NFR-007 states: it passes only when every
claimed phase has an owner threshold, the complete declared sample executes, each
per-phase summary recomputes, and no phase exceeds its own threshold. Each phase is
judged independently against its own owner-declared budget. Those budgets are
`resourceBudgets` entries (`maximumWallClockNanoseconds` per `phase`) in the frozen
qualification profile. The profile author supplies them. Neither this document nor
this repository holds any budget number.

`{ comparator: le, baseline: external-reference }` expresses that directly.
`baseline: external-reference` (FR-021-AC-10, `engineering-assurance` PR #125) is a
per-dimension reference value the calling checker resolves from a source outside the
plan at evaluation time -- here, the frozen profile's `resourceBudgets` entry for the
phase under evaluation, read via `maximumWallClockNanoseconds`. The reference value is
never written into this plan: this document names where the checker must look, and the
checker supplies the resolved number the same way it supplies every other baseline
value. `le` holds when the phase's median duration is no greater than that phase's own
`maximumWallClockNanoseconds`, which is exactly NFR-007's "no phase exceeds its own
threshold" applied once per phase, with no `margin`.

The per-phase budget comparison belongs to Task-031 and TC-044, and it is not
implemented yet: Task-031 is `not_started` and TC-044 is Planned. Today,
`src/qualification_profile.rs` only validates that a profile declares uniquely
identified budgets and that every evidence requirement names a declared budget. No
code compares measured durations against those budgets yet. When Task-031 builds that
comparison, it evaluates this plan's `decision_rule` once per phase, resolving each
phase's `baseline: external-reference` value from that phase's
`resourceBudgets.maximumWallClockNanoseconds` in the frozen profile -- it does not
invent a separate rule.

`estimator: median` narrows the previous definition, in which the owner selected the
per-phase summary. It is the per-phase median of valid observations. It is still
computed and compared independently for each phase, so a favorable aggregate cannot
hide one phase over its budget.

## Protected Apparatus

`contracts/e02-draft-2.schema.json` declares the `QualificationResourceBudget`,
`PerformanceObservation`, and `PerformanceSummary` shapes that decide what a profile
must declare and what an observation must retain. `src/qualification_profile.rs`
parses and validates a candidate profile against that schema, enforces unique budget
and guarantee identities, and refuses an evidence requirement that names an undeclared
budget. `tests/qualification_profile.rs` is the test suite that exercises those
refusals. Editing any of the three changes what the gate measures or what it accepts
as a valid profile, not the production chain being measured.

## Negative Controls

See `negative_controls` in the frontmatter. `apparatus-edit` follows directly from
`protected_apparatus` above. `suppressed-observation` and `stale-evidence` are both
grounded in `src/qualification_profile.rs`'s existing validation, ahead of Task-031's
still-unbuilt measurement path: `ensure_known_guarantees_and_budgets` refuses to freeze
a profile with a dangling budget reference, and the RFC 8785 identity over the bound
`candidateRef`/`dependencyClosureRef`/`productionCallerRef`/`environmentRef` set
prevents a result collected under one subject or environment from presenting as
current under another.

