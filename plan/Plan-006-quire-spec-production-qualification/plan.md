---
id: Plan-006
title: "Quire-spec bounded production qualification"
type: Plan
status: active
relationships:
  - { target: "ix://agent-ix/quire-verification/StR-002", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-022", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-023", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-024", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-025", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-026", type: references }
  - { target: "ix://agent-ix/quire-verification/FR-027", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-007", type: references }
  - { target: "ix://agent-ix/quire-verification/NFR-008", type: references }
---
# Implementation Plan: Quire-spec Bounded Production Qualification

## Requirements Summary

### Stakeholder Requirement

- [ ] **StR-002**: Admit one exact `quire-spec` production profile only through affirmative, bounded evidence and an external owner disposition.

### Functional Requirements

- [ ] **FR-022**: Freeze the exact candidate, caller, commands, population, guarantees, environment, budgets, evidence, exclusions, and producer revisions.
- [ ] **FR-023**: Retain a rights-cleared corpus with exact bytes, population classes, provenance, and reproducible lineage.
- [ ] **FR-024**: Bind exact independent, shared, derived, absent, and disagreeing oracle contributions.
- [ ] **FR-025**: Freeze one closed technique configuration or owned gap for every evidence obligation and produce exact discriminating results from accepted typed observations.
- [ ] **FR-026**: Execute the real production caller, candidate, portable result, and assurance handoff at accepted revisions.
- [ ] **FR-027**: Seal one reproducible evidence core and bind exactly one external owner disposition.

### Non-Functional Requirements

- [ ] **NFR-007**: Measure every owner-declared production phase against its own threshold with raw observations and bounded local load.
- [ ] **NFR-008**: Reproduce every identity and result from retained or authorized exact bytes.

## Dependency Graph

- `FR-022 -> FR-023, FR-024, FR-026`: the frozen profile defines the input population, oracle obligations, and exact production interfaces.
- `FR-023 -> FR-024, FR-025`: oracle and generative evidence operate over the closed, rights-cleared corpus.
- `FR-024 -> FR-025`: differential and agreement claims require classified oracle authority and dependence.
- `FR-025 -> FR-026`: the real-chain run consumes the accepted candidate and the completed component evidence.
- `FR-026 -> NFR-007, NFR-008`: performance and reproduction measure the exact assembled production path.
- `NFR-007 + NFR-008 -> FR-027`: the report cannot be dispositioned until measurements and independent reproduction are complete.
- `FR-027 -> StR-002`: the verification owner decides only after the same sealed, bounded claim is reviewable.

### External gates

Task-025 records exact accepted E02, FS02, FS05, parser/compiler, production-caller,
Engineering Assurance, and Quoin handoff revisions. A missing revision becomes a
named prerequisite inside that task; it never authorizes this repository to copy the
producer's schema or implementation. Agents A, C, D, and contract-agent-c retain their
existing ownership.

Merging this planning artifact does not unblock issue 13 execution. Plan-006 is tracked
as active planning, while Task-025 and all dependent implementation remain gated on the
owner-authored profile dimensions, phase thresholds, and accepted exact producer inputs.

The merged `quire-verification` revision
`4a5e1961d353db45aa227f8c6ef964d9f67eae29` closes reviewed portable checkpoints in
Tasks 027 through 029. It does not complete any Plan-006 task: execution observations,
the promoted differential method, native-profile admission, the selected production
binary/caller, production-chain evidence, measurements, reproduction, sealing, and
disposition remain open at their existing owners.

Quire Specification PR 21 is integrated into draft composed-baseline revision
`d7b4cdee12c259324061a2b15199d3d3c960eedf`. Its reviewed FR-062 projection keeps the
canonical protocol result distinct from E02, but that draft is not a promoted native
profile or executable admission artifact. Existing protocol candidate revision
`1-draft.3` predates the integrated FR-061/FR-062 bytes, so A must mint a new immutable
candidate. F-owned FR-115/TC-145 and the composed full review remain open before the
shared baseline can be accepted.

### Shared dependencies

The existing schema-derived Rust contracts, RFC 8785 JCS implementation, strict
shared-reference reader, bounded native-process executor, evidence assessor, and
sealed-report machinery remain single implementations. Plan-006 extends them under the
new requirements instead of creating qualification-only copies.

## Test Plan

- [x] **TC-039**: property-test complete profiles, omissions, open dimensions, changed identities, and unavailable producer revisions.
- [x] **TC-040**: property-test corpus population, exact bytes, provenance, rights, exclusions, generated lineage, and mutations.
- [ ] **TC-041**: exercise independent, shared, derived, absent, changed, and disagreeing oracles through the bounded process boundary.
- [ ] **TC-042**: execute and discriminate example, property, fuzz, metamorphic, differential, mutation, shrinking, budget, and replay behavior.
- [ ] **TC-043**: execute the real caller-to-candidate-to-result-to-assurance chain and refuse every one-seam substitution.
- [ ] **TC-044**: evaluate owner thresholds from retained raw phase observations and reproduce exact and environmentally different results.
- [ ] **TC-045**: recompute all source bindings, reject reseals and invalid dispositions, and bind one unchanged bounded claim.
- [ ] **IT-004-SC-01..07**: run the selected production profile from freeze through independent review and external disposition.

Every automated acceptance binding uses bare compiler-checked `ix-trace-rs` markers.
Fuzzing, mutation, benchmarking, and production-chain execution retain their exact
seeds, budgets, commands, stopping rules, and raw artifacts.

## Remaining Work

### Track A: Critical path, scheduled serially

- **A1 = Task-025** Freeze profile and interface gate — Hard; exit: one immutable profile names every accepted dependency or a typed blocker.
- **A2 = Task-026** Retain qualification corpus — Medium; exit: every declared class has exact rights-cleared bytes or an explicit gap.
- **A3 = Task-027** Bind oracle obligations — Hard; exit: every guarantee has classified oracle authority, dependence, and disagreement handling.
- **A4 = Task-028** Build property and differential evidence — Hard; exit: generated examples and metamorphic/differential relations replay deterministically.
- **A5 = Task-029** Add fuzz and mutation discrimination — Hard; exit: declared budgets complete and every critical seeded mutant is killed for the intended reason.
- **A6 = Task-030** Execute the real production chain — Hard; exit: the exact caller and assurance handoff run with no test-only substitution.
- **A7 = Task-031** Measure the production budget — Medium; exit: every claimed phase has raw observations and an owner-threshold decision.
- **A8 = Task-032** Reproduce artifacts and results — Hard; exit: an independent run resolves exact bytes and explains every permitted difference.
- **A9 = Task-033** Seal and disposition the report — Hard; exit: one external decision binds the unchanged evidence core and bounded claim.

### Track Gate: Independent acceptance

- **Gate = Task-034** Qualification review — Hard; pass: IT-004, exact Rust review, QUOIN gap analysis, and the external disposition leave no unresolved in-scope blocker.

Until the upstream TestMatrix status-header contract and coverage reader agree, every
task that changes a test status or adds a test case manually recounts the Test Execution
Summary by category and overall before review.

## Parallel Execution Summary

The logical graph contains opportunities to prepare corpus candidates and oracle
inventories after the profile is fixed. On the current shared desktop, execution is
intentionally serialized while four other agent lanes are active:

```text
Task-025 -> Task-026 -> Task-027 -> Task-028 -> Task-029 -> Task-030 -> Task-031 -> Task-032 -> Task-033 -> Task-034
```

Parallelism may be reconsidered after those lanes finish, but no task may share a
mutable worktree, evidence store, Cargo target directory, or generated artifact set.

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
|------|-------|-------------------|------------------------|--------|
| Task-025 | A | FR-022 | TC-039 | blocked |
| Task-026 | A | FR-023 | TC-040 | blocked |
| Task-027 | A | FR-024 | TC-041 | in_progress |
| Task-028 | A | FR-025 enablement | TC-042 portable checkpoint | blocked |
| Task-029 | A | FR-025 completion | TC-042 | blocked |
| Task-030 | A | FR-026 | TC-043 | not_started |
| Task-031 | A | NFR-007 evidence | TC-044 benchmark portion | not_started |
| Task-032 | A | NFR-008 | verifies TC-044 | not_started |
| Task-033 | A | FR-027 | TC-045 | not_started |
| Task-034 | Gate | StR-002, FR-022..FR-027, NFR-007..NFR-008 | verifies IT-004 and TC-039..TC-045 | not_started |

## Coordination Rules

This plan changes only `quire-verification`. It consumes accepted, exact artifacts
from the Agent A, C, D, and contract-agent-c lanes and reports incompatibility to the
owning lane; it does not edit their repositories or invent a substitute boundary.
Shared files and retained evidence remain single-writer.

Substantive behavior or interface changes use QUOIN `/specify` and `/spec-review`
before implementation. Review remediation updates the affected spec and repeats those
cycles only for a major design change. Rust changes receive the exact
`/home/peter/dev/agent-skills/rust-review/SKILL.md` review and final semantic QUOIN
`/gap-analysis`. Local Cargo work uses at most two jobs and serializes heavy phases.
Hosted workflows remain manual `workflow_dispatch`; this plan does not trigger CI.
