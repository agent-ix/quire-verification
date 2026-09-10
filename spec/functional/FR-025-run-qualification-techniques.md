---
id: FR-025
title: "Produce discriminating qualification results"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-004"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-023"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-024"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-017"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/NFR-005"
    type: "depends_on"
  - target: "ix://agent-ix/engineering-assurance/FR-014"
    type: "depends_on"
  - target: "ix://agent-ix/engineering-assurance/NFR-004"
    type: "depends_on"
---
# FR-025: Produce discriminating qualification results

## Description

For every required evidence obligation in one frozen qualification profile, the
qualification technique planner SHALL freeze one closed execution configuration or one
owned unavailable-technique gap. It SHALL deterministically generate cases and assess
typed Engineering Assurance observations into exact `TechniqueResult` records without
executing a process, storing Quoin evidence, or treating an unfinished campaign as a
pass.

## Inputs

- One frozen profile, qualified corpus, and qualification-oracle manifest with their
  exact identities.
- One exact accepted Quoin verification-catalog export and strict-reader proof for its
  source reference.
- Candidate closed `QualificationTechniqueManifest` data covering each required profile
  evidence obligation.
- Exact Rust harness, property, generator, relation, shrinker, oracle-binding, and
  Engineering Assurance request references selected before execution.
- Exact accepted verification-catalog advice and method records used to select fuzz and
  mutation producers, plus the consumer-owned Rust harness manifest when one exists.
- Typed results and observation references produced through the exact accepted Rust
  Engineering Assurance producer-execution interface.

## Outputs

- One immutable schema-derived `QualificationTechniqueManifest` with an RFC 8785
  identity.
- Deterministic next-case or shrink requests whose identity binds the manifest,
  obligation, configuration, prior observations, and ordinal.
- Exact public `TechniqueResult` records plus retained counterexample and technique
  artifacts supplied to Quoin.
- Explicit owned unavailable-technique, execution, unfinished-budget, counterexample,
  surviving-mutant, and replay findings; none is an evidence-sufficiency or qualification
  disposition.
- One validated QUOIN fuzz-generation report that distinguishes emitted targets,
  semantically unsuitable selections, ungrounded entry points, and an absent repository
  harness without treating an emitted source file as executed evidence.

## Behavior

- `QualificationTechniqueManifest` SHALL carry `contractVersion`, `schemaVersion`,
  `kind`, `manifestId`, `profileIdentity`, `corpusIdentity`, `oracleManifestIdentity`,
  `catalogIdentity`, `configurations`, and `gaps`.
- Each profile evidence obligation SHALL occur in exactly one configuration or exactly
  one owned gap, never both.
- Each configuration SHALL carry `configurationId`, `obligationId`, `guaranteeId`,
  exact catalog `techniqueId`, `propertyRef`, a closed corpus
  `inputProjection`, `budgetId`, exact `stoppingRule`, `harnessRef`,
  `requestTemplateRef`, nonempty unique `limitations`, and one closed `technique`.
- The planner SHALL require `propertyRef` to be a strict-reader-proved `property`,
  `harnessRef` and every executable procedure reference to be an
  `executable-projection`, `requestTemplateRef` to be an `invocation`, each
  `mutantRef` to be a `fault-model`, and each assigned test reference to be a `trace`.
- `inputProjection` SHALL reuse the closed ordered
  `QualificationOracleInputProjection` contract.
- `technique.family` SHALL be one of `example`, `property`, `metamorphic`,
  `differential`, `fuzz`, or `mutation`.
- The planner SHALL map `bdd-spec-by-example` to `example`,
  `property-based-testing` to `property`, `metamorphic-testing` to `metamorphic`,
  `differential-testing` to `differential`, `fuzzing` to `fuzz`, and
  `mutation-testing` to `mutation`.
- An `example` technique SHALL carry only its `family` tag.
- The planner SHALL derive an example technique's case limit from the number of entries
  in the configuration's ordered input projection.
- A `property` technique SHALL carry `generatorRef`, `shrinkerRef`, one unsigned
  decimal `seed`, positive `maximumCases`, and positive `maximumShrinks`.
- A `metamorphic` technique SHALL carry `relationProcedureRef`, `transformRef`, one
  unsigned decimal `seed`, positive `maximumCases`, and positive `maximumShrinks`.
- A `differential` technique SHALL carry nonempty unique `oracleBindingIds`,
  `comparisonProcedureRef`, one unsigned decimal `seed`, positive `maximumCases`, and
  positive `maximumShrinks`.
- A `fuzz` technique SHALL carry `targetRef`, nonempty unique
  `seedCorpusEntryIds`, `minimizerRef`, one unsigned decimal `seed`, and positive
  `maximumExecutions`; Task-029 owns this form's implementation and completion evidence.
- A `mutation` technique SHALL carry nonempty unique closed mutant records. Each
  mutant SHALL bind `mutantId`, `guardId`, `mutantRef`, `assignedTestRef`, and
  `expectedFailure`; Task-029 owns this form's implementation and completion evidence.
- A manifest gap SHALL carry `gapId`, `obligationId`, `owner`, nonempty `reason`,
  nonempty unique `limitations`, and the exact set-like `admissionRules` list, empty when
  the gap is unrelated to source-profile admission.
- The caller SHALL supply every seed, case or shrink limit, executable reference, and
  procedure reference before the planner freezes the manifest.
- The planner SHALL NOT invent a technique parameter, reference, budget assignment, or
  source-profile admission result.
- Every decimal seed SHALL use the canonical unsigned 64-bit representation `0` or a
  nonzero digit followed by at most nineteen decimal digits, with values above
  `18446744073709551615` refused.
- Every case ordinal SHALL use the same canonical unsigned 64-bit representation, begin
  at zero, and remain strictly below the selected technique's declared case or execution
  limit.
- Every case, shrink, and execution count SHALL be a positive integer no greater than
  100,000.
- Every manifest array SHALL remain within the NFR-005 100,000-item boundary.
- The planner SHALL refuse duplicate identifiers, unknown or conflicting obligations,
  unknown guarantees, unretained input entries, absent oracle bindings, a budget or
  stopping rule that differs from the obligation's frozen profile assignment, a foreign
  tagged field, or a technique ID absent from the exact accepted catalog.
- The planner SHALL NOT privately mint `differential-testing`.
- Differential work SHALL remain blocked until `agent-ix/spec-artifacts-process#86`
  publishes an accepted catalog revision containing that method.
- For a formal-clause guarantee, the planner SHALL require the frozen profile to bind the
  exact accepted native Quire source-profile identity and admission evidence for numeric
  semantics, presence/null/invalid semantics, containment/references, collections,
  calls/results, source authority, evaluation instants, the recorded reference-design
  gate, and the temporal boundary.
- The planner SHALL determine formal-clause and export-mapping scope only from the frozen
  guarantee's closed `claimAuthority` tag.
- The planner SHALL retain every applicable formal-clause rule lacking accepted evidence
  in the owning obligation gap's exact `admissionRules` set. It SHALL refuse a
  configuration while that set is nonempty and SHALL refuse a gap that omits, adds, or
  misstates any rule carried by the frozen profile.
- The planner SHALL NOT treat a first-party parser, compiler, executable, or source
  repository as evidence that the native source profile passed an admission rule.
- An OCL 2.4, SysML v2, or FRETish mapping profile SHALL remain an export-only target with
  preserved, conditional, unrepresented, and refused obligations in its exact loss
  report.
- The planner SHALL NOT use a Java, JVM, Maven, Eclipse, Electron, Node, or Python tool as
  a production or qualification dependency without a separate explicit owner decision.
- Acceptance by an upstream export-format tool SHALL NOT serve as differential evidence
  for source-clause correctness.
- A configuration for an `export-mapping` guarantee SHALL use only first-party semantic
  evidence bound to the exact loss report and SHALL NOT use the upstream format tool as
  a differential oracle.
- Property-test source generation SHALL use the installed QUOIN `spec-correctness`
  workflow. An ungrounded criterion SHALL remain a validated SpecReview finding and
  blocker rather than authorizing a parallel generator or weaker hand-written property.
- Task-029 fuzz-target generation SHALL use the installed QUOIN `spec-fuzz` workflow.
  An ungrounded fuzz obligation SHALL remain a validated finding and blocker.
- The repository owner SHALL establish a consumer-owned Rust harness selected from the
  accepted catalog before fuzz-target generation.
- When an installed host executable has no repository harness, the planner SHALL retain an
  absent-harness finding without authorizing a target file that cannot compile or run.
- The `spec-fuzz` selection SHALL come from the exact retained advisor and catalog records.
  A selected obligation that does not describe an input surface SHALL remain a method-fit
  finding and SHALL NOT be converted into a token target.
- Each emitted fuzz target SHALL call the grounded public Rust input surface directly,
  carry both workflow-required trace carriers and a bare compiler-checked `ix-trace-rs`
  marker, and perform no file, network, process, correctness-assertion, panic, or unchecked
  result-unwrapping work of its own.
- Emitting or compiling a fuzz target SHALL NOT count as fuzz evidence. A fuzz obligation
  remains unfinished until the exact accepted producer-execution result records the
  selected target, seed corpus, seed, execution count, stopping state, tool and harness
  identities, candidate identity, budget, and retained observations.
- A fuzz crash or hang SHALL retain the original input as a blocking candidate
  counterexample until replay of the declared minimizer's exact output through the accepted
  execution boundary reproduces the same declared violation.
- When replay of the declared minimizer's output does not reproduce the violation, the
  planner SHALL refuse that output without discarding the original counterexample.
- Mutation execution SHALL use an accepted catalog-declared Rust mutation producer. The
  repository SHALL NOT add a source rewriter, test runner, transcript parser, or mutation
  evidence store.
- Each seeded mutant observation SHALL bind the exact manifest mutant, source revision,
  generated fault identity, assigned bare-traced test, producer and configuration
  identities, execution result, and expected failure. `killed`, `survived`, `timed-out`,
  `crashed`, and `unfinished-budget` SHALL remain distinct.
- A mutant SHALL count as killed only when its assigned test fails for the declared reason.
  Another test failing, setup failure, compilation failure outside the declared fault, or
  an unclassified producer result SHALL NOT satisfy the mutant obligation.
- First-party property, generator, transform, shrinker, comparator, mutator, assertion,
  and result-adapter logic SHALL be Rust and traced with bare `ix-trace-rs` attributes.
- `quire-verification` SHALL produce pure deterministic case, shrink, and comparison
  decisions. It SHALL NOT spawn a candidate or tool process, interpret arbitrary stdout,
  create a qualification evidence store, or copy Engineering Assurance execution logic.
- The caller SHALL submit each exact request through the accepted Rust Engineering
  Assurance producer-execution interface. The planner SHALL consume only typed results
  from that exact interface revision.
- Every next-case identity SHALL bind the exact configuration, seed, case ordinal,
  corpus inputs, and any prior shrink path. Reordering set-like inputs SHALL NOT change
  the identity; ordered input projections and shrink paths SHALL retain their order.
- Each next-case or shrink request SHALL be a closed schema-derived
  `QualificationCaseRequest` carrying the manifest identity, configuration identifier
  and identity, canonical case ordinal, normalized prior-observation references, and
  ordered generated-artifact shrink path under its own RFC 8785 identity.
- A completed result SHALL bind the exact manifest, configuration, request, Engineering
  Assurance result, candidate, dependency closure, corpus inputs, property, harness,
  catalog method, oracle contributions, observations, stopping state, and artifacts.
- A candidate counterexample SHALL produce a completed `TechniqueResult` with logical
  outcome `violated`; successful completion of the exact stopping rule SHALL produce
  logical outcome `satisfied` only for the declared property or relation.
- Engineering Assurance `unavailable`, `refused`, `failed`, `timed-out`,
  `malformed-response`, or `containment-failure` states SHALL remain distinct execution
  findings and SHALL NOT produce a logical pass or fabricated observation.
- Reaching a resource budget before the stopping rule SHALL produce an incomplete result.
  Generating no counterexample before an unfinished stopping rule SHALL NOT produce a
  pass.
- Shrinking SHALL retain the original counterexample and ordered shrink path.
- The planner SHALL accept a shrink result only after replay against the exact candidate
  and applicable oracle revisions reproduces the same declared violation.
- Each result SHALL preserve the FR-017 oracle contribution and dependence model without
  aggregating separate results or promoting shared, derived, or unknown evidence to
  independent.
- Changing any bound manifest, configuration, catalog, request, result, observation,
  stopping, replay, or artifact identity SHALL refuse reuse.
- Permuting only set-like inputs SHALL leave the applicable RFC 8785 identity unchanged.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-025-CON-1 | First-party property, generator, transform, shrinker, comparator, mutator, assertion, and result-adapter logic SHALL be Rust and traced through `ix-trace-rs`. | Language | Test (TC-042) |
| FR-025-CON-2 | The accepted versioned Rust Engineering Assurance producer-execution interface SHALL be the sole candidate, oracle, harness, and external-tool execution boundary and exclude forbidden foreign runtimes from the production or qualification dependency closure. | Security | Test (TC-042) |
| FR-025-CON-3 | Quoin SHALL own evidence retention and authored sufficiency while technique-planner output excludes aggregate scores, qualification decisions, and human dispositions. | Ownership | Inspection (TC-042) |
| FR-025-CON-4 | Case generation, shrinking, comparison, and result construction SHALL be deterministic for the same exact inputs. | Reproducibility | Test (TC-042) |
| FR-025-CON-5 | Fuzz and mutation execution SHALL use accepted catalog-declared common Rust producers through the Engineering Assurance boundary; repository-local producer drivers, source rewriters, transcript parsers, and evidence stores are prohibited. | Ownership | Test (TC-042) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-025-AC-1 | Every evidence obligation has one closed configuration or one owned gap; every configuration binds the exact profile, corpus, oracle manifest, catalog method, property, family-specific inputs, projection, budget, stopping rule, harness, request template, and limitations. Missing, duplicate, unknown, conflicting, cross-tagged, or privately invented values refuse the manifest. | Test (TC-042) |
| FR-025-AC-2 | Every example, property, metamorphic, and differential configuration produces the same case identity from the same seed, ordinal, projection, and ordered shrink path; changing any bound input changes identity or refuses reuse, and set-only permutations do not. | Test (TC-042) |
| FR-025-AC-3 | Completed satisfaction, completed violation, unavailable, refused, failed, timed-out, malformed-response, containment-failure, unsupported method, and unfinished-budget states remain distinct `TechniqueResult` or finding outcomes. Independent arrival order cannot erase a result. | Test (TC-042) |
| FR-025-AC-4 | A minimized counterexample retains the original case and shrink path and contributes only after exact candidate and applicable oracle replay reproduces the same violation; changed or non-reproducing replay is refused. | Test (TC-042) |
| FR-025-AC-5 | Each Task-029 seeded mutant records one exact qualification-critical guard, generated fault identity, accepted producer configuration, and assigned bare-traced Rust test. The mutant is killed only when that test fails for the declared reason; survived, timed-out, crashed, unrelated-failure, and unfinished-budget results remain distinct blockers. | Test (TC-042) |
| FR-025-AC-6 | The exact QUOIN-selected fuzz obligations produce either runnable grounded Rust targets or validated per-obligation findings. Target creation alone remains undischarged; an unfinished budget remains incomplete, and every crash or hang retains its original input and contributes a violation only after exact minimized replay reproduces it. | Test (TC-042) |

## Error Conditions

| Code | Condition | Result |
|------|-----------|--------|
| `invalid_qualification_technique_manifest` | The closed manifest is malformed, conflicts with the frozen profile/corpus/oracle/catalog identities, incompletely covers required obligations, or carries invalid family-specific data. | Refuse freezing and return no partial manifest. |
| `invalid_qualification_case` | A case, shrink path, replay, request, result, observation, stopping state, or artifact differs from the frozen configuration. | Refuse the contribution and return no fabricated result. |
| `qualification_technique_unavailable` | An accepted catalog method, grounded QUOIN-generated source, exact harness, oracle binding, or accepted Engineering Assurance interface is unavailable. | Retain one owned gap; do not substitute another technique. |
| Engineering Assurance execution state | The accepted Rust producer reports unavailable, refused, failed, timed out, malformed response, or containment failure. | Preserve that exact state as an execution finding and do not emit satisfaction or violation. |

## Dependencies

The planner consumes FR-017 dependence semantics and the frozen FR-022 through FR-024
profile, corpus, and oracle boundaries. Process execution and evidence production depend
on `agent-ix/engineering-assurance#34`. Differential technique identity depends on
`agent-ix/spec-artifacts-process#86`. Task-028 implements example, property, metamorphic,
and differential planning plus results; Task-029 implements fuzz and mutation and is the
single completion owner for FR-025 and TC-042. Formal-clause technique execution remains
blocked until the profile contract binds the owner-selected native Quire source profile
and accepted evidence or owned gaps for every carried-forward admission rule from the
2026-09-09 ruling on `agent-ix/quire-contract-ir#53`.
