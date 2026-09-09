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

Collect the one setup plus five three-phase repetitions from a controlled
monotonic clock, including one invalid observation. Inspect raw order, unique
observation IDs, phase/repetition pairs, and identities; recompute each valid
phase's count, minimum, median, and maximum; and mutate one environment identity
before recomputation. Exercise empty and duplicate IDs, duplicate/missing/reordered
series entries, repetition bounds, and negative, fractional,
maximum-safe-integer, and one-above-maximum duration values.
Exercise both tagged observation variants by adding invalid-only fields to a
valid observation and `elapsedNanoseconds` to an invalid observation.

## Expected Results

The report retains every raw sample in order. Setup never enters an incremental
summary. The invalid and identity-mismatched observations keep their reasons,
are not rewritten as zero, and are excluded from summaries. Out-of-domain values
retain their exact JSON scalar spelling in `rejectedElapsed`. Zero and the maximum
safe integer are admitted; negative, fractional, and larger values remain
invalid. Cross-variant fields refuse. Each remaining integer-nanosecond summary recomputes exactly from its
own phase using the declared lower-median rule. The valid series has exactly 16
unique observations in its prescribed order; every identity or series-shape
mutation refuses with `invalid_measurement_series`.
