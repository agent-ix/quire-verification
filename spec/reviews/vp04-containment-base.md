---
id: SR-071
title: "Base review of observed VP04 process containment"
type: SpecReview
analysis: base
scope: "quire-verification@743afca; NFR-006, TC-038 and their matrix rows"
review_set: all
---
# Base Review: Observed VP04 Process Containment

## Summary

The base checklist reviewed the reopened process-containment requirement, its
single TC binding, identifiers, boundary cases, error outcomes, and status rows.
The amendment is concrete and testable at revision `743afca`; implementation
evidence remains deliberately reopened.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|----|----------|---------|------|--------------|
| FND-001 | high | Resolved in the reviewed specification: the prior wording allowed a successful signal request to stand in for observed process-group termination; AC-3, AC-4, AC-5, and new AC-8 now require observed disappearance or an explicit containment failure. | NFR-006-AC-3..5, NFR-006-AC-8, TC-038 | implementation-bug-despite-evidence |
