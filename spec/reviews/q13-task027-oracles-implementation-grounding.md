---
id: SR-185
title: "Implementation-grounding review of Task-027 qualification oracles"
type: SpecReview
analysis: base
scope: "quire-verification@67632d6156ae8e9ca62f4339235016c08428560f; FR-024, TC-041, QualificationOracleManifest, qualification_oracle.rs"
review_set: all
---
# Implementation-grounding review of Task-027 qualification oracles

## Summary

The implementation-grounding pass found that the accepted review set did not make
expected-value origin or the asserted separation of candidate and oracle ancestry
mechanically inspectable. The requirement, schema, Rust validator, and discriminating
tests now bind those values explicitly. Result integration remains open until Engineering
Assurance publishes the accepted Rust producer-execution interface.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Resolved: `expectedRelation.origin` and separate nonempty candidate/oracle ancestry references now prevent a candidate-produced expected value or intersecting declared ancestry from being accepted as independent. | FR-024-AC-2, TC-041 step 3 |
| FND-002 | medium | Open external dependency: FR-024-AC-3 and the execution-result dimensions of FR-024-AC-4 cannot be implemented until Engineering Assurance issue #11 publishes an accepted versioned Rust producer-execution interface. | FR-024-AC-3..4, Task-027 |
| FND-003 | low | Static inspection and the module boundary show no Task-027 process runner, evidence store, fixture script, stdout parser, sufficiency decision, or human disposition in quire-verification. | FR-024-CON-1..2, TC-041 |
