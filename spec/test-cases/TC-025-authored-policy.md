---
id: TC-025
title: "Retain authored policy and reject ordinal scoring"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-012"
    type: "verifies"
---
# TC-025: Retain authored policy and reject ordinal scoring

## Description

Verify selection priority and explanation remain authored data and the policy
contract refuses a numeric score.

## Test Procedure

Validate and plan with required, recommended, and optional policy rules and
selections carrying distinct explanations. Then add a numeric `score` to one
otherwise valid policy rule and validate it again.

## Expected Results

The valid plan preserves every authored priority and explanation without a
derived numeric value. The policy containing `score` fails contract validation.
