---
id: FR-024
title: "Bind independent qualification oracles"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-004"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-022"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-023"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-017"
    type: "depends_on"
  - target: "ix://agent-ix/engineering-assurance/FR-014"
    type: "depends_on"
  - target: "ix://agent-ix/engineering-assurance/NFR-004"
    type: "depends_on"
---
# FR-024: Bind independent qualification oracles

## Description

For every guarantee in one frozen qualification profile, the oracle planner SHALL
freeze exactly one closed manifest that names one or more executable oracle bindings or
exactly one owned no-oracle gap. Each binding SHALL identify its authority,
implementation, revision, corpus input projection, expected relation, dependence class,
limitations, bounded invocation, and disagreement procedure before execution.

## Inputs

- One frozen `QualificationProfile` and its identity.
- One frozen `QualificationCorpusManifest`, its identity, and its retained entry bytes.
- A candidate `QualificationOracleManifest` containing closed binding and gap records.
- Opaque strict-reader outputs for every oracle definition, oracle executable projection,
  comparison executable projection, Engineering Assurance execution request, comparison
  request, disagreement procedure, run, and retained observation reference.
- An accepted Rust Engineering Assurance interface in the frozen profile and structured
  results produced by that exact interface revision.

## Outputs

- One immutable schema-derived oracle manifest with an RFC 8785 identity.
- One validated projection of each Engineering Assurance execution/comparison result per
  binding and corpus projection, including both exact observations when comparison is
  possible.
- Explicit owned no-oracle, dependence, execution, or disagreement findings; none is a
  qualification disposition.

## Behavior

- `QualificationOracleManifest` SHALL carry `contractVersion`, `schemaVersion`, `kind`,
  `manifestId`, `profileIdentity`, `corpusIdentity`, `bindings`, and `gaps`.
- The planner SHALL refuse a duplicate binding or gap identifier.
- The planner SHALL require every frozen guarantee identifier to occur in at least one
  binding or exactly one gap, never both.
- Each binding SHALL carry `bindingId`, `guaranteeId`, `oracleRef`, `executableRef`,
  `comparisonProcedureRef`, `comparisonExecutableRef`, a closed `inputProjection`, a
  closed `expectedRelation`, a closed `dependence`, nonempty unique `limitations`,
  `disagreementProcedureRef`, and exact `executionRequestRef` and `comparisonRequestRef`
  invocation references produced under the accepted Engineering Assurance protocol.
- `inputProjection` SHALL carry `mode` `each-entry` or `ordered-batch` and a nonempty
  unique ordered `entryIds` array.
- `expectedRelation` SHALL carry nonempty `relationId`, `description`,
  `expectedOutcome` `agreed`, and `origin` `oracle`, `independent-third-party`,
  `candidate`, `shared`, or `unknown`; the exact `comparisonProcedureRef` owns the
  predicate semantics.
- `dependence` SHALL be exactly one closed tagged form: `independent` with a nonempty
  unique array of strict-reader-proved source `evidenceRefs` plus nonempty unique and
  disjoint `candidateAncestryRefs` and `oracleAncestryRefs`; `shared` with nonempty
  `groupId` and nonempty unique `bases`; `derived` with one `sourceBindingId`; or
  `unknown` with one nonempty `reason`.
- A shared `basis` SHALL be one of `candidate`, `oracle`, `executable`,
  `comparison-procedure`, `expected-value`, or `implementation-ancestry`.
- Each no-oracle gap SHALL carry `gapId`, `guaranteeId`, `owner`, nonempty `reason`, and
  nonempty `limitations`.
- The planner SHALL refuse a guarantee that has both a gap and an executable binding.
- The planner SHALL require every projected `entryId` to name a retained corpus entry.
- The planner SHALL bind the exact ordered entry identifiers into the projection.
- An `ordered-batch` execution request SHALL bind one input role `entry:<entryId>` for
  every projected entry in declared order. An `each-entry` request SHALL bind exactly
  one such role naming a member of the declared projection.
- The planner SHALL require every bound input digest to identify the
  strict-reader-retained bytes for that corpus entry.
- The planner SHALL refuse a rights-limited or empty corpus gap as an oracle input.
- The planner SHALL validate every artifact reference through the strict shared-reference
  reader with its declared role.
- The planner SHALL require the accepted Engineering Assurance interface, execution
  request, returned run, and observation to agree on exact executable and request
  identities before the result can contribute.
- The planner SHALL distinguish `independent`, `shared`, `derived`, and `unknown`
  dependence using [FR-017](./FR-017-retain-oracle-dependence.md).
- The planner SHALL require every derived source to resolve to exactly one other binding.
- The planner SHALL reject self-derived, absent-source, or cyclic binding ancestry by an
  iterative traversal within the repository input bounds.
- The planner SHALL forbid fields from another dependence form.
- The planner SHALL refuse `independent` when the candidate and oracle ancestry sets
  intersect or the expected-value origin is `candidate`, `shared`, or `unknown`.
- A `candidate` or `shared` expected-value origin SHALL require `shared` dependence with
  basis `expected-value`.
- An oracle definition, executable, comparison procedure, observation, or expected value
  produced from the candidate or sharing an exact reference SHALL NOT be classified as
  independent. An `independent` declaration without positive exact evidence SHALL be
  refused; q-v SHALL NOT infer undisclosed causal independence from unequal identifiers.
- `quire-verification` SHALL NOT implement process supervision, spawn a process outside
  the Engineering Assurance interface, materialize an evidence store, or interpret
  arbitrary process output.
- The `quire-verification` consumer SHALL submit the exact invocation through the
  accepted Rust Engineering Assurance producer-execution interface, which owns bounded
  host execution and evidence production.
- The planner SHALL consume only typed result values defined and validated by the
  accepted Rust Engineering Assurance library interface; it SHALL NOT parse a producer's
  stdout as an assurance result.
- The caller-owned response adapter SHALL accept only a schema-derived
  `QualificationOracleProducerResponse` or `QualificationComparisonProducerResponse`
  whose binding, guarantee, corpus projection, relation, and observation references
  match the selected binding; every foreign or missing field SHALL produce the
  Engineering Assurance `malformed_response` state.
- A producer response SHALL NOT carry the frozen manifest identity because the manifest
  already binds the canonical execution request; the consumer SHALL add the manifest
  identity only after validating that request reference, avoiding a recursive identity.
- The Engineering Assurance interface, request, result,
  executable, and retained observation references SHALL all pass the strict reader
  before a completed result contributes.
- The accepted Engineering Assurance result SHALL prove direct bounded execution with no
  shell, inherited environment, unbounded stream/deadline, or lost containment failure.
- The planner SHALL preserve unavailable, refused, failed, timed-out,
  malformed-response, containment-failure, and cancelled Engineering Assurance states,
  including their closed reason categories where present, as typed execution findings
  rather than fabricated observations.
- A schema-derived `QualificationOracleExecutionRecord` SHALL bind `contractVersion`,
  `schemaVersion`, `kind`, `manifestIdentity`, `bindingId`, `guaranteeId`, the exact
  executed corpus projection, `engineeringAssuranceRevision`, `executionProtocol`,
  `requestIdentity`, `resultIdentity`, `resultRef`, and one closed execution state.
- A completed execution record SHALL carry exactly one strict-reader-proved
  `observationRef`.
- A non-completed execution record SHALL carry no observation reference and preserve
  the exact Engineering Assurance state and refusal or failure reason.
- A comparison record SHALL bind the manifest, binding, guarantee, corpus projection,
  Engineering Assurance revision, comparison request and result identities, result
  reference, expected relation, candidate observation reference, oracle observation
  reference, and outcome `agreed`, `disagreed`, or `unresolved`.
- A `disagreed` comparison SHALL retain both exact observations and carry resolution
  `unresolved`.
- A separate result from the exact declared disagreement procedure MAY attribute or
  bound the difference, but SHALL NOT issue the qualification disposition owned by
  FR-027.
- When supplied, a schema-derived `QualificationOracleDisagreementRecord` SHALL bind
  the manifest, binding, guarantee, exact comparison identity, declared disagreement
  procedure reference, strict-reader-proved procedure-result reference, and nonempty
  attribution or bound. A foreign procedure reference or a disposition field SHALL be
  refused.
- Agreement SHALL support only the exact relation named by the binding.
- Agreement SHALL NOT imply broader semantic correctness, oracle independence, evidence
  sufficiency, or qualification acceptance.
- If no qualified oracle exists for a guarantee, the planner SHALL retain the owned gap
  rather than substitute a self-consistency check or drop the guarantee.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-024-CON-1 | Oracle and comparison execution SHALL occur only through the accepted versioned Rust Engineering Assurance producer-execution interface. | Security | Test (TC-041) |
| FR-024-CON-2 | The repository SHALL return schema-derived manifests, references, observations, and findings to the caller; Quoin remains evidence owner and Engineering Assurance remains assurance-execution owner. | Ownership | Inspection (TC-041) |
| FR-024-CON-3 | Oracle planning and comparison SHALL be deterministic under binding, gap, proof, and observation input permutations. | Reproducibility | Test (TC-041) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-024-AC-1 | The frozen manifest binds the exact profile and corpus identities and covers every frozen guarantee with one or more closed oracle bindings or exactly one owned no-oracle gap with nonempty limitations; missing, duplicate, unknown, simultaneously bound-and-gapped, or unowned guarantees refuse the manifest. | Test (TC-041) |
| FR-024-AC-2 | Independent, shared, derived, and unknown bindings retain exactly their tagged dependence data; a self-produced or unknown-origin expected value, intersecting ancestry, shared exact reference, absent shared group/basis, missing independence evidence, absent/self/cyclic derived source, foreign tagged field, or caller-only independence assertion cannot produce an independent contribution. | Test (TC-041) |
| FR-024-AC-3 | An execution or comparison result contributes only after the strict-reader-proved Engineering Assurance interface, request identity, result identity, strict-reader-proved executable, and retained corpus projection match. Every non-completed Engineering Assurance state yields a typed execution finding. Agreement supports only the declared relation, and disagreement retains both exact observations with an unresolved outcome while only the exact disagreement procedure may add attribution or bounds. | Test (TC-041) |
| FR-024-AC-4 | Changing any profile, corpus, guarantee, oracle, executable, Engineering Assurance interface/request/result, comparison procedure, disagreement procedure, projection, input entry, expected relation, dependence, limitation, observation, or prior manifest identity refuses reuse; permuting set-like inputs leaves the JCS manifest or comparison identity unchanged. | Test (TC-041) |

## Error Conditions

| Code | Condition | Result |
|------|-----------|--------|
| `invalid_qualification_oracle_manifest` | The manifest is structurally invalid, incompletely covers guarantees, conflicts with the frozen profile/corpus, projects an unretained entry, or carries inconsistent dependence lineage. | Refuse freezing and return no partial manifest. |
| `invalid_qualification_oracle_binding` | A strict reference, Engineering Assurance interface/request/result, executable digest, projection, expected relation, limitation, observation, or reuse identity differs from the frozen binding. | Refuse that result or comparison and return no fabricated observation. |
| Engineering Assurance execution state | The accepted Rust producer reports unavailable, refused, failed, timed out, malformed response, containment failure, or cancellation. | Preserve that exact typed state as an execution finding; do not convert it to agreement or disagreement. |

## Dependencies

Oracle obligations consume [FR-022](./FR-022-freeze-qualification-claim.md),
[FR-023](./FR-023-retain-qualification-corpus.md), and the existing dependence
model in [FR-017](./FR-017-retain-oracle-dependence.md). Host execution and evidence
production depend on the accepted versioned Rust interface owned by Engineering
Assurance issue `agent-ix/engineering-assurance#34`; absence remains an explicit
producer-owned blocker and does not authorize a local implementation.
