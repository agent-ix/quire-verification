---
id: SR-146
title: "Risk and complexity review of refreshed VP04 parser portfolio requirements"
type: SpecReview
analysis: risk-complexity
scope: "FR-019..FR-021, NFR-006, IT-003 and MP-001"
review_set: all
---
# Risk and Complexity Review of Refreshed VP04 Parser Portfolio Requirements

## Summary

Native process control, external protocol interpretation, and report identity
remain the principal risks. Fixed exact inputs, strict response relations,
schema-derived types, raw observations, and a late human gate bound the scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-019 remains technically high risk because one result crosses descriptor-pinned process execution, stream/status protocol parsing, evidence construction, and conservative assessment; focused layers plus real IT-003 execution mitigate it. | FR-019, NFR-006, TC-034..TC-035, IT-003 |
| FND-002 | high | FR-021 remains high integrity risk because a human artifact meets a JCS-sealed core; exact bytes, closed values, non-circular digesting, and negative binding cases are mandatory. | FR-021, TC-037 |
| FND-003 | medium | FR-020 has scheduler, cache, timer, and invalid-sample uncertainty; the fixed 16-slot order, raw observations, lower median, and narrow interpretation mitigate it without a performance threshold. | FR-020, MP-001, TC-036 |
| FND-004 | medium | NFR-006's expected-status extension touches already-qualified liveness code; reopen only that contract, preserve all containment tests, and rerun the exact Rust review. | NFR-006-AC-3..8, TC-038 |
