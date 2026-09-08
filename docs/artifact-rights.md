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
| Node.js runtime | External runtime used to execute tests; no npm dependency is distributed | Runtime's own terms; no linked package added | Recheck supported/runtime license for a release image. |
| CI actions | `actions/checkout@11d5960a326750d5838078e36cf38b85af677262` and `actions/setup-node@49933ea5288caeca8642d1e84afbd3f7d6820020` | Both declare `MIT`; revisions are pinned | Preserve upstream notices in any redistributed action bundle and requalify pin updates. |
| GNU AGPL license text | Free Software Foundation verbatim license document | Verbatim AGPLv3 text in `LICENSE` | Preserve unmodified text. |

There are no npm runtime or development dependencies. The tests use only
Node.js built-ins. The implementation does not import or redistribute standards
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
