---
id: TC-023
title: "Enforce the JSON resource envelope"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/NFR-005"
    type: "verifies"
---
# TC-023: Enforce the JSON resource envelope

## Description

Verify qualified-path JSON ingestion rejects documents, nesting, and arrays
beyond their stated limits without panicking.

## Test Procedure

Pass valid JSON at each exact limit and invalid JSON immediately beyond each
limit through the corresponding public Rust boundary.

## Expected Results

At-limit inputs reach normal validation, over-limit inputs return stable resource
limit errors, and no case panics or performs an unbounded recursive walk. This
procedure remains pending until the Rust implementation exists.

