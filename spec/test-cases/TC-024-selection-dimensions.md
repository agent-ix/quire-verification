---
id: TC-024
title: "Keep applicability readiness and classification independent"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-010"
    type: "verifies"
---
# TC-024: Keep applicability readiness and classification independent

## Description

Verify planning preserves the three applicability states independently from
capability readiness and always emits an authored or explicit unknown property
classification.

## Test Procedure

Build plans for applicable, not-applicable, and unknown selections with a ready
capability, then repeat an applicable selection with its capability absent.
Include one property with an authored classification and one whose
classification is omitted.

## Expected Results

Each task retains the selected applicability, readiness changes only with the
capability/prerequisite state, invocation requires both applicable and ready,
the authored classification remains byte-for-byte unchanged, and the omitted
classification becomes `unknown`.
