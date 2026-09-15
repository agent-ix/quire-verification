---
id: FR-030
title: "Analyze relational and finite-trace hyperproperties"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-005"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-017"
    type: "depends_on"
  - target: "ix://agent-ix/quire-specification/FR-191"
    type: "references"
---
# FR-030: Analyze relational and finite-trace hyperproperties

## Description

When a checked relational or finite-trace-set request is executed, the
relational provider SHALL bind every quantified trace variable to one declared
closed population and evaluate the ordered prefix through explicit total
alignment and correlation relations.

## Inputs

- Checked relational expressions and exact multi-execution semantic subjects.
- Ordered quantifier prefix, closed trace populations, typed alignment/correlation relations.
- Declassification and environment assumptions, finite bounds, and provider identity.

## Outputs

- Proved, refuted, incomplete, requires-bound, unsupported, or failed method result.
- Paired or trace-set witness/counterexample with exact trace identities and alignments.

## Behavior

- Quantifiers SHALL execute left to right over their declared trace populations.
- Distinct variables SHALL NOT be satisfied by duplicating one trace identity unless the authored population explicitly contains distinct equal-valued traces.
- Cross-execution expressions SHALL compare only explicitly typed values at total, unambiguous aligned positions or correlated event identities.
- Declassification and environment assumptions SHALL restrict the population before quantification.
- The provider SHALL include every declassification and environment assumption in result identity.
- Missing population closure, missing or ambiguous alignment, or an exceeded finite bound SHALL produce a non-conclusive disposition.
- The provider SHALL NOT correlate executions by local variable, event, or trace name alone.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-030-CON-1 | Relational evaluation and result adaptation SHALL be first-party Rust over explicitly typed trace identities. | Language | Inspection (TC-048) |
| FR-030-CON-2 | Oracle dependence SHALL remain explicit. | Ownership | Test (TC-048) |
| FR-030-CON-3 | The provider SHALL NOT promote dependent oracle evidence to independent evidence. | Ownership | Test (TC-048) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-030-AC-1 | A two-execution property uses explicit input/output correspondence and returns an exact paired witness or counterexample. | Test (TC-048) |
| FR-030-AC-2 | Reversing a mixed universal/existential prefix is preserved and can change the result over the same closed population. | Test (TC-048) |
| FR-030-AC-3 | Missing closure, absent or ambiguous alignment, exceeded bounds, local-name correlation, or duplicated trace identity produces incomplete/requires-bound or refusal rather than a semantic verdict. | Test (TC-048) |

## Dependencies

- [FR-017](./FR-017-retain-oracle-dependence.md) owns oracle-dependence classification.
- `ix://agent-ix/quire-specification/FR-191` owns the accepted relational semantics.
