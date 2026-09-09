---
id: SR-093
title: "Failure-domain review of shared-reference packet adoption"
type: SpecReview
analysis: failure-domain
scope: "quire-verification@7b6c11d; FR-016, IT-002, TC-029 and TC-031"
review_set: all
---
# Failure-Domain Review of Shared-Reference Packet Adoption

## Summary

The review checked parser, version dispatch, immutable identity, source-map,
migration, canonicalization, snapshot, and resource failures. The specification
keeps structural validity separate from semantic acceptance and does not convert
any refusal or exhaustion into a usable reference.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|----|----------|---------|------|--------------|
| FND-001 | high | Resolved in the reviewed revision: exact version/kind allocation prevents version-1 artifact/semantic or version-2 source-map/migration values from reaching a nearby decoder. | FR-016-AC-6, TC-029 | wrong-requirement |
| FND-002 | high | Duplicate decoded keys must be detected before host-map materialization, including escape-equivalent names; schema validation over a collapsed `serde_json::Value` cannot satisfy the requirement. | FR-016-AC-3, TC-029 | correct-requirement-no-evidence |
| FND-003 | medium | Digest mismatch, immutable identity conflict, stale selection, malformed source correspondence, and multiple applicable mappings have separate required outcomes and one-axis controls. | FR-016-AC-3, TC-029 | correct-requirement-no-evidence |
| FND-004 | medium | A changed, incomplete, or unproven packet snapshot must fail before interpretation; no network schema lookup or producer-code fallback is available. | FR-016-AC-7, FR-016-CON-2, TC-031 | correct-requirement-no-evidence |
