---
id: TC-037
title: "Bind portfolio disposition and claim limits"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-019"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-020"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-003"
    type: "verifies"
---
# TC-037: Bind portfolio disposition and claim limits

## Description

Verify that VP04 cannot qualify without one human disposition bound to the exact
report and that neither execution nor measurement wording exceeds the observed
parser-fixture boundary.

## Test Procedure

Inspect the report before disposition, with each admitted disposition value, with
two dispositions, and with a disposition copied from another report. Inspect the
measurement context, exclusions, uncertainty, alternative explanations,
evidence owner, sufficiency field, and claim-limit text.

## Expected Results

The undisposed report remains incomplete. Exactly one bound `accept`, `revise`,
or `reject` value is representable; duplicate or foreign dispositions refuse.
Quoin remains evidence and sufficiency owner, sufficiency remains not-made, and
the report makes no model, evaluator, production, ROI, delivery, throughput,
scale, certification, or general-effectiveness claim.
