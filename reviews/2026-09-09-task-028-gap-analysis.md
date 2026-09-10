---
id: SR-206
title: "Gap analysis of Plan-006 Task-028 portable checkpoint"
type: SpecReview
analysis: gap-analysis
scope: "plan/Plan-006-quire-spec-production-qualification/tasks/Task-028-property-differential-evidence.md, spec/tests.md, spec/functional/FR-022-freeze-qualification-claim.md, spec/functional/FR-025-run-qualification-techniques.md, src/qualification_profile.rs, src/qualification_technique.rs, tests/qualification_profile.rs, tests/qualification_technique.rs, tests/props_fr_025.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/Plan-006", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TM-001", type: references }
  - { target: "ix://agent-ix/quire-verification/Task-028", type: reviews }
---
# Gap Analysis of Plan-006 Task-028 Portable Checkpoint

## Summary

The implemented portable Task-028 slice is specified, schema-derived, Rust-reviewed,
locally green, and fully owned by FR-022/FR-025. The task and Plan-006 remain incomplete:
four FR-025 criteria and three Task-028 execution/result/replay subtasks await explicit
external dependencies, and no production formal-clause admission is claimed.

## Verdict

**FAIL — expected blocked checkpoint.** Task-028 has `status: blocked`, FR-025 is 1/5
backed, and the exact missing producer/catalog/admission evidence remains open. This is
not a failure of the implemented portable code and is not permission to add local
substitutes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Task-028 is blocked with three unchecked execution/result/replay subtasks because Engineering Assurance #34 has not published its accepted Rust producer interface and Task-029 owns final fuzz/mutation completion. | Task-028, engineering-assurance#34, Task-029 |
| FND-002 | high | FR-025-AC-2 through AC-5 remain unbacked. AC2 awaits the accepted `differential-testing` catalog entry; AC3 and AC4 await Engineering Assurance typed results/replay; AC5 belongs to Task-029. | FR-025-AC-2..FR-025-AC-5, spec-artifacts-process#86 |
| FND-003 | high | No accepted native Quire profile has discharged all ten carried-forward admission rules. The contract now reports each missing rule as an owned gap and refuses a formal configuration, so production formal-clause coverage remains unclaimed. | FR-022-AC-5, FR-025-AC-1, quire-contract-ir#53 |
| FND-004 | medium | The opaque dependency-closure reference cannot prove absence of forbidden foreign runtimes. Engineering Assurance #34 and quire-research#64 must provide accepted reusable evidence; no repository-local scanner exists. | FR-025-CON-2, Task-028 |
| FND-005 | low | Bare `quire coverage` still surfaces one known vendored `ix-trace-rs` symbol because the repository module cannot run alone; all 101 discovered Rust candidates are tagged and bound and no first-party Task-028 symbol is untracked. | `.ix/modules/quire-verification/manifest.yaml`, contracts/README.md |

## Coverage

- Quire CLI: 0.31.0, engine `0.46.0@ca7362d4`.
- Matrix reconciliation: 182/210 rows backed; zero status lies.
- FR-025 acceptance criteria: 1/5 backed. FR-025-AC-1 is backed by the generated
  `proptest`; AC2 through AC5 are deliberately unbacked and recorded in SR-196.
- TC-042 has six portable Rust test symbols plus one generated property symbol, while its
  matrix status remains Planned because the full procedure is not executable.
- Rust binding census: 101/101 discovered candidates tagged and bound; 93/94 self-named
  candidates bind by name, with the generated underscored property name bound through its
  compiler-checked attribute.
- Reverse ownership: `src/qualification_profile.rs` is owned by FR-022 and
  `src/qualification_technique.rs` by FR-025. The schema additions, stable error codes,
  public frozen types, normalization, and case planner each have an owning requirement.
- Stub inspection found no `todo!`, `unimplemented!`, debug output, placeholder success,
  production panic, unsafe block, local process runner, evidence store, or foreign-runtime
  adapter in the reviewed slice.
- The optional intent-to-test-to-code semantic pass was skipped. QUOIN
  `/spec-correctness`, the all-lens specification review, and the exact Rust review each
  performed their required targeted semantic checks.
