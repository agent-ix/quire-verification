---
id: TM-001
title: "Bounded verification and production qualification test matrix"
type: TestMatrix
---
# Test Matrix

## Overview

This matrix maps every normative acceptance criterion in the E02 compatibility,
VP01 qualification, VP02 native-planning, VP03 evidence-assessment, VP04
bounded-parser-portfolio, and proposed production-qualification slices to a
concrete test case. The implemented TC-001 through TC-038 evidence runs in Rust
through compiler-checked `ix-trace-rs` markers. TC-039 and TC-040 implement the
qualification profile and corpus-validation boundaries; TC-041 implements the exact
Engineering Assurance consumer boundary, while TC-042 through TC-045 define the remaining
test-first qualification scope. TC-047 through TC-052 and IT-005 define the
complete-V1 method/evidence development slice independently of production
ceremony. The production profile and corpus remain
externally blocked, so no production qualification is claimed. The IT-003 automated
core is retained separately from the still-pending external disposition. The
rejected JavaScript prototype is not evidence.

## Test Matrix Rules

1. Every functional acceptance criterion maps to at least one test case.
2. Meaningful option combinations cover input kinds, applicability, readiness, and result states.
3. Defined count and freshness constraints cover their boundary or refusal behavior.
4. Documented contract, catalog, planning, evidence, and execution errors have explicit cases.
5. Planning and result-state transitions are represented where the component controls them.
6. Duplicate, empty, reordered, malformed, stale, and simultaneous-failure edges are represented.

## Requirements Traceability

### Stakeholder Requirement Coverage

| Stakeholder Req | Trace to US/FR | Test/Validation | Coverage Status |
|-----------------|----------------|-----------------|-----------------|
| StR-002 | US-004, FR-022..FR-027, NFR-007..NFR-008 | StR-002-VC-1..3 via TC-039..TC-045 and IT-004 | 🚧 Planned |

### User Story Coverage

| User Story | Acceptance Criteria | Test Cases | Coverage Status |
|------------|---------------------|------------|-----------------|
| US-001 | Ready task; unsafe fault task | TC-001..TC-004, TC-009..TC-011 | ✅ Complete |
| US-002 | Exact fresh result; competing results | TC-012..TC-017 | ✅ Complete |
| US-003 | Reordered catalog; adapter failure | TC-005..TC-008, TC-018..TC-021 | ✅ Complete |
| US-001, US-003 | Qualified selection semantics | TC-010, TC-024, TC-025 | ✅ Complete |
| US-001 | Actionable native planning and strict references | TC-026..TC-031 | ✅ Complete |
| US-002 | Conservative freshness and oracle dependence | TC-032, TC-033 | ✅ Complete |
| US-001, US-002, US-003 | Bounded native parser portfolio, measured effort, and disposition | TC-034..TC-038 | 🚧 Automated evidence complete; external disposition pending |
| US-004 | One exact production qualification profile | TC-039..TC-045, IT-004 | 🚧 Planned |
| US-005 | Complete-V1 methods, replayable evidence, and exact ledger accounting | TC-047..TC-052, IT-005 | 🚧 Task-040 implementation and focused traces in progress |

### Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
|----------------|---------------------|------------|-----------------|
| FR-001 | FR-001-AC-1..5 | TC-001, TC-002 | ✅ Complete |
| FR-002 | FR-002-AC-1..4 | TC-003, TC-004, TC-028 | ✅ Complete |
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
| FR-014 | FR-014-AC-1..4 | TC-027, TC-030 | ✅ Complete |
| FR-015 | FR-015-AC-1..5 | TC-028, TC-030 | ✅ Complete |
| FR-016 | FR-016-AC-1..7 | TC-029, TC-031 | ✅ Implemented against accepted packet |
| FR-017 | FR-017-AC-1..7 | TC-033 | ✅ Complete |
| FR-018 | FR-018-AC-1..5 | TC-032 | ✅ Complete |
| FR-019 | FR-019-AC-1..6 | TC-034, TC-035, IT-003 | ✅ Implemented; automated core retained |
| FR-020 | FR-020-AC-1..9 | TC-036, TC-037, IT-003 | ✅ Implemented; automated core retained |
| FR-021 | FR-021-AC-1..5 | TC-037, IT-003 | ✅ Implemented; external disposition pending |
| FR-022 | FR-022-AC-1..5 | TC-039, IT-004 | 🚧 Planned |
| FR-023 | FR-023-AC-1..4 | TC-040, IT-004 | ✅ Rust boundary implemented; production corpus blocked |
| FR-024 | FR-024-AC-1..4 | TC-041, IT-004 | 🚧 Rust boundary implemented; EA candidate blocked by TC-031 regression |
| FR-025 | FR-025-AC-1..6 | TC-042, IT-004 | 🚧 Planned |
| FR-026 | FR-026-AC-1..4 | TC-043, IT-004 | 🚧 Planned |
| FR-027 | FR-027-AC-1..5 | TC-045, IT-004 | 🚧 Planned |
| FR-028 | FR-028-AC-1..3 | TC-002, TC-029, TC-031, TC-046 | ✅ Complete |
| FR-029 | FR-029-AC-1..3 | TC-047, IT-005 | ✅ Complete |
| FR-030 | FR-030-AC-1..3 | TC-048, IT-005 | ✅ Complete |
| FR-031 | FR-031-AC-1..3 | TC-049, IT-005 | ✅ Complete |
| FR-032 | FR-032-AC-1..3 | TC-050, IT-005 | 🚧 Focused implementation and review complete; plan integration pending |
| FR-033 | FR-033-AC-1..3 | TC-052, IT-005 | 🚧 Focused implementation and review complete; plan integration pending |
| FR-034 | FR-034-AC-1..3 | TC-051, IT-005 | 🚧 Focused implementation and review complete; plan integration pending |

### Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
|--------------------|---------------------|---------------------|--------|
| NFR-001 | Property and deterministic-order tests | TC-012, TC-013, TC-015, TC-016, TC-018 | ✅ Complete |
| NFR-002 | Static inspection and fixture validation | TC-003, TC-005, TC-006, TC-028 | ✅ Complete |
| NFR-003 | Adverse-state tests and claim inspection | TC-008, TC-014, TC-015, TC-020 | ✅ Complete |
| NFR-004 | Rust source, quality gates, and maintained toolchain/dependency guidance | TC-022 | ✅ Complete |
| NFR-005 | Boundary resource-limit tests | TC-023 | ✅ Complete |
| NFR-006 | Native process containment tests | TC-038 | ✅ Complete |
| NFR-007 | Owner-threshold performance measurement | TC-044, IT-004 | 🚧 Planned |
| NFR-008 | Independent artifact and result reproduction | TC-044, TC-045, IT-004 | 🚧 Planned |
| NFR-009 | Deterministic bounded method and ledger accounting | TC-047..TC-049, TC-051, IT-005 | 🚧 Specified; implementation pending |

### Qualification Constraint Coverage

| Constraint | Test Cases | Coverage Status |
|------------|------------|-----------------|
| FR-022-CON-1..2 | TC-039 | 🚧 Planned |
| FR-023-CON-1 | TC-040 | ✅ Complete |
| FR-024-CON-1..2 | TC-041 | 🚧 Focused TC-041 passes; EA candidate blocked by TC-031 regression |
| FR-025-CON-1..4 | TC-042 | 🚧 Planned |
| FR-026-CON-1..2 | TC-043 | 🚧 Planned |
| FR-027-CON-1..2 | TC-045 | 🚧 Planned |
| FR-029-CON-1..3 | TC-047 | ✅ Complete |
| FR-030-CON-1..3 | TC-048 | ✅ Complete |
| FR-031-CON-1..2 | TC-049 | ✅ Complete |
| FR-032-CON-1..2 | TC-050 | 🚧 Specified; implementation pending |
| FR-034-CON-1..3 | TC-051 | 🚧 Specified; implementation pending |

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
| TC-022 | Enforce the Rust qualified-path implementation | Static | P0 | NFR-004-AC-1..NFR-004-AC-6 | ✅ Passed |
| TC-023 | Enforce the JSON resource envelope | Property | P0 | NFR-005-AC-1..NFR-005-AC-3 | ✅ Passed |
| TC-024 | Keep applicability, readiness, and classification independent | Unit | P0 | FR-010-AC-1, FR-011-AC-1..FR-011-AC-2 | ✅ Passed |
| TC-025 | Retain authored policy and reject ordinal scoring | Unit | P0 | FR-012-AC-1..FR-012-AC-2 | ✅ Passed |
| TC-026 | Retain every actionable prerequisite | Property | P0 | FR-013-AC-1..FR-013-AC-5 | ✅ Passed |
| TC-027 | Authorize only bounded native runner requests | Property | P0 | FR-005-AC-2, FR-013-AC-2, FR-014-AC-1..FR-014-AC-4 | ✅ Passed |
| TC-028 | Reject cross-variant verification inputs | Property | P0 | FR-002-AC-4, FR-015-AC-1..FR-015-AC-4, NFR-002-AC-3 | ✅ Passed |
| TC-029 | Refuse malformed or unsupported shared references | Property | P0 | FR-016-AC-1..FR-016-AC-6 | ✅ Implemented |
| TC-030 | Build a heterogeneous parser portfolio plan | Integration | P0 | FR-013-AC-2, FR-014-AC-1..FR-014-AC-2, FR-015-AC-3, FR-015-AC-5 | ✅ Passed |
| TC-031 | Execute the independent shared-reference consumer | Integration | P0 | FR-016-AC-1..FR-016-AC-7 | ❌ Fails with the pinned EA candidate's `serde_json/arbitrary_precision` regression |
| TC-032 | Apply the earliest evidence freshness bound | Unit | P0 | FR-018-AC-1..FR-018-AC-5 | ✅ Passed |
| TC-033 | Retain correlated and unknown oracle dependence | Property | P0 | FR-017-AC-1..FR-017-AC-7 | ✅ Passed |
| TC-034 | Execute native parser baseline and adverse cases | Integration | P0 | FR-019-AC-1..FR-019-AC-2 | ✅ Passed |
| TC-035 | Retain parser portfolio gaps and stale evidence | Property | P0 | FR-019-AC-3..FR-019-AC-6 | ✅ Passed |
| TC-036 | Record reproducible effort observations | Benchmark | P0 | FR-020-AC-1..FR-020-AC-5, FR-020-AC-8..FR-020-AC-9 | ✅ Passed |
| TC-037 | Bind portfolio disposition and claim limits | Integration | P0 | FR-020-AC-6..FR-020-AC-7, FR-021-AC-1..FR-021-AC-5, NFR-003-AC-2 | ✅ Passed |
| TC-038 | Bound native parser process execution | Integration | P0 | NFR-006-AC-1..NFR-006-AC-8 | ✅ Complete |
| TC-039 | Freeze and reject qualification profiles | Property | P0 | FR-022-AC-1..FR-022-AC-5 | 🚧 In progress |
| TC-040 | Qualify corpus population and lineage | Property | P0 | FR-023-AC-1..FR-023-AC-4 | ✅ Passed |
| TC-041 | Classify oracle independence and disagreement | Integration | P0 | FR-024-AC-1..FR-024-AC-4 | 🚧 Focused tests pass; EA candidate blocked by TC-031 regression |
| TC-042 | Run generative, metamorphic, differential, and mutation evidence | Property | P0 | FR-025-AC-1..FR-025-AC-6, FR-025-CON-1..FR-025-CON-4, NFR-004-AC-1..NFR-004-AC-3 | 🚧 Planned |
| TC-043 | Execute and refuse the production chain | Integration | P0 | FR-026-AC-1..FR-026-AC-4 | 🚧 Planned |
| TC-044 | Measure and reproduce the exact production profile | Benchmark | P0 | NFR-007-AC-1..NFR-007-AC-4, NFR-008-AC-1..NFR-008-AC-2, NFR-008-AC-4 | 🚧 Planned |
| TC-045 | Seal and bind the qualification disposition | Integration | P0 | FR-027-AC-1..FR-027-AC-5, NFR-008-AC-3 | 🚧 Planned |
| TC-046 | Accept E02 cross-repository contract without result loss | Unit | P0 | FR-028-AC-1..FR-028-AC-3 | ✅ Passed |
| TC-047 | Model generation and domain-valid shrinking | Property | P0 | FR-029-AC-1..FR-029-AC-3, FR-029-CON-1..FR-029-CON-3, NFR-009 | ✅ Passed |
| TC-048 | Relational and finite-trace hyperproperty analysis | Property | P0 | FR-030-AC-1..FR-030-AC-3, FR-030-CON-1..FR-030-CON-3, NFR-009 | ✅ Passed |
| TC-049 | Linked method plan execution and accounting | Property | P0 | FR-031-AC-1..FR-031-AC-3, FR-031-CON-1..FR-031-CON-2, NFR-009 | ✅ Passed |
| TC-050 | Semantic evidence graph integrity | Property | P0 | FR-032-AC-1..FR-032-AC-3, FR-032-CON-1..FR-032-CON-2 | 🚧 Focused traced test; review pending |
| TC-051 | Complete-V1 176-row qualification ledger | Property | P0 | FR-034-AC-1..FR-034-AC-3, FR-034-CON-1..FR-034-CON-3, NFR-009 | 🚧 Focused traced test; review pending |
| TC-052 | Immutable counterexample lineage and replay | Property | P0 | FR-033-AC-1..FR-033-AC-3 | 🚧 Focused traced test; review pending |

## Option Permutation Matrix

| Test Case | Dimension A | Dimension B | Dimension C | Expected Behavior |
|-----------|-------------|-------------|-------------|-------------------|
| TC-009 | applicable | ready | qualified native capability | invocation allowed |
| TC-009 | applicable | ready | qualified manual capability | native invocation refused |
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
| TC-033 | transitive oracle edge | observation edge | three contributions | one connected correlation finding with sorted members and bases |
| TC-033 | valid derived source | source outside exact matches | derived relation | source-not-assessed finding; no independence claim |
| TC-033 | two exact candidates | one obligation | ambiguous selection | neither candidate enters the dependence graph |
| TC-033 | one selected result | two exact obligations | same contribution member | member retained once in top-level findings |
| TC-034 | valid source | direct parse | exact native candidate | completed and satisfied |
| TC-034 | seeded syntax mutation | expected located refusal | exact native candidate | completed and violated |
| TC-034 | valid source | unreadable native response | exact native candidate | failed and not-evaluated |
| TC-035 | exact result | shared observation | distinct techniques | correlation retained; no independence claim |
| TC-037 | exact report | no disposition | otherwise complete execution | incomplete until human disposition |
| TC-037 | exact sealed core | one foreign disposition | otherwise complete execution | stable refusal; no disposition adopted |
| TC-038 | exact executable | shell-like argument characters | separate argument vector | child observes unchanged single argument |
| TC-039 | complete profile | one unaccepted producer revision | all other dimensions exact | blocked prerequisite; no local substitute |
| TC-040 | required corpus class | zero retained inputs | explicit population denominator | unresolved class remains visible |
| TC-041 | candidate and oracle agree | shared implementation ancestry | declared independent | dependence mismatch blocks acceptance |
| TC-042 | fuzz budget starts | stopping rule not reached | no counterexample observed | incomplete rather than passed |
| TC-043 | exact candidate | test-only caller | real handoff omitted | production-chain qualification refused |
| TC-044 | passing aggregate | one phase above threshold | complete raw observations | phase violation remains visible |
| TC-045 | exact core | one claim-expanding disposition | valid reviewer identity | disposition refused; core unchanged |
| TC-047 | satisfiable checked domain | valid smaller candidate | same native failing disposition | shrink retained |
| TC-047 | unsatisfiable checked domain | any seed | no executable cases | domain refusal; no pass |
| TC-048 | same closed trace population | universal-existential prefix | existential-universal prefix | authored order preserved; results may differ |
| TC-048 | equal local names | distinct trace identities | no explicit correlation | incomplete/refused; no inferred match |
| TC-049 | failed required node | dependent child | independent sibling | child blocked; sibling executes and remains unchanged |
| TC-051 | all development vectors accounted | future production evidence absent | no final decision | local development passes; future states remain planned/unexecuted |

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
| Oracle contributions | Min for evaluated result | 1 contribution | TC-033 | Accept after strict role validation |
| Oracle contributions | Under min for evaluated result | 0 contributions | TC-033 | Schema refusal before assessment |
| Contribution identity | Exact | 1 occurrence within a result | TC-033 | Accept |
| Contribution identity | Duplicate | 2 equal IDs within a result | TC-033 | `duplicate_oracle_contribution_id` refusal |
| Derived lineage | Cycle | Two contributions derive from each other | TC-033 | `invalid_oracle_lineage` refusal |
| Selected graph membership | Above max | 2 exact candidates for one obligation | TC-033 | Ambiguous candidates excluded from the graph |
| Selected member identity | Duplicate path | 1 result selected by 2 obligations | TC-033 | One structured `(resultId, contributionId)` member |
| Measurement repetitions | Exact | 5 repetitions | TC-036 | Retain every phase observation |
| Measurement repetitions | Under prescribed | 4 repetitions | TC-036 | Report remains incomplete |
| Measurement series | Exact | 1 setup plus 5 repetitions of 3 automated phases | TC-036 | 16 unique ordered observations |
| Elapsed duration | Min | 0 integer nanoseconds | TC-036 | Retain as a valid observed boundary |
| Elapsed duration | Under min | -1 nanosecond | TC-036 | Retain invalid reason; exclude from summary |
| Elapsed duration | Max | 9007199254740991 integer nanoseconds | TC-036 | Retain as a valid observed boundary |
| Elapsed duration | Outside | 9007199254740992 nanoseconds | TC-036 | Retain invalid reason; exclude from summary |
| Elapsed duration | Wrong type | 0.5 nanoseconds | TC-036 | Retain invalid reason; exclude from summary |
| Manual dispositions | Exact | 1 report-bound value | TC-037 | Accept, revise, or reject is representable |
| Manual dispositions | Above max | 2 values | TC-037 | Refuse ambiguous disposition |
| Native invocation deadline | Min | Smallest positive represented duration | TC-038 | Accept declared deadline |
| Native invocation deadline | Max | 10 seconds | TC-038 | Accept declared deadline; terminate at that deadline |
| Native invocation deadline | Under min | 0 seconds | TC-038 | Refuse before launch |
| Native invocation deadline | Outside | More than 10 seconds | TC-038 | Refuse before launch |
| Captured standard output | Exact | 16 MiB | TC-038 | Accept stream |
| Captured standard output | Outside | 16 MiB plus one byte | TC-038 | Terminate and reap with `output_limit_exceeded` |
| Captured standard error | Exact | 16 MiB | TC-038 | Accept stream |
| Captured standard error | Outside | 16 MiB plus one byte | TC-038 | Terminate and reap with `output_limit_exceeded` |
| Admitted executable identity | Exact | Pathname replaced after descriptor admission | TC-038 | Execute the admitted descriptor; replacement cannot substitute bytes |
| Native process group | Outside | Direct child exits while one descendant holds both pipes | TC-038 | Terminate group, reap direct child, and return no live member |
| Qualification profile dimensions | Exact | Every required dimension present | TC-039 | Emit one immutable profile |
| Qualification profile dimensions | Under minimum | One required dimension absent | TC-039 | Refuse profile emission |
| Required corpus-class coverage | Exact | At least one qualified input or one explicit gap per class | TC-040 | Preserve complete denominator |
| Model scalar domain | Lower boundary | Inclusive minimum and representable just-outside value | TC-047 | Generate minimum; retain just-outside refusal partition |
| Model scalar domain | Upper boundary | Inclusive maximum and representable just-outside value | TC-047 | Generate maximum; retain just-outside refusal partition |
| Shrink order | Exact | Strictly smaller domain-valid candidate | TC-047 | Replay and retain only on same failing disposition |
| Trace alignment | Under minimum | One read position without exactly one match | TC-048 | Incomplete; no comparison by index or name |
| Plan node accounting | Exact | One terminal disposition per node | TC-049 | Pass |
| Plan node accounting | Under/above | Zero or two dispositions for one node | TC-049 | Refuse accounting |
| Qualification primary rows | Exact | 176 unique accepted capability IDs | TC-051 | Canonical ledger |
| Qualification primary rows | Under/above | 175 rows or one duplicate among 176 entries | TC-051 | Incomplete/refused ledger |
| Required oracle coverage | Under minimum | No oracle and no explicit no-oracle gap | TC-041 | Refuse evidence plan |
| Mutation discrimination | Exact | One named guard removal per critical claim | TC-042 | Named traced test fails for intended reason |
| Local Cargo concurrency | Max | 2 jobs | TC-044 | Accept bounded run |
| Local Cargo concurrency | Outside | More than 2 jobs | TC-044 | Refuse qualification run |
| Qualification dispositions | Exact | 1 bound value | TC-045 | Finalize accept, revise, or reject |
| Qualification dispositions | Above max | 2 values | TC-045 | Refuse ambiguity |

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
| ERR-019 | Oracle contribution ID is duplicated or a required/conditional field is absent | TC-033 | Schema or `duplicate_oracle_contribution_id` refusal before assessment |
| ERR-020 | Native parser process or result protocol fails unexpectedly | TC-034 | Failed execution plus not-evaluated logical outcome |
| ERR-021 | Required portfolio runner, binding, tool, or resource is absent | TC-035 | Blocked task, actionable setup, and no fabricated result |
| ERR-022 | Measurement sample is interrupted, negative, or bound to another environment | TC-036 | Retained invalid sample with reason; excluded from summaries |
| ERR-023 | Manual disposition is absent, duplicated, unsupported, or bound to another report | TC-037 | Incomplete or stable refusal; never accepted implicitly |
| ERR-024 | Native process exceeds time or output bounds | TC-038 | Child terminated and reaped; typed failed result with no logical claim |
| ERR-025 | Executable path or native response violates the selected process boundary | TC-038 | Refusal before launch or typed failed result after launch |
| ERR-026 | Measurement observation identity, phase/repetition cardinality, or order is invalid | TC-036 | `invalid_measurement_series` refusal |
| ERR-027 | A derived source is absent, self-referential, or cyclic | TC-033 | `invalid_oracle_lineage` refusal before assessment |
| ERR-028 | An oracle/observation envelope lacks an exact role-matched strict-reader output | TC-033 | Existing shared-reference refusal before result matching |
| ERR-029 | Qualification profile omits or opens a required claim dimension | TC-039 | Refuse profile emission |
| ERR-030 | Corpus bytes, lineage, rights, class, or selection rationale is absent or contradictory | TC-040 | Refuse corpus qualification or retain explicit class gap |
| ERR-031 | Required oracle is absent, correlated as independent, changed, or disagrees | TC-041 | Retain no-oracle/dependence/disagreement gap |
| ERR-032 | Generative budget is incomplete or a mutant survives | TC-042 | Retain incomplete or violated obligation with artifacts |
| ERR-033 | Production chain uses a test substitute or an unaccepted external revision | TC-043 | Refuse qualification at the named owner seam |
| ERR-034 | Performance threshold is missing or exceeded | TC-044 | Keep performance gate incomplete or violated |
| ERR-035 | Reproduction cannot resolve exact bytes or explain a difference | TC-044 | Keep affected evidence unresolved |
| ERR-036 | Qualification disposition is missing, foreign, duplicated, stale, or claim-expanding | TC-045 | Incomplete or stable refusal; core unchanged |
| ERR-037 | Checked generation domain is unsatisfiable or a precondition would require rejection sampling | TC-047 | Domain refusal; no cases and no semantic pass |
| ERR-038 | Shrink leaves the domain, is not strictly smaller, changes the failing clause/disposition, or fails exact replay | TC-047, TC-052 | Reject child revision and retain original counterexample |
| ERR-039 | Trace population is open, alignment is absent/ambiguous, bound is exhausted, or correlation relies on a local name | TC-048 | Incomplete/requires-bound or refusal; no semantic verdict |
| ERR-040 | Plan reference/provider is missing, method is unsupported, prerequisite fails, or dependency cycle exists | TC-049 | One typed node disposition; only descendants block and siblings remain |
| ERR-041 | Evidence identity kind, subject revision, relation, digest, or discharge path is invalid or circular | TC-050 | Refuse affected discharge; preserve graph and sibling evidence |
| ERR-042 | Qualification inventory row/vector is missing/duplicated, ecosystem pin changes, or evidence is self-produced | TC-051 | Refuse or leave affected future claim incomplete; development continues |
| ERR-043 | Counterexample transformation loses replay material, correspondence, domain validity, or failing disposition | TC-052 | Preserve immutable revision with limitation or reject child; never overwrite parent |

## State Transition Coverage

| State Model | Transition | Test Case | Expected |
|-------------|------------|-----------|----------|
| Plan readiness | prepared prerequisites → ready | TC-009 | native-invocable only when applicable with a qualified native capability and in-limit budget |
| Plan readiness | missing prerequisites → blocked | TC-010 | setup tasks emitted |
| Plan readiness | missing capability → unsupported | TC-010 | unsupported gap emitted |
| Evidence freshness | within age → beyond age | TC-014 | fresh status becomes stale |
| Combined freshness | at earliest expiry → after earliest expiry | TC-032 | fresh status becomes stale |
| Oracle dependence | independent → shared or derived | TC-033 | correlation finding appears without aggregate score |
| Oracle dependence | exact-matched → source outside exact graph | TC-033 | derived member becomes explicitly source-not-assessed |
| Portfolio execution | check returns false | TC-019 | completed plus violated |
| Portfolio execution | check throws | TC-019 | failed plus not-evaluated |
| Human review | no disposition | TC-020 | not-run and incomplete |
| Required task | allocation present → absent | TC-026 | ready task becomes blocked and remains visible |
| Native runner | ready → prerequisite missing | TC-027 | runner request becomes null |
| Domain portfolio | expected syntax acceptance → expected syntax refusal | TC-034 | completed logical outcome changes from satisfied to violated |
| Domain portfolio | required native prerequisite present → absent | TC-035 | affected work becomes blocked; no result appears |
| Domain evidence | at earliest expiry → one instant after | TC-035 | fresh result becomes stale with findings retained |
| VP04 disposition | absent → one exact owner value | TC-037 | report becomes disposition-complete without changing automated results |
| VP04 disposition | one exact owner value → foreign or duplicate | TC-037 | finalization refuses without changing the sealed core |
| Native process | running → time/output limit | TC-038 | child is terminated and reaped; result becomes failed |
| Qualification profile | draft dimensions → frozen identity | TC-039 | evidence collection may begin only for exact profile |
| Oracle status | independent agreement → disagreement or shared ancestry | TC-041 | affected claim becomes unresolved |
| Production chain | all exact seams → one changed or missing seam | TC-043 | chain becomes blocked or failed at named boundary |
| Qualification report | incomplete core → one exact disposition | TC-045 | final decision binds the same bounded claim |
| Method-plan node | ready → unsupported/failed | TC-049 | exactly one terminal disposition; independent siblings unchanged |
| Counterexample | original → transformed → replayed | TC-052 | immutable ordered lineage with claim-relative equivalence |
| Qualification vector | planned → unexecuted → passed/failed/unsupported/waived | TC-051 | each transition remains explicit and never gates development execution |

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
| EC-028 | Correlation edges connect transitively across different bases | FR-017 | TC-033 | One dependence component can be split into false independent confirmations |
| EC-029 | Derived contribution lineage is self-referential or cyclic | FR-017 | TC-033 | Circular evidence can be presented as corroboration |
| EC-030 | Contribution or result input order changes | FR-017 | TC-033 | Equivalent evidence can produce a different assessment identity |
| EC-031 | An ambiguous exact candidate shares an oracle with a selected result | FR-017 | TC-033 | Unselected evidence can alter top-level dependence findings |
| EC-032 | One exact result satisfies two inventoried obligations | FR-017 | TC-033 | One contribution can be double-counted as two dependence members |
| EC-015 | Equivalent timestamps use different RFC 3339 offsets | FR-018 | TC-032 | Text comparison can disagree about one instant |
| EC-016 | Maximum age addition exceeds the timestamp domain | FR-018 | TC-032 | Arithmetic can wrap, panic, or extend evidence validity |
| EC-017 | An expected parser refusal is represented as process failure | FR-019 | TC-034 | A found syntax defect can be erased as infrastructure noise |
| EC-018 | Two parser techniques reuse one observation | FR-019 | TC-035 | Method count can be mistaken for independent corroboration |
| EC-019 | Setup duration is combined with incremental run duration | FR-020 | TC-036 | Integration cost cannot be reproduced or compared honestly |
| EC-020 | An invalid timing observation is rewritten as zero | FR-020 | TC-036 | Summaries become artificially favorable |
| EC-021 | A disposition from another report is copied onto the candidate | FR-021 | TC-037 | Human acceptance can be falsely attributed |
| EC-022 | A disposition is included in the bytes whose digest it references | FR-021 | TC-037 | Report identity becomes circular or implementation-defined |
| EC-023 | Shell text or inherited environment changes native invocation | NFR-006 | TC-038 | Reviewed arguments or execution context can be replaced silently |
| EC-024 | A native child fills one pipe while the runner waits on another | NFR-006 | TC-038 | The qualified run can deadlock despite a nominal timeout |
| EC-025 | Candidate pathname changes after validation but before execution | NFR-006 | TC-038 | Different executable bytes can replace the reviewed candidate |
| EC-026 | A direct child exits while a descendant retains the output pipes | NFR-006 | TC-038 | The runner can block beyond its deadline or leave a live orphan |
| EC-027 | A multiline `#[trace(...)]` compiles while the configured Quire reader sees only its first line | NFR-004 | TC-022 | Claimed acceptance criteria remain unbacked despite a green Rust test |
| EC-033 | A large convenient corpus omits a declared production class | FR-023 | TC-040 | Sample count can masquerade as population coverage |
| EC-034 | Candidate and oracle share the same parser or generator | FR-024 | TC-041 | Correlated agreement can masquerade as independent confirmation |
| EC-039 | One artifact is retained under authorizing rights and also cited as a reference-only or unauthorized gap candidate | FR-023 | TC-040 | Contradictory rights decisions for one immutable identity can enter one corpus manifest |
| EC-035 | An unfinished fuzz run finds no counterexample | FR-025 | TC-042 | Budget exhaustion can be reported as a pass |
| EC-036 | A test adapter bypasses the real production caller or assurance handoff | FR-026 | TC-043 | Component success can masquerade as production-chain qualification |
| EC-037 | A favorable aggregate hides one hot-path phase over budget | NFR-007 | TC-044 | Production latency breach can disappear in the summary |
| EC-038 | A report is tampered and internally resealed while retained source bytes change | NFR-008 | TC-045 | Self-consistency can masquerade as source-bound integrity |
| EC-040 | Empty/singleton domains omit just-outside refusal partitions | FR-029 | TC-047 | Generator coverage can look complete while boundary behavior is absent |
| EC-041 | A smaller counterexample violates a cross-field relationship | FR-029, FR-033 | TC-047, TC-052 | Shrinking can leave the checked semantic domain |
| EC-042 | Reversing a mixed quantifier prefix is normalized away | FR-030 | TC-048 | A different hyperproperty can be evaluated silently |
| EC-043 | Two equal-valued traces or local names are treated as one identity | FR-030 | TC-048 | A quantifier can be satisfied by duplicating/correlating the wrong execution |
| EC-044 | Failed prerequisite erases a successful sibling branch | FR-031 | TC-049 | Useful independent evidence disappears from accounting |
| EC-045 | Evidence cycle or producer self-test discharges its own claim | FR-032, FR-034 | TC-050, TC-051 | Circular evidence can masquerade as independent qualification |
| EC-046 | Redaction removes a replay field but retains the old counterexample claim | FR-032, FR-033 | TC-050, TC-052 | Non-replayable evidence can masquerade as qualified replay |
| EC-047 | Missing future production evidence is represented as a failed development gate | FR-034 | TC-051, IT-005 | Production ceremony can unnecessarily stop provider implementation and local integration |

## Integration Test Matrix

### Cross-Project Integrations

| Integration ID | Purpose | Target Project | Type | Test Cases | Status |
|----------------|---------|----------------|------|------------|--------|
| IT-001 | Compare retained catalog with real producer output | agent-ix/quoin plus spec-artifacts-process | service | TC-021 | ✅ |
| IT-002 | Qualify the shared-reference packet with an independent Rust reader | agent-ix/quire-specification FS02/FS05 | contract | TC-031 | ✅ Implemented against `8ab058be` |
| IT-003 | Execute the bounded native parser portfolio and retain measured evidence | agent-ix/quire-spec-language plus Quoin evidence handoff | command/process | TC-034..TC-038 | ✅ Complete — bounded owner disposition retained |
| IT-004 | Qualify one exact production profile end to end | Exact caller, quire-spec, portable contracts, Engineering Assurance, and Quoin | command/process/contracts | TC-039..TC-045 | 🚧 Planned; external dependencies gated |
| IT-005 | Integrate complete-V1 method results through evidence and ledger development boundaries | QSpec checked-package/runtime, quire-analyze, accepted inventory/corpus, and Quoin intake contracts | Rust contracts/providers | TC-047..TC-052 | 🚧 SC-01..SC-03 passed; SC-04..SC-07 pending; production evidence is non-blocking state |

### Integration Test Details

| Test Case | Integration | Scenario | Input | Expected | Priority |
|-----------|-------------|----------|-------|----------|----------|
| TC-021 | IT-001 | Pinned exact-source happy path | Real module root and pinned revisions | 33 methods, 0 duplicates, 0 unreadable roots | P0 |
| TC-021 | IT-001 | Producer or fixture drift | Any missing, extra, or changed method field | Nonzero comparison exit | P0 |
| TC-021 | IT-001 | Unreadable or conflicting module | Real loader diagnostic | Nonzero comparison exit | P0 |
| TC-031 | IT-002 | Exact supported shared references | Pinned schema, artifacts, and positive vectors | Every B-used role accepted | P0 |
| TC-031 | IT-002 | One-axis reference mutations | Pinned adverse vectors and B mutations | Stable refusal before planning | P0 |
| TC-031 | IT-002 | Unmapped JCS or trace/fault/manual role | Consumer role composition | No permissive fallback; integration fails | P0 |
| TC-034 | IT-003 | Valid exact source | Real parse and parse-format-reparse commands | Two completed satisfied results | P0 |
| TC-034 | IT-003 | One declared syntax mutation | Real parse command and expected diagnostic | Completed violated result; unexpected failure stays failed | P0 |
| TC-035 | IT-003 | Missing prerequisite and expired exact result | Mutated plan and assessment time | Blocked work and stale finding remain explicit | P0 |
| TC-036 | IT-003 | Five prescribed measured repetitions | Raw monotonic phase observations | Setup separated; summaries recompute exactly | P0 |
| TC-037 | IT-003 | Exact report and review procedure | Verification-owner disposition | One bound value and bounded claims | P0 |
| TC-038 | IT-003 | Controlled time, output, argument, environment, path, status, and protocol cases | Native process adapter | Bounded typed results and no orphan process | P0 |
| TC-039 | IT-004 | Complete and one-axis-mutated qualification profiles | Owner-authored exact boundary | One immutable profile or explicit refusal/prerequisite | P0 |
| TC-040 | IT-004 | Representative, boundary, malformed, generated, and mutated corpus | Exact bytes, classes, provenance, rights, and lineage | Complete denominator or explicit gaps | P0 |
| TC-041 | IT-004 | Independent, shared, derived, absent, and disagreeing oracles | Exact bounded oracle executions | Correct dependence and disagreement findings | P0 |
| TC-042 | IT-004 | Example, property, fuzz, metamorphic, differential, and mutation techniques | Exact seeds, budgets, guards, and stopping rules | Discriminating results and replayable counterexamples | P0 |
| TC-043 | IT-004 | Real chain and one-seam substitutions | Exact caller, candidate, result, and assurance interfaces | Exact handoff or named blocked/failed seam | P0 |
| TC-044 | IT-004 | Performance and independent reproduction | Raw observations, thresholds, and retained identities | Per-phase decision and reproducible evidence | P0 |
| TC-045 | IT-004 | Complete, incomplete, tampered, and disposition variants | Sealed core and external owner decision | Bounded final report or stable refusal | P0 |
| TC-047 | IT-005 | Domain partitions, unsatisfiable models, shrinking, and replay | Checked model domains and native subject | Deterministic cases or exact refusal; replayable minimal failure | P0 |
| IT-005-SC-01 | IT-005 | Execute model generation, shrinking, and exact native replay | Checked model domains and native subject | Every admitted case and retained shrink satisfies FR-029; rejected preconditions never pass | P0 |
| IT-005-SC-02 | IT-005 | Evaluate relational traces with ordered quantifiers and exact alignment | Closed trace populations and typed relational expressions | Exact paired witness or typed non-conclusion; local names never correlate traces | P0 |
| IT-005-SC-03 | IT-005 | Dispatch a linked plan with unavailable, failed, and cyclic variants | Linked nodes, provider manifests, and dependency edges | Every node has one disposition and independent siblings remain unchanged | P0 |
| TC-048 | IT-005 | Ordered quantifiers, closure, alignment, and correlation variants | Closed trace populations and relational expressions | Exact paired/trace-set result or typed non-conclusion | P0 |
| TC-049 | IT-005 | Acyclic, unsupported, failed-prerequisite, sibling, and cyclic plan variants | Linked plan and provider manifests | One disposition per node with isolated failure | P0 |
| TC-050 | IT-005 | Valid, stale, cross-kind, circular, redacted, and unavailable evidence | Typed subjects, results, artifacts, and relations | Exact graph or scoped refusal/limitation | P0 |
| TC-051 | IT-005 | Exact 176 rows plus row/vector/pin/evidence/decision mutations | Accepted inventory, corpus, lock, and results | Canonical ledger; future gaps explicit and development unblocked | P0 |
| TC-052 | IT-005 | Original, transformed, redacted, and changed-verdict lineages | Exact counterexamples, operations, correspondences, and replay | Immutable valid lineage or retained limitation/refusal | P0 |

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
| Unit | 18 | 18 | 0 | 0 | 100% |
| Integration | 11 | 8 | 1 | 2 | 73% |
| Property | 19 | 13 | 0 | 6 | 68% |
| Static | 2 | 2 | 0 | 0 | 100% |
| Benchmark | 2 | 1 | 0 | 1 | 50% |
| Overall | 52 | 42 | 1 | 9 | 81% |
