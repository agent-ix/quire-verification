# E02 bounded compatibility evaluation

Date: 2026-09-07 PDT / 2026-09-08 UTC. Scope: private fixture planning, result
assessment, and bounded local catalog-adapter checks. No represented external
verification tool, parser, production system, fault injection, or human review
ran.

## Agreed fixture scope and controls

The admitted subject is a synthetic parser at revision `parser-r1`. The packet
uses pinned Quoin/catalog sources, private provisional identity/profile names,
synthetic bindings and native artifact references, a 60-second fixture budget,
and no credentials or retained user data. The acceptance threshold for this
slice was deterministic parity for every pinned method ID plus explicit plans
and result states for the required positive, adverse, missing-input, stale,
unsupported, untriggered, and observation-inadequate cases.

The healthy controls are the unconflicted 33-method export, ready state-property
and fuzz tasks, a ready manual inspection, and a fresh supported fixture result.
Adverse/refusal controls are a duplicate definition, unreadable catalog source,
unknown/not-applicable advice, stale policy, absent capability, provisional relational/trace
capabilities, missing fault authorization/injection/observation bindings,
failed/exhausted execution, wrong execution scope, stale evidence, missing
evidence, duplicate results for one exact run binding, untriggered recovery,
missing observations, simultaneous partial/assumption/logical failures, and a
logical violation. Local portfolio controls also include shuffled source order,
an adapted field corruption, and an adapter exception.

## Results

- All 33 existing method IDs and descriptive fields survive adaptation. The
  exact-source command reports 33 methods, zero duplicates, and zero unreadable
  roots at the pinned revisions.
- A duplicate or unreadable input preserves Quoin's active first-wins view for
  diagnosis while making the portable snapshot unqualified.
- The retained plan contains 14 tasks: six verification tasks and eight visible
  prerequisite tasks. It distinguishes all five input shapes. State property,
  fuzz, and manual tasks are ready; relational, trace, and fault tasks are
  blocked. The fault task cannot be invoked.
- The retained assessment accounts for all 12 declared obligations. Its expected
  states are one supported, one failed, one unsupported, one observation-
  inadequate, one untriggered, one stale, two missing (one absent and one scope
  mismatch), one incomplete, one partial-participation, one unmet-assumption,
  and one violated.
- Execution, logical outcome, activation, participation, assumptions, adequacy,
  freshness, provenance, and native artifact references remain independently
  inspectable. Exact plan/task/run/producer/adapter bindings are required,
  duplicate exact results are ambiguous, and every simultaneous independent
  finding is enumerated. No sufficiency decision or aggregate score is emitted.
- The draft 2020-12 schema is executed over all four JSON fixtures and 52
  embedded or adapted public contract instances. Runtime constants and
  validators for the five public contracts are derived from that schema;
  cross-task dependency graph checks remain explicit semantic validation.
- Digests and structured identity comparisons use RFC 8785 JCS. Catalog parity
  resolves methods by `techniqueId`, and injected violations or execution
  errors are retained as violated or failed `TechniqueResult` records.
- The retained local portfolio at source revision
  `f6b5587fa5e4f39dab6ee632f729906489e4fb2b` executed two complementary
  methods over the adapter: unit-style parity compared every preserved field
  across 33 definitions, and a metamorphic check compared the output after
  reversed and rotated input orders. Both local obligations are supported. Its
  required manual inspection is blocked on a verification owner and is recorded
  as `not-run`/incomplete rather than accepted.

## Measured effort

The one-time implementation and rewrite window was measured from the E02 claim
at `2026-09-08T00:41:35Z` through the retained Rust local portfolio at
`2026-09-08T06:11:18Z`: 5 hours 29 minutes 43 seconds. It includes the QUOIN
specification/review/plan cycle, the rejected prototype, full Rust replacement,
fixture and portfolio work, documentation, failure correction, Rust review, and
tests. It excludes earlier source reading and future owner disposition.

Repeatable local costs on this checkout were:

| Operation | Wall time | Maximum RSS |
|---|---:|---:|
| Complete Rust test suite (25 tests) | 0.55 s | 165,828 KiB |
| Release fixture validation (4 files, 52 contract instances) | 0.01 s | 9,676 KiB |
| Release plan/result fixture rendering | 0.01 s | 9,944 KiB |
| Release bounded local portfolio | 0.01 s | 9,940 KiB |
| Exact-source Quoin parity plus release Rust checker | 1.50 s | 295,468 KiB |

These are local workflow measurements, not delivery estimates or efficacy/ROI
claims. The test-suite row includes Cargo's test harness and compiler process;
the release-binary rows isolate the executable path. The exact-source check
includes loading Quoin through its installed TypeScript development tooling,
which dominates its memory use and remains outside this crate.

## Disposition and limitations

Agent B disposition: the bounded compatibility implementation in this pull
request is complete and produces useful gaps without changing catalog or
evidence ownership. This is not completion of parent E02 or its VP01–VP04
track. VP01 still needs its acceptance dependencies and reviewed migration;
VP02/VP03 still need the accepted shared identity/version seam and consumer
integration; VP03 still lacks correlated-oracle evidence; and VP04 still needs
a reviewed native parser or aggregate producer portfolio, a seeded domain
defect, agreed thresholds, and the recorded reviewer disposition. E01, FS02,
FS05, and E04 remain open.

Fixture and local portfolio success establish only the represented
adapter/planner/intake cases and the two named adapter properties. They do not
qualify methods outside that local subject, establish independence between
domain oracles, certify the parser, complete the required inspection, or
authorize production execution.
