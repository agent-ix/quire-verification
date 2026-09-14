# E02 accepted cross-repository verification interface

Status: accepted private cross-repository Rust interface, 2026-09-14. The
accepted owner is `agent-ix/quire-verification`; the frozen interface is
`quire-verification/e02-draft-2` at repository revision
`d2fc0d8134c776d2f1a17fd249f016b18a6ba124`. This acceptance authorizes exact
Rust consumers only. It does not publish a normative schema, accept a
represented verification tool, or make an evidence-sufficiency decision.

## Decisions

`TechniqueDefinition` preserves an existing Quoin method ID and its descriptive,
class, applicability, evidence-kind, and tooling-hint fields. A definition says
what a method can mean and where it may apply. It does not claim that a tool is
installed or qualified.

`ToolCapability` is separately identified and binds one technique to a native
producer/adapter, supported input kinds and profiles, explicit prerequisites,
limits, qualification state, and claim limits. It never stores native results.

`SelectionPolicy` contains explainable rules with source provenance. A rule has
an authored priority (`required`, `recommended`, or `optional`) rather than a
calculated strength score. Standards-derived rules require an exact edition,
objective, interpretation, and distribution-rights disposition. Unknown
classification stays unknown. An expired policy adds a forced freshness
prerequisite and cannot authorize invocation merely because a caller labels it
available.

`VerificationPlan` inventories properties before selecting tasks. Every task has
one of five input shapes: `state`, `relational`, `trace`, `fault`, or `manual`.
Applicability (`applicable`, `not-applicable`, `unknown`) and readiness (`ready`,
`blocked`, `unsupported`) are independent. Missing bindings, generators,
harnesses, tools, permissions, observations, or reviewer assignments remain
actionable prerequisites. Fault tasks cannot become ready without an explicit
non-production environment authorization and an injection binding.

`TechniqueResult` keeps execution, logical outcome, activation, participation,
assumptions, observation adequacy, exact scope, provenance, and native artifact
references as separate fields. Each evaluated result also retains one or more
closed oracle contributions that identify its oracle and observation and state
whether their relationship is independent, shared, derived, or unknown. The
compatibility layer validates contribution identity and lineage, reports
dependence among uniquely selected results, and assesses results. It does not
retain referenced artifacts or decide whether the evidence is sufficient.
Those remain Quoin and human-owner responsibilities.

## Accepted shared identity seam

The slice uses an opaque `artifactRef` shape with authority, identity, revision,
digest, and profile fields. It compares these fields exactly and never derives a
new authored identity. The exact retained FS02/FS05 packet at
`agent-ix/quire-specification@8ab058beff03cc76c0f39cc06c87ef98f76e8d78`
confirms this seam for private cross-repository consumption. A consumer must
use the strict reader and must not normalize, upgrade, downgrade, or replace
the selected identities.

The verification side requires the shared contract to distinguish:

- authored source identity and source revision from semantic property identity;
- model/package identity from a runtime object population or execution run;
- language edition, semantic profile, wire/canonicalization version, backend
  capability, and package release;
- immutable dependency-closure and binding digests from display names;
- generated lineage from adopted authored identity; and
- current/pre/post snapshots belonging to one invocation, including creation
  and deletion.

Readers must refuse unknown required features, changed dependencies, ambiguous
or foreign authorities, and malformed source correspondence. Version migration
must record explicit equivalence and must not rewrite historical evidence.

## Applicability and readiness

Quoin advice proves a positive applicability match when it supplies rule/value
reasons. Quoin's `inconclusive` state cannot prove non-applicability; the adapter
maps it to `unknown`. A future explicit exclusion rule may establish
`not-applicable`. An absent or unsupported capability changes readiness only.

The adapter preserves Quoin's first-wins active method and deterministic order
for compatibility. A duplicate or unreadable source makes a portable snapshot
unqualified, because silently freezing a partial or conflicted catalog would
turn an operational diagnostic into a new authority.

## Result matching and freshness

Evidence joins require exact property, subject revision, semantic profile,
bindings, dependency closure, execution scope, plan, task, run, producer and
adapter identities and versions. Tool name and method ID alone never match an
obligation. More than one result for the same exact binding is ambiguous and is
refused rather than resolved by timestamp or input order.

`PropertyInventoryItem` is the pre-run property shape used by a plan.
Its non-empty authored classification is preserved; an omitted classification
is emitted as `unknown`, and an explicitly empty value is rejected.
`EvidenceObligation` adds the selected `techniqueId` and required
`resultBinding` for intake assessment; this prevents a plan property with
several selected methods from pretending it already identifies one result.

A matching result can still be stale, failed, incomplete, partially
participating, assumption-invalid, unsupported, untriggered,
observation-inadequate, violated, or supported. The single `status` field is a
compatibility summary only. `gaps` enumerates every applicable independent
dimension, so a partial, assumption-invalid, logically violated result exposes
all three findings. The inventory denominator includes properties with no
result.

No global score, certification status, or inferred sufficiency is produced.

## Acceptance disposition

The E02 interface is accepted for exact private cross-repository Rust use. Its
strict reader rejects unknown contract versions, unsupported shared-reference
versions, wrong roles, identity or content drift, ambiguous equivalence, and
any `TechniqueResult` projection that omits a required result dimension. These
refusals occur before planning, matching, or assessment and never produce a
replacement result.

VP01, VP03, VP04, public schema publication, reusable-artifact terms, and any
evidence-sufficiency or external-tool claim remain separately gated. This
acceptance neither promotes them nor creates a compatibility path for draft-1.
