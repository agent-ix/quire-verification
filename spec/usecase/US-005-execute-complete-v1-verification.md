---
id: US-005
title: "Execute complete-V1 verification methods"
type: US
relationships:
  - target: "ix://agent-ix/quire-verification/StR-001"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/StR-002"
    type: "traces_to"
---
# US-005: Execute complete-V1 verification methods

## Story

**As a** verification owner
**I want** independently implemented methods to produce replayable, typed results over exact complete-V1 subjects
**So that** I can distinguish semantic evidence from method failure and make an attributable, scope-limited qualification decision.

## Context

The accepted complete-V1 architecture adds model-based generation, relational
and finite-trace hyperproperties, linked method plans, continuous/hybrid
analysis, bounded synthesis, and an inventory-derived qualification ledger. The
existing bounded compatibility and production-qualification slices remain
inputs; they are not reinterpreted as evidence for capabilities they did not
execute.

## Acceptance Examples (Illustrative)

### US-005-EX-1: Independent methods remain attributable

- **Given** an exact checked subject and an acyclic plan selecting multiple providers
- **When** one provider refutes its obligation and another fails operationally
- **Then** both terminal dispositions remain distinct and replayable

### US-005-EX-2: Incomplete qualification stays incomplete

- **Given** a ledger with one unexecuted required vector among 176 primary rows
- **When** the owner evaluates complete-V1 qualification completeness
- **Then** the affected claim remains incomplete until evidence and an attributed decision exist

## Constraints (Contextual)

The accepted QSpec owns shared semantics and I14/I15/I18-I21.
This repository adopts those boundaries and reports contradictions upstream
instead of changing a shared interface unilaterally.

## Dependencies (Contextual)

Checked packages and semantic subjects come from the accepted native compiler
lane. Hybrid and synthesis method results come from `quire-analyze` issue #33.
Integrated execution and the final ecosystem campaign remain external consumers.

## Priority and Risk (Informative)

Priority is P0. Conflating unsupported, incomplete, failed, or heuristic output
with a semantic pass would create a false qualification claim. Missing production
ceremony is ledger state for later acceptance, not a development blocker.

## Traceability (Informative)

[FR-029](../functional/FR-029-generate-model-tests.md) through
[FR-034](../functional/FR-034-qualify-complete-v1-ledger.md) implement this story.
