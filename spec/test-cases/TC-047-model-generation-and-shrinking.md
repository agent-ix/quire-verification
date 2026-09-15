---
id: TC-047
title: "Model generation and domain-valid shrinking"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-029"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-009"
    type: "verifies"
---
# TC-047: Model generation and domain-valid shrinking

## Description

Verify deterministic complete boundary partitions, unsatisfiable-domain refusal,
well-founded shrinking, and exact native replay over checked model domains.

## Test Procedure

Generate cases for empty, singleton, bounded scalar, collection, and cross-field
domains using fixed seeds. Mutate preconditions and relationships to create an
unsatisfiable domain. Starting from a failing case, request valid, equal,
out-of-domain, and verdict-changing shrinks and replay each retained step against
the exact native subject.

## Expected Results

Every applicable endpoint, just-outside refusal, empty/singleton, and cross-field
partition occurs deterministically. Unsatisfiable domains refuse before execution.
Only strictly smaller domain-valid candidates reproducing the same failing clause
and disposition are retained; rejected preconditions never count as passes.

