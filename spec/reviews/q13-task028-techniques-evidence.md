---
id: SR-192
title: "Evidence review of Task-028 qualification techniques"
type: SpecReview
analysis: evidence
scope: "quire-verification@80627b6; FR-025, TC-042; q13-task028-techniques-advice.json; q13-task028-techniques-properties.json"
review_set: all
---
# Evidence review of Task-028 qualification techniques

## Summary

The required QUOIN advisor recommends combinatorial/property evidence for AC-1/2,
example/unit evidence for AC-3, and property evidence for AC-4/5; every authored `Test`
class agrees. Four of five criteria are property-extractable, and the one concrete state
matrix intentionally remains example-shaped.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | The implementation must run installed QUOIN `spec-correctness` for the four extractable criteria and retain generated Rust tests or its validated grounding findings. Hand-written weaker substitutes do not discharge that gate. | FR-025-AC-1..2, FR-025-AC-4..5, Task-028 |
| FND-002 | medium | Differential evidence is unavailable because `differential-testing` is absent from the accepted catalog. spec-artifacts-process#86 owns the reusable method; a private local identifier is forbidden. | FR-025-AC-1..4 |
| FND-003 | medium | Producer execution, replay observations, and failure-state discrimination need real typed results from Engineering Assurance #34. Synthetic manifest tests can back portable semantics but cannot mark TC-042 complete. | FR-025-AC-3..4, TC-042 |
| FND-004 | low | QUOIN initially treated concessive `while` in AC-5 as temporal and recommended model checking/runtime monitoring. Rewording removed the false hit, and quoin#359 retains the exact regression for the shared advisor rather than adding local filtering. | FR-025-AC-5 |
