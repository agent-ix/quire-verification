---
id: SR-056
title: "Scope-boundary review of the VP04 native parser portfolio specification"
type: SpecReview
analysis: scope-boundary
scope: "quire-verification@21c894d; VP04 ownership and external seams"
review_set: all
---
# Scope-Boundary Review of the VP04 Native Parser Portfolio Specification

## Summary

Reviewed portable verification, native parser, Engineering Assurance, Quoin,
human decision, operating-system, fixture-rights, and publication ownership
against the current Agent A/B/C assignments.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | high | Agent A must amend or disposition the shared JCS and role vocabulary, including the new review-disposition role; B must not fork the shared schema or assign nearby generic kinds. | FR-016-AC-5, FR-021-CON-7, specification PR #8 |
| FND-002 | low | B owns the native request adapter, portable TechniqueResults, conservative assessment, domain-specific timing observations, internal report shape, and exact disposition consumption. | FR-019..FR-021, NFR-006 |
| FND-003 | low | Agent A owns parser syntax, CLI behavior, and candidate qualification; IT-003 consumes an exact reviewed revision and cannot reinterpret a parse as model or evaluator evidence. | IT-003, FR-019-CON-1..2 |
| FND-004 | low | Agent C owns reusable Engineering Assurance implementation and Quoin-side integration. B authors MP-001 as a validated consumer and emits a portable report without adding a general measurement framework or changing Quoin. | MP-001, engineering-assurance#59 |
| FND-005 | low | Quoin owns evidence and sufficiency; the external human authors the disposition; B verifies supplied provenance but makes no authentication claim. | FR-021-CON-3..7, TC-037 |
| FND-006 | low | The first portfolio is limited to the exact reviewed Linux syntax profile and cleared synthetic fixtures; other operating systems, production, publication, model/evaluator work, and public licensing remain outside scope. | NFR-006 Scope, spec/spec.md |

## External Dependencies

| Dependency | Treatment | Contract |
|------------|-----------|----------|
| FS02/FS05 references | Guaranteed by independent Rust contract tests after amendment | FR-016, IT-002, TC-031 |
| Native parser | Guaranteed at one exact reviewed revision through real commands | IT-003, TC-034, TC-038 |
| Engineering Assurance artifact provider | Guaranteed only for the selected schema bytes; runtime ownership remains Agent C | MP-001, engineering-assurance#59 |
| Quoin evidence intake | External owner; B emits a versioned report and adds no store | evidenceOwner `Quoin` |
| Human disposition | Externally authored exact artifact; provenance verified, authentication not claimed | FR-021, TC-037 |
| Fixture rights | Required before IT-003; no inferred redistribution grant | IT-003 Preconditions |
