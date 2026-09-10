---
id: SR-193
title: "Risk and complexity review of Task-028 qualification techniques"
type: SpecReview
analysis: risk-complexity
scope: "quire-verification@6adbbc2; FR-022, FR-025, NFR-004, NFR-005"
review_set: all
---
# Risk and complexity review of Task-028 qualification techniques

## Summary

FR-025 has high technical risk and high external-contract volatility because it joins
closed identities, generative state, independent oracles, an evolving execution seam,
and a newly ruled native-profile admission boundary. The reviewed slice isolates pure
portable Rust semantics and keeps dependent evidence incomplete.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Mitigated: identity and combinatorial risk are addressed by schema-derived closed variants, JCS identities, one-axis mutations, deterministic property tests, strict reference roles, and fixed collection/operation bounds. | FR-025-AC-1..2, NFR-005 |
| FND-002 | high | Mitigated: volatile Engineering Assurance, catalog, and native-profile contracts are exact prerequisites. The portable implementation must not bake their provisional shapes or introduce fallback execution. | FR-025-CON-2, Task-028 |
| FND-003 | medium | Task-029 mutation/fuzz completion is computationally costly on the shared workstation. The plan keeps Cargo at two jobs, runs high-load phases serially, and does not dispatch hosted CI. | FR-025-AC-5, Task-029 |
| FND-004 | low | FR-022 budget-assignment remediation is low implementation complexity but high claim impact; a one-axis profile test must land before any Task-028 configuration freezes. | FR-022-AC-1..2, TC-039 |
| FND-005 | high | Mitigated: a total claim-authority union replaces natural-language scope inference and makes the native admission/export mapping boundary schema-testable before Task-028 consumes it. | FR-022-AC-5 |

## Risk Register

| Requirement | Technical Risk | Volatility | Mitigation |
|---|---|---|---|
| FR-022 | medium | medium | Exact budget key plus TC-039 mutation |
| FR-025 | high | high | Portable slice, typed prerequisites, property and mutation gates |
| NFR-004 | medium | low | Rust-only inspection and bare trace compilation |
| NFR-005 | medium | low | Existing fixed document/depth/array envelope |
