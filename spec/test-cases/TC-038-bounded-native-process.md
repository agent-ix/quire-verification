---
id: TC-038
title: "Bound native parser process execution"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/NFR-006"
    type: "verifies"
---
# TC-038: Bound native parser process execution

## Description

Verify that native parser invocation has no shell or inherited-environment path,
enforces exact time and output boundaries, and returns typed failures without an
orphan process or logical claim.

## Test Procedure

Invoke controlled native child fixtures through the public runner with exact and
adverse executable paths, arguments, environments, working directories, 10-second
time boundaries, separate and simultaneous 16-MiB stream boundaries, exit
statuses, signals, UTF-8, and method responses. After every adverse case, verify
the child has been reaped.

## Expected Results

The exact bounded request succeeds. Relative, external, absent, non-regular, and
symbolic-link paths refuse before launch. A child at the time limit or one byte
over either stream limit is terminated and reaped with its stable error. Signals,
unexpected codes, invalid UTF-8, and malformed responses remain distinct failed
results with no panic, partial success, or logical outcome.
