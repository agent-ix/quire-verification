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
## Quire Verification Bounded Compatibility Slice

## 1. Purpose

This specification defines the observable behavior of the private bounded E02
verification compatibility slice. It gives maintainers a reviewed requirements
baseline for the code, fixtures, retained reports, and continuous-integration
checks already present in this repository.

## 2. Scope

### 2.1 In Scope

- Five portable public contracts: `TechniqueDefinition`, `ToolCapability`,
  `SelectionPolicy`, `VerificationPlan`, and `TechniqueResult`.
- Runtime validation derived from one JSON Schema packet.
- Adaptation of pinned Quoin method-catalog and advice exports.
- Prerequisite-aware verification planning across five input shapes.
- Exact evidence identity, scope, freshness, and result-state assessment.
- RFC 8785 JSON Canonicalization Scheme for digests and structured equality.
- A bounded local catalog portfolio that retains its own failures as results.
- Fixture validation, retained reports, and Rust repository CI checks for this slice.

### 2.2 Out of Scope

- Acceptance or replacement of the shared FS02/FS05 identity and version model.
- Publication of these private provisional contracts as cross-repository authority.
- Execution of a native parser, model checker, fuzzer, fault injector, or production system.
- A domain-subject portfolio, correlated-oracle evidence, or aggregate producer portfolio.
- A human acceptance disposition or an automated evidence-sufficiency decision.
- A duplicate evidence store or migration of Quoin-owned evidence.
- Backward compatibility for the rejected JavaScript module API, npm package, scripts, or runtime behavior outside the versioned JSON contracts and fixtures.

## 3. System Overview

The component adapts existing Quoin catalog data into portable technique records,
constructs explicit verification plans, validates externally supplied technique
results, and assesses those results against exact obligations. It also provides a
small local portfolio for checking its catalog adapter without representing that
external verification techniques have run.

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
| US-001 | Plan portable verification work | Specified; Rust pending |
| US-002 | Assess evidence without false attribution | Specified; Rust pending |
| US-003 | Detect compatibility regressions locally | Specified; Rust pending |
| FR-001 | Validate the five public contracts | Specified; Rust pending |
| FR-002 | Derive runtime contract behavior from the schema | Specified; Rust pending |
| FR-003 | Validate every fixture through an explicit handler | Specified; Rust pending |
| FR-004 | Adapt Quoin catalog and advice without changing meaning | Specified; Rust pending |
| FR-005 | Build prerequisite-aware verification plans | Specified; Rust pending |
| FR-006 | Match evidence by exact identity and execution scope | Specified; Rust pending |
| FR-007 | Preserve independent result findings | Specified; Rust pending |
| FR-008 | Canonicalize digest and identity inputs with JCS | Specified; Rust pending |
| FR-009 | Retain bounded portfolio failures as results | Specified; Rust pending |
| NFR-001 | Deterministic structured comparison | Specified; Rust pending |
| NFR-002 | Contract source maintainability | Specified; Rust pending |
| NFR-003 | Conservative assurance claims | Specified; Rust pending |
| NFR-004 | Rust implementation for the qualified path | Specified; Rust pending |
| NFR-005 | Bounded JSON ingestion | Specified; Rust pending |
| IT-001 | Real Quoin method-catalog parity | Specified; Rust pending |

## 6. Identification and Traceability

Artifact identifiers use the `US-NNN`, `FR-NNN`, `NFR-NNN`, `IT-NNN`, and
`TC-NNN` forms. Frontmatter relationships connect each normative requirement to
its driving story and each test artifact to the requirement it verifies.

## 7. Verification Strategy

Automated Rust tests verify examples, error paths, exact-binding dimensions, and
metamorphic order invariance. Fixture validation executes the versioned schema
against every JSON fixture. The Quoin integration test loads the real method
catalog from a pinned checkout and compares it with the retained export.

## 8. Lifecycle and Change Control

This specification describes a private compatibility slice being migrated to its
required Rust implementation. A future
FS02/FS05 decision may require a reviewed migration of the provisional artifact
reference seam. Such a migration must update requirements, tests, fixtures, and
retained reports before implementation claims change.

## 9. References

- RFC 8785, JSON Canonicalization Scheme.
- ISO/IEC/IEEE 29148, Requirements Engineering.
- `contracts/e02-draft-1.schema.json`, the private versioned contract packet.
