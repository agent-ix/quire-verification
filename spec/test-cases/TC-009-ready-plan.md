---
id: TC-009
title: "Build ready tasks across distinct input kinds"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-005"
    type: "verifies"
---
# TC-009: Build ready tasks across distinct input kinds

## Description

Verify planning retains all five input shapes and authorizes only applicable,
ready work.

## Test Procedure

Build the retained portfolio plan and inspect its verification tasks, inventory,
readiness, applicability, and invocation flags.

## Expected Results

All five input kinds are present; qualified fully prepared selections are ready;
and only applicable ready tasks allow invocation.

