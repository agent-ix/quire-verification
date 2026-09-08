# Quoin catalog and evidence compatibility

## Inspected baseline

- Quoin `7072d65db7e236e819d99be42e094ba548e9496a`:
  `src/advisor/methods.ts`, `src/advisor/advise.ts`,
  `src/auditor/combinatorial.ts`, and `src/evidence/**`.
- spec-artifacts-process
  `61a20e010d5e758f52864ad3152ccdb304a39d27`:
  `spec_artifacts_process/manifest.yaml`.
- quire-contract-ir `decc99a430a0894de489102dbab04e83d1fb804f` and
  live issue #58 for formalization coverage-state ownership.

The captured Quoin export contains 33 method IDs, sorted by bytewise ID, with no
duplicate or unreadable module at the pinned inputs. Quoin merges method IDs
first-wins, reports duplicates and unreadable roots, treats applicability axes
as open, recommends when any observable rule value matches, orders advice by
descending reason count then bytewise ID, and distinguishes mismatch,
uncatalogued vocabulary, and inconclusive advice.

## Preserved behavior

| Existing behavior | Compatibility action |
|---|---|
| Method ID is the catalog identity | `techniqueId` is byte-identical; the test compares all 33 IDs. |
| Active merge is first-wins | The active exported method remains visible; duplicate diagnostics make the portable snapshot unqualified. |
| Missing/unreadable modules are reported | Diagnostics survive adaptation and prevent a completeness claim. |
| Applicability is rule/value matching | Reasons are preserved verbatim; positive matches become `applicable`. |
| Inconclusive means no rule matched | It becomes `unknown`, never `not-applicable`. |
| Advice order is deterministic | Input order and adapted ID order are tested bytewise. |
| `tooling` is descriptive catalog data | Values become legacy tooling hints, never installed-capability claims. |
| Evidence adapters emit native run/finding data | Results retain native artifact references; this repository stores none. |
| Quoin retains evidence and human sufficiency | This repository returns assessment views only. |
| Combinatorial analysis computes gaps | No covering-array generator is inferred or implemented here. |
| Cargo-mutants intake aggregates tool output | No individual-mutant-to-requirement attribution is inferred. |

## Actual gap

The existing catalog has no portable, independently versioned capability,
policy, plan, or result contract. Advice does not resolve harnesses, bindings,
permissions, runner limits, task dependencies, result scope, or freshness. The
implemented adapter adds those boundaries without relocating catalog data or
changing Quoin execution/evidence ownership.

Quoin's current `RunEntry.outcome` is limited to pass/fail/skip/error, while its
adapter contract separately reports producer states that cannot be represented.
Its `RunRecord` pins suite, commit, tool, evidence kind, timestamp, and entries;
it does not carry the full E02 property/profile/binding/dependency/scope,
plan/task/run/producer/adapter binding, or activation/adequacy tuple. VP03
therefore needs an additive intake/envelope or linked result reference owned by
Quoin. Mapping unsupported, exhausted,
untriggered, or observation-inadequate records onto the nearest `RunEntry`
outcome would lose information and is explicitly refused by this slice.

Portable intake now requires one exact result binding. Multiple results for the
same plan/task/run/producer/adapter tuple are reported as ambiguous. A
compatibility summary status remains for current consumers, while the gap list
enumerates all simultaneous execution, activation, participation, assumption,
adequacy, and logical findings. The `runRef` role and version domains remain
provisional pending FS02/FS05; this repository does not invent a Quoin-side run
identity.

The current cargo-mutants adapter aggregates caught/missed/timeout outcomes per
production function, excludes unviable mutants, and intentionally binds no
requirement without a requirement-to-production-code relation. E02 preserves
that limit; it does not invent individual-mutant attribution.

## Existing owners and amendments

| Delta | Existing owner | Verification-side request |
|---|---|---|
| Catalog/module/package locking | quoin #287 | Additive export of exact module/source revision and digest; retain installation behavior. |
| Evidence discharge/inward completeness | quoin #272 | Accept exact scoped result references; preserve live obligation derivation and human dispositions. |
| Coverage states and clause addresses | quire-contract-ir #58 | Align missing/incomplete/unsupported states and property identity; do not collapse result dimensions. |
| Shared model/source identity | FS02 | Confirm the opaque identity seam and dependency/binding digest domains. |
| Version/canonical compatibility | FS05 | Confirm independent profile/version axes and historical evidence behavior. |

Quoin #272 still names quire-rs #373 and #374 as blockers. Both endpoints return
HTTP 410 (`deleted`) on 2026-09-07. This is a stale dependency requiring owner
retargeting under CO01; deletion is not evidence that the underlying work is
complete or unnecessary.

No public issue amendment is made from this private packet. A later coordinated
writer can prepare a narrowly scoped public-capable change without private
roadmap rationale after E01 review.
