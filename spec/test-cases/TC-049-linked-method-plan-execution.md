---
id: TC-049
title: "Linked method plan execution and accounting"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-031"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-009"
    type: "verifies"
---
# TC-049: Linked method plan execution and accounting

## Description

Verify deterministic dependency dispatch, per-node terminal accounting, sibling
isolation, cycle refusal, and authored aggregation without source-admission drift.

## Test Procedure

Execute an acyclic plan containing independent branches and all four edge kinds.
In separate variants remove a provider, select an unsupported method, fail one
prerequisite, introduce a cycle, make an aggregation self-referential, and
permute set-like inputs while retaining ordered edges.

## Expected Results

Ready nodes dispatch in stable dependency order and every node receives exactly
one terminal disposition. Failures block only typed descendants. Unsupported
methods and cycles do not erase siblings. Equivalent set permutations retain
identity; changed ordered dependencies, assumptions, methods, bounds, or
providers change identity. No plan or aggregation changes clause admission.

