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
derived contribution with an exact source contribution, an unknown relationship,
an unresolved singleton shared group, a valid derived source outside the
exact-matched graph, and a non-evaluated result with an empty contribution
array. Supply every accepted oracle and observation envelope through the strict
reader. Repeat with every required-field deletion, an extra field, a missing
relation, wrong relation-conditional fields, duplicate contribution ID, absent
derived source, self-reference, and a two-node derived cycle. Remove each opaque
validated reference, substitute each wrong role, and mutate one reference
dimension at a time. Permute result and contribution order, join correlation
edges transitively, and inspect the complete assessment envelope.

## Expected Results

Every evaluated result has at least one closed role-valid contribution; a
non-evaluated result may use an empty array. Schema errors, strict-reference
mismatches, duplicate IDs, and invalid derived lineage refuse with their stable
codes before classification. Shared and derived edges emit one deterministic
finding per connected component with exact sorted members and bases; declared
unknown, unresolved singleton shared, and out-of-graph derived evidence remain
explicitly unknown. Reordering changes no JCS output. Selected contributions
remain separately inspectable, and the output contains no numeric score or
aggregate assurance or sufficiency decision.
