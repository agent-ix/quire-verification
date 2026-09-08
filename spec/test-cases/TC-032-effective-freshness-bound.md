---
id: TC-032
title: "Apply the earliest evidence freshness bound"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-018"
    type: verifies
---
# TC-032: Apply the Earliest Evidence Freshness Bound

## Description

Verify that evidence assessment applies both absolute result validity and the
obligation maximum age, preserves findings for stale evidence, and refuses
invalid temporal ordering.

## Test Procedure

Create exact-matched obligations and results in which each bound is earlier in
turn. Assess immediately before, exactly at, and one nanosecond after the
effective expiry, including equivalent instants written with different RFC 3339
offsets. Repeat with a future production time, validity before production,
malformed timestamps, a missing maximum age, maximum ages at and outside the JSON
safe-integer boundary, and checked expiry overflow. Add simultaneous logical,
participation, and assumption findings to the stale case.

## Expected Results

The earlier applicable bound controls freshness, the exact boundary is fresh,
the next instant is stale, invalid temporal inputs refuse before classification,
and stale evidence retains every independent finding and native reference.
