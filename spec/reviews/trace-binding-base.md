---
id: SR-081
title: "Base review of Quire-readable Rust trace bindings"
type: SpecReview
analysis: base
scope: "quire-verification@41f7a57; NFR-004-AC-3, TC-022, TM-001 trace-binding rows"
review_set: all
---
# Base Review: Quire-readable Rust Trace Bindings

## Summary

The base checklist reviewed the amended trace-binding criterion, test procedure,
negative edge case, and reopened status rows. The specification now distinguishes
compiler acceptance from successful Quire discovery and binding and is concrete
enough to govern the formatting-only remediation.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|----|----------|---------|------|--------------|
| FND-001 | high | Resolved in the reviewed specification: the previous criterion required a compiled bare trace marker but did not require Quire to discover every argument, allowing NFR-006-AC-2 and AC-3 to remain unbacked behind a green Rust test. | NFR-004-AC-3, TC-022, EC-027 | wrong-requirement |
| FND-002 | low | The master index and matrix truthfully reopen NFR-004, NFR-006, TC-022, and TC-038 until the marker and coverage evidence are corrected. | spec/spec.md, TM-001 | correct-requirement-no-evidence |
