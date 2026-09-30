---
id: TC-031
title: "Execute the independent shared-reference consumer"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-016"
    type: verifies
  - target: "ix://agent-ix/quire-verification/IT-002"
    type: verifies
---
# TC-031: Execute the Independent Shared-Reference Consumer

## Description

Verify the Rust E02 reader executes the exact selected Agent A schema and fixture
packet without using A's producer validation code.

## Test Procedure

Run the Rust reader — which consumes the schemas,
fixtures, and derived wire types through `contracts::shared_reference` — against
every case without importing or executing the producer test crate. Exercise the core-only JCS preimage, all six role payloads, structural and
semantic mutations, pairwise wrong-role substitutions, exact-reference
one-axis mutations, feature-set permutations, raw duplicate-key control, and
B-authored source-map/migration cases validated by the retained version-1 schema.

## Expected Results

All selected positive cases pass, every adverse case refuses in its specified
category before planning or result matching, and B-specific JCS, trace,
fault-model, review-procedure, oracle, observation, review-disposition,
property, subject, binding, dependency-closure, environment, run, and
native-output roles are exercised independently of the producer.
Version-1 source-map/migration and version-2 artifact/semantic records are
accepted only under their selected kind/version allocation.
