---
id: TC-044
title: "Measure and reproduce the exact production profile"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/NFR-007"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-008"
    type: "verifies"
---
# TC-044: Measure and reproduce the exact production profile

## Description

Verify owner-threshold coverage, raw performance observations, bounded local
concurrency, exact artifact resolution, deterministic replay, stochastic metadata,
and exploratory-environment claim limits.

## Test Procedure

Run MP-002 for every declared phase and reproduce the candidate and evidence from
the retained source, binary, lockfile, corpus, oracle, environment, seed, command,
and result references. Remove one owner threshold, exceed one phase threshold,
invalidate one sample, alter one environment, and attempt a reproduction with more
than two Cargo jobs.

## Expected Results

Every phase summary recomputes from retained observations and no aggregate masks a
violation. Missing thresholds or incomplete samples block the gate. Exact deterministic
evidence replays; stochastic differences retain their cause and distribution. The
unclaimed environment remains exploratory and excessive local concurrency refuses.

