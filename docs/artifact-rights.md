# E02 artifact and dependency rights inventory

Status: private implementation inventory, 2026-09-07. This resolves the exact
terms for this bounded slice; it does not complete E04's ecosystem-wide review.

| Artifact | Origin/inbound basis | Terms in this repository | Remaining release question |
|---|---|---|---|
| `src/**`, `test/**` | New Agent-IX implementation; no copied Quoin source | `AGPL-3.0-or-later` | Confirm contributor authority and release packet. |
| Private interface/compatibility prose | Newly authored from maintained private architecture and observed behavior | `AGPL-3.0-or-later` for this implementation packet | E01/E04 must select terms for any public normative prose. |
| `fixtures/quoin-catalog-export.json` | Generated from Quoin `7072d65` reading spec-artifacts-process `61a20e` catalog data | Preserved under compatible `AGPL-3.0-or-later`; provenance embedded | Decide whether a public fixture should contain full definitions or only stable IDs/behavior vectors. |
| Plan/result fixtures and local portfolio report | Newly authored synthetic/local data; no real users, traces, credentials, standards tables, or production configuration | `AGPL-3.0-or-later` | Public conformance-vector terms remain an E01/E04 decision. |
| Native artifact references | Synthetic URI/digest references only; target artifacts are not copied | No inbound artifact included | Each real producer artifact keeps its own terms and retention policy. |
| Node.js runtime | External runtime used to execute tests and scripts | Runtime's own terms | Recheck supported/runtime license for a release image. |
| `ajv@8.20.0`, `ajv-formats@3.0.1` | npm packages used to compile and execute the draft 2020-12 contract schema | MIT | Preserve dependency license notices in redistributed bundles. |
| `canonicalize@4.0.0` | npm implementation of RFC 8785 JSON Canonicalization Scheme | Apache-2.0 | Preserve the license and any required notices in redistributed bundles. |
| Transitive npm packages | `fast-deep-equal@3.1.3` (MIT), `fast-uri@3.1.7` (BSD-3-Clause), `json-schema-traverse@1.0.0` (MIT), `require-from-string@2.0.2` (MIT) | Upstream terms, exactly locked in `package-lock.json` | Preserve applicable license notices in redistributed bundles. |
| CI actions | `actions/checkout@11d5960a326750d5838078e36cf38b85af677262` and `actions/setup-node@49933ea5288caeca8642d1e84afbd3f7d6820020` | Both declare `MIT`; revisions are pinned | Preserve upstream notices in any redistributed action bundle and requalify pin updates. |
| GNU AGPL license text | Free Software Foundation verbatim license document | Verbatim AGPLv3 text in `LICENSE` | Preserve unmodified text. |

The exact dependency versions, tarball integrity values, and complete transitive
set are locked in `package-lock.json`; `npm audit --omit=dev` is part of the
bounded verification pass. The implementation does not import or redistribute standards
tables, third-party tool code, Quoin implementation source, private traces,
templates, generated runtimes, or external corpora.

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
