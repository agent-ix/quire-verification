---
id: SR-082
title: "Failure-domain review of Quire-readable Rust trace bindings"
type: SpecReview
analysis: failure-domain
scope: "quire-verification@41f7a57; NFR-004-AC-3, TC-022, EC-027"
review_set: all
---
# Failure-domain Review: Quire-readable Rust Trace Bindings

## Summary

The failure-domain review examined parser disagreement, partial argument
discovery, false completion, malformed attributes, and ownership boundaries.
The new requirement makes partial trace discovery a failure even when the macro
compiles and the test executes.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|----|----------|---------|------|--------------|
| FND-001 | high | Resolved in the reviewed specification: a multiline trace marker can compile while continuation-line AC arguments remain invisible to the configured Quire reader; TC-022 now rejects any such partial discovery. | NFR-004-AC-3, TC-022, EC-027 | wrong-requirement |
| FND-002 | low | No callback, mutable entity, graph traversal, user-supplied logic, runtime path, or new identity domain is introduced; the failure surface is the static Rust-to-Quire trace seam only. | NFR-004 Scope, TC-022 | correct-requirement-no-evidence |
