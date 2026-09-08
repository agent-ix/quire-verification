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

## Priority and Risk (Informative)

Priority is critical because false attribution can turn unrelated or incomplete
evidence into an unsupported assurance claim.

## Traceability (Informative)

The story drives exact matching, independent finding preservation, and canonical
structured comparison.
