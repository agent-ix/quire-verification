---
id: SR-213
title: "Risk and complexity review of Task-029 fuzz and mutation evidence"
type: SpecReview
analysis: risk-complexity
scope: "16922e1: FR-025, NFR-004..NFR-005, NFR-008, Task-029"
review_set: all
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-025", type: reviews }
  - { target: "ix://agent-ix/quire-verification/Task-029", type: reviews }
---
# Risk and Complexity Review of Task-029 Fuzz and Mutation Evidence

## Summary

FR-025 remains high technical risk and high contract volatility. Search evidence is
probabilistic, mutation runs are computationally costly, and the shared Rust execution
interface is still changing. The spec limits work, preserves negative results, and keeps
portable preparation separate from unavailable execution.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Mitigated: emitted targets and completed budgets could create false confidence. The target is explicitly undischarged, an unfinished stopping rule is incomplete, and crash/hang claims require exact replay. | FR-025-AC-6, SR-207 |
| FND-002 | high | Mitigated: mutation false positives are controlled by an exact mutant identity, assigned test, and declared failure reason; unrelated failures do not kill a mutant. | FR-025-AC-5 |
| FND-003 | high | Open volatility: Engineering Assurance #34 has not published the consumer interface. Task-029 must pin the accepted revision and run a cross-repository contract case before adapting results. | FR-025-CON-2, engineering-assurance#34 |
| FND-004 | medium | Local fuzz/mutation campaigns can overload the shared desktop. Existing two-job and serial high-load rules remain mandatory, with no hosted workflow dispatch. | AD-002 decision 7, Task-029 notes |

## Risk Register

| Requirement | Technical Risk | Volatility | Mitigation |
| --- | --- | --- | --- |
| FR-025 | high | high | Closed identities, typed states, common producers, replay, and external interface pin |
| NFR-004 | medium | low | Rust-only executable paths and compiled bare trace markers |
| NFR-005 | high | low | Existing fixed byte/depth/collection limits plus direct fuzz surface |
| NFR-008 | high | medium | Original counterexamples, exact replay, and anti-reseal mutation evidence |

