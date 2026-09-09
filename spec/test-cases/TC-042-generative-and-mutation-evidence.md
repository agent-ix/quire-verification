---
id: TC-042
title: "Run generative, metamorphic, differential, and mutation evidence"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-025"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-004"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-006"
    type: "verifies"
---
# TC-042: Run generative, metamorphic, differential, and mutation evidence

## Description

Verify that every selected qualification technique executes under its exact property,
partition, seed, budget, stopping rule, oracle, Rust trace binding, and containment
boundary and preserves all non-success states.

## Test Procedure

Execute example, property, fuzz, metamorphic, differential, and mutation cases with
fixed controls. Force a counterexample, timeout, exhausted budget, unsupported tool,
surviving mutant, and failed process separately. Remove each qualification-critical
guard targeted by the mutation plan and replay minimized counterexamples.

## Expected Results

All results and artifacts remain attributable. Every forced non-success state blocks
its obligation without masking other findings. Guard-removal mutants fail a named
bare-`ix-trace-rs` Rust test for the intended reason, minimized cases replay, and an
unfinished fuzz budget remains incomplete.

