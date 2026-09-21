# E02 artifact, dependency, and executable inventory

Status: private Rust implementation inventory, 2026-09-07. This resolves the
exact terms and executable ownership for this bounded slice; it does not
complete E04's ecosystem-wide review.

## Retained artifacts and dependencies

| Artifact | Origin/inbound basis | Terms in this repository | Remaining release question |
|---|---|---|---|
| `src/**/*.rs`, `tests/**/*.rs`, and `build.rs` | New Agent-IX Rust implementation; no copied Quoin source | `AGPL-3.0-or-later` | Confirm contributor authority and release packet. |
| `contracts/e02-draft-2.schema.json` and private interface prose | Newly authored from maintained private architecture and observed behavior | `AGPL-3.0-or-later` for this implementation packet | E01/E04 must select terms for public normative prose and conformance vectors. |
| Retained shared-reference schemas and fourteen fixture files | Byte-identical data-only snapshot from private `agent-ix/quire-specification@8ab058beff03cc76c0f39cc06c87ef98f76e8d78`; exact paths and SHA-256 values are recorded in `contracts/shared-reference-UPSTREAM.md` and `contracts/shared-reference-snapshot.sha256` | Accepted for private independent qualification only | Public reusable-artifact terms remain unresolved; exclude the snapshot from public promotion until an owner selects them. |
| `fixtures/quoin-catalog-export.json` | Generated from Quoin `7072d65` reading spec-artifacts-process `61a20e` catalog data | Preserved under compatible `AGPL-3.0-or-later`; provenance embedded | Decide whether a public fixture should contain full definitions or only stable IDs and behavior vectors. |
| Plan/result fixtures and local portfolio report | Newly authored synthetic/local data; no real users, traces, credentials, standards tables, or production configuration | `AGPL-3.0-or-later` | Public conformance-vector terms remain an E01/E04 decision. |
| Native artifact references | Synthetic URI/digest references only; target artifacts are not copied | No inbound artifact included | Each real producer artifact keeps its own terms and retention policy. |
| Registry Rust dependencies | Runtime, schema, serialization, cryptography, generation, and test crates exactly locked in `Cargo.lock` | SPDX licenses accepted by `deny.toml`; checked by `cargo deny check` | Preserve required notices and re-run the dependency gate on every lock update. |
| `ix-trace-rs` | Agent-IX test trace macro crate, taken as a Cargo git dependency pinned to revision `2ce4ebf47f726b9d76388220545cd0abda8a5cfb` in `Cargo.toml` and `fuzz/Cargo.toml` | `AGPL-3.0-or-later`; allowed by `deny.toml` | Move the pinned revision to adopt upstream changes; changes belong upstream rather than in a local fork. |
| Quoin TypeScript runtime and `ts-node` | External, pinned integration producer used only for IT-001; it is outside this crate and communicates through JSON | Upstream project's own terms | Keep this toolchain outside the Rust production and qualified execution path. |
| CI action | `actions/checkout@11d5960a326750d5838078e36cf38b85af677262` | MIT; revision pinned | Preserve the upstream notice in any redistributed action bundle and requalify pin updates. |
| GNU AGPL license text | Free Software Foundation verbatim license document | Verbatim AGPLv3 text in `LICENSE` | Preserve unmodified text. |

`Cargo.lock` and the trace snapshot's `UPSTREAM.md` are the dependency source
of truth. `cargo deny check` validates
advisories, licenses, duplicate-version policy, and allowed sources. The
implementation does not import or redistribute standards tables, Quoin
implementation source, private traces, templates, generated runtimes, or
external corpora. The only redistributed tool code is the recorded, test-only
`ix-trace-rs` Rust snapshot needed for credential-independent CI compilation.

## Executable disposition

| Previous executable | Disposition | Rust replacement or boundary |
|---|---|---|
| `src/*.mjs` contract, catalog, planner, evidence, and portfolio modules | Deleted; no API or compatibility shim retained | Library modules under `src/*.rs` |
| `scripts/validate-fixtures.mjs` | Deleted | `validate-fixtures` Rust binary |
| `scripts/render-fixtures.mjs` | Deleted | `render-fixtures` Rust binary |
| `scripts/run-bounded-portfolio.mjs` | Deleted | `run-bounded-portfolio` Rust binary |
| `scripts/check-quoin-baseline.mjs` | Deleted | `check-quoin-baseline` Rust binary reading bounded JSON on standard input |
| `test/*.mjs`, `package.json`, and `package-lock.json` | Deleted | `tests/verification.rs`, `Cargo.toml`, and `Cargo.lock` |
| Quoin catalog production | Remains owned by Quoin; no duplicate implementation is introduced | Versioned JSON enters the Rust `check-quoin-baseline` boundary |
| Engineering Assurance execution and evidence | Remains separately owned by Engineering Assurance | No framework, evidence store, or execution adapter is duplicated here |

No first-party JavaScript or TypeScript executable is retained in the governed
source, test, or script paths. Quoin's external TypeScript command is an
integration fixture producer and is never loaded into this Rust process.

Exact questions contributed to E04 are:

1. Which open document/software terms govern the eventual public interface
   schemas and conformance vectors independently of this private AGPL code?
2. Should the public parity corpus include full catalog definitions from the
   AGPL module or a generated ID/field digest vector with separate retrieval?
3. What contributor and patent policy applies to the open verification
   interfaces?
4. What notice/source-offer bundle is required if this adapter ships inside a
   hosted service, plugin, container, or downloadable package?
5. Which native result artifacts may be redistributed versus referenced, and
   what data/retention rules apply to real traces or fault receipts?

Publication remains blocked until those questions receive a release-specific
owner/legal disposition and the private-history disclosure check passes.
