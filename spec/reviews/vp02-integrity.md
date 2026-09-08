---
id: SR-032
title: "Integrity review of the VP02 native-planning specification"
type: SpecReview
analysis: integrity
scope: "quire-verification@f7a3dcf; VP02 trust and bypass boundaries"
review_set: all
---
# Integrity Review of the VP02 Native-Planning Specification

## Summary

Reviewed downgrade, bypass, identity-confusion, permission, evidence-ownership,
and untrusted-input paths.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | high | Until the shared owner adds B's JCS domain and exact role mappings, accepting PR #8 would require a digest-domain or kind downgrade. | FR-016-AC-5 |
| FND-002 | medium | The strict reader must reject duplicate JSON member names before materializing a map; schema validation over an already-collapsed value cannot satisfy this criterion. | FR-016-AC-3, TC-029 |
| FND-003 | low | Descriptive catalog text never confers capability, non-invocable tasks have null runner requests, and fault work requires explicit non-production authorization plus independent bindings. | FR-014-AC-1..3, TC-027 |
| FND-004 | low | Quoin and authored-human ownership are explicit; neither plan nor runner may create a second evidence store or sufficiency decision. | FR-014-AC-4 |
| FND-005 | medium | Resolved: shared plan-level prerequisites no longer borrow the first affected task's property or technique identity; `requiredBy` records every exact consumer deterministically. | FR-013-AC-4, TC-026 |
| FND-006 | medium | Resolved during re-review: composite task IDs now have an explicit injectivity requirement, preventing delimiter-based identity confusion in dependency and `requiredBy` edges. | FR-013-AC-5, TC-026 |

## Trust Boundaries

Raw JSON, shared schema/fixtures, local artifact bytes, capability declarations,
permissions, and availability claims are untrusted inputs. The Rust reader and
planner are trusted only after their exact versions and tests are qualified.
