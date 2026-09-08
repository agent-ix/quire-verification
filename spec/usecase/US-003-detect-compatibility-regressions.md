---
id: US-003
title: "Detect compatibility regressions locally"
type: US
relationships:
  - target: "ix://agent-ix/quire-verification/StR-001"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-003"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-004"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-009"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-010"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-011"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-012"
    type: "traces_to"
---
# US-003: Detect compatibility regressions locally

## Story

**As a** compatibility maintainer
**I want** repeatable checks against the pinned Quoin catalog and contract fixtures
**So that** field drift, order dependence, schema divergence, and local execution failures are reviewable.

## Context

The compatibility slice must preserve Quoin's authored method identity while
remaining explicit about unreadable or conflicting sources. A portfolio that
throws on its own adverse result cannot report the failure it was meant to find.

## Acceptance Examples (Illustrative)

### US-003-EX-1: Reordered catalog retains parity

- **Given** the same unique methods in a different array order
- **When** catalog parity runs
- **Then** every method is compared by technique identifier and the check succeeds

### US-003-EX-2: Adapter failure remains reviewable

- **Given** an adapter that raises an execution error
- **When** the bounded portfolio runs
- **Then** the report contains failed technique results and remains available

## Priority and Risk (Informative)

Priority is high because a pre-sorted fixture can hide an index-based comparison
bug and a thrown check can erase the evidence needed to diagnose it.

## Traceability (Informative)

The story drives fixture execution, catalog adaptation, and bounded portfolio
failure-retention and qualified selection requirements.
