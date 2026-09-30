# License decision before implementation or publication

Private implementation decision, 2026-09-07.

The code, tests, private interface drafts, and compatibility fixtures added for
E02 use **SPDX `AGPL-3.0-or-later`**. This is the exact owner-approved license
for this bounded implementation and is compatible with the declarations on the
Quoin and spec-artifacts-process material represented by the baseline fixture.
The repository includes the corresponding license text in `LICENSE`.

The implementation is cleanly written against observed interfaces. It does not
copy Quoin implementation source. The Quoin catalog fixture is a generated
snapshot containing method data from spec-artifacts-process under AGPL terms.
Native result artifacts remain references and are not copied into this repository.

`Cargo.toml` and `Cargo.lock` are the authoritative Rust dependencies. The dependency
policy gate is `cargo deny check`; redistributed bundles must preserve the
applicable upstream notices.

This decision does not select the eventual license for normative language prose,
public schemas, or conformance vectors. Those remain gated by E01/E04. It does not
relicense existing source or authorize publication of this private history.

Every release still needs inbound rights, exact
outbound license, dependency compatibility, notices, generated-content review,
and publication approval. Repository visibility and license are separate
decisions.
