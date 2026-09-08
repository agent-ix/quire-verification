# E02 provisional verification interfaces

Status: private reviewed implementation candidate, 2026-09-07. The interfaces
are executable in this repository but remain provisional across repositories
until E01, FS02, and FS05 supply the shared publication, identity, and version
contracts.

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
classification stays unknown.

`VerificationPlan` inventories properties before selecting tasks. Every task has
one of five input shapes: `state`, `relational`, `trace`, `fault`, or `manual`.
Applicability (`applicable`, `not-applicable`, `unknown`) and readiness (`ready`,
`blocked`, `unsupported`) are independent. Missing bindings, generators,
harnesses, tools, permissions, observations, or reviewer assignments remain
actionable prerequisites. Fault tasks cannot become ready without an explicit
non-production environment authorization and an injection binding.

`TechniqueResult` keeps execution, logical outcome, activation, participation,
assumptions, observation adequacy, exact scope, provenance, and native artifact
references as separate fields. The compatibility layer assesses results but does
not retain artifacts or decide whether the evidence is sufficient. Those remain
Quoin and human-owner responsibilities.

## Provisional shared identity seam

The slice uses an opaque `artifactRef` shape with authority, identity, revision,
digest, and profile fields. It compares these fields exactly and never derives a
new authored identity. FS02/FS05 must replace or confirm this seam with the
shared source/model/package identity contract before cross-repository acceptance.

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
bindings, dependency closure, and execution scope. Tool name and method ID alone
never match an obligation. A matching result can still be stale, failed,
incomplete, unsupported, untriggered, observation-inadequate, violated, or
supported. The inventory denominator includes properties with no result.

No global score, certification status, or inferred sufficiency is produced.

## Acceptance disposition

Promote the E02 interface shape for review because it preserves current method
identity, exercises all required subject/task forms, and makes the adverse and
incomplete states observable. Defer VP01 acceptance until E01 is reviewed and
the artifact license/publication packet is approved. Defer VP02 and VP03
acceptance until FS02 and FS05 confirm the identity/version seam. Defer VP04
execution claims until those predecessors are accepted and an engineering owner
agrees the bounded workflow and thresholds.
