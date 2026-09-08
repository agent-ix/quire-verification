---
id: US-001
title: "Plan portable verification work"
type: US
relationships:
  - target: "ix://agent-ix/quire-verification/StR-001"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-002"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-005"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-010"
    type: "traces_to"
---
# US-001: Plan portable verification work

## Story

**As a** verification integrator
**I want** a portable plan that distinguishes verification work from unresolved prerequisites
**So that** I can invoke only work whose inputs, capabilities, and safety conditions are ready.

## Context

Verification subjects require different input forms and operational conditions.
Treating applicability, capability support, and runtime readiness as one state can
authorize unsafe work or hide the exact setup still required.

## Acceptance Examples (Illustrative)

### US-001-EX-1: Ready task can be invoked

- **Given** an applicable property and a qualified compatible capability with all prerequisites
- **When** a plan is built
- **Then** its verification task is ready and invocation is allowed

### US-001-EX-2: Unsafe fault task remains blocked

- **Given** a fault property without non-production authorization or an injection binding
- **When** a plan is built
- **Then** prerequisite work is visible and the fault task cannot be invoked

## Priority and Risk (Informative)

Priority is high because conflating applicability with readiness can create an
unsafe invocation path and conceal missing work.

## Traceability (Informative)

The story drives the public contract validation, schema-derived runtime, and
prerequisite-aware planning and qualified selection requirements.
