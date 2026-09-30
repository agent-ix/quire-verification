---
id: Plan-005
title: "VP04 native parser portfolio"
type: Plan
status: done
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-019", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-020", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-021", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-006", type: references }
---
# Implementation Plan: VP04 Native Parser Portfolio

## Requirements Summary

### Functional Requirements

- [x] **FR-019**: Execute two complementary bounded native parser techniques and retain separate manual work.
- [x] **FR-020**: Preserve reproducible setup and incremental effort observations without broad performance claims.
- [x] **FR-021**: Seal an immutable automated report core before binding one exact external human disposition.

### Non-Functional Requirements

- [x] **NFR-006**: Preserve the qualified containment boundary while admitting one exact expected exit status per invocation.

## Dependency Graph

- `NFR-006 -> Task-020`: the bounded process primitive must return one exact admitted status with complete streams before the parser's expected exit-1 refusal can be interpreted.
- `Plan-003/Task-015 + Plan-004/Task-019 + Task-020 -> Task-021`: qualified VP02 planning and VP03 assessment are complete; only the reopened expected-status process extension remains before portfolio execution.
- `Task-021 -> Task-022`: effort observations measure the exact admitted parser-portfolio phases and identities.
- `Plan-003/Task-012 + Task-021 + Task-022 -> Task-023`: the report needs the strict shared-reference reader, JCS identity domain, review-disposition role, execution records, and measurements.

### Shared dependencies

Plan-003 Task-012 remains the only shared-reference reader and the authoritative
E02 schema remains the only source for public contracts and the internal VP04
payload shape. Agent A owns the qualified native parser and shared FS02/FS05 wire.
Agent C owns reusable Engineering Assurance providers and generic report/chain
hosting; this plan consumes the validated MP-001 artifact and returns only VP04
domain observations and payloads.

### Cross-cutting constraints

NFR-001, NFR-003, NFR-004, and NFR-005 continue to require RFC 8785 JCS,
conservative claims, a Rust-only qualified path with compiler-checked
`ix-trace-rs` evidence, and bounded JSON ingestion. They are existing qualified
controls rather than duplicated VP04 work.

### The seams

Task-020 extends the bounded Linux child-process adapter beside the existing
Rust runner request types. Tasks 021 through 023 extend the single schema-derived
contract, planner, evidence assessor, measurement recorder, and VP04 payload
boundary.

## Test Plan

### Native process boundary

- [x] **TC-038** (`NFR-006-AC-1..8`): exercise descriptor-pinned direct argv and environment isolation; deadline admission and expiry; exact 16 MiB stream limits and first excess byte; concurrent drains; signals, statuses, UTF-8, protocol failures; root-relative no-symlink admission; pathname substitution; dedicated process groups; descendant cleanup; direct-child reaping; and Quire binding of every compiled marker argument.

### Portfolio execution

- [x] **TC-034** (`FR-019-AC-1..2`): run valid direct and roundtrip techniques, a completed metamorphic mismatch, the declared syntax diagnostic, and unexpected process/protocol failures.
- [x] **TC-035** (`FR-019-AC-3..6`): remove native prerequisites, cross the earliest freshness boundary, reuse or obscure observation relationships, and reject manual work at the native runner.

### Effort observations

- [x] **TC-036** (`FR-020-AC-1..5, FR-020-AC-8..9`): validate the exact 16-slot series, phase and repetition identities, integer domain, retained invalid observations, lower-median summaries, and deterministic recomputation.
- [x] **TC-037** (`FR-020-AC-6..7`, `FR-021-AC-1..5`): validate measurement interpretation and claim limits, the internal schema-derived report, non-circular core digest, absent and exact disposition states, stable refusals, and Quoin ownership.

### Qualification

- [x] **TC-022**: retain the Rust-only qualified path and compile every acceptance binding through `ix-trace-rs`.
- [x] **TC-023**: retain bounded JSON ingestion for every new public entry point.

## Completed Work

### Track P: Process primitive (reopened)

- **P1 = Task-020** Expected-status process extension — Medium; exit: one declared status from 0 through 255 returns with complete streams while invalid declarations, signals, and different statuses preserve every qualified containment guarantee.

### Track A: Critical Path (serial, dependency-gated)

- **A1 = Task-021** Native parser portfolio execution — Hard; exit: exact baseline, adverse, missing-prerequisite, freshness, correlation, and manual-task behavior is retained without fabricated support.
- **A2 = Task-022** Reproducible effort observations — Medium; exit: all 16 observations and every recomputable phase summary retain exact identity and validity.
- **A3 = Task-023** Report-core and disposition binding — Hard; exit: one schema-derived automated core has a non-circular JCS digest and exact external-disposition validation.

## Parallel Execution Summary

```text
Task-020 expected-status extension ---------------------+
Plan-003/Task-015 + Plan-004/Task-019 (complete) -------+-> Task-021 -> Task-022 -> Task-023
Plan-003/Task-012 + shared JCS/disposition roles ----------------------------+
```

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
|------|-------|-------------------|------------------------|--------|
| Task-020 | P | NFR-006 | TC-038 | done |
| Task-021 | A | FR-019 | TC-034, TC-035 | done |
| Task-022 | A | FR-020 | TC-036, TC-037 | done |
| Task-023 | A | FR-021 | TC-037 | done |

## Coordination Rules

Task-020's established descriptor and process-group behavior stays frozen while
the reviewed expected-status request/output slice is added. Tasks 021 through
023 then proceed serially. B does not edit Agent A's parser or shared schema,
Agent C's reusable Engineering Assurance implementation, Quoin, or Filament.
Changes to behavior return through QUOIN `/specify` and the owner-selected
all-lens `/spec-review` before implementation. Every acceptance test uses
compiled `ix-trace-rs` markers.
