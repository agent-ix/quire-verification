---
id: AP-003
title: Complete-V1 development method assurance profile
type: AssuranceProfile
status: proposed
owner: agent-ix-verification-owner
schema_version: 0.2
profile_kind: general
scope: lane-D development interfaces and implementations for Verification issues 34 through 38 and Analyze issue 33
impact_assessments:
  - id: impact-false-semantic-verdict
    scenario: method failure, unsupported scope, or incomplete work is promoted to a semantic pass
    severity: critical
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/quire-verification/FR-031
  - id: impact-false-qualification-accounting
    scenario: a missing inventory row, correlated evidence, or production ceremony is hidden by an aggregate status
    severity: material
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/quire-verification/FR-034
review_policy:
  mode: require
  operations: [spec-review, code-review, gap-analysis]
relationships:
  - target: ix://agent-ix/quire-verification/US-005
    type: governs
  - target: ix://agent-ix/quire-verification/FR-029
    type: governs
  - target: ix://agent-ix/quire-verification/FR-034
    type: governs
---
# Complete-V1 development method assurance profile

## Decision Boundary

This profile governs the development contracts and implementations in the lane-D
ticket chain. It does not gate development on production qualification and does
not authorize production use, deployment, release, certification, or acceptance
of a future candidate.

## Impact Scenarios

The governed harms are false semantic verdicts, erased method failures,
non-replayable counterexamples, correlated/self evidence, missing inventory
rows, and accidental coupling of development progress to future production
ceremony.

## Evidence Policy

Each implementation ticket supplies local Rust tests, exact requirement traces,
typed adverse and non-conclusive cases, deterministic identities, and replay
evidence appropriate to its contract. Unavailable external or production
evidence remains explicit state and does not delay unrelated development.

## Exceptions

No exception may collapse a non-conclusive method state into a semantic verdict,
erase a required ledger row, or make production ceremony a prerequisite for
development work. Other exceptions require an owner, scope, rationale, expiry,
and stated evidence impact.

