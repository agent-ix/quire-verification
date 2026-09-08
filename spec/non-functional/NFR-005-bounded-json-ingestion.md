---
id: NFR-005
title: "Bounded JSON ingestion"
type: NFR
quality_attribute: reliability
relationships:
  - target: "ix://agent-ix/quire-verification/FR-001"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-003"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-004"
    type: "constrains"
  - target: "ix://agent-ix/quire-verification/FR-006"
    type: "constrains"
---
# NFR-005: Bounded JSON ingestion

## Statement

When JSON enters through a public command or library boundary, the compatibility slice SHALL enforce a finite document, nesting, and collection resource envelope before performing unbounded work.

## Scope

This requirement applies to schemas, fixtures, Quoin catalog and advice input,
planning requests, result intake, and bounded portfolio snapshots.

## Rationale

Qualified-path inputs can be malformed or unexpectedly large. Parsing or walking
them without a fixed envelope can exhaust memory, stack, or validation time.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Maximum accepted serialized JSON document size | At most 16 MiB | 16 MiB | Test (TC-023) |
| Maximum accepted JSON nesting depth | At most 128 levels | 128 levels | Test (TC-023) |
| Maximum accepted items in any JSON array | At most 100,000 items | 100,000 items | Test (TC-023) |

## Verification

Boundary tests exercise values exactly at and immediately beyond each limit,
including a deeply nested adversarial document and an oversized collection.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-005-AC-1 | A public ingestion path rejects a serialized JSON document larger than 16 MiB before deserialization. | Test (TC-023) |
| NFR-005-AC-2 | Contract validation rejects a JSON value nested deeper than 128 levels. | Test (TC-023) |
| NFR-005-AC-3 | Contract validation rejects any JSON array containing more than 100,000 items. | Test (TC-023) |

## Dependencies

- Constrains public and external-input boundaries in [FR-001](../functional/FR-001-validate-public-contracts.md), [FR-003](../functional/FR-003-validate-all-fixtures.md), [FR-004](../functional/FR-004-adapt-quoin-catalog.md), and [FR-006](../functional/FR-006-match-evidence-exactly.md).

