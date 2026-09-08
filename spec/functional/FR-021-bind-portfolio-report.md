---
id: FR-021
title: "Bind the portfolio report and human disposition"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-002"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/US-003"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-016"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-019"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-020"
    type: "depends_on"
---
# FR-021: Bind the portfolio report and human disposition

## Description

When VP04 assembles a retained portfolio report, the report assembler SHALL seal
the automated report core before attaching exactly one external human disposition
that refers to that core without creating a circular digest.

## Inputs

- The exact plan, results, assessment, effort observations, interpretation, and claim limits produced under [FR-019](./FR-019-execute-bounded-parser-portfolio.md) and [FR-020](./FR-020-record-portfolio-effort.md).
- An externally authored `review-disposition` artifact carrying exact bytes,
  provenance, reviewer identity, procedure reference, observation time, one
  allowed value, and the sealed core digest.

## Outputs

- A closed, versioned internal VP04 report envelope containing the immutable automated core and either no disposition or one exact verified disposition reference.
- Stable refusal codes `duplicate_disposition`, `foreign_disposition`, `unsupported_disposition`, and `invalid_report_digest`.

## Behavior

- The report assembler SHALL validate the complete automated core before digesting it.
- The report assembler SHALL compute `sha256-jcs:<hex>` over the automated core
  alone under the selected shared JCS canonical-identity domain.
- When a disposition is supplied, the report assembler SHALL verify its exact
  artifact bytes and require its core digest, review procedure, reviewer identity,
  observation time, and value to match the selected report and procedure.
- When no disposition is supplied, the report assembler SHALL retain the manual
  task as not-run and the report as incomplete.
- If multiple, foreign, unsupported, digest-invalid, or procedure-invalid
  dispositions are supplied, then the report assembler SHALL refuse finalization
  with the corresponding stable code.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-021-CON-1 | The internal VP04 report definition SHALL remain outside the five-contract public root. | Contract ownership | Test |
| FR-021-CON-2 | The schema generator SHALL derive the internal VP04 report definition from the same authoritative E02 schema document. | Contract ownership | Test |
| FR-021-CON-3 | The assembler SHALL treat the disposition as external human evidence. | Claim boundary | Inspection |
| FR-021-CON-4 | The assembler SHALL NOT generate, infer, or authenticate the reviewer beyond the exact supplied provenance and selected procedure. | Claim boundary | Inspection |
| FR-021-CON-5 | The report SHALL identify Quoin as evidence owner. | Ownership | Test |
| FR-021-CON-6 | The report SHALL record sufficiency as `not-made`. | Ownership | Test |
| FR-021-CON-7 | The assembler SHALL require the exact independently reviewed FS05 role selected for a human review-disposition artifact. | Compatibility | Test |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-021-AC-1 | One closed internal schema definition validates the VP04 report envelope without admitting it as a sixth public contract or duplicating its Rust shape by hand. | Test (TC-037) |
| FR-021-AC-2 | The sealed core contains the exact candidate, subject, mutation, commands, dependency closure, environment, plan, tasks, runs, producers, adapters, assessment time, raw effort observations, interpretation, and claim limits needed to reproduce the automated evidence. | Test (TC-037) |
| FR-021-AC-3 | The core digest excludes the disposition, and one verified disposition artifact refers to that digest with a unique nonempty disposition ID, exact reviewer and procedure identity, observation time, and value `accept`, `revise`, or `reject`. | Test (TC-037) |
| FR-021-AC-4 | An absent disposition leaves the report incomplete; duplicate, foreign, unsupported, digest-invalid, or procedure-invalid dispositions refuse with the corresponding stable code. | Test (TC-037) |
| FR-021-AC-5 | Finalization preserves every automated result and measurement unchanged, identifies Quoin as evidence owner, records sufficiency as `not-made`, and makes no claim that supplied reviewer provenance is cryptographically authenticated. | Test (TC-037) |

## Dependencies

- Implements [US-002](../usecase/US-002-assess-evidence-exactly.md) and [US-003](../usecase/US-003-detect-compatibility-regressions.md).
- Requires strict shared references from [FR-016](./FR-016-read-shared-references-strictly.md), bounded execution from [FR-019](./FR-019-execute-bounded-parser-portfolio.md), and effort evidence from [FR-020](./FR-020-record-portfolio-effort.md).
