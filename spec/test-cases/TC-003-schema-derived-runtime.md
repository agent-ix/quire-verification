---
id: TC-003
title: "Inspect and test schema-derived runtime definitions"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-002"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-002"
    type: "verifies"
---
# TC-003: Inspect and test schema-derived runtime definitions

## Description

Verify public validators and exported vocabularies originate from the versioned
JSON Schema packet.

## Test Procedure

Inspect `src/contracts.rs` for embedded schema loading and runtime compilation, then run the
runtime-constants test against the corresponding schema definitions.

## Expected Results

There is one authored public-contract definition set, and every tested runtime
constant equals the authoritative schema value.
