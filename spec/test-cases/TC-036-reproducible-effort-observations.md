---
id: TC-036
title: "Record reproducible effort observations"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-020"
    type: "verifies"
---
# TC-036: Record reproducible effort observations

## Description

Verify that setup and incremental portfolio timing observations remain separate,
retain invalid samples, and produce exactly recomputable per-phase summaries.

## Test Procedure

Collect the prescribed five repetitions from a controlled monotonic clock,
including one invalid observation. Inspect raw order and identities, recompute
each valid phase's count, minimum, median, and maximum, and mutate one environment
identity before recomputation.

## Expected Results

The report retains every raw sample in order. Setup never enters an incremental
summary. The invalid and identity-mismatched observations keep their reasons,
are not rewritten as zero, and are excluded from summaries. Each remaining
integer-nanosecond summary recomputes exactly from its own phase.
