---
id: NFR-003
title: "Conservative assurance claims"
type: NFR
quality_attribute: functional_suitability
relationships:
  - target: "ix://agent-ix/quire-verification/FR-004"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-007"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-009"
    type: "constrains"
---
# NFR-003: Conservative assurance claims

## Statement

The compatibility slice SHALL expose unsupported, incomplete, unqualified, ambiguous, and pending-human states without converting them into tool execution or evidence-sufficiency claims.

## Scope

This policy applies to adapted catalogs, verification plans, evidence
assessments, retained fixtures, and bounded portfolio reports.

## Rationale

Compatibility data and locally exercised adapter checks do not establish that a
represented verification backend ran or that a human accepted its evidence.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Adverse or pending states converted to supported evidence | 0 | 0 | Test (TC-008, TC-014, TC-015) |
| Automated sufficiency decisions emitted | 0 | 0 | Test (TC-020) |

## Verification

Automated tests exercise unsupported, failed, incomplete, ambiguous, stale,
untriggered, observation-inadequate, partial, assumption-unmet, violated, and
pending-human states. Inspection checks the retained claim-limit text.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-003-AC-1 | Every tested adverse, ambiguous, unqualified, or pending state remains distinct from supported evidence. | Test (TC-008, TC-014, TC-015) |
| NFR-003-AC-2 | Bounded reports retain explicit claim limits and make no automated sufficiency decision. | Test (TC-020) |

## Dependencies

- Constrains [FR-004](../functional/FR-004-adapt-quoin-catalog.md), [FR-007](../functional/FR-007-preserve-result-findings.md), and [FR-009](../functional/FR-009-retain-portfolio-failures.md).
