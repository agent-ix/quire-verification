# Private contract packet

`e02-draft-1.schema.json` is the authoritative definition of the five E02
contracts and their shared provisional artifact-reference seam. The Rust build
derives wire types and vocabulary constants from this file. At runtime,
`src/contracts.rs` compiles the complete draft 2020-12 schema, including
conditional assertions that are not constructive Rust types. The only additional plan validation is for duplicate
task IDs, unknown dependency references, and dependency cycles, which require
cross-record graph checks. This is a private versioned schema, not a published
normative schema or an accepted replacement for FS02/FS05 identities.

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

`shared-reference-snapshot.sha256` covers both schemas and all fourteen selected
amendment fixtures. `shared-reference-UPSTREAM.md` records their exact private
upstream revision and containment terms. These files are independent input data
for the Rust consumer; no producer validation code or network schema retrieval
is part of the reader.

The fixture packet deliberately includes state, relational, trace, fault, and
manual inputs. The schema keeps their required inputs distinct rather than
placing them in one universal predicate type.

`cargo run --locked --bin validate-fixtures` discovers every JSON file in `fixtures/`, validates
each complete fixture with a schema definition, and validates every embedded or
adapted instance of the five public contracts. An unhandled new fixture fails
the check.
