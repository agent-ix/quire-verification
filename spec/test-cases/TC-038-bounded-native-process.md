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

Run the Linux executor concurrently against immutable hard links to one completed
Cargo-built controlled Rust child, with each case using its own execution root.
Exercise exact and adverse root-relative executable paths, then replace the
admitted pathname before one invocation. Exercise arguments, environments, fresh
working directories, positive declared deadlines through the 10-second cap,
separate and simultaneous 16-MiB stream boundaries, exit statuses, signals,
UTF-8, and method responses. Have direct children leave descendants with retained
and closed output pipes. After every termination trigger, verify the direct child
is reaped and the process group has disappeared before a result can claim
termination; inject a group-observation timeout and verify that it becomes
`process_containment_failure` with the original error code retained as `trigger`.

## Expected Results

The exact bounded request succeeds and invokes the admitted descriptor even when
the request pathname is replaced afterward. Parallel fixture preparation leaves
no writable handle on the admitted executable. Relative, external, absent,
non-regular, and symbolic-link paths refuse before launch; an unsupported Linux
descriptor/process-group profile refuses with its stable code. An absent, zero,
or above-cap deadline refuses. A group at its accepted deadline, one byte over
either stream limit, or a descendant surviving its direct child is terminated;
the direct child is reaped and group disappearance is observed before termination
is claimed. Signals, unexpected codes, invalid UTF-8, malformed responses, and
containment failures remain distinct failed results with no panic, partial
success, or logical outcome.
