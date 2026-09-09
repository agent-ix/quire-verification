---
id: TC-043
title: "Execute and refuse the production chain"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-026"
    type: "verifies"
---
# TC-043: Execute and refuse the production chain

## Description

Verify the real selected caller-to-`quire-spec`-to-portable-result-to-assurance path
and distinguish every external seam from a test-only substitute.

## Test Procedure

Execute the exact accepted chain once, then substitute a test adapter, changed
candidate, unsupported interface version, stale dependency, missing assurance
handoff, transport failure, and copied producer payload separately. Interrupt each
seam after prior observations exist.

## Expected Results

The exact chain preserves every identity through the assurance handoff and claims
only executed semantic stages. Every substitute or failed seam refuses qualification,
names its existing owner, and retains all valid upstream observations without
duplicating producer behavior in `quire-verification`.

