# VP04 verification-owner disposition procedure

Review the immutable automated core in `evidence/vp04-parser-portfolio-core.json` at
the report's `coreDigest`. Confirm that the candidate, fixtures, plan, native
runs, results, assessment, measurements, interpretation, and claim limits match
the evidence presented for VP04.

Record exactly one disposition value:

- `accept` when the bounded parser evidence and claim limits are acceptable;
- `revise` when identified changes are required before acceptance; or
- `reject` when this candidate must not qualify.

The disposition must name the displayed core digest, the reviewer identity
`agent-ix-verification-owner`, this exact procedure reference, an RFC 3339
observation time, and a unique nonempty disposition ID. The supplied reviewer
identity is provenance; this procedure does not claim cryptographic identity
authentication. Quoin remains the evidence and sufficiency owner.
