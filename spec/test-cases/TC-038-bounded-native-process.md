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

Verify that native parser invocation pins the admitted executable, has no shell
or inherited-environment path, contains a dedicated process group, enforces
exact time and output boundaries, and returns typed failures without a live
descendant or logical claim.

## Test Procedure

Admit a controlled native child through the public runner with exact and adverse
root-relative executable paths, then replace the admitted pathname before one
invocation. Exercise arguments, environments, fresh working directories,
positive declared deadlines through the 10-second cap, separate and simultaneous
16-MiB stream boundaries, exit statuses, signals, UTF-8, and method responses.
Have one direct child leave a descendant holding both output pipes. After every
adverse case, verify the direct child is reaped and no live member remains in its
dedicated process group.

## Expected Results

The exact bounded request succeeds and invokes the admitted descriptor even when
the request pathname is replaced afterward. Relative, external, absent,
non-regular, and symbolic-link paths refuse before launch; an unsupported Linux
descriptor/process-group profile refuses with its stable code. An absent, zero,
or above-cap deadline refuses. A group at its accepted deadline, one byte over
either stream limit, or a descendant surviving its direct child is terminated;
the direct child is reaped and no live group member remains. Signals, unexpected
codes, invalid UTF-8, and malformed responses remain distinct failed results
with no panic, partial success, or logical outcome.
