---
id: AD-001
title: Rust architecture for the bounded verification path
type: ArchitectureDescription
status: accepted
owner: agent-ix-architecture-owner
system: quire-verification bounded compatibility slice
relationships:
  - target: ix://agent-ix/quire-verification/StR-001
    type: realizes
  - target: ix://agent-ix/quire-verification/NFR-004
    type: realizes
  - target: ix://agent-ix/quire-verification/NFR-005
    type: realizes
  - target: ix://agent-ix/quire-verification/FR-002
    type: references
  - target: ix://agent-ix/quire-verification/FR-008
    type: references
---
# Rust architecture for the bounded verification path

## System Boundary

The system is one Rust library crate with repository-owned Rust binaries for
fixture validation, fixture rendering, bounded portfolio execution, and real
Quoin baseline comparison. Quoin remains an external TypeScript producer and
orchestrator. The boundary between them is versioned JSON read from files or
standard input. The crate does not store evidence or decide sufficiency.
Rust-native Engineering Assurance execution and storage remain owned by that
component; this crate supplies portable contracts and does not duplicate its
assurance framework.

## Views

The contract module embeds `contracts/e02-draft-1.schema.json`, compiles its
draft 2020-12 validator once, and exposes generated Rust wire types plus
vocabularies derived from the same schema. The catalog adapter converts pinned
Quoin JSON into portable technique records. The planner constructs a validated
acyclic task graph. The evidence module performs exact JCS-based matching and
classification. The bounded-portfolio module composes those pure services and
converts check violations or execution errors into `TechniqueResult` values.

Public byte-ingestion helpers enforce the 16 MiB document limit before parsing.
An iterative resource walk enforces depth 128 and 100,000 array items before
schema validation. IDs and digests use newtypes at Rust API boundaries, ordered
maps and stable sorts control observable ordering, and library errors use one
typed envelope with stable codes.

## Decisions

1. Rust owns every executable path because the component is eligible for the qualified production chain.
2. The checked-in JSON Schema remains the single authored definition of the five public wire contracts; Rust types are generated from it rather than maintained independently.
3. Runtime schema validation remains at untrusted JSON boundaries so generated types cannot weaken draft conditionals or unknown-field refusal.
4. RFC 8785 canonical bytes feed both SHA-256 digests and structured identity comparison.
5. Core transforms remain synchronous and side-effect free; filesystem and standard-stream access stay in binaries and bounded ingestion helpers.
6. Plan graph validation uses iterative traversal with explicit limits; caller input never reaches recursive graph code or a panic path.
7. Quoin integration remains a real external-loader test over JSON; the Rust crate does not embed or port Quoin's TypeScript CLI.
8. A failed portfolio technique becomes a failed result and a false comparison becomes a violated result, preserving the report in both cases.
9. The Rust crate replaces the JavaScript prototype completely; no compatibility shim, dual execution mode, npm entry point, or JavaScript API is retained.
10. Rust acceptance tests import the bare `#[trace(...)]` attribute from the pinned `ix-trace-rs` crate so Quire can discover compiler-checked trace bindings statically.

The rejected alternative was retaining the JavaScript prototype. A TypeScript
port was also rejected because the component is eligible for the qualified path.
Moving Quoin orchestration into Rust was rejected because it expands this slice
across an established TypeScript CLI boundary without improving the core contract.

## Risks

- **Schema generation compatibility — owner: implementation owner; review by 2026-10-07.** The selected generator must support the schema's references and conditionals; runtime schema validation remains authoritative if generated typing is less restrictive.
- **Behavioral migration drift — owner: verification owner; review before merge.** Every TC must be reimplemented as a traced Rust test and fixtures must remain semantically equivalent.
- **Unmeasured production throughput — owner: architecture owner; review when production placement is proposed.** No latency or throughput claim is made in this slice; a MeasurementPlan and benchmark threshold are required before placement on a measured hot path.
- **Provisional identity seam — owner: FS02/FS05 owners; review after their accepted decision.** Historical evidence must not be rewritten during a future migration.
