---
id: TC-006
title: "Reject an unhandled fixture and inspect the CI gate"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-003"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-002"
    type: "verifies"
---
# TC-006: Reject an unhandled fixture and inspect the CI gate

## Description

Verify a newly discovered fixture cannot bypass validation and that CI runs the
fixture validator separately.

## Test Procedure

Add a temporary JSON fixture without a handler, run fixture validation, remove
the temporary file, and inspect `.github/workflows/ci.yml` for the distinct
fixture-validation step before the remaining Rust test suite.

## Expected Results

Validation fails and names the temporary fixture; CI orders fixture validation
before the test suite. The dynamic error-path procedure is not yet automated.
