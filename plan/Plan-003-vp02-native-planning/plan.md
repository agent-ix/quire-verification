---
id: Plan-003
title: "VP02 native planning"
type: Plan
status: active
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-013", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-014", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-015", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-016", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-004", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-005", type: references }
---
# Implementation Plan: VP02 Native Planning

## Requirements Summary

### Functional Requirements

- [x] **FR-013**: Emit an actionable prerequisite DAG.
- [ ] **FR-014**: Construct exact native runner requests.
- [ ] **FR-015**: Preserve role-specific verification inputs.
- [ ] **FR-016**: Read shared references strictly.

### Non-Functional Requirements

- [ ] **NFR-004**: Keep all first-party executable and test paths in Rust.
- [ ] **NFR-005**: Apply the existing bounded JSON envelope before strict reading.

## Dependency Graph

- `Plan-001/Task-003 -> Task-011`: prerequisite coalescing extends the existing Rust planner.
- `FS02 + FS05 amendment -> Task-012`: the independent reader needs representable JCS and role identities.
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

- [ ] **TC-027**: Emit complete runner requests only for invocable work.
- [ ] **TC-028**: Accept complete role-specific inputs and reject cross-variant fields.
- [ ] **TC-030**: Build a heterogeneous parser portfolio plan.

### Shared reference integration

- [ ] **TC-029**: Refuse malformed, unsupported, conflicting, or ambiguous shared references.
- [ ] **TC-031**: Execute the independent Rust consumer over the pinned shared packet.

### Qualified-path gates

- [x] **TC-022**: Enforce the Rust-only qualified path.
- [x] **TC-023**: Enforce the bounded JSON envelope.

## Remaining Work

### Track A: Critical Path (serial)

- **A1 = Task-011** Actionable prerequisite DAG — Medium; exit: every missing need is one deterministic setup dependency and mandatory work remains visible.
- **A2 = Task-012** Strict shared-reference reader — Hard; exit: the independent Rust reader accepts exact vectors and refuses every one-axis adverse case.
- **A3 = Task-013** Role-specific inputs — Medium; exit: all five closed variants retain exact roles and reject cross-variant data.
- **A4 = Task-014** Native runner requests — Medium; exit: only individually authorized work receives an exact invocation envelope.
- **Gate = Task-015** VP02 qualification — Hard; pass: Rust review and QUOIN gap analysis have no unresolved high finding.

## Parallel Execution Summary

```text
Plan-001/Task-003 -> Task-011 -------------------+
FS02 + FS05 ------> Task-012 -> Task-013 --------+-> Task-014 -> Task-015
```

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
|------|-------|-------------------|------------------------|--------|
| Task-011 | A | FR-013 | TC-026 | done |
| Task-012 | A | FR-016 | TC-029, TC-031 | blocked |
| Task-013 | A | FR-015 | TC-028, TC-030 | blocked |
| Task-014 | A | FR-014 | TC-027, TC-030 | blocked |
| Task-015 | Gate | NFR-004, NFR-005 | TC-022, TC-023 | blocked |

## Coordination Rules

Task-011 may proceed against the reviewed existing planner. Tasks 012 through 015
remain blocked until Agent A resolves the accepted shared-contract findings. B
does not edit A's schema, C's Engineering Assurance implementation, Quoin, or
Filament. New behavior that changes these requirements returns to `/specify` and
`/spec-review` before implementation.
