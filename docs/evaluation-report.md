# E02 bounded compatibility evaluation

Date: 2026-09-07 PDT / 2026-09-08 UTC. Scope: private fixture planning and
result assessment only. No represented external verification tool, production
system, or fault injection ran.

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
unknown applicability, absent capability, provisional relational/trace
capabilities, missing fault authorization/injection/observation bindings,
failed/exhausted execution, wrong execution scope, stale evidence, missing
evidence, untriggered recovery, missing observations, and a logical violation.

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
- The retained assessment accounts for all 10 declared obligations. Its expected
  states are one supported, one failed, one unsupported, one observation-
  inadequate, one untriggered, one stale, two missing (one absent and one scope
  mismatch), one incomplete, and one violated.
- Execution, logical outcome, activation, participation, assumptions, adequacy,
  freshness, provenance, and native artifact references remain independently
  inspectable. No sufficiency decision or aggregate score is emitted.

## Measured effort

The one-time implementation window was measured from the E02 claim at
`2026-09-08T00:41:35Z` through the measurement at `2026-09-08T01:10:31Z`:
28 minutes 56 seconds. It includes contract/adapter/planner/result code, fixture
construction, documentation, failure correction, and tests after the claim. It
excludes earlier source reading and future owner review.

Repeatable local costs on this checkout were:

| Operation | Wall time | Maximum RSS |
|---|---:|---:|
| Six-file dependency-free test suite | 0.20 s | 64,488 KiB |
| Plan/result fixture rendering | 0.03 s | 48,384 KiB |
| Exact-source Quoin parity check | 1.49 s | 292,132 KiB |

These are local workflow measurements, not delivery estimates or efficacy/ROI
claims. The exact-source check includes loading Quoin through its installed
TypeScript development tooling, which dominates its memory use.

## Disposition and limitations

Agent B disposition: promote the E02 compatibility boundary for owner/shared-
interface review. It produces useful gaps without changing catalog or evidence
ownership. Defer VP01 acceptance pending E01 and artifact-publication review;
defer VP02/VP03 acceptance pending FS02/FS05; defer VP04 execution acceptance
pending those predecessors, an engineering owner, agreed thresholds, and actual
native method runs. External/domain reviewer disposition is pending and is not
invented here.

Fixture success establishes only the represented adapter/planner/intake cases.
It does not qualify property, fuzz, metamorphic, monitoring, fault, or inspection
tools; establish independence between their oracles; certify the parser; or
authorize production execution.
