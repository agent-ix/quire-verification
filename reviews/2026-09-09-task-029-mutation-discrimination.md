---
id: SR-217
title: "Task-029 local mutation discrimination review"
type: SpecReview
analysis: evidence
scope: "bfce783 plus Task-029 mutation manifest tests and eight serial cargo-mutants 27.1.0 runs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-025", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TC-042", type: reviews }
  - { target: "ix://agent-ix/quire-verification/Task-029", type: reviews }
---
# Task-029 Local Mutation Discrimination Review

## Summary

Eight qualification-critical Rust guards were mutated one at a time with the
catalog-declared `cargo-mutants` producer. Each run used one Cargo job, two jobserver
tasks, a skipped baseline, and exactly one assigned integration test. All eight final
runs were killed by the assigned test. These are local diagnostic runs; they are not
accepted qualification evidence because the Engineering Assurance typed producer
interface remains unpublished.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Open external dependency: cargo-mutants native outcomes cannot be promoted to qualification observations until Engineering Assurance #34 supplies the accepted typed execution/result boundary. | FR-025-AC-5, FR-025-CON-2, Task-029 |
| FND-002 | high | Resolved: replacing the inclusive JSON byte limit with an exclusive limit survived TC-023 and TC-029. Both tests now prove an exactly 16 MiB valid JSON value is accepted; rerunning each mutant kills it. | NFR-005-AC-1, FR-016-AC-3, TC-023, TC-029 |
| FND-003 | high | Resolved: bypassing native claim-authority validation survived because malformed cases were refused by schema or earlier reference gates. TC-039 now supplies every unrelated proof while omitting the exact native source-profile proof, and the assigned test kills the bypass. | FR-022-AC-2, TC-039 |
| FND-004 | high | Resolved: bypassing report core bindings survived the measurement-only test. A separate TC-037 test now changes a schema-valid retained task identifier and kills the bypass at the relational binding gate. | FR-021-AC-2, TC-037 |

## Final Mutant Matrix

| Mutant | Guard | Assigned test | Final result |
| --- | --- | --- | --- |
| `jcs-equality` | replace `contracts::jcs_equal` with `Ok(true)` | `tc_016_matches_rfc_8785_canonicalization` | killed |
| `bounded-json` | replace `>` with `>=` in `contracts::parse_bounded_json` | `tc_023_enforces_json_resource_limits` | killed |
| `strict-reference-json` | replace `>` with `>=` in `shared_reference::parse_strict_bounded_json` | `tc_029_accepts_exact_v2_roles_and_refuses_each_selected_failure` | killed |
| `native-response` | replace `native_process::parse_native_json_output` with `Ok(Default::default())` | `tc_038_returns_distinct_status_signal_encoding_and_protocol_failures` | killed |
| `lineage` | replace `evidence::validate_lineage` with `Ok(())` | `tc_033_rejects_duplicate_and_invalid_lineage` | killed |
| `claim-authority` | replace `qualification_profile::validate_claim_authority` with `Ok(())` | `tc_039_closes_nonformal_native_and_export_claim_authorities` | killed |
| `technique-coverage` | replace `qualification_technique::validate_coverage` with `Ok(vec![])` | `tc_042_refuses_conflicting_coverage_references_and_catalog_methods` | killed |
| `report-bindings` | replace `report::validate_core_bindings` with `Ok(())` | `tc_037_refuses_core_task_binding_drift` | killed |

The retained TC-042 mutation manifest binds the same eight guard identifiers to closed
fault-model and trace references, freezes their deterministic order, admits ordinals zero
through seven, and refuses ordinal eight. No source rewriter, runner, transcript parser,
or mutation evidence store was added.

## Qualification Boundary

The local runs establish that the assigned tests discriminate these generated faults in
this worktree. They do not establish candidate qualification, campaign completion, or an
Engineering Assurance observation. Task-029 remains blocked until the common Rust
producer executes and returns the same bindings through its accepted typed interface.
