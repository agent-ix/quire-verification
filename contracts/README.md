# Private contract packet

`e02-draft-2.schema.json` is the authoritative definition of the five E02
contracts. Its artifact envelopes resolve the retained shared-reference draft-2
schema rather than restating that schema's fields. The `validate_contract`,
wire-type, and RFC 8785 canonicalization boundary that compiles this schema
now lives in public crate `quire-verification-contracts` (PLAT-861); this
repository depends on it and re-exports it at `contracts`, so internal call
sites are unchanged. This copy of `e02-draft-2.schema.json` is retained here
because `src/qualification_oracle_execution.rs` and the acceptance tests embed
it directly (`include_bytes!`/`include_str!`) rather than through the crate
boundary. The only additional plan validation is for duplicate task IDs,
unknown dependency references, and dependency cycles, which require
cross-record graph checks. This is the accepted private versioned interface for
cross-repository Rust consumers, not a public normative schema. Its retained
shared-reference packet is pinned at
`agent-ix/quire-specification@8ab058beff03cc76c0f39cc06c87ef98f76e8d78`;
see `shared-reference-UPSTREAM.md` for the public reusable-artifact terms now
selected for the draft-2 schema fragment.

The same schema owns the internal `QualificationProfile` definition used by the
Rust production-qualification planner. It is not a sixth public E02 root contract.
The definition closes candidate, dependency, caller, command, population,
semantic-guarantee, environment, resource-budget, evidence, exclusion, and
producer-interface dimensions. Every selected producer role carries either one
exact accepted shared-reference envelope or one typed unavailable state with an
owner and reason. Rust code requires every embedded artifact envelope to match an
exact `ValidatedSharedReference` from the strict shared-reference reader. The schema
and Rust gate map portable contracts, shared references, Engineering Assurance, and
Quoin evidence to `source`; parser/compiler to `executable-projection`; and the
production caller to `invocation`. The implementation derives the
profile representation from this definition, computes its identity with RFC 8785 JCS,
and returns all unavailable interfaces as blocked prerequisites without copying
producer-owned schemas. Each guarantee has a closed claim-authority form. Native formal
clauses name the sole native Quire source profile and carry one `accepted` observation or
owned `gap` for every carried-forward admission rule; the presence of a reference does
not imply admission. OCL 2.4, SysML v2, and FRETish remain export mappings bound to an
exact loss report.

The schema also owns the internal `QualificationCorpusManifest` used to retain
qualification inputs. It is not a public E02 root contract. Rust binds its exact
population and class denominator to a frozen `QualificationProfile`, accepts input,
generator, and replay bytes only through strict-reader proofs, checks mutation ancestry
and replay output digests, and preserves each missing class as one owner-assigned gap.
When rights permit only a reference or prohibit retention, the gap retains the candidate
reference and rights decision while the Rust corpus retains no candidate bytes and grants
no class coverage.

The same schema owns the internal `QualificationOracleManifest`. Rust binds it to
the exact frozen profile and corpus, validates every reference through the strict
shared-reference reader, classifies declared implementation dependence, checks the
derived-binding graph, and preserves every uncovered guarantee as an owned gap. Host
and process execution remain in the accepted Rust Engineering Assurance interface;
this repository does not add a local runner, evidence store, fixture script, or
arbitrary process-output parser.

The schema also owns the internal `QualificationTechniqueManifest`. Rust binds every
profile evidence obligation to exactly one closed catalog-backed configuration or owned
gap, checks exact profile, corpus, oracle, catalog, budget, stopping-rule, and strict
artifact references, and freezes a normalized RFC 8785 identity. Native formal-clause
gaps reproduce the exact unadmitted rule set from the profile. Case identities use
canonical unsigned decimal ordinals, normalize only set-like prior observations, and
return a closed schema-derived `QualificationCaseRequest`; ordered input projections and
shrink paths remain ordered. The planner makes no process call,
stores no evidence, invents no catalog method, and emits no qualification disposition.

`PropertyInventoryItem.classification` is optional on planner input so an
unclassified property can be represented. When present it must be non-empty;
the planner emits `unknown` when it was omitted.

The shared-reference reader separately derives Rust wire types from the exact
retained `shared-reference-1-draft.schema.json` and
`shared-reference-2-draft/schema.json` files. Version 2 is selected only for
artifact and semantic envelopes; version 1 is retained only for source-map and
migration envelopes. Runtime validation compiles the complete schemas locally,
then applies authority, identity, byte-digest, JCS, feature-set, source-map, and
explicit-migration checks. No unknown version or kind falls through to another
decoder.

The planner accepts reference-bearing property inputs only when the caller also
supplies the matching opaque value returned by the strict shared-reference
reader. Matching uses exact RFC 8785 JCS bytes and the artifact's selected role.
Evidence assessment applies that same rule to every result oracle and
observation reference. Evaluated results retain at least one closed oracle
contribution with an explicit dependence relation; other results may carry an
empty contribution array. E02 draft-1 has no compatibility or migration path.

`shared-reference-snapshot.sha256` covers both schemas and all fourteen selected
amendment fixtures. `shared-reference-UPSTREAM.md` records their exact private
upstream revision and containment terms. These files are independent input data
for the Rust consumer; no producer validation code or network schema retrieval
is part of the reader.

The fixture packet deliberately includes state, relational, trace, fault, and
manual inputs. The schema keeps their required inputs distinct rather than
placing them in one universal predicate type.

Native capabilities declare closed positive `resourceLimits`, and plan or
property budgets use the corresponding closed two-dimensional budget. A
property budget replaces the plan budget for its task. Only applicable, ready,
qualified `native-producer` work within both limits receives a closed runner
request; manual and all other work retain `runnerRequest: null` and
`invocationAllowed: false`.

`cargo run --locked --bin validate-fixtures` discovers every JSON file in `fixtures/`, validates
each complete fixture with a schema definition, and validates every embedded or
adapted instance of the five public contracts. An unhandled new fixture fails
the check.
