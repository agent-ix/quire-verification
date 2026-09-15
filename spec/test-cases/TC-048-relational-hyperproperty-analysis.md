---
id: TC-048
title: "Relational and finite-trace hyperproperty analysis"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-030"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-009"
    type: "verifies"
---
# TC-048: Relational and finite-trace hyperproperty analysis

## Description

Verify explicit multi-execution correspondence, ordered quantifiers, closed
trace populations, total alignment, assumptions, and paired witnesses.

## Test Procedure

Evaluate the same finite population under universal-existential and
existential-universal prefixes. Mutate trace identities, closure declarations,
alignment cardinality, declassification assumptions, and finite bounds. Attempt
to correlate equal local names and to reuse one trace for distinct identities.

## Expected Results

Quantifier order is retained and may change the result. Valid requests return
exact paired or trace-set witnesses. Missing closure, absent or ambiguous
alignment, exceeded bounds, name-only correlation, and identity duplication
remain incomplete, requires-bound, or refused without a semantic verdict.

