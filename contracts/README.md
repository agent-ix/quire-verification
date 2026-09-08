# Private contract packet

`e02-draft-1.schema.json` is the authoritative definition of the five E02
contracts and their shared provisional artifact-reference seam. At module load,
`src/contracts.mjs` compiles the schema with Ajv 2020-12 and derives the public
contract constants from it. The only additional plan validation is for duplicate
task IDs, unknown dependency references, and dependency cycles, which require
cross-record graph checks. This is a private versioned schema, not a published
normative schema or an accepted replacement for FS02/FS05 identities.

The fixture packet deliberately includes state, relational, trace, fault, and
manual inputs. The schema keeps their required inputs distinct rather than
placing them in one universal predicate type.

`npm run validate:fixtures` discovers every JSON file in `fixtures/`, validates
each complete fixture with a schema definition, and validates every embedded or
adapted instance of the five public contracts. An unhandled new fixture fails
the check.
