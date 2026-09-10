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
- Opaque strict-reader outputs for every oracle definition, executable projection,
  Engineering Assurance execution request, comparison request, disagreement procedure,
  run, and retained observation reference.
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
  `comparisonProcedureRef`, a closed `inputProjection`, a closed `expectedRelation`, a
  closed `dependence`, nonempty unique `limitations`, `disagreementProcedureRef`, and
  exact `executionRequestRef` and `comparisonRequestRef` invocation references produced
  under the accepted Engineering Assurance protocol.
- `inputProjection` SHALL carry `mode` `each-entry` or `ordered-batch` and a nonempty
  unique ordered `entryIds` array.
- `expectedRelation` SHALL carry nonempty `relationId`, `description`, and
  `expectedOutcome` `agreed`; the exact `comparisonProcedureRef` owns the predicate
  semantics.
- `dependence` SHALL be exactly one closed tagged form: `independent` with a nonempty
  unique array of strict-reader-proved source `evidenceRefs`; `shared` with nonempty
  `groupId` and nonempty unique `bases`; `derived` with one `sourceBindingId`; or
  `unknown` with one nonempty `reason`.
- A shared `basis` SHALL be one of `candidate`, `oracle`, `executable`,
  `comparison-procedure`, `expected-value`, or `implementation-ancestry`.
- Each no-oracle gap SHALL carry `gapId`, `guaranteeId`, `owner`, nonempty `reason`, and
  nonempty `limitations`.
- The planner SHALL refuse a guarantee that has both a gap and an executable binding.
- The planner SHALL require every projected `entryId` to name a retained corpus entry.
- The planner SHALL bind the exact ordered entry identifiers into the projection.
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
- An oracle definition, executable, comparison procedure, observation, or expected value
  produced from the candidate or sharing an exact reference SHALL NOT be classified as
  independent. An `independent` declaration without positive exact evidence SHALL remain
  unknown; q-v SHALL NOT infer undisclosed causal independence from unequal identifiers.
- `quire-verification` SHALL NOT execute the oracle, spawn a process, materialize an
  evidence store, or interpret arbitrary process output.
- The caller SHALL submit the exact invocation through the accepted Rust Engineering
  Assurance producer-execution interface, which owns bounded host execution and evidence
  production.
- The planner SHALL consume only typed result values defined and validated by the
  accepted Rust Engineering Assurance library interface; it SHALL NOT parse a producer's
  stdout as an assurance result.
- The accepted Engineering Assurance result SHALL prove direct bounded execution with no
  shell, inherited environment, unbounded stream/deadline, or lost containment failure.
- The planner SHALL preserve unavailable, refused, failed, timed-out, and
  malformed-response Engineering Assurance states as typed execution findings rather than
  fabricated observations.
- A comparison record SHALL bind the manifest, binding, guarantee, corpus projection,
  expected relation, candidate observation reference, oracle observation reference, and
  outcome `agreed`, `disagreed`, or `unresolved`.
- A `disagreed` comparison SHALL retain both exact observations and remain unresolved.
- A separate result from the exact declared disagreement procedure MAY attribute or
  bound the difference, but SHALL NOT issue the qualification disposition owned by
  FR-027.
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
| FR-024-AC-2 | Independent, shared, derived, and unknown bindings retain exactly their tagged dependence data; a self-produced expected value, shared exact reference, absent shared group/basis, missing independence evidence, absent/self/cyclic derived source, foreign tagged field, or caller-only independence assertion cannot produce an independent contribution. | Test (TC-041) |
| FR-024-AC-3 | An execution or comparison result contributes only after the accepted Engineering Assurance interface revision, request identity, strict-reader-proved executable, and retained corpus projection match. Agreement supports only the declared relation; an Engineering Assurance failure yields a typed execution finding, and disagreement retains both exact observations with an unresolved outcome while only the exact disagreement procedure may add attribution or bounds. | Test (TC-041) |
| FR-024-AC-4 | Changing any profile, corpus, guarantee, oracle, executable, Engineering Assurance interface/request/result, comparison procedure, disagreement procedure, projection, input entry, expected relation, dependence, limitation, observation, or prior manifest identity refuses reuse; permuting set-like inputs leaves the JCS manifest or comparison identity unchanged. | Test (TC-041) |

## Error Conditions

| Code | Condition | Result |
|------|-----------|--------|
| `invalid_qualification_oracle_manifest` | The manifest is structurally invalid, incompletely covers guarantees, conflicts with the frozen profile/corpus, projects an unretained entry, or carries inconsistent dependence lineage. | Refuse freezing and return no partial manifest. |
| `invalid_qualification_oracle_binding` | A strict reference, Engineering Assurance interface/request/result, executable digest, projection, expected relation, limitation, observation, or reuse identity differs from the frozen binding. | Refuse that result or comparison and return no fabricated observation. |
| Engineering Assurance execution state | The accepted Rust producer reports unavailable, refused, failed, timed out, malformed response, or containment failure. | Preserve that exact typed state as an execution finding; do not convert it to agreement or disagreement. |

## Dependencies

Oracle obligations consume [FR-022](./FR-022-freeze-qualification-claim.md),
[FR-023](./FR-023-retain-qualification-corpus.md), and the existing dependence
model in [FR-017](./FR-017-retain-oracle-dependence.md). Host execution and evidence
production depend on the accepted versioned Rust interface owned by Engineering
Assurance issue `agent-ix/engineering-assurance#11`; absence remains an explicit
producer-owned blocker and does not authorize a local implementation.
