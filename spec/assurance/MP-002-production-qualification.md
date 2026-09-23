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
      the QualificationProfile, QualificationResourceBudget, and
      QualificationEvidenceRequirement shapes in contracts/e02-draft-2.schema.json,
      the validation and freeze checks in src/qualification_profile.rs, and the
      tests in tests/qualification_profile.rs that pin those refusals are protected,
      so editing any of them to loosen what a profile must declare -- including where
      each budget's maximumWallClockNanoseconds reference comes from -- changes the
      recorded digests
  - kind: suppressed-observation
    description: >-
      ensure_known_guarantees_and_budgets refuses to validate or freeze a profile
      whose evidenceRequirements name a budgetId absent from resourceBudgets, so a
      budgeted phase cannot be dropped by deleting its budget while an evidence
      requirement still names it; deleting both instead yields a different frozen
      profile identity. This guards phase-level omission only: omission of an
      individual run or observation is not yet guarded, because the Task-031
      measurement path that would retain every observation does not exist
  - kind: stale-evidence
    description: >-
      candidateRef, dependencyClosureRef, productionCallerRef, and environmentRef
      must each match a strict-reader-validated reference of the expected role
      before freeze, and the frozen profile's identity is an RFC 8785 digest over
      the whole profile, so changing any of them yields a different identity
      (tc_039_changes_identity_for_every_bound_claim_dimension). Binding each
      collected result to that identity, so an earlier profile's result cannot
      present as current, is Task-031's still-unbuilt measurement path
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
plan at evaluation time -- here, one `resourceBudgets` entry of the frozen profile,
read via its `maximumWallClockNanoseconds`. The dimension is the entry's `budgetId`,
which the profile validation requires to be unique; `phase` is not required to be
unique, so the checker evaluates the rule once per budget entry, comparing the median
duration of that entry's `phase`, in nanoseconds, against that entry's
`maximumWallClockNanoseconds`. A phase named by more than one budget entry must
therefore satisfy every one of them. The reference value is never written into this
plan: this document names where the checker must look, and the checker supplies the
resolved number the same way it supplies every other baseline value. `le` holds when
the median is no greater than the budget, which is exactly NFR-007's "no phase exceeds
its own threshold", with no `margin`.

The per-phase budget comparison belongs to Task-031 and TC-044, and it is not
implemented yet: Task-031 is `not_started` and TC-044 is Planned. Today,
`src/qualification_profile.rs` only validates that a profile declares uniquely
identified budgets and that every evidence requirement names a declared budget. No
code compares measured durations against those budgets yet. When Task-031 builds that
comparison, it evaluates this plan's `decision_rule` once per `resourceBudgets` entry,
resolving each `baseline: external-reference` value from that entry's
`maximumWallClockNanoseconds` in the frozen profile -- it does not invent a separate
rule.

`estimator: median` narrows the previous definition, in which the owner selected the
per-phase summary. It is the per-phase median of valid observations. It is still
computed and compared independently for each phase, so a favorable aggregate cannot
hide one phase over its budget.

## Protected Apparatus

`contracts/e02-draft-2.schema.json` declares the `QualificationProfile`,
`QualificationResourceBudget`, and `QualificationEvidenceRequirement` shapes that decide
what a profile must declare, including each budget's `phase` and
`maximumWallClockNanoseconds`. `src/qualification_profile.rs` validates a candidate
profile against that schema, enforces unique budget and guarantee identities, refuses
an evidence requirement that names an undeclared budget, and computes the frozen
profile's RFC 8785 identity. `tests/qualification_profile.rs` is the test suite that
pins those refusals and that identity. Editing any of the three changes where this
plan's reference values come from or what counts as a valid profile, not the
production chain being measured.

This list protects the reference side of the decision rule only. The side that
produces the measured duration -- the Task-031 harness, its timer, and its observation
records -- does not exist yet, so nothing that produces this plan's estimate is
protected today. When Task-031 builds it, its files join this list under a new
`definition_version`.

## Negative Controls

See `negative_controls` in the frontmatter. `apparatus-edit` follows directly from
`protected_apparatus` above. `suppressed-observation` and `stale-evidence` are both
grounded in `src/qualification_profile.rs`'s existing validation, and each covers only
the profile half of its scenario: `ensure_known_guarantees_and_budgets` refuses a
profile with a dangling budget reference, and the RFC 8785 profile identity changes
whenever `candidateRef`, `dependencyClosureRef`, `productionCallerRef`, or
`environmentRef` changes. The observation half -- retaining every run, and binding each
result to the profile identity it was collected under -- belongs to Task-031's
still-unbuilt measurement path.

