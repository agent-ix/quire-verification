---
id: FR-032
title: "Link exact semantic subjects to evidence"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-005"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-006"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-007"
    type: "depends_on"
  - target: "ix://agent-ix/quire-specification/FR-310"
    type: "references"
  - target: "ix://agent-ix/quire-specification/FR-337"
    type: "references"
---
# FR-032: Link exact semantic subjects to evidence

## Description

When method or qualification evidence is assembled, the evidence-graph builder SHALL link each exact requirement and semantic subject independently to its assumptions, implementation, method, provider run, immutable artifacts, and attributed decision.

## Inputs

- Typed source, package, model, binding, clause, semantic-subject, implementation, method, provider, run, artifact, evidence, and decision identities.
- Exact relation, scope, provenance, rights, availability, and limitation data.

## Outputs

- Immutable typed evidence graph and evidence-intake records.
- Typed identity, relation, stale-subject, digest, cycle, or limitation refusal.

## Behavior

- Graph edges SHALL use the closed relations `derived-from`, `evaluates`, `implements`, `generated-by`, `replays`, `supports`, `contradicts`, and `decides`.
- Every edge SHALL carry its exact scope and provenance.
- No identity kind SHALL substitute for another, even when textual digests match.
- A discharge path SHALL begin at the exact claim, reach independently produced evidence, and end at an attributed decision.
- A cycle without an independent evidence node SHALL confer no support.
- The graph builder SHALL retain every cycle without independent evidence as visible unresolved state.
- Missing, redacted, inaccessible, or non-replayable artifacts SHALL retain explicit claim impacts.
- Intake SHALL validate producer bytes.
- Intake SHALL NOT execute or reinterpret a producer.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-032-CON-1 | Evidence retention and authored sufficiency remain Quoin-owned; this repository SHALL emit typed intake records without creating a duplicate store. | Ownership | Inspection (TC-050) |
| FR-032-CON-2 | Evidence relations SHALL be append-only; corrections supersede immutable records. | Integrity | Test (TC-050) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-032-AC-1 | Every accepted method result traces to exact requirement, assumptions, semantic subject, implementation, method/provider, run, and artifacts. | Test (TC-050) |
| FR-032-AC-2 | Cross-kind identity reuse, stale subjects, digest mismatch, unsupported relation, or circular self-evidence refuses discharge while preserving the graph and sibling evidence. | Test (TC-050) |
| FR-032-AC-3 | Redaction, loss, inaccessibility, or absent replay material remains an explicit limitation on every affected claim and never broadens evidence. | Test (TC-050) |

## Dependencies

- [FR-006](./FR-006-match-evidence-exactly.md) and [FR-007](./FR-007-preserve-result-findings.md) provide exact matching and independent finding behavior.
- [interface-002](../interface/interface-002-qualification-evidence-authority.md) defines the adopted I18-I21 boundary.
