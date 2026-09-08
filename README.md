# quire-verification

Private bounded E02 implementation, 2026-09-07. The compatibility adapter,
planning interfaces, and evidence assessment in this repository are provisional
until their E01, FS02, and FS05 dependencies are reviewed. No represented native
verification tool is claimed to have run merely because its fixture validates.

## Charter

Portable technique/capability/policy/plan/result contracts and qualified planning integration with Quoin.

## Implemented slice

The current slice defines `TechniqueDefinition`, `ToolCapability`,
`SelectionPolicy`, `VerificationPlan`, and `TechniqueResult`; adapts a pinned
Quoin catalog export without relocating it; constructs prerequisite-aware task
plans; and assesses result scope and freshness without storing evidence or
making sufficiency decisions.

Run the complete Rust checks with:

```sh
cargo run --locked --bin validate-fixtures
cargo fmt --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
cargo deny check
cargo run --locked --bin render-fixtures
```

Run the bounded local portfolio against an exact checkout revision and retain
its JSON report from standard output with:

```sh
cargo run --locked --bin run-bounded-portfolio -- "$(git rev-parse HEAD)" 2026-09-08T02:00:00Z
```

That command executes catalog field parity keyed by `techniqueId` and
permutation-invariance checks. A check violation or execution failure is
retained as its own `TechniqueResult`, so the report remains available for
assessment.
It also emits an `inspection` result whose execution status remains `not-run`
until a verification owner records the required human disposition.

After independently checking out the pinned Quoin and module revisions, recheck
the captured catalog (the Quoin checkout must have its locked development
dependencies installed):

```sh
cd /path/to/quoin
node --no-warnings --loader ts-node/esm --input-type=module \
  -e 'import { loadMethodCatalog } from "./src/advisor/index.ts"; console.log(JSON.stringify(loadMethodCatalog([process.argv[1]]), null, 2));' \
  /path/to/spec-artifacts-process/spec_artifacts_process \
  | cargo run --locked --manifest-path /path/to/quire-verification/Cargo.toml --bin check-quoin-baseline
```

The fixture suite covers catalog identity parity, conflicting definitions,
applicability versus readiness, state/relational/trace/fault/manual input shapes,
missing prerequisites, stale evidence, untriggered behavior, missing
observations, exact plan/task/run/producer/adapter mismatches, ambiguous duplicate
results, and simultaneous independent failures.

## Boundaries

No duplicate evidence store, hidden normative dependency, or unqualified backend promise. Quoin retains its existing orchestration and catalog ownership across the versioned JSON boundary. Engineering Assurance owns its Rust-native execution and evidence implementation; this crate owns the portable contract and planning surface. Existing Quire/Quoin/Filament interfaces must be inspected before replacement or extraction. Research and private issue links must not enter a public release packet.

## Tracking

- [Creation ticket](https://github.com/agent-ix/quire-research/issues/31)
- [Program epic](https://github.com/agent-ix/quire-research/issues/28)
- [First specification issue](https://github.com/agent-ix/quire-verification/issues/1)
- [Private formalization board](https://github.com/orgs/agent-ix/projects/20)

Read [the interface decision](docs/interface-decision.md), [compatibility map](docs/quoin-compatibility.md),
the [evaluation report](docs/evaluation-report.md), [retained local portfolio](evidence/local-catalog-portfolio.json),
and [artifact rights inventory](docs/artifact-rights.md) before extending the
slice. Read [license decision](LICENSE-DECISION.md) before adding code or
normative artifacts. Publication requires a fresh review; do not flip private
history public.
