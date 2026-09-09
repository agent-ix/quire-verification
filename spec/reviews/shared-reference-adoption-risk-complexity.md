---
id: SR-097
title: "Risk and complexity review of shared-reference packet adoption"
type: SpecReview
analysis: risk-complexity
scope: "quire-verification@7b6c11d; FR-016, IT-002, TC-029 and TC-031"
review_set: all
---
# Risk and Complexity Review of Shared-Reference Packet Adoption

## Summary

FR-016 has high technical risk because raw parsing, schema dispatch,
cryptographic identities, local byte correspondence, source maps, and explicit
migration meet at one trust boundary. Volatility is medium because the selected
interfaces remain draft, while exact revision pinning and no fallback make this
implementation slice stable.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|----|----------|---------|------|--------------|
| FND-001 | high | A permissive or post-collapse JSON parser could accept ambiguous identities. Pre-map duplicate detection, bounded intake, closed schema validation, and negative/property cases are the required mitigation. | FR-016-AC-3, TC-029 | correct-requirement-no-evidence |
| FND-002 | high | Confusing byte digests with the selected JCS domain or accepting a nearby role can join unrelated evidence. Exact domain syntax, role selection, local byte verification, and one-axis mutations mitigate the risk. | FR-016-AC-1, FR-016-AC-5, TC-029 | correct-requirement-no-evidence |
| FND-003 | medium | A future draft can change wire meaning. The exact merged revision, snapshot manifest, explicit kind/version dispatch, and prohibition on inferred migration contain that volatility. | FR-016-AC-6..7, TC-031 | correct-requirement-no-evidence |
| FND-004 | medium | Public terms for the reusable packet are unsettled. Keeping the snapshot private and data-only avoids turning implementation qualification into publication. | FR-016-CON-2 | correct-requirement-no-evidence |
