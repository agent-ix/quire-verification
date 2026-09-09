---
id: SR-142
title: "Failure-domain review of refreshed VP04 parser portfolio requirements"
type: SpecReview
analysis: failure-domain
scope: "FR-019..FR-021, NFR-006, IT-003, MP-001, TC-034..TC-038"
review_set: all
---
# Failure-Domain Review of Refreshed VP04 Parser Portfolio Requirements

## Summary

The review covered native status and stream failures, parser protocol ambiguity,
measurement variant confusion, graph reuse, freshness, and disposition identity.
The identified specification gaps were resolved in the reviewed requirements.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Resolved: an expected exit-1 diagnostic is now distinguishable from signals and unexpected statuses without bypassing process containment or losing captured stderr. | NFR-006-AC-5, FR-019-AC-2 | wrong-requirement |
| FND-002 | medium | Resolved: exact-one JSON decoding, wrong-stream data, trailing bytes, foreign identity, wrong digest, and count drift now fail the native method protocol instead of becoming support. | FR-019 Behavior, FR-019-AC-1..2, TC-034 | missing-requirement |
| FND-003 | medium | Resolved: valid and invalid measurement variants are disjoint, and out-of-domain durations retain their scalar spelling without entering summaries as zero. | FR-020-AC-3, TC-036 | missing-requirement |
| FND-004 | low | The absent human disposition remains an explicit incomplete state; the final external decision is a planned gate rather than an automated default. | FR-021-AC-3..5, IT-003-SC-09 | correct-requirement-no-evidence |
