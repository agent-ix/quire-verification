---
id: TC-037
title: "Bind portfolio disposition and claim limits"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-019"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-020"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-021"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-003"
    type: "verifies"
---
# TC-037: Bind portfolio disposition and claim limits

## Description

Verify that VP04 cannot qualify without one human disposition bound to the sealed
automated report core and that neither execution nor measurement wording exceeds
the observed parser-fixture boundary.

## Test Procedure

Validate the closed internal report shape, seal the automated core, and inspect
the report before disposition, with each admitted disposition value, with two
dispositions, and with a disposition copied from another report. Inspect the
measurement context, exclusions, uncertainty, alternative explanations,
evidence owner, sufficiency field, and claim-limit text. In a non-ignored local
test, read the committed VP04 automated-core artifact, recompute its RFC 8785 JCS
digest, revalidate its closed report/core shape and exact reference bindings, and
recompute its assessment from its retained inventory and results.

## Expected Results

The undisposed report remains incomplete. The internal schema does not add a
sixth public contract. The core digest excludes disposition fields. Exactly one
bound `accept`, `revise`, or `reject` value is representable; duplicate or foreign
dispositions refuse. Quoin remains evidence and sufficiency owner, sufficiency
remains not-made, supplied reviewer provenance is not called authenticated, and
the report makes no model, evaluator, production, ROI, delivery, throughput,
scale, certification, or general-effectiveness claim. The committed core's
recorded digest equals the recomputed digest, its bindings validate, and its
retained assessment equals the independently recomputed assessment; artifact or
report-logic drift fails the default local test suite.
