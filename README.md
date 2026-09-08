# quire-verification

Private bootstrap, 2026-09-07. Repository shell and specification work only; no implemented or released capability is claimed.

## Charter

Portable technique/capability/policy/plan/result contracts and qualified planning integration with Quoin.

## First work

Inspect current Quoin catalog/advisor/intake; define TechniqueDefinition, ToolCapability, SelectionPolicy, VerificationPlan and TechniqueResult; preserve method IDs and create parity fixtures; model prerequisites, scope, freshness and adequacy separately; retain native runner and Quoin evidence ownership. Decide code ownership/license before extracting existing implementation.

## Boundaries

No duplicate evidence store, hidden normative dependency, or unqualified backend promise. Existing Quire/Quoin/Filament interfaces must be inspected before replacement or extraction. Research and private issue links must not enter a public release packet.

## Tracking

- [Creation ticket](https://github.com/agent-ix/quire-research/issues/31)
- [Program epic](https://github.com/agent-ix/quire-research/issues/28)
- [First specification issue](https://github.com/agent-ix/quire-verification/issues/1)
- [Private formalization board](https://github.com/orgs/agent-ix/projects/20)

Read [license decision](LICENSE-DECISION.md) before adding code or normative artifacts. Publication requires a fresh review; do not flip private history public.

## Implementation language — owner directive

Executable implementation, including production and qualification tools/tests, is Rust. New TypeScript choices require explicit owner approval; surface all other executable languages. Quoin remains in its current implementation for now: contain its spread through versioned structured interfaces, do not rewrite it wholesale. Filament changes are excluded. See AGENTS.md and the language gate on the owning issue. Normative prose/schema need not be executable Rust.
