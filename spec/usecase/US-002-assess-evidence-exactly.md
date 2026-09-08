---
id: US-002
title: "Assess evidence without false attribution"
type: US
relationships:
  - target: "ix://agent-ix/quire-verification/StR-001"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-006"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-007"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-008"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-017"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-018"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-019"
    type: "traces_to"
  - target: "ix://agent-ix/quire-verification/FR-021"
    type: "traces_to"
---
# US-002: Assess evidence without false attribution

## Story

**As an** evidence assessor
**I want** results matched to obligations by their complete identity and execution scope
**So that** stale, foreign, ambiguous, or incomplete evidence cannot be mistaken for support.

## Context

A technique name or property label is insufficient to identify one execution.
The assessor needs every independent adverse dimension to remain visible even
when another dimension determines the summary status.

## Acceptance Examples (Illustrative)

### US-002-EX-1: Exact fresh result is assessed

- **Given** one result whose binding and scope equal an obligation in every required dimension
- **When** evidence is assessed before its freshness limit
- **Then** the result is classified from its independent execution and logical fields

### US-002-EX-2: Competing exact results are refused

- **Given** two results for the same exact obligation and execution binding
- **When** evidence is assessed
- **Then** the obligation is ambiguous and neither result is selected

### US-002-EX-3: The earliest freshness limit controls

- **Given** a result validity limit earlier than the obligation-derived expiry
- **When** evidence is assessed after the result validity limit
- **Then** the result is stale even though the obligation maximum age has not elapsed

### US-002-EX-4: Shared observations remain correlated

- **Given** two technique results derived from the same exact observation
- **When** their evidence is assessed together
- **Then** the results remain separate and their shared oracle dependence is visible

## Priority and Risk (Informative)

Priority is critical because false attribution can turn unrelated or incomplete
evidence into an unsupported assurance claim.

## Traceability (Informative)

The story drives exact matching, independent finding preservation, canonical
structured comparison, conservative freshness, and explicit oracle dependence
without aggregate scoring, including the bounded parser portfolio.
