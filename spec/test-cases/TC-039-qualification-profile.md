---
id: TC-039
title: "Freeze and reject qualification profiles"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-022"
    type: "verifies"
---
# TC-039: Freeze and reject qualification profiles

## Description

Verify that one complete owner-authored production profile receives a reproducible
JCS identity and that every missing, open-ended, stale, foreign, provisional, or
changed claim dimension is refused or retained as a prerequisite.

## Test Procedure

Build one complete profile, permute non-semantic input order, and recompute its
identity. Delete each command, caller, population, guarantee, environment, budget,
evidence, exclusion, and external-version field independently. Substitute an
unaccepted FS02/FS05 revision, name an absent guarantee or budget from one evidence
requirement, and mutate each identity-bearing value. Keep a nonempty authored guarantee
set while adding unrelated commands and catalog techniques.

## Expected Results

The complete and semantically equivalent profiles have one identity. Every omitted
or open dimension refuses emission, every unaccepted producer revision yields an
exact blocked prerequisite without a local schema, and every semantic mutation
changes identity while leaving the original bytes unchanged. Commands and catalog
techniques do not augment the authored guarantee set.
