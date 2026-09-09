---
id: SR-072
title: "Failure-domain review of observed VP04 process containment"
type: SpecReview
analysis: failure-domain
scope: "quire-verification@743afca; NFR-006 and TC-038"
review_set: all
---
# Failure-Domain Review: Observed VP04 Process Containment

## Summary

The review covered spawn-time file state, signal failure, delayed process exit,
direct-child reaping, surviving descendants, output readers, and bounded cleanup.
Each unobserved containment path now has an explicit non-success outcome.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|----|----------|---------|------|--------------|
| FND-001 | high | Resolved in the reviewed specification: queued `SIGKILL` is not proof that a descendant has stopped, so the executor must observe group disappearance after reaping and must expose `process_containment_failure` if signaling or observation fails. | NFR-006-AC-3..5, NFR-006-AC-8 | implementation-bug-despite-evidence |
| FND-002 | medium | Resolved in the test contract: concurrently copied executables could be observed write-busy on hosted Linux; TC-038 now uses isolated immutable links to a completed Cargo-built fixture. | TC-038 | correct-requirement-no-evidence |
