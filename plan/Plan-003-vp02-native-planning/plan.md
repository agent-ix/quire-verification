---
id: Plan-003
title: "VP02 native planning"
type: Plan
status: done
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-002", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-013", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-014", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-015", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-016", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-004", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-005", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-002", type: references }
---
# Implementation Plan: VP02 Native Planning

## Requirements Summary

### Functional Requirements

- [x] **FR-013**: Emit an actionable prerequisite DAG.
- [x] **FR-014**: Construct exact native runner requests.
- [x] **FR-015**: Preserve role-specific verification inputs.
- [x] **FR-016**: Read shared references strictly.

### Non-Functional Requirements

- [x] **NFR-002-AC-3**: Resolve shared artifact envelopes from the retained schema without copying their definition.
- [x] **NFR-004**: Keep all first-party executable and test paths in Rust.
- [x] **NFR-005**: Apply the existing bounded JSON envelope before strict reading.

### Contract Enablement

- [x] **FR-002-AC-4**: Generate draft-2 input types through the exact retained shared-reference schema.

## Dependency Graph

- `Plan-001/Task-003 -> Task-011`: prerequisite coalescing extends the existing Rust planner.
- `quire-specification@8ab058be -> Task-012`: the accepted FS02/FS05 packet supplies the selected JCS domain, roles, version split, and vectors.
- `Task-012 -> Task-013`: closed role-specific inputs depend on the selected shared references.
- `Task-011 + Task-013 -> Task-014`: runner requests require both prerequisite and input semantics.
- `Task-011..Task-014 -> Task-015`: qualification covers the complete VP02 slice.

### Shared dependencies

Task-012 is the only shared-reference reader. Downstream tasks reuse its generated
types and stable error envelope rather than authoring another identity model.

### Cross-cutting constraints

NFR-004 and NFR-005 apply to every task. Quoin remains the evidence owner; native
producers retain native artifacts and Agent A owns the shared schema.

## Test Plan

### Prerequisites

- [x] **TC-026**: Coalesce exact missing needs, retain mandatory work, and reject near matches.

### Native request and input variants

- [x] **TC-027**: Emit complete runner requests only for invocable work.
- [x] **TC-028**: Accept complete role-specific inputs and reject cross-variant fields.
- [x] **TC-030**: Build a heterogeneous parser portfolio plan.

### Shared reference integration

- [x] **TC-029**: Refuse malformed, unsupported, conflicting, or ambiguous shared references.
- [x] **TC-031**: Execute the independent Rust consumer over the pinned shared packet.

### Qualified-path gates

- [x] **TC-022**: Enforce the Rust-only qualified path.
- [x] **TC-023**: Enforce the bounded JSON envelope.

## Remaining Work

### Track A: Critical Path (serial)

- **A1 = Task-011** Actionable prerequisite DAG — done.
- **A2 = Task-012** Strict shared-reference reader — done at `ce4e03e`; the independent Rust reader accepts exact vectors and refuses every selected adverse case.
- **A3 = Task-013** Role-specific inputs — done at `70f9d6b`; the breaking draft-2 schema resolves the retained shared definition and all five closed variants retain exact validated roles and reject cross-variant data.
- **A4 = Task-014** Native runner requests — done at `86ebb0b`; only individually authorized work receives an exact invocation envelope.
- **Gate = Task-015** VP02 qualification — done at exact Rust review SR-128; the bundle is closed for independent QUOIN gap analysis and reopens on any blocking finding.

## Parallel Execution Summary

```text
Plan-001/Task-003 -> Task-011 -------------------+
FS02/FS05@8ab058be -> Task-012 -> Task-013 ------+-> Task-014 -> Task-015
```

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
|------|-------|-------------------|------------------------|--------|
| Task-011 | A | FR-013 | TC-026 | done |
| Task-012 | A | FR-016 | TC-029, TC-031 | done |
| Task-013 | A | FR-002, FR-015, NFR-002 | TC-028, TC-030 | done |
| Task-014 | A | FR-005, FR-013, FR-014, FR-015, NFR-006 | TC-027, TC-030 | done |
| Task-015 | Gate | NFR-004, NFR-005 | TC-022, TC-023 | done |

## Coordination Rules

Task-012 consumed exact merged specification revision `8ab058be` after the
QUOIN-reviewed adoption at `9fb29ff` and passed exact Rust review SR-100. B does
not edit A's schema, C's Engineering Assurance implementation, Quoin, or
Filament. New behavior that changes these requirements returns to `/specify` and
`/spec-review` before implementation.

Task-013 implements the QUOIN-reviewed draft-2 amendment at `c791101`
(SR-102 through SR-109), passed exact Rust review SR-118, and closed at gap
analysis SR-119. Its schema references the exact retained shared packet without
copying that packet's artifact-reference field definitions. Task-014 completed
the remaining FR-015-AC-5 and TC-030 runner-eligibility integration.

Task-014 implements the QUOIN-authored runner boundary at `c683b5b`, accepted
through composite review workflow `4a60dcc5-e3fc-4d46-b3f9-7806027dfd19` and
SR-120 through SR-127 at `adaff03`. It owns the closed request, budget precedence,
capability ceilings, manual refusal, and TC-027/TC-030 integration behavior.
Implementation `86ebb0b`, retained portfolio `96344bc`, and exact Rust review
SR-128 establish the completed candidate. All 44 first-party Rust tests, strict
Clippy, formatting, fixture validation, dependency policy, and 47/47 Quire Rust
trace bindings pass before the independent gap-analysis audit.
