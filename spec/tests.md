---
id: TM-001
title: "Bounded compatibility slice test matrix"
type: TestMatrix
---
# Test Matrix

## Overview

This matrix maps every normative acceptance criterion in the E02 compatibility
and VP01 qualification slice to a concrete test case. The 23 merged cases run in
Rust and bind through compiler-checked `ix-trace-rs` markers. TC-024 and TC-025
are specified for the VP01 increment and remain pending implementation. The
rejected JavaScript prototype is not accepted as evidence.

## Test Matrix Rules

1. Every functional acceptance criterion maps to at least one test case.
2. Meaningful option combinations cover input kinds, applicability, readiness, and result states.
3. Defined count and freshness constraints cover their boundary or refusal behavior.
4. Documented contract, catalog, planning, evidence, and execution errors have explicit cases.
5. Planning and result-state transitions are represented where the component controls them.
6. Duplicate, empty, reordered, malformed, stale, and simultaneous-failure edges are represented.

## Requirements Traceability

### User Story Coverage

| User Story | Acceptance Criteria | Test Cases | Coverage Status |
|------------|---------------------|------------|-----------------|
| US-001 | Ready task; unsafe fault task | TC-001..TC-004, TC-009..TC-011 | ✅ Complete |
| US-002 | Exact fresh result; competing results | TC-012..TC-017 | ✅ Complete |
| US-003 | Reordered catalog; adapter failure | TC-005..TC-008, TC-018..TC-021 | ✅ Complete |
| US-001, US-003 | Qualified selection semantics | TC-010, TC-024, TC-025 | 🚧 Pending |

### Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
|----------------|---------------------|------------|-----------------|
| FR-001 | FR-001-AC-1..5 | TC-001, TC-002 | ✅ Complete |
| FR-002 | FR-002-AC-1..3 | TC-003, TC-004 | ✅ Complete |
| FR-003 | FR-003-AC-1..3 | TC-005, TC-006 | ✅ Complete |
| FR-004 | FR-004-AC-1..3 | TC-007, TC-008, TC-021 | ✅ Complete |
| FR-005 | FR-005-AC-1..6 | TC-009, TC-010, TC-011 | ✅ Complete |
| FR-006 | FR-006-AC-1..5 | TC-012, TC-013, TC-014 | ✅ Complete |
| FR-007 | FR-007-AC-1..5 | TC-014, TC-015 | ✅ Complete |
| FR-008 | FR-008-AC-1..4 | TC-016, TC-017 | ✅ Complete |
| FR-009 | FR-009-AC-1..6 | TC-018, TC-019, TC-020 | ✅ Complete |
| FR-010 | FR-010-AC-1..5 | TC-010, TC-024, TC-025 | 🚧 Pending |

### Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
|--------------------|---------------------|---------------------|--------|
| NFR-001 | Property and deterministic-order tests | TC-012, TC-013, TC-015, TC-016, TC-018 | ✅ Complete |
| NFR-002 | Static inspection and fixture validation | TC-003, TC-005, TC-006 | ✅ Complete |
| NFR-003 | Adverse-state tests and claim inspection | TC-008, TC-014, TC-015, TC-020 | ✅ Complete |
| NFR-004 | Rust source and quality gates | TC-022 | ✅ Complete |
| NFR-005 | Boundary resource-limit tests | TC-023 | ✅ Complete |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---------|-------|------|----------|-----------|--------|
| TC-001 | Accept only five public contract kinds | Unit | P0 | FR-001-AC-1..FR-001-AC-2 | ✅ Passed |
| TC-002 | Reject contradictory and malformed contracts | Unit | P0 | FR-001-AC-3..FR-001-AC-5 | ✅ Passed |
| TC-003 | Derive runtime definitions from schema | Static | P0 | FR-002-AC-1..FR-002-AC-2, NFR-002-AC-1 | ✅ Passed |
| TC-004 | Reject invalid plan dependency graphs | Unit | P1 | FR-002-AC-3 | ✅ Passed |
| TC-005 | Execute schema against every retained fixture | Integration | P0 | FR-003-AC-1, NFR-002-AC-2 | ✅ Passed |
| TC-006 | Reject unhandled fixture and inspect CI | Integration | P1 | FR-003-AC-2..FR-003-AC-3, NFR-002-AC-2 | ✅ Passed |
| TC-007 | Preserve complete Quoin method records | Unit | P0 | FR-004-AC-1 | ✅ Passed |
| TC-008 | Qualify catalog conflicts and advice | Unit | P0 | FR-004-AC-2..FR-004-AC-3, NFR-003-AC-1 | ✅ Passed |
| TC-009 | Build ready tasks across input kinds | Unit | P0 | FR-005-AC-1..FR-005-AC-2 | ✅ Passed |
| TC-010 | Expose unsupported and blocked work | Unit | P0 | FR-005-AC-3..FR-005-AC-5 | ✅ Passed |
| TC-011 | Reject ambiguous compatible capability | Unit | P1 | FR-005-AC-6 | ✅ Passed |
| TC-012 | Reject every evidence-binding mismatch | Property | P0 | FR-006-AC-1..FR-006-AC-2, NFR-001-AC-2 | ✅ Passed |
| TC-013 | Refuse ambiguous or malformed identity | Unit | P0 | FR-006-AC-3..FR-006-AC-4, NFR-001-AC-2 | ✅ Passed |
| TC-014 | Distinguish freshness and result states | Unit | P0 | FR-006-AC-5, FR-007-AC-1, NFR-003-AC-1 | ✅ Passed |
| TC-015 | Retain findings and ownership fields | Unit | P0 | FR-007-AC-2..FR-007-AC-5, NFR-001-AC-2, NFR-003-AC-1 | ✅ Passed |
| TC-016 | Canonicalize valid JSON with JCS | Property | P0 | FR-008-AC-1..FR-008-AC-2, NFR-001-AC-1 | ✅ Passed |
| TC-017 | Reject values outside the JCS domain | Unit | P0 | FR-008-AC-3..FR-008-AC-4 | ✅ Passed |
| TC-018 | Compare catalog parity by technique ID | Property | P0 | FR-009-AC-1..FR-009-AC-2, NFR-001-AC-1 | ✅ Passed |
| TC-019 | Return violated and failed results | Unit | P0 | FR-009-AC-3..FR-009-AC-4 | ✅ Passed |
| TC-020 | Retain pending human work and claim limits | Unit | P0 | FR-009-AC-5..FR-009-AC-6, NFR-003-AC-2 | ✅ Passed |
| TC-021 | Execute real Quoin catalog baseline | Integration | P0 | FR-004-AC-1, FR-009-AC-1 | ✅ Passed |
| TC-022 | Enforce the Rust qualified-path implementation | Static | P0 | NFR-004-AC-1..NFR-004-AC-4 | ✅ Passed |
| TC-023 | Enforce the JSON resource envelope | Property | P0 | NFR-005-AC-1..NFR-005-AC-3 | ✅ Passed |
| TC-024 | Keep applicability, readiness, and classification independent | Unit | P0 | FR-010-AC-1..FR-010-AC-2 | 🚧 Pending |
| TC-025 | Retain authored policy and reject ordinal scoring | Unit | P0 | FR-010-AC-3..FR-010-AC-4 | 🚧 Pending |

## Option Permutation Matrix

| Test Case | Dimension A | Dimension B | Dimension C | Expected Behavior |
|-----------|-------------|-------------|-------------|-------------------|
| TC-009 | applicable | ready | qualified capability | invocation allowed |
| TC-010 | unknown | ready | qualified capability | invocation refused; readiness unchanged |
| TC-010 | not-applicable | ready | qualified capability | invocation refused; readiness unchanged |
| TC-010 | applicable | unsupported | absent capability | unsupported gap retained |
| TC-010 | applicable | blocked | stale policy | freshness prerequisite retained |
| TC-010 | applicable | blocked | fault inputs incomplete | three safety prerequisites retained |
| TC-024 | not-applicable | ready | qualified capability | readiness remains ready; invocation refused |
| TC-024 | unknown | ready | qualified capability | readiness remains ready; invocation refused |
| TC-024 | applicable | unsupported | absent capability | applicability remains applicable; invocation refused |
| TC-014 | completed | satisfied | adequate | supported |
| TC-014 | completed | satisfied | not activated | untriggered |
| TC-014 | completed | unknown | inadequate | inadequate |
| TC-014 | failed | not-evaluated | not assessed | failed |
| TC-015 | completed | violated | partial and assumption unmet | all independent gaps retained |

## Constraint Boundary Tests

| Constraint | Boundary Type | Test Value | Test Case | Expected |
|------------|---------------|------------|-----------|----------|
| Five public root contracts | Exact | 5 valid kinds | TC-001 | Pass |
| Five public root contracts | Outside set | Internal obligation object | TC-001 | Reject |
| Exact result cardinality | Min | 0 exact candidates | TC-012 | Missing |
| Exact result cardinality | Exact | 1 exact candidate | TC-014 | Classify result |
| Exact result cardinality | Above max | 2 exact candidates | TC-013 | Ambiguous |
| Result identity uniqueness | Exact | 1 result ID occurrence | TC-013 | Pass |
| Result identity uniqueness | Above max | 2 equal result IDs | TC-013 | Reject |
| Freshness | At limit | Age equal to maximum | TC-014 | Fresh |
| Freshness | Above limit | Age greater than maximum | TC-014 | Stale |
| JCS numeric domain | Boundary | Finite JSON number | TC-016 | Canonicalize |
| JCS numeric domain | Outside | NaN or Infinity | TC-017 | Reject |

## Error Paths

| Error ID | Condition | Test Case | Expected |
|----------|-----------|-----------|----------|
| ERR-001 | Public contract has contradictory execution and logical fields | TC-002 | Schema rejection |
| ERR-002 | Plan graph has duplicate, unknown, or cyclic dependency | TC-004 | Semantic rejection |
| ERR-003 | Discovered fixture has no handler | TC-006 | Named validation failure |
| ERR-004 | Catalog source has duplicate or unreadable input | TC-008 | Unqualified diagnostic view |
| ERR-005 | Multiple capabilities match without explicit selection | TC-011 | Ambiguity error |
| ERR-006 | Result binding differs in one required dimension | TC-012 | Missing with mismatch gap |
| ERR-007 | Multiple exact results exist | TC-013 | Ambiguous refusal |
| ERR-008 | Input is outside the JSON/JCS domain | TC-017 | Canonicalization error |
| ERR-009 | Portfolio comparison finds field drift | TC-019 | Violated result returned |
| ERR-010 | Portfolio adapter throws | TC-019 | Failed result returned |
| ERR-011 | Selection-policy rule contains an ordinal score | TC-025 | Contract rejection |

## State Transition Coverage

| State Model | Transition | Test Case | Expected |
|-------------|------------|-----------|----------|
| Plan readiness | prepared prerequisites → ready | TC-009 | invocable only when applicable |
| Plan readiness | missing prerequisites → blocked | TC-010 | setup tasks emitted |
| Plan readiness | missing capability → unsupported | TC-010 | unsupported gap emitted |
| Evidence freshness | within age → beyond age | TC-014 | fresh status becomes stale |
| Portfolio execution | check returns false | TC-019 | completed plus violated |
| Portfolio execution | check throws | TC-019 | failed plus not-evaluated |
| Human review | no disposition | TC-020 | not-run and incomplete |

## Edge Cases

| ID | Description | Related Req | Test Case | Risk if Untested |
|----|-------------|-------------|-----------|------------------|
| EC-001 | Source methods arrive in reverse or rotated order | FR-009 | TC-018 | Index-coupled parity can pass only on sorted fixtures |
| EC-002 | Three independent adverse result dimensions occur together | FR-007 | TC-015 | Summary status can erase useful findings |
| EC-003 | Two exact candidates have identical timestamps | FR-006 | TC-013 | Input order can select evidence silently |
| EC-004 | Array order differs while object meaning is otherwise valid | FR-008 | TC-016 | Canonical comparison can collapse ordered data |
| EC-005 | Malformed, over-deep, or out-of-domain JSON reaches canonicalization | FR-008 | TC-017, TC-023 | Hashing can fail opaquely or become input-dependent |
| EC-006 | Policy freshness prerequisite is listed as available | FR-005 | TC-010 | A caller can waive an expired policy locally |
| EC-007 | Property classification is omitted | FR-010 | TC-024 | Absence can be mistaken for an authored low-risk class |

## Integration Test Matrix

### Cross-Project Integrations

| Integration ID | Purpose | Target Project | Type | Test Cases | Status |
|----------------|---------|----------------|------|------------|--------|
| IT-001 | Compare retained catalog with real producer output | agent-ix/quoin plus spec-artifacts-process | service | TC-021 | ✅ |

### Integration Test Details

| Test Case | Integration | Scenario | Input | Expected | Priority |
|-----------|-------------|----------|-------|----------|----------|
| TC-021 | IT-001 | Pinned exact-source happy path | Real module root and pinned revisions | 33 methods, 0 duplicates, 0 unreadable roots | P0 |
| TC-021 | IT-001 | Producer or fixture drift | Any missing, extra, or changed method field | Nonzero comparison exit | P0 |
| TC-021 | IT-001 | Unreadable or conflicting module | Real loader diagnostic | Nonzero comparison exit | P0 |

## Resolved Coverage Gaps

| Gap ID | Resolution | Evidence |
|--------|------------|----------|
| GAP-001 | Added table-driven duplicate, unknown-edge, and cycle rejection. | TC-004 |
| GAP-002 | Added an isolated temporary fixture directory and unhandled-file refusal. | TC-006 |
| GAP-003 | Added ambiguous compatible capability refusal. | TC-011 |
| GAP-004 | Replaced every governed executable and test with Rust and added CI gates. | TC-022 |
| GAP-005 | Added byte, nesting, and array-size limits at public ingestion boundaries. | TC-023 |

## Test Execution Summary

| Category | Total | Passed | Failed | Pending | Coverage |
|----------|-------|--------|--------|---------|----------|
| Unit | 16 | 14 | 0 | 2 | 87.5% |
| Integration | 3 | 3 | 0 | 0 | 100% |
| Property | 4 | 4 | 0 | 0 | 100% |
| Static | 2 | 2 | 0 | 0 | 100% |
| Overall | 25 | 23 | 0 | 2 | 92% |
