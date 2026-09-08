---
id: TC-033
title: "Retain correlated and unknown oracle dependence"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-017"
    type: verifies
---
# TC-033: Retain Correlated and Unknown Oracle Dependence

## Description

Verify that multiple technique results retain exact oracle lineage and declared
dependence without being averaged or presented as independent by identifier alone.

## Test Procedure

Assess a bounded inventory containing results with independent oracle and
observation references, a shared observation, a shared dependence group, a
derived contribution, an unknown relationship, and a non-evaluated result with
no contribution. Permute result and contribution order, mutate one reference
dimension at a time, and inspect the complete assessment envelope.

## Expected Results

Shared and derived evidence emits deterministic correlation findings, unknown
dependence remains unknown, exact-reference mutations do not join, non-evaluated
evidence may omit contributions, and the output contains neither a numeric score
nor an aggregate assurance or sufficiency decision.
