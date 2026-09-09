---
id: SR-086
title: "Risk and complexity review of Quire-readable Rust trace bindings"
type: SpecReview
analysis: risk-complexity
scope: "quire-verification@41f7a57; NFR-004-AC-3, TC-022"
review_set: all
---
# Risk and Complexity Review: Quire-readable Rust Trace Bindings

## Summary

The implementation risk is low because the expected delta changes one test
attribute's source layout and no runtime behavior. Volatility is medium because
the external Quire reader may later gain multiline support, while the required
observable binding result remains stable.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|----|----------|---------|------|--------------|
| FND-001 | low | The technical change is formatting-only, but false completion is high impact; requiring both Cargo compilation and Quire coverage is the mitigation. | NFR-004-AC-3, TC-022 | implementation-bug-despite-evidence |
| FND-002 | low | The specification avoids pinning a permanent single-line syntax rule; it pins the stable outcome that every named argument must be discovered and bound, so a future Quire parser improvement does not require a semantic change. | NFR-004-AC-3 | correct-requirement-no-evidence |
