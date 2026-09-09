---
id: SR-075
title: "Evidence review of observed VP04 process containment"
type: SpecReview
analysis: evidence
scope: "quire-verification@743afca; NFR-006 and TC-038"
review_set: all
---
# Evidence Review: Observed VP04 Process Containment

## Summary

The mandatory QUOIN advisor ran against the corrected revision. AC-8 is authored
as Test and the catalog recommends BDD/unit evidence; the existing compiled Rust
integration and unit seams can supply both without a clock-threshold assertion.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|----|----------|---------|------|--------------|
| FND-001 | high | The earlier passing local repetitions did not discriminate signal delivery from observed termination; TC-038 must now fail if the executor returns before the group disappears. | NFR-006-AC-3..5, NFR-006-AC-8, TC-038 | implementation-bug-despite-evidence |
| FND-002 | medium | AC-8 needs two complementary checks: a real hosted descendant case for observed disappearance and an injected observer timeout for stable `process_containment_failure`/`trigger` behavior. | NFR-006-AC-8, TC-038 | correct-requirement-no-evidence |
