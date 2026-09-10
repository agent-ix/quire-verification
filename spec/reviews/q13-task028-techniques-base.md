---
id: SR-188
title: "Base review of Task-028 qualification-technique specification"
type: SpecReview
analysis: base
scope: "quire-verification@80627b6; FR-022, FR-025, NFR-005, TC-039, TC-042, Task-025, Task-028"
review_set: all
---
# Base review of Task-028 qualification-technique specification

## Summary

The review checked identifier/link integrity, closed inputs and outputs, error paths,
boundaries, matrix ownership, and the Rust/Engineering Assurance/Quoin split. The
specification is implementable for its portable subset and makes each unavailable
producer capability an explicit gap.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Resolved before implementation: an evidence obligation carried a stopping rule but no owner-authored budget assignment, so Task-028 could have selected any profile budget. FR-022 and TC-039 now require one exact existing `budgetId`. | FR-022-AC-1..2, TC-039 |
| FND-002 | medium | Resolved before implementation: the draft duplicated family state, left reference roles implicit, admitted optional-obligation omission that the profile schema forbids, and omitted finite seed/count bounds. The reviewed text now defines one tagged `technique`, exact roles, complete obligation coverage, and closed bounds. | FR-025-AC-1, TC-042 |
| FND-003 | medium | Open external dependencies are explicit: Engineering Assurance #34 owns typed producer execution, spec-artifacts-process #86 owns `differential-testing`, and accepted native-profile admission evidence remains absent. Each blocks only its dependent result path and authorizes no substitute. | FR-025-CON-2, Task-028 |
| FND-004 | low | TC-042 and FR-025 remain Planned/unbacked until the reviewed Rust implementation and Task-029 completion gate exist; the matrix makes no premature coverage claim. | FR-025, TC-042 |
