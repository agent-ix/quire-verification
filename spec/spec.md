---
type: master-requirements
name: quire-verification
org: agent-ix
component_type: rust-library
implementation_language: rust
tags:
  - verification
  - evidence
  - compatibility
depends_on:
  - quoin-method-catalog
standards_alignment:
  - iso-iec-ieee-29148
  - rfc-8785
---
# Master Requirements Specification
## Quire Verification Compatibility, Native Planning, and Evidence Assessment

## 1. Purpose

This specification defines the observable behavior of the private E02
compatibility boundary, the VP01 registry/policy qualification slice, the VP02
native planning contract, the VP03 evidence-assessment contract, and the VP04
bounded native parser portfolio. It
gives maintainers the reviewed requirements baseline for the code, fixtures,
retained reports, and continuous-integration checks in this repository.

## 2. Scope

### 2.1 In Scope

- Five portable public contracts: `TechniqueDefinition`, `ToolCapability`,
  `SelectionPolicy`, `VerificationPlan`, and `TechniqueResult`.
- Runtime validation derived from one JSON Schema packet.
- Adaptation of pinned Quoin method-catalog and advice exports.
- Qualification of registry and policy snapshots against the pinned Quoin baseline.
- Prerequisite-aware verification planning across five input shapes.
- Explicit authored-or-unknown classification and explainable authored priority.
- Actionable prerequisite DAGs that keep required and exhausted work visible.
- Bounded native runner requests with explicit capabilities, permissions, budgets, and native artifact interfaces.
- Closed role-specific state, relational, trace, fault, and manual input variants.
- Strict consumption of an independently accepted shared FS02/FS05 reference packet.
- A heterogeneous parser portfolio plan with property, differential, and manual work.
- Exact evidence identity, scope, freshness, and result-state assessment.
- Conservative combination of result and obligation freshness bounds.
- Explicit oracle and observation dependence without aggregate scoring.
- RFC 8785 JSON Canonicalization Scheme for digests and structured equality.
- A bounded local catalog portfolio that retains its own failures as results.
- A bounded native parser portfolio with direct and metamorphic techniques, a seeded syntax defect, and a report-bound manual disposition.
- Reproducible separation of one-time setup effort from incremental technique and run effort.
- Direct native-process execution with closed arguments, environment, working directory, time, and output bounds.
- An immutable automated report core whose non-circular digest binds an external human disposition.
- Fixture validation, retained reports, and Rust repository CI checks for this slice.

### 2.2 Out of Scope

- Acceptance or replacement of the shared FS02/FS05 identity and version model.
- Acceptance of E02, VP01, or VP02 before their recorded prerequisite and disposition gates close.
- Publication of these private provisional contracts as cross-repository authority.
- Execution of a model checker, fuzzer, fault injector, generated backend, evaluator, or production system.
- A parser portfolio outside the reviewed bounded syntax profile or a general parser-effectiveness claim.
- A VP04 native process profile for an operating system other than the exact reviewed Linux environment.
- An automated evidence-sufficiency decision.
- ROI, delivery, production-throughput, scale, or certification conclusions from bounded timing observations.
- A duplicate evidence store or migration of Quoin-owned evidence.
- Backward compatibility for the rejected JavaScript module API, npm package, scripts, or runtime behavior outside the versioned JSON contracts and fixtures.

## 3. System Overview

The component adapts existing Quoin catalog data into portable technique records,
constructs explicit verification plans, validates externally supplied technique
results, and assesses those results against exact obligations. It also provides a
small local portfolio for checking its catalog adapter without representing that
external verification techniques have run.

VP02 adds a native-runner and strict-reference consumer contract without
absorbing native tools or Quoin evidence storage.

VP04 composes the accepted VP02 and VP03 behaviors with one exact native parser
candidate. It runs direct parse acceptance and parse-format-reparse stability,
retains a separate human disposition, and records setup and incremental effort
without extending the observed syntax-fixture claim.

The intended users are verification integrators, evidence assessors, and
maintainers of the Quoin compatibility boundary.

## 4. Requirements Architecture

User intent lives under `usecase/`. Binding behavior lives under `functional/`.
Quality constraints live under `non-functional/`. The real Quoin boundary test
lives under `integration/`. Criterion-level verification coverage is indexed in
`tests.md`, with detailed procedures under `test-cases/`.

## 5. Requirements Index

| ID | Title | Status |
|----|-------|--------|
| US-001 | Plan portable verification work | Partial; VP04 specified and blocked |
| US-002 | Assess evidence without false attribution | Partial; VP03/VP04 blocked |
| US-003 | Detect compatibility regressions locally | Partial; VP04 specified and blocked |
| FR-001 | Validate the five public contracts | Implemented; verified |
| FR-002 | Derive runtime contract behavior from the schema | Implemented; verified |
| FR-003 | Validate every fixture through an explicit handler | Implemented; verified |
| FR-004 | Adapt Quoin catalog and advice without changing meaning | Implemented; verified |
| FR-005 | Build prerequisite-aware verification plans | Implemented; verified |
| FR-006 | Match evidence by exact identity and execution scope | Implemented; verified |
| FR-007 | Preserve independent result findings | Implemented; verified |
| FR-008 | Canonicalize digest and identity inputs with JCS | Implemented; verified |
| FR-009 | Retain bounded portfolio failures as results | Implemented; verified |
| FR-010 | Keep applicability and readiness independent | Implemented; verified |
| FR-011 | Emit authored or unknown classification | Implemented; verified |
| FR-012 | Preserve authored prioritization | Implemented; verified |
| FR-013 | Emit an actionable prerequisite DAG | Specified; pending implementation |
| FR-014 | Construct exact native runner requests | Specified; pending implementation |
| FR-015 | Preserve role-specific verification inputs | Specified; pending implementation |
| FR-016 | Read shared references strictly | Specified; blocked on FS02/FS05 amendment |
| FR-017 | Retain oracle dependence without aggregation | Specified; blocked on FS05 role amendment |
| FR-018 | Enforce every evidence freshness bound | Implemented; verified |
| FR-019 | Execute a bounded native parser portfolio | Specified; blocked on VP02/VP03 |
| FR-020 | Record reproducible portfolio effort | Specified; blocked on FR-019 |
| FR-021 | Bind the portfolio report and human disposition | Specified; blocked on shared roles and FR-019/FR-020 |
| NFR-001 | Deterministic structured comparison | Implemented; verified |
| NFR-002 | Contract source maintainability | Implemented; verified |
| NFR-003 | Conservative assurance claims | Implemented; verified |
| NFR-004 | Rust implementation for the qualified path | Specified; documentation check pending |
| NFR-005 | Bounded JSON ingestion | Implemented; verified |
| NFR-006 | Bound native process execution | Implemented; verified |
| IT-001 | Real Quoin method-catalog parity | Implemented; verified |
| IT-002 | Independent shared-reference consumer qualification | Specified; blocked on FS02/FS05 amendment |
| IT-003 | Native parser portfolio execution | Specified; blocked on VP02/VP03 |

## 6. Identification and Traceability

Artifact identifiers use the `US-NNN`, `FR-NNN`, `NFR-NNN`, `IT-NNN`, and
`TC-NNN` forms. Frontmatter relationships connect each normative requirement to
its driving story and each test artifact to the requirement it verifies.

## 7. Verification Strategy

Automated Rust tests verify examples, error paths, exact-binding dimensions,
native planning, strict shared references, and metamorphic order invariance.
Fixture validation executes the versioned schema against every JSON fixture.
The Quoin integration test loads the real method catalog from a pinned checkout;
the shared-reference integration test uses only the pinned data packet through
an independent Rust consumer. The native parser integration runs exact real
`parse` and `format` commands, records adverse and incomplete cases, and retains
the report-bound human disposition and raw effort observations. The process
boundary is direct and finite; the final report binds a separately supplied
human disposition to a sealed automated core.

## 8. Lifecycle and Change Control

This specification describes a private compatibility slice implemented and
verified in Rust, the VP01 registry/policy qualification increment, the specified
VP02 planning increment, and the specified VP03 evidence-assessment increment.
VP01 acceptance remains gated on E02 and a recorded owner/reviewer disposition.
VP02 acceptance remains gated on VP01 and FS02. VP03 acceptance remains gated on
E02 and FS05. VP04 acceptance remains gated on VP02, VP03, exact native producer
qualification, fixture rights, and one report-bound verification-owner disposition.
FR-016, FR-017, FR-021, and IT-002 cannot proceed until the shared owner resolves
B's recorded canonical-domain and reference-role findings. Any accepted breaking
draft-2 transition must update requirements, tests, fixtures, and retained reports
without reinterpreting draft-1 evidence.

## 9. References

- RFC 8785, JSON Canonicalization Scheme.
- ISO/IEC/IEEE 29148, Requirements Engineering.
- `contracts/e02-draft-1.schema.json`, the private versioned contract packet.
- `agent-ix/quire-specification` PR #8 at `609790e`, conditional shared-reference input.
- `agent-ix/quire-spec-language`, the exact reviewed Rust parser candidate selected by IT-003.
