---
id: SR-152
title: "Base review of VP04 independent-review remediation"
type: SpecReview
analysis: base
scope: "NFR-004, TC-022, TC-037, TC-038 and the affected test-matrix rows"
review_set: all
---
# Base Review of VP04 Independent-Review Remediation

## Summary

The owner-selected full review checked the specification delta required by
independent review SR-151. The delta makes the vendored trace population
explicit and adds discriminating procedures for relative symlinks and retained
core drift without changing VP04 runtime behavior or its bounded claim.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Resolved in the reviewed delta: TC-038 now requires relative as well as absolute executable/root symlinks, so `NO_SYMLINKS` is tested independently of `BENEATH`. | NFR-006-AC-6..7, TC-038 | correct-requirement-no-evidence |
| FND-002 | medium | Resolved in the reviewed delta: TC-037 now requires a default local ratchet over the committed core digest, schema, bindings and recomputed assessment. | FR-021-AC-1..5, TC-037 | correct-requirement-no-evidence |
| FND-003 | low | Resolved in the reviewed delta: NFR-004 and TC-022 state the first-party trace population and classify vendored dependency symbols under their owning matrix. | NFR-004-AC-6, TC-022 | missing-requirement |

