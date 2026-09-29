# Contributor and session guidance

Read README.md, LICENSE-DECISION.md and the owning issue before editing. This is a private bootstrap repository, not a released implementation.

## Hash / digest / pin antipattern: present, slated for removal

This repository still contains hashes, digests, SHAs, pins, checksum catalogs and
records that track files, versions or tools. That is an antipattern and it is slated
for removal. Do not introduce any new use of it. When you touch code or specs that
contain one, remove it as part of the change. The only hash that stays is a canonical
identity digest that binds a proof to the exact content it proved. Package versions
live in Cargo.toml / package.json and their lockfiles only; reports name the app
version they ran.

- Preserve authored/source identity and explicit unsupported/incomplete results.
- Use the first specification issue to establish requirements, acceptance fixtures and scoped implementation/CI work before scaffolding speculative crates.
- Inspect existing shared model, contract and Quoin interfaces before duplicating them.
- Do not infer licensing from process or repository boundaries.
- Keep private research, traces and issue links out of public releases. No public posting, visibility change, or source relicensing without explicit approval.
- Use isolated worktrees for concurrent code sessions; preserve unrelated edits.
- Run `cargo test --workspace --all-features --locked` for the complete Rust compiler/test gate; use the additional formatting, Clippy, dependency-policy, and fixture commands documented in README.md for changes in their scope.

<!-- required-language-policy:start -->
## Required implementation language and containment

Owner directive (2026-09-07): first-party production and possible qualification paths must be **Rust**, including libraries, CLI/tools, generators, validators, canonicalization, result adapters, test assertions and fixture/CI audits. "Glue", prototypes, scripts and tests are not exemptions; a Rust wrapper around non-Rust semantic logic is not remediation.

- Every new TypeScript choice needs explicit owner approval for its exact scope. Any other executable language must be brought to the owner's attention; do not silently introduce or expand it.
- Quoin's existing implementation is explicitly retained for now: **contain its spread, do not schedule a wholesale Quoin rewrite**. Keep new reusable semantics in Rust behind versioned, structured interfaces. Existing Node/TypeScript is not blanket permission for new components or expanded language scope.
- Reuse shared assurance contracts and Quoin evidence ownership; do not recreate local evidence frameworks. Classify each helper as an existing shared capability, a missing reusable capability, or domain-specific logic before porting.
- Complete /specify and /spec-review for the changed requirements/interfaces, resolve findings, then implement and verify. Preserve ongoing work. Do not claim a language port or qualification complete from a policy update.
- Filament repositories/tickets are excluded from this remediation. Interface consumption does not authorize producer changes.

Existing Quoin retention is the only accommodation recorded here; this block grants no new TypeScript approval.
<!-- required-language-policy:end -->

/specify and /spec-review are mandatory before implementation. Preserve already-written work; pause further unreviewed implementation/merge.
