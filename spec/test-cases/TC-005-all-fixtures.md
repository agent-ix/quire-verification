---
id: TC-005
title: "Execute the schema against every retained fixture"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-003"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-002"
    type: "verifies"
---
# TC-005: Execute the schema against every retained fixture

## Description

Verify fixture discovery executes schema validation across all retained JSON
fixtures and their embedded or adapted public contracts.

## Test Procedure

Run `npm run validate:fixtures` and the fixture-schema-validation unit test.

## Expected Results

The command reports four fixtures and 52 public contract instances, and the unit
test lists the four expected filenames.

