---
id: US-004
title: "Qualify one quire-spec production profile"
type: US
relationships:
  - target: "ix://agent-ix/quire-verification/StR-002"
    type: "traces_to"
---
# US-004: Qualify one quire-spec production profile

## Story

**As a** verification owner
**I want** one `quire-spec` production profile evaluated against representative,
adverse, independent, and measured evidence
**So that** I can accept, revise, or reject that exact profile without mistaking a
narrow successful fixture for general qualification.

## Context

The next decision
needs a declared production caller and population, broader tests, independent
oracles, resource measurements, and exact dependency identities.

## Acceptance Examples (Illustrative)

### US-004-EX-1: Bounded profile is accepted

- **Given** every declared input class and adverse case has fresh evidence
- **When** the owner reviews the sealed report and its claim limits
- **Then** the owner may accept only that exact profile

### US-004-EX-2: Missing evidence blocks acceptance

- **Given** a required input class, oracle disagreement, or performance budget is unresolved
- **When** the report is assembled
- **Then** it remains incomplete and identifies the exact gap

## Constraints (Contextual)

The work consumes versioned outputs from other agents and repositories. It does not
move their schemas, compiler logic, assurance orchestration, or evidence stores into
`quire-verification`.

## Dependencies (Contextual)

The production-profile decision depends on accepted revisions for every shared
identity and producer interface it actually uses. Broader FS02/FS05 work remains
Agent A-owned, and Engineering Assurance remains Agent C-owned.

## Priority and Risk (Informative)

The work is sequenced after the merged Rust groundwork. Starting corpus or adapter
implementation before the profile boundary is reviewed risks duplicated work and
cross-agent interface drift.

## Traceability (Informative)

FR-022 through FR-027 implement this story.

