---
id: AP-001
title: Qualified Rust verification-path profile
type: AssuranceProfile
status: active
owner: agent-ix-verification-owner
profile_version: 0.2
profile_kind: general
scope: one candidate revision of the private quire-verification bounded compatibility slice
impact_assessments:
  - id: impact-false-evidence-support
    scenario: a foreign, stale, ambiguous, or adverse result is attributed as support
    severity: critical
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/quire-verification/FR-006
  - id: impact-unbounded-ingestion
    scenario: an adversarial JSON input exhausts qualified-path memory or stack
    severity: material
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/quire-verification/NFR-005
  - id: impact-language-disqualification
    scenario: executable logic remains in a language barred from the qualified path
    severity: material
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/quire-verification/NFR-004
  - id: impact-unbounded-native-process
    scenario: a native verification child hangs, floods output, inherits undeclared state, or is invoked through an ambiguous shell boundary
    severity: material
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/quire-verification/NFR-006
review_policy:
  mode: require
  operations: [spec-review, code-review, gap-analysis]
relationships:
  - target: ix://agent-ix/quire-verification/StR-001
    type: governs
  - target: ix://agent-ix/quire-verification/NFR-004
    type: governs
  - target: ix://agent-ix/quire-verification/NFR-005
    type: governs
  - target: ix://agent-ix/quire-verification/NFR-006
    type: governs
---
# Qualified Rust verification-path profile

## Decision Boundary

This profile covers one candidate revision of the private bounded compatibility
slice, including the bounded VP04 native process adapter. It does not accept the
provisional FS02/FS05 identity seam, authorize publication, or qualify any
represented external verification tool.

## Impact Scenarios

The governed failures are false evidence support, resource exhaustion at JSON
or native process boundaries, and retention of non-Rust executable logic in the
qualified path. Each can be detected before merge through deterministic tests
and inspection.

## Evidence Policy

Evidence must identify the exact candidate revision, producer, adapter, plan,
task, run, subject, bindings, dependency closure, and execution scope. Spec
review, the exact `agent-skills/rust-review` workflow, and Quoin gap analysis are
mandatory for this candidate revision.

## Exceptions

No exceptions are authorized. JSON, JSON Schema, Markdown, and workflow YAML are
data or configuration and are outside the executable-language restriction.
