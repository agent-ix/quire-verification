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

Pin the accepted specification revision, load its shared schema and B-relevant
positive/adverse fixtures as data, and run the Rust reader against every case.
Confirm the dependency revision and fixture digests before evaluation.

## Expected Results

All selected positive cases pass, all adverse cases refuse in the expected
category, and B-specific JCS, trace, fault-model, review-procedure, property,
subject, binding, dependency-closure, environment, run, and native-output roles
are exercised independently of the producer.
