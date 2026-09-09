---
id: TC-034
title: "Execute native parser baseline and adverse cases"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-019"
    type: "verifies"
---
# TC-034: Execute native parser baseline and adverse cases

## Description

Verify that the exact native parser candidate produces separate direct-parse and
parse-format-reparse baseline results and that the declared syntax mutation's
exact diagnostic code and location are reported as a logical violation rather
than an unexpected execution failure.

## Test Procedure

Run the real native parser and formatter over the exact valid source, then run
the direct technique over its one declared syntax mutation. Separately substitute
an unexpected process failure and an unreadable native response. Mutate each
required success and diagnostic field, move payloads to the wrong stream, append
a second JSON value and non-whitespace trailing bytes, and change the expected
exit status.

## Expected Results

Both valid-source techniques complete with satisfied outcomes and complete
participation. The exact expected mutation diagnostic completes with a violated
outcome. Any completed metamorphic mismatch is violated and retains both
differing artifacts. The unexpected process and protocol cases return failed
execution with not-evaluated outcomes, and every result identifies its exact task
and run.
