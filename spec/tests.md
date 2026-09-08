---
id: TM-001
title: "Bounded compatibility slice test matrix"
type: TestMatrix
---
# Test Matrix

## Overview

This matrix maps every normative acceptance criterion in the E02 compatibility,
VP01 qualification, VP02 native-planning, and VP03 evidence-assessment slices to
a concrete test case. The 27 implemented cases run in Rust through compiler-checked
`ix-trace-rs` markers; TC-027 through TC-031 and TC-033 remain pending until their
reviewed implementation and shared-reference dependencies are ready. The rejected
JavaScript prototype is not evidence.

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
| US-001, US-003 | Qualified selection semantics | TC-010, TC-024, TC-025 | ✅ Complete |
| US-001 | Actionable native planning and strict references | TC-026..TC-031 | 🚧 Planned |
| US-002 | Conservative freshness and oracle dependence | TC-032, TC-033 | 🚧 Partial |

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
| FR-010 | FR-010-AC-1..2 | TC-010, TC-024 | ✅ Complete |
| FR-011 | FR-011-AC-1..2 | TC-024 | ✅ Complete |
| FR-012 | FR-012-AC-1..2 | TC-025 | ✅ Complete |
| FR-013 | FR-013-AC-1..5 | TC-026, TC-030 | ✅ Complete |
| FR-014 | FR-014-AC-1..4 | TC-027, TC-030 | 🚧 Planned |
| FR-015 | FR-015-AC-1..5 | TC-028, TC-030 | 🚧 Planned |
| FR-016 | FR-016-AC-1..6 | TC-029, TC-031 | ⛔ Blocked on FS02/FS05 amendment |
| FR-017 | FR-017-AC-1..5 | TC-033 | ⛔ Blocked on FS05 role amendment |
| FR-018 | FR-018-AC-1..5 | TC-032 | ✅ Complete |

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
| TC-024 | Keep applicability, readiness, and classification independent | Unit | P0 | FR-010-AC-1, FR-011-AC-1..FR-011-AC-2 | ✅ Passed |
| TC-025 | Retain authored policy and reject ordinal scoring | Unit | P0 | FR-012-AC-1..FR-012-AC-2 | ✅ Passed |
| TC-026 | Retain every actionable prerequisite | Property | P0 | FR-013-AC-1..FR-013-AC-5 | ✅ Passed |
| TC-027 | Authorize only bounded native runner requests | Property | P0 | FR-014-AC-1..FR-014-AC-4 | 🚧 Planned |
| TC-028 | Reject cross-variant verification inputs | Property | P0 | FR-015-AC-1..FR-015-AC-4 | 🚧 Planned |
| TC-029 | Refuse malformed or unsupported shared references | Property | P0 | FR-016-AC-1..FR-016-AC-6 | ⛔ Blocked |
| TC-030 | Build a heterogeneous parser portfolio plan | Integration | P0 | FR-013-AC-1..FR-013-AC-2, FR-014-AC-1..FR-014-AC-2, FR-015-AC-5 | 🚧 Planned |
| TC-031 | Execute the independent shared-reference consumer | Integration | P0 | FR-016-AC-1..FR-016-AC-6 | ⛔ Blocked |
| TC-032 | Apply the earliest evidence freshness bound | Unit | P0 | FR-018-AC-1..FR-018-AC-5 | ✅ Passed |
| TC-033 | Retain correlated and unknown oracle dependence | Property | P0 | FR-017-AC-1..FR-017-AC-5 | ⛔ Blocked |

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
| TC-015 | completed | violated | partial and assumption unmet | all independent gaps retained |
| TC-026 | required | absent allocation | missing resource | blocked task and setup dependency retained |
| TC-027 | applicable | ready | complete qualified runner | bounded request and invocation allowed |
| TC-027 | applicable | blocked | incomplete fault authorization | null runner request and invocation refused |
| TC-030 | parser property | differential relation | manual review | distinct inputs and eligibility retained |
| TC-029 | supported semantic ref | reordered unique features | exact canonical identity | accepted as the same feature set |
| TC-029 | supported semantic ref | duplicate feature | exact canonical identity | refused before planning |
| TC-032 | result expiry earlier | obligation expiry later | exact boundary | fresh at boundary; stale after |
| TC-032 | result expiry later | obligation expiry earlier | exact boundary | fresh at boundary; stale after |
| TC-033 | distinct techniques | shared observation | declared independent | correlated finding; no independence claim |
| TC-033 | distinct techniques | distinct observations | unknown relation | unknown dependence; no aggregate score |

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
| Combined freshness | Exact | Assessment at earliest expiry | TC-032 | Fresh |
| Combined freshness | Outside | One representable instant after earliest expiry | TC-032 | Stale |
| Maximum age | Min | 0 seconds | TC-032 | Fresh only at production instant |
| Maximum age | Max | 9007199254740991 seconds | TC-032 | Accept or refuse only on checked timestamp overflow |
| Maximum age | Under min | -1 second | TC-032 | Refuse before classification |
| Maximum age | Outside | 9007199254740992 seconds | TC-032 | Refuse before classification |
| JCS numeric domain | Boundary | Finite JSON number | TC-016 | Canonicalize |
| JCS numeric domain | Outside | NaN or Infinity | TC-017 | Reject |
| Shared byte digest | Exact | SHA-256 of supplied bytes | TC-029 | Accept selected reference |
| Shared byte digest | Mismatch | One changed byte | TC-029 | Refuse before planning |
| Required feature set | Duplicate | Same decoded feature twice | TC-029 | Refuse before comparison |

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
| ERR-012 | Property classification is an empty string | TC-024 | Contract rejection |
| ERR-013 | Required work lacks a prerequisite or budget | TC-026 | Blocked task and actionable setup dependency |
| ERR-014 | A non-invocable task carries a runner request | TC-027 | Contract or planner refusal |
| ERR-015 | An input omits a role or carries a cross-variant field | TC-028 | Closed-variant rejection |
| ERR-016 | Shared reference has a duplicate key, unknown version or feature, wrong role, or mismatched digest | TC-029, TC-031 | Stable pre-planning refusal |
| ERR-017 | Result production is in the future or validity precedes production | TC-032 | `invalid_temporal_order` refusal |
| ERR-018 | Oracle dependence is unknown, shared, or derived | TC-033 | Retain exact dependence finding; make no independence claim |
| ERR-019 | Oracle contribution ID is duplicated or required relation is absent | TC-033 | Contract refusal before assessment |

## State Transition Coverage

| State Model | Transition | Test Case | Expected |
|-------------|------------|-----------|----------|
| Plan readiness | prepared prerequisites → ready | TC-009 | invocable only when applicable |
| Plan readiness | missing prerequisites → blocked | TC-010 | setup tasks emitted |
| Plan readiness | missing capability → unsupported | TC-010 | unsupported gap emitted |
| Evidence freshness | within age → beyond age | TC-014 | fresh status becomes stale |
| Combined freshness | at earliest expiry → after earliest expiry | TC-032 | fresh status becomes stale |
| Oracle dependence | independent → shared or derived | TC-033 | correlation finding appears without aggregate score |
| Portfolio execution | check returns false | TC-019 | completed plus violated |
| Portfolio execution | check throws | TC-019 | failed plus not-evaluated |
| Human review | no disposition | TC-020 | not-run and incomplete |
| Required task | allocation present → absent | TC-026 | ready task becomes blocked and remains visible |
| Native runner | ready → prerequisite missing | TC-027 | runner request becomes null |

## Edge Cases

| ID | Description | Related Req | Test Case | Risk if Untested |
|----|-------------|-------------|-----------|------------------|
| EC-001 | Source methods arrive in reverse or rotated order | FR-009 | TC-018 | Index-coupled parity can pass only on sorted fixtures |
| EC-002 | Three independent adverse result dimensions occur together | FR-007 | TC-015 | Summary status can erase useful findings |
| EC-003 | Two exact candidates have identical timestamps | FR-006 | TC-013 | Input order can select evidence silently |
| EC-004 | Array order differs while object meaning is otherwise valid | FR-008 | TC-016 | Canonical comparison can collapse ordered data |
| EC-005 | Malformed, over-deep, or out-of-domain JSON reaches canonicalization | FR-008 | TC-017, TC-023 | Hashing can fail opaquely or become input-dependent |
| EC-006 | Policy freshness prerequisite is listed as available | FR-005 | TC-010 | A caller can waive an expired policy locally |
| EC-007 | Property classification is omitted | FR-011 | TC-024 | Absence can be mistaken for an authored low-risk class |
| EC-008 | Two tasks require the same missing prerequisite | FR-013 | TC-026 | Duplicate setup work or inconsistent dependency IDs |
| EC-009 | Descriptive tooling text names an unavailable executable | FR-014 | TC-027 | Catalog prose can be mistaken for installed capability |
| EC-010 | A field from one input variant appears on another | FR-015 | TC-028 | Reader can guess a false unary interpretation |
| EC-011 | JCS semantic digest is placed in an exact byte-digest field | FR-016 | TC-029 | Unrelated identity domains can be treated as equal |
| EC-012 | Required feature set is reordered or duplicated | FR-016 | TC-029 | Equivalent sets can fail or duplicate requirements can pass |
| EC-013 | Result and obligation expiry disagree | FR-018 | TC-032 | A later bound can silently extend stale evidence |
| EC-014 | Different methods reuse one observation or derived oracle | FR-017 | TC-033 | Correlated outputs can be presented as independent confirmation |
| EC-015 | Equivalent timestamps use different RFC 3339 offsets | FR-018 | TC-032 | Text comparison can disagree about one instant |
| EC-016 | Maximum age addition exceeds the timestamp domain | FR-018 | TC-032 | Arithmetic can wrap, panic, or extend evidence validity |

## Integration Test Matrix

### Cross-Project Integrations

| Integration ID | Purpose | Target Project | Type | Test Cases | Status |
|----------------|---------|----------------|------|------------|--------|
| IT-001 | Compare retained catalog with real producer output | agent-ix/quoin plus spec-artifacts-process | service | TC-021 | ✅ |
| IT-002 | Qualify the shared-reference packet with an independent Rust reader | agent-ix/quire-specification FS02/FS05 | contract | TC-031 | ⛔ Blocked |

### Integration Test Details

| Test Case | Integration | Scenario | Input | Expected | Priority |
|-----------|-------------|----------|-------|----------|----------|
| TC-021 | IT-001 | Pinned exact-source happy path | Real module root and pinned revisions | 33 methods, 0 duplicates, 0 unreadable roots | P0 |
| TC-021 | IT-001 | Producer or fixture drift | Any missing, extra, or changed method field | Nonzero comparison exit | P0 |
| TC-021 | IT-001 | Unreadable or conflicting module | Real loader diagnostic | Nonzero comparison exit | P0 |
| TC-031 | IT-002 | Exact supported shared references | Pinned schema, artifacts, and positive vectors | Every B-used role accepted | P0 |
| TC-031 | IT-002 | One-axis reference mutations | Pinned adverse vectors and B mutations | Stable refusal before planning | P0 |
| TC-031 | IT-002 | Unmapped JCS or trace/fault/manual role | Consumer role composition | No permissive fallback; integration fails | P0 |

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
| Unit | 16 | 16 | 0 | 0 | 100% |
| Integration | 5 | 3 | 0 | 2 | 60% |
| Property | 10 | 6 | 0 | 4 | 60% |
| Static | 2 | 2 | 0 | 0 | 100% |
| Overall | 33 | 27 | 0 | 6 | 82% |
