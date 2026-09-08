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
verification compatibility slice. It gives maintainers the reviewed and
implemented requirements baseline for the code, fixtures, retained reports,
and continuous-integration checks present in this repository.

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
| US-001 | Plan portable verification work | Implemented; verified |
| US-002 | Assess evidence without false attribution | Implemented; verified |
| US-003 | Detect compatibility regressions locally | Implemented; verified |
| FR-001 | Validate the five public contracts | Implemented; verified |
| FR-002 | Derive runtime contract behavior from the schema | Implemented; verified |
| FR-003 | Validate every fixture through an explicit handler | Implemented; verified |
| FR-004 | Adapt Quoin catalog and advice without changing meaning | Implemented; verified |
| FR-005 | Build prerequisite-aware verification plans | Implemented; verified |
| FR-006 | Match evidence by exact identity and execution scope | Implemented; verified |
| FR-007 | Preserve independent result findings | Implemented; verified |
| FR-008 | Canonicalize digest and identity inputs with JCS | Implemented; verified |
| FR-009 | Retain bounded portfolio failures as results | Implemented; verified |
| NFR-001 | Deterministic structured comparison | Implemented; verified |
| NFR-002 | Contract source maintainability | Implemented; verified |
| NFR-003 | Conservative assurance claims | Implemented; verified |
| NFR-004 | Rust implementation for the qualified path | Implemented; verified |
| NFR-005 | Bounded JSON ingestion | Implemented; verified |
| IT-001 | Real Quoin method-catalog parity | Implemented; verified |

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

This specification describes a private compatibility slice implemented and
verified in Rust. This lifecycle correction was made after the Rust rewrite was
merged; it records the already reviewed and verified state and introduces no new
behavioral requirement. A future FS02/FS05 decision may require a reviewed
migration of the provisional artifact reference seam. Such a migration must
update requirements, tests, fixtures, and retained reports before implementation
claims change.

## 9. References

- RFC 8785, JSON Canonicalization Scheme.
- ISO/IEC/IEEE 29148, Requirements Engineering.
- `contracts/e02-draft-1.schema.json`, the private versioned contract packet.
