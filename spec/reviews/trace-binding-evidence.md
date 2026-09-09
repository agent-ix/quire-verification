---
id: SR-085
title: "Evidence review of Quire-readable Rust trace bindings"
type: SpecReview
analysis: evidence
scope: "quire-verification@41f7a57; NFR-004-AC-3, NFR-004-M-4, TC-022"
review_set: all
---
# Evidence Review: Quire-readable Rust Trace Bindings

## Summary

The mandatory QUOIN advisor and the concrete failure evidence were reviewed.
The advisor recommends property testing for the universal AC and does not catalog
the coverage-inspection metric; the authored static census is retained because
the required evidence is the complete repository binding report itself.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|----|----------|---------|------|--------------|
| FND-001 | low | QUOIN marks NFR-004-AC-3's authored Inspection as a mismatch against generic property-based-testing advice. Inspection is retained because only the full Quire binding census can prove that every actual acceptance marker argument is discoverable; generated marker samples would not prove the repository population. | NFR-004-AC-3, TC-022 | correct-requirement-no-evidence |
| FND-002 | low | QUOIN marks NFR-004-M-4's “Coverage inspection” method uncatalogued. It is retained as the exact command-backed method and paired with Rust compilation; no advisor entry replaces either half of the required cross-reader evidence. | NFR-004-M-4, TC-022 | correct-requirement-no-evidence |
| FND-003 | medium | The current 107/159 coverage report is discriminating evidence: NFR-006-AC-2 and AC-3 are false while the multiline marker compiles. Implementation acceptance therefore requires those exact targets to become true without hiding other unbacked rows. | NFR-006-AC-2, NFR-006-AC-3, TC-038 | implementation-bug-despite-evidence |
