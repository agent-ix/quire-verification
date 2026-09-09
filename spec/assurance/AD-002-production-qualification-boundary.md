---
id: AD-002
title: Quire-spec production qualification boundary
type: ArchitectureDescription
status: proposed
owner: agent-ix-architecture-owner
system: quire-verification qualification campaign
relationships:
  - target: ix://agent-ix/quire-verification/StR-002
    type: realizes
  - target: ix://agent-ix/quire-verification/FR-022
    type: realizes
  - target: ix://agent-ix/quire-verification/FR-026
    type: realizes
  - target: ix://agent-ix/quire-verification/NFR-007
    type: references
  - target: ix://agent-ix/quire-verification/NFR-008
    type: references
---
# Quire-spec production qualification boundary

## System Boundary

`quire-verification` owns the qualification profile, corpus/oracle manifests,
portable plan and result semantics, bounded qualification runners, claim checks,
and report-core assembly. Agent A-owned repositories own shared identity/version
schemas and the parser/compiler. Agent C-owned Engineering Assurance owns shared
assurance-chain orchestration. Quoin owns catalog orchestration, evidence retention,
and authored sufficiency. The production caller remains with its current owner.

All interactions cross exact versioned structured interfaces. The qualification
repository consumes immutable producer artifacts and never copies their semantic
implementation as a shortcut. Filament producers and unrelated contract-agent work
remain out of scope.

## Views

The control flow is profile freeze → external-interface readiness → corpus
qualification → oracle binding → technique execution → real production-chain run →
performance/reproduction → sealed core → independent review → owner disposition.
Every stage emits portable references and typed gaps. Downstream stages do not start
when a required upstream revision or owner decision is unavailable.

The data view separates profile identity, corpus lineage, oracle independence,
technique execution, logical outcome, measurements, production-chain boundaries,
review findings, evidence ownership, and the human decision. JCS identities bind
closed JSON structures; retained byte digests reach through those structures into
candidate, dependency, corpus, oracle, and result artifacts.

## Decisions

1. The first task freezes a narrow production claim before corpus or adapter work.
2. FS02/FS05 and parser/compiler outputs are consumed at accepted exact revisions;
   unresolved broader producer work remains an explicit prerequisite.
3. Independent oracles are preferred where semantic overlap exists, while no-oracle
   domains remain visible gaps or require an owner-approved alternative method.
4. Example, property, fuzz, metamorphic, mutation, differential, performance, and
   integration evidence remain separate contributions rather than one score.
5. The real production caller and assurance handoff must execute; test-only
   substitution cannot qualify the integration.
6. All new first-party executable paths are Rust and all acceptance bindings use
   bare compiler-checked `ix-trace-rs` markers.
7. Local execution uses at most two Cargo jobs and serializes high-load phases.
   GitHub workflows stay `workflow_dispatch` only.
8. Qualification ends with a human disposition over a sealed, reproducible core;
   Quoin retains evidence and sufficiency ownership.

## Risks

- **Boundary volatility — owner: verification owner; review before Task-026.** Agent
  A/C interfaces may change while active. Pin only accepted revisions and keep
  dependent tasks blocked until compatibility cases pass.
- **Corpus bias — owner: corpus owner; review before technique execution.** A large
  convenient corpus may omit important production classes. Require a population and
  selection rationale with explicit empty classes.
- **Oracle correlation — owner: evidence owner; review before final assembly.** Tools
  may share parsers, fixtures, or generators. Record dependence and retain disagreement.
- **Shared-desktop pressure — owner: implementation owner; review per run.** Bounded
  local concurrency and serial high-load phases are mandatory.
- **Premature qualification — owner: verification owner; review at every gate.** The
  report remains incomplete until every required task, review, measure, and external
  decision is present.

