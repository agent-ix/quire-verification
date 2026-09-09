---
id: FR-018
title: "Enforce every evidence freshness bound"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-002"
    type: implements
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: depends_on
  - target: "ix://agent-ix/quire-verification/FR-006"
    type: depends_on
  - target: "ix://agent-ix/quire-verification/FR-007"
    type: depends_on
---
# FR-018: Enforce Every Evidence Freshness Bound

## Description

When an exact result is assessed, the evidence assessor SHALL classify freshness
against every applicable result and obligation time bound and use the earliest
expiry as the effective limit.

## Inputs

- Assessment time.
- Result production time and optional absolute validity limit.
- Obligation maximum age.

## Outputs

- A fresh or stale assessment classification with independent result findings.
- A stable temporal-input refusal when timestamps or their ordering are invalid.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-018-AC-1 | The effective expiry is the earlier of result `validUntil` and result `producedAt` plus obligation `maxAgeSeconds`; the result is stale only when assessment time is later than that expiry. | Test (TC-032) |
| FR-018-AC-2 | Assessment exactly at the effective expiry remains fresh, and one nanosecond after the expiry is stale; equivalent RFC 3339 offsets yield the same classification. | Test (TC-032) |
| FR-018-AC-3 | A result with `producedAt` later than assessment time or `validUntil` earlier than `producedAt` is refused with stable code `invalid_temporal_order`. | Test (TC-032) |
| FR-018-AC-4 | An invalid timestamp, a missing obligation maximum age, a maximum age outside `0..=9007199254740991` seconds, or checked expiry arithmetic outside the RFC 3339 timestamp domain is refused before result classification. | Test (TC-032) |
| FR-018-AC-5 | A stale classification takes precedence over the compatibility summary while preserving every applicable execution, logical, activation, participation, assumption, adequacy, provenance, and native-artifact field and finding. | Test (TC-032) |

## Dependencies

- Contract validation comes from [FR-001](./FR-001-validate-public-contracts.md).
- Exact matching and independent finding preservation come from [FR-006](./FR-006-match-evidence-exactly.md) and [FR-007](./FR-007-preserve-result-findings.md).
