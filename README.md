# quire-verification

Deciding how to verify a requirement is a planning problem before it is a
testing problem. A requirement might call for a property test, a fuzz target, a
metamorphic check or a human inspection, and each of those needs a tool,
inputs, permissions and a time budget that may not exist yet.
quire-verification is the Rust crate that turns a requirement into an explicit
verification plan: which techniques apply, which tool capability could run each
one, and exactly what is still missing.

It exists so the layer above never has to guess. Quoin already knows which
verification methods fit a requirement; what it lacked was a portable,
versioned way to say "this tool is qualified for this method, this task is
blocked on a missing harness, this result is stale." This crate defines those
contracts and plans against them. It does not run tools and it does not decide
whether evidence is enough: Quoin keeps the evidence, and a person decides
sufficiency.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/images/hero-dark.svg">
  <img alt="Quoin's method catalog and its advice for a requirement supply the techniques that apply. quire-verification combines them with tool capabilities and a selection policy into a verification plan whose tasks are ready, blocked or unsupported. A native producer runs the ready tasks, and the technique results come back to be assessed for scope and freshness. Quoin keeps the evidence and a person decides whether it is enough." src="docs/images/hero-light.svg">
</picture>

> **Status: prerelease, private repository.** The planning interfaces and
> evidence assessment are accepted for exact private Rust consumers only. No
> schema is published, and each native verification tool is still separately
> gated: a validating fixture never claims a tool ran.

## The five contracts

All five are defined in [`contracts/e02-draft-2.schema.json`](contracts/e02-draft-2.schema.json)
and validated through the public `quire-verification-contracts` crate.

| Contract | What it says |
|---|---|
| `TechniqueDefinition` | What a verification method means and where it may apply. Keeps Quoin's method id byte for byte. Claims no tool is installed. |
| `ToolCapability` | Binds one technique to a native producer and adapter, with input kinds, prerequisites, resource limits, qualification state and claim limits. |
| `SelectionPolicy` | Explainable rules with source provenance and an authored priority (`required`, `recommended`, `optional`), never a computed score. An expired policy adds a freshness prerequisite. |
| `VerificationPlan` | An inventory of properties, then tasks. Each task has one of five input shapes: `state`, `relational`, `trace`, `fault` or `manual`. |
| `TechniqueResult` | One run's outcome with execution status, logical outcome, activation, assumptions, observation adequacy, exact scope and provenance kept as separate fields. |

## How a plan is built

Applicability and readiness are separate answers, so "this method fits but we
cannot run it yet" stays visible instead of being dropped.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/images/task-states-dark.svg">
  <img alt="Every plan task carries two independent answers. Applicability is applicable, not-applicable or unknown; readiness is ready, blocked or unsupported. Only a task that is applicable, ready and qualified, within budget, gets a native runner request." src="docs/images/task-states-light.svg">
</picture>

- Quoin's `inconclusive` advice becomes `unknown`, never `not-applicable`.
- Missing generators, harnesses, tools, permissions, observations or reviewer
  assignments are listed as prerequisites to satisfy.
- Fault tasks stay blocked without an explicit non-production authorization
  and an injection binding.
- A native runner request is closed and finite: the selected capability,
  adapter, permissions, input and one budget. Manual work never receives
  invocation permission.

Evidence assessment then compares each result with the plan: exact
plan/task/run/producer/adapter binding, scope, freshness, and every
simultaneous finding. Ambiguous duplicate results are reported, not merged.

## Using it

The smallest real use is the fixture plan in
[`tests/verification.rs`](tests/verification.rs), which validates the shared
references a request carries and builds the plan:

```rust
let references = validated_fixture_references(request)?;
build_plan_from_validated(request, &references)
```

where `request` is the `planRequest` in
[`fixtures/portfolio.json`](fixtures/portfolio.json) and
`build_plan_from_validated` is `quire_verification::planner::build_verification_plan`.
The same test file adapts Quoin's exports with
`adapt_quoin_catalog(&fixture("quoin-catalog-export.json"))` and assesses
results with `assess_evidence(&packet, &references)`.

From the command line:

```sh
cargo run --locked --bin validate-fixtures      # every fixture against the schema
cargo run --locked --bin render-fixtures        # the retained evidence packet
cargo run --locked --bin run-bounded-portfolio -- "$(git rev-parse HEAD)" 2026-09-08T02:00:00Z
```

`run-bounded-portfolio` checks catalog field parity keyed by `techniqueId` and
permutation invariance. A violated check or a failed execution is kept as its
own `TechniqueResult`, and an `inspection` result stays `not-run` until a
person records the disposition.

## Reference

| Dimension | Values |
|---|---|
| Input shape | `state`, `relational`, `trace`, `fault`, `manual` |
| Applicability | `applicable`, `not-applicable`, `unknown` |
| Readiness | `ready`, `blocked`, `unsupported` |
| Execution status | `not-run`, `running`, `completed`, `failed`, `cancelled`, `exhausted`, `invalid-input`, `unsupported` |
| Logical outcome | `satisfied`, `violated`, `unknown`, `not-evaluated`, `inconclusive` |

Oracle contributions record whether a result's oracle and observation are
independent, shared, derived or unknown. Further reading:
[interface decision](docs/interface-decision.md),
[Quoin compatibility map](docs/quoin-compatibility.md),
[contract packet](contracts/README.md) and the
[historical evaluation report](docs/evaluation-report.md).

## The Quire ecosystem

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/images/ecosystem-dark.svg">
  <img alt="The Quire repositories. The main chain runs from the standard, quire-specification, to the compiler, quire-spec-language, then quire-contract-ir, quire-contract-codegen and quire-contract-runtime. quire-spec-language also feeds quire-protocol, and quire-contract-ir feeds quire-analyze. quire-observation feeds quire-protocol and quire-mltl. Above the chain, quire-driver runs it, quire-integration tests it and quire-verification plans verification. Below it, shared libraries used throughout: quire-exact, quire-semantic-value, quire-walk, quire-canonical and quire-verification-contracts." src="docs/images/ecosystem-light.svg">
</picture>

quire-verification sits above the main chain. It depends on
`quire-verification-contracts` (the schema and canonicalization boundary),
`quire-analyze` (method contract) and `engineering-assurance` (native
execution and evidence), all from `Cargo.toml`. Quoin supplies the catalog and
advice across a versioned JSON boundary and keeps its own orchestration.

## Building and testing

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
cargo fuzz build -D --sanitizer none
cargo deny check
```

Acceptance tests carry a compiler-checked `#[trace(...)]` marker from
`ix-trace-rs`; `quire coverage --scope . --json` reports them against the
specs in [`spec/`](spec/). Implementation is Rust; see [AGENTS.md](AGENTS.md)
for the language policy.

## Contributing and licence

Every contribution needs a signed CLA; see [CONTRIBUTING.md](CONTRIBUTING.md)
and [CLA.md](CLA.md). The code is AGPL-3.0-or-later ([LICENSE](LICENSE)); the
reasoning, and what it leaves undecided for specs and public schemas, is in
[LICENSE-DECISION.md](LICENSE-DECISION.md). Repository visibility and licence
are separate decisions: publication needs a fresh review.
