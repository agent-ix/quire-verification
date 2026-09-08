---
id: TC-014
title: "Distinguish freshness and result states"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-006"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-007"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-003"
    type: "verifies"
---
# TC-014: Distinguish freshness and result states

## Description

Verify stale and adverse result dimensions remain distinct from supported
evidence.

## Test Procedure

Assess the retained result fixture and compare every obligation status with its
expected unsupported, failed, incomplete, untriggered, inadequate, partial,
assumption-unmet, violated, stale, missing, or supported state.

## Expected Results

All expected states match, including freshness taking precedence over an
otherwise supportive logical outcome.

