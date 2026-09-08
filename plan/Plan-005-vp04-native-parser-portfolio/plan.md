---
id: Plan-005
title: "VP04 native parser portfolio"
type: Plan
status: active
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-019", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-020", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-021", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-006", type: references }
---
# Implementation Plan: VP04 Native Parser Portfolio

## Requirements Summary

### Functional Requirements

- [ ] **FR-019**: Execute two complementary bounded native parser techniques and retain separate manual work.
- [ ] **FR-020**: Preserve reproducible setup and incremental effort observations without broad performance claims.
- [ ] **FR-021**: Seal an immutable automated report core before binding one exact external human disposition.

### Non-Functional Requirements

- [ ] **NFR-006**: Bound direct native process execution by executable, arguments, environment, directory, time, output, status, and protocol.

## Dependency Graph

- `NFR-006 -> Task-020`: the bounded process primitive is independently testable with controlled native child fixtures before the real parser is selected.
- `Plan-003/Task-015 + Plan-004/Task-019 + Task-020 -> Task-021`: portfolio execution requires qualified VP02 planning, qualified VP03 assessment, and bounded process behavior.
- `Task-021 -> Task-022`: effort observations measure the exact admitted parser-portfolio phases and identities.
- `Plan-003/Task-012 + Task-021 + Task-022 -> Task-023`: the report needs the strict shared-reference reader, JCS identity domain, review-disposition role, execution records, and measurements.
- `Task-023 -> Task-024`: integration and qualification consume the sealed report behavior and then obtain one externally authored disposition.

### Shared dependencies

Plan-003 Task-012 remains the only shared-reference reader and the authoritative
E02 schema remains the only source for public contracts and the internal VP04
report shape. Agent A owns the qualified native parser and shared FS02/FS05 wire.
Agent C owns the reusable Engineering Assurance provider; this plan only consumes
the validated MP-001 artifact and emits its domain observations.

### Cross-cutting constraints

NFR-001, NFR-003, NFR-004, and NFR-005 continue to require RFC 8785 JCS,
conservative claims, a Rust-only qualified path with compiler-checked
`ix-trace-rs` evidence, and bounded JSON ingestion. They are existing qualified
controls rather than duplicated VP04 work.

### The seams

Task-020 adds a bounded Linux child-process adapter beside the existing Rust
runner request types. Tasks 021 through 023 extend the single schema-derived
contract, planner, evidence assessor, and report boundary. Task-024 invokes the
real reviewed `quire-spec` CLI and retains the report for external disposition.

## Test Plan

### Native process boundary

- [ ] **TC-038** (`NFR-006-AC-1..6`): exercise direct argv and environment isolation; deadline admission and expiry; exact 16 MiB stream limits and first excess byte; concurrent drains; signals, statuses, UTF-8, protocol failures; executable-root and symbolic-link refusal; and child reaping.

### Portfolio execution

- [ ] **TC-034** (`FR-019-AC-1..2`): run valid direct and roundtrip techniques, a completed metamorphic mismatch, the declared syntax diagnostic, and unexpected process/protocol failures.
- [ ] **TC-035** (`FR-019-AC-3..6`): remove native prerequisites, cross the earliest freshness boundary, reuse or obscure observation relationships, and reject manual work at the native runner.

### Effort observations

- [ ] **TC-036** (`FR-020-AC-1..5, FR-020-AC-8..9`): validate the exact 16-slot series, phase and repetition identities, integer domain, retained invalid observations, lower-median summaries, and deterministic recomputation.
- [ ] **TC-037** (`FR-020-AC-6..7`, `FR-021-AC-1..5`): validate measurement interpretation and claim limits, the internal schema-derived report, non-circular core digest, absent and exact disposition states, stable refusals, and Quoin ownership.

### Integration and qualification

- [ ] **IT-003-SC-01..09**: execute the exact qualified Rust parser from portable plan through bounded results, assessment, measurements, sealed report core, and external owner disposition.
- [ ] **TC-022**: retain the Rust-only qualified path and compile every acceptance binding through `ix-trace-rs`.
- [ ] **TC-023**: retain bounded JSON ingestion for every new public entry point.

## Remaining Work

### Track P: Process primitive (ready)

- **P1 = Task-020** Bounded native process executor — Hard; exit: every admitted child terminates with bounded output and a typed result, and every adverse child is reaped.

### Track A: Critical Path (serial, dependency-gated)

- **A1 = Task-021** Native parser portfolio execution — Hard; exit: exact baseline, adverse, missing-prerequisite, freshness, correlation, and manual-task behavior is retained without fabricated support.
- **A2 = Task-022** Reproducible effort observations — Medium; exit: all 16 observations and every recomputable phase summary retain exact identity and validity.
- **A3 = Task-023** Report-core and disposition binding — Hard; exit: one schema-derived automated core has a non-circular JCS digest and exact external-disposition validation.
- **Gate = Task-024** VP04 integration and qualification — Hard; pass: IT-003, Rust review, QUOIN gap analysis, and the required external owner disposition have no unresolved in-scope finding.

## Parallel Execution Summary

```text
Task-020 -----------------------------------------------+
Plan-003/Task-015 + Plan-004/Task-019 -----------------+-> Task-021 -> Task-022 -> Task-023 -> Task-024
Plan-003/Task-012 + shared JCS/disposition roles ----------------------------+
qualified parser + cleared fixtures -------------------------------------------> Task-024
external owner disposition ----------------------------------------------------> Task-024
```

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
|------|-------|-------------------|------------------------|--------|
| Task-020 | P | NFR-006 | TC-038 | not_started |
| Task-021 | A | FR-019 | TC-034, TC-035 | blocked |
| Task-022 | A | FR-020 | TC-036, TC-037 | blocked |
| Task-023 | A | FR-021 | TC-037 | blocked |
| Task-024 | Gate | FR-019, FR-020, FR-021, NFR-006 | IT-003, TC-022, TC-023, TC-034..TC-038 | blocked |

## Coordination Rules

Task-020 may proceed against controlled Rust child fixtures. Tasks 021 through
024 remain blocked until their explicit predecessors and external candidate/role
gates clear. B does not edit Agent A's parser or shared schema, Agent C's reusable
Engineering Assurance implementation, Quoin, or Filament. Changes to behavior
return through QUOIN `/specify` and the owner-selected all-lens `/spec-review`
before implementation. Every acceptance test uses compiled `ix-trace-rs` markers.
