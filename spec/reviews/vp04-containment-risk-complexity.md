---
id: SR-076
title: "Risk review of observed VP04 process containment"
type: SpecReview
analysis: risk-complexity
scope: "quire-verification@743afca; NFR-006 and TC-038"
review_set: all
---
# Risk Review: Observed VP04 Process Containment

## Summary

NFR-006 remains high technical risk and low volatility because it combines
security containment, concurrent pipe readers, operating-system lifecycle state,
and hard time/output limits. The amended evidence directly targets the observed
hosted race.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|----|----------|---------|------|--------------|
| FND-001 | high | Signal completion, direct-child reap, descendant exit, pipe closure, and process-group disappearance occur on different schedules; mitigate with one bounded state loop plus real concurrent hosted tests and an injected timeout control. | NFR-006-AC-3..5, NFR-006-AC-8, TC-038 | implementation-bug-despite-evidence |
| FND-002 | medium | Executing a freshly copied fixture is volatile under parallel hosted I/O; an immutable hard link to the completed Cargo artifact removes that writer-state variable without weakening descriptor substitution coverage. | TC-038 | correct-requirement-no-evidence |
