---
id: FR-033
title: "Preserve immutable counterexample lineage"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-005"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-032"
    type: "depends_on"
  - target: "ix://agent-ix/quire-specification/FR-312"
    type: "references"
---
# FR-033: Preserve immutable counterexample lineage

## Description

When a counterexample is transformed, replayed, translated, normalized, shrunk,
or redacted, the lineage builder SHALL create a new immutable revision that
retains the exact parent, operation, correspondence, provenance, and scoped
replay result.

## Inputs

- Exact source subject, original typed values or traces, failing claim, and provider run.
- Transformation identity/options, parent revision, correspondence, and replay result.

## Outputs

- Ordered immutable counterexample lineage revisions.
- Typed invalid-domain, changed-verdict, broken-correspondence, or replay-material-loss finding.

## Behavior

- A transformation SHALL NOT overwrite its input revision.
- Two revisions SHALL be equivalent only for a named claim when both are domain-valid and replay to the same scoped failure under an explicit value/execution correspondence.
- The lineage builder SHALL reject a shrink or translation that changes domain validity, the failing claim, or disposition without discarding the parent.
- Redaction SHALL list removed fields and each affected claim.
- The lineage builder SHALL retain a revision that lost replay-required material as non-replayable evidence.
- The lineage builder SHALL prevent that revision from acting as a replay-qualified counterexample substitute.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-033-AC-1 | Original, transformed, and replayed revisions retain ordered immutable parentage, exact typed values/traces, provider identity, operation, correspondence, and replay outcome. | Test (TC-052) |
| FR-033-AC-2 | A domain-invalid, correspondence-breaking, or verdict-changing transformation is refused and leaves every prior revision unchanged. | Test (TC-052) |
| FR-033-AC-3 | Redacted, inaccessible, or lost replay material remains visible with exact claim impacts and cannot satisfy replay qualification. | Test (TC-052) |

## Dependencies

- [FR-032](./FR-032-link-semantic-evidence.md) owns the typed evidence graph.
- `ix://agent-ix/quire-specification/FR-312` owns accepted lineage semantics.
