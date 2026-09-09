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
  - target: "ix://agent-ix/quire-verification/FR-011"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-012"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-013"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-014"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-015"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-016"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-019"
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

Native execution also needs exact versioned references, explicit permissions and
budgets, and role-specific inputs. A parser property check, a differential
comparison, and a manual review cannot share one guessed unary-predicate request.

## Acceptance Examples (Illustrative)

### US-001-EX-1: Ready task can be invoked

- **Given** an applicable property and a qualified compatible capability with all prerequisites
- **When** a plan is built
- **Then** its verification task is ready and invocation is allowed

### US-001-EX-2: Unsafe fault task remains blocked

- **Given** a fault property without non-production authorization or an injection binding
- **When** a plan is built
- **Then** prerequisite work is visible and the fault task cannot be invoked

### US-001-EX-3: Parser portfolio keeps distinct work

- **Given** property, differential, and manual parser obligations with different prerequisites
- **When** a plan is built
- **Then** each obligation retains its own input, setup work, and invocation eligibility

## Priority and Risk (Informative)

Priority is high because conflating applicability with readiness can create an
unsafe invocation path and conceal missing work.

## Traceability (Informative)

The story drives the public contract validation, schema-derived runtime,
prerequisite-aware planning, qualified selection, native runner, role-specific
input, shared-reference, and bounded parser-portfolio requirements.
