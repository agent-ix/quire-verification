---
id: AP-002
title: Quire-spec bounded production qualification profile
type: AssuranceProfile
status: proposed
owner: agent-ix-verification-owner
schema_version: 0.2
profile_kind: general
scope: one exact quire-spec candidate, production caller, input population, environment, resource budget, and claim profile
impact_assessments:
  - id: impact-false-production-qualification
    scenario: narrow or correlated evidence is attributed as broad production support
    severity: critical
    verifiability:
      class: probabilistic
      stochastic_dependency: both
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/quire-verification/FR-022
  - id: impact-oracle-blind-spot
    scenario: self-generated expected output is treated as independent correctness evidence
    severity: material
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/quire-verification/FR-024
  - id: impact-production-chain-drift
    scenario: a test adapter passes while the real caller or assurance handoff fails
    severity: material
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/quire-verification/FR-026
  - id: impact-hot-path-budget-breach
    scenario: the exact production profile exceeds its owner-supplied time or resource budget
    severity: material
    verifiability:
      class: probabilistic
      stochastic_dependency: both
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/quire-verification/NFR-007
review_policy:
  mode: require
  operations: [spec-review, code-review, gap-analysis]
relationships:
  - target: ix://agent-ix/quire-verification/StR-002
    type: governs
  - target: ix://agent-ix/quire-verification/FR-022
    type: governs
  - target: ix://agent-ix/quire-verification/FR-027
    type: governs
  - target: ix://agent-ix/quire-verification/NFR-007
    type: governs
  - target: ix://agent-ix/quire-verification/NFR-008
    type: governs
---
# Quire-spec bounded production qualification profile

## Decision Boundary

This profile governs one exact candidate and one owner-authored production claim.
It accepts no guarantee, command, caller, corpus class, environment, or performance
budget that is absent from the frozen profile. A later revision or broader profile
requires new evidence and a new disposition.

## Impact Scenarios

The governed harms are false broad qualification, correlated-oracle attribution,
test-only integration substitution, and unmeasured hot-path regression. Each harm
must have evidence capable of detecting it before the candidate is accepted.

## Evidence Policy

The candidate requires reviewed profile and corpus definitions, independent-oracle
analysis, discriminating property/fuzz/metamorphic/mutation evidence, real-chain
integration, measured owner thresholds, reproducibility, exact Rust review,
semantic QUOIN gap analysis, and an external verification-owner disposition.
Evidence must retain every failure and limitation and must use exact versioned
references from the owning producers.

## Exceptions

No language, evidence, identity, review, or performance exception is pre-authorized.
A missing external interface blocks only its dependent task and does not authorize a
local copy. GitHub-hosted qualification remains manual and separately authorized.

