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

Exercise all three `claimAuthority` forms. For `native-formal-clause`, delete, duplicate,
wrong-role, and rename each of the ten admission rules and change the exact source-profile
reference. For `export-mapping`, change the mapping-profile and loss-report references,
remove the strict-reader proof for the exact loss-report reference, and attempt to use
the mapping as source authority. For `non-formal`, remove the exclusion reason.

## Expected Results

The complete and semantically equivalent profiles have one identity. Every omitted
or open dimension refuses emission, every unaccepted producer revision yields an
exact blocked prerequisite without a local schema, and every semantic mutation
changes identity while leaving the original bytes unchanged. Commands and catalog
techniques do not augment the authored guarantee set. Native admission is accepted only
with all ten exact observations. Non-formal and export-mapping forms remain explicit and
cannot be promoted to native formal-clause authority.
