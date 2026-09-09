---
id: TC-030
title: "Build a heterogeneous parser portfolio plan"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-013"
    type: verifies
  - target: "ix://agent-ix/quire-verification/FR-014"
    type: verifies
  - target: "ix://agent-ix/quire-verification/FR-015"
    type: verifies
---
# TC-030: Build a Heterogeneous Parser Portfolio Plan

## Description

Verify one parser request retains complementary property, differential, and
manual-review work without pretending they share one unary input or readiness.

## Test Procedure

Build a pinned parser plan with a state/property task carrying a validated
`snapshot` envelope and requiring a generator and harness, a
relational/differential task requiring two native parser bindings, and a manual
artifact review carrying a validated `review-procedure` envelope and requiring
an assigned reviewer. Give the native capabilities explicit resource ceilings
and an in-limit plan budget. Run ready, missing-prerequisite,
unsupported-capability, and absent-budget variants.

## Expected Results

All three obligations remain present with distinct inputs. Only individually
prepared native tasks with a selected in-limit budget are invocable; missing
tools, bindings, reviewer, and budget become deterministic setup dependencies.
Manual review keeps its input and readiness but has a null runner request and is
never native-invocable.
