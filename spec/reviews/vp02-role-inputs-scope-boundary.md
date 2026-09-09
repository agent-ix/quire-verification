---
id: SR-108
title: "Scope-boundary review of E02 draft-2 role-specific inputs"
type: SpecReview
analysis: scope-boundary
scope: "quire-verification@432036c; Task-013 ownership and external seams"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-015", type: reviews }
---

## Summary

Task-013 owns the private E02 draft-2 schema, generated Rust contract surface,
strict-reader-to-planner membership check, input preservation, and fixtures. It
does not own shared-reference definition, Quoin orchestration, native execution,
evidence storage, sufficiency, or Engineering Assurance.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Resolved: shared reference fields remain owned by `quire-specification@8ab058be`; E02 resolves that exact retained data instead of creating a second portable definition. | FR-002-AC-4, NFR-002-AC-3, FR-016-CON-1 |
| FND-002 | medium | Resolved: the strict reader guarantees reference validity and yields opaque values; the planner guarantees exact membership, role eligibility, preservation, and readiness. Neither assumes the other's responsibility. | FR-015-AC-4, FR-016 |
| FND-003 | low | Quoin remains an external TypeScript producer/orchestrator, and Rust-native Engineering Assurance remains Agent C's component. This crate owns only portable contracts and verification behavior. | AD-001, AP-001 |
| FND-004 | low | Native runner envelope construction remains Task-014; Task-013 may retain a null existing runner request but cannot authorize execution or convert manual work. | FR-014, FR-015-AC-3, Task-014 |

## Allocation

| Responsibility | Owner | Class |
|---|---|---|
| E02 draft-2 public schema and generated types | quire-verification contract module | core |
| Strict reference validation and opaque output | quire-verification shared-reference reader | core |
| Role preservation and membership check | quire-verification planner | core |
| Shared-reference schema definition | quire-specification | external, guaranteed by pinned packet tests |
| Quoin catalog/advice production | Quoin | external, guaranteed by compatibility tests |
| Native execution | Task-014/Task-020 boundaries | downstream infrastructure |
| Assurance storage and execution | Engineering Assurance | external Agent C ownership |
