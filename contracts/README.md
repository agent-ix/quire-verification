# Private contract packet

`e02-draft-1.schema.json` collects the five E02 contract definitions and their
shared provisional artifact-reference seam. It is private, versioned, and
executable by the runtime validators in `src/contracts.mjs`; it is not a
published normative schema or an accepted replacement for FS02/FS05 identities.

The fixture packet deliberately includes state, relational, trace, fault, and
manual inputs. The schema keeps their required inputs distinct rather than
placing them in one universal predicate type.
