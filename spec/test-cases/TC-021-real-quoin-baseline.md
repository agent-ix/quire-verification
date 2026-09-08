---
id: TC-021
title: "Execute the real Quoin catalog baseline integration"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/IT-001"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-004"
    type: "verifies"
---
# TC-021: Execute the real Quoin catalog baseline integration

## Description

Verify the real pinned Quoin catalog loader still produces the retained catalog
baseline.

## Test Procedure

Execute every step and success criterion in
[IT-001](../integration/IT-001-real-quoin-catalog-parity.md) with the pinned real
Quoin and module checkouts.

## Expected Results

The loader and Rust baseline checker exit zero and report 33 methods, zero duplicate
definitions, and zero unreadable roots.
