---
id: FR-017
title: "Retain oracle dependence without aggregation"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-002"
    type: implements
  - target: "ix://agent-ix/quire-verification/FR-006"
    type: depends_on
  - target: "ix://agent-ix/quire-verification/FR-007"
    type: depends_on
  - target: "ix://agent-ix/quire-verification/FR-016"
    type: depends_on
  - target: "ix://agent-ix/quire-specification/FS05"
    type: depends_on
---
# FR-017: Retain Oracle Dependence Without Aggregation

## Description

When a technique result claims a logical outcome, the evidence assessor SHALL
retain the exact oracle contributions and their declared dependence relationships
without converting multiple contributions into a global assurance score.

## Inputs

- Evidence obligations and exact-matched technique results.
- An opaque validation context returned by the one strict shared-reference
  reader for every oracle and observation envelope used by a contribution.
- Oracle contributions carrying a contribution ID, exact oracle and observation
  references, and one relation: `independent`, `shared`, `derived`, or `unknown`.
- A `shared` contribution's nonempty `dependenceGroupId`, or a `derived`
  contribution's exact closed `derivedFrom` pair of `resultId` and
  `contributionId`. Other relations carry neither field.

## Outputs

- One non-authoritative assessment entry per obligation.
- Each selected result's contributions sorted by contribution ID.
- A top-level `oracleDependenceFindings` array. A `correlated-oracle` finding
  contains sorted unique `members` and sorted unique `bases` drawn from
  `declared-group`, `derived`, `observation`, and `oracle`. An
  `oracle-dependence-unknown` finding contains one exact member and reason
  `declared-unknown`, `shared-group-unresolved`, or `source-not-assessed`.

The assessor forms an undirected graph over contributions belonging to every
result that exactly matches at least one obligation. It adds an edge for an
exact-JCS-equal oracle reference, exact-JCS-equal observation reference, equal
declared dependence group, or a derived-source pair. Each connected component
with at least two members emits exactly one `correlated-oracle` finding. A
derived source must resolve uniquely anywhere in the supplied result set before
assessment; if that valid source is outside the exact-matched graph, the derived
member emits `source-not-assessed` instead of an independence claim.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-017-AC-1 | Every TechniqueResult carries an `oracleContributions` array of closed contribution objects. A `satisfied` or `violated` logical outcome requires at least one contribution; other outcomes may carry an empty array. Each contribution requires a nonempty ID, exact oracle and observation references, and one declared relation. `shared` requires only a nonempty `dependenceGroupId`; `derived` requires only a closed exact `derivedFrom` pair; `independent` and `unknown` admit neither conditional field. | Test (TC-033) |
| FR-017-AC-2 | Before result matching or classification, the assessor requires every contribution envelope to match an opaque strict-reader output by exact JCS equality and exact `oracle` or `observation` role. A missing validation output, wrong role, unsupported version, changed reference dimension, or extra contribution field refuses with the existing schema/shared-reference error rather than entering the dependence graph. | Test (TC-033) |
| FR-017-AC-3 | Contribution IDs are unique within each result. Every derived pair resolves to exactly one other supplied contribution, is not a self-reference, and the directed derived graph is acyclic. Duplicate IDs refuse as `duplicate_oracle_contribution_id`; absent, self, or cyclic sources refuse as `invalid_oracle_lineage` before any assessment is returned. | Test (TC-033) |
| FR-017-AC-4 | Distinct technique or result identifiers do not establish independence. Exact-matched contributions connected by an exact shared oracle, observation, declared group, or derived pair emit exactly one `correlated-oracle` finding per connected component, with at least two lexically sorted `(resultId, contributionId)` members and unique lexically sorted bases. | Test (TC-033) |
| FR-017-AC-5 | Relation `unknown`, a `shared` group represented by only one exact-matched member, or a valid derived source outside the exact-matched graph emits one exact `oracle-dependence-unknown` finding with the corresponding declared reason and makes no independence claim. | Test (TC-033) |
| FR-017-AC-6 | Permuting result or contribution input order cannot change the JCS assessment. Each selected result's complete contributions are retained separately in contribution-ID order, and dependence findings are JCS-sorted. | Test (TC-033) |
| FR-017-AC-7 | The assessment preserves each obligation and contribution separately, emits no numeric method score or aggregate assurance level, identifies Quoin as evidence owner, and records sufficiency as `not-made`. | Test (TC-033) |

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-017-CON-1 | The assessor SHALL use the exact independently reviewed FS05 reference roles selected for oracle and observation artifacts. | Compatibility | Test (TC-033) |
| FR-017-CON-2 | The assessor SHALL label independence only as the declared relationship supported by the supplied exact references. | Claim boundary | Inspection (TC-033) |
| FR-017-CON-3 | The assessor SHALL NOT claim undisclosed causal independence. | Claim boundary | Inspection (TC-033) |

## Dependencies

- Exact obligation matching and independent result findings come from [FR-006](./FR-006-match-evidence-exactly.md) and [FR-007](./FR-007-preserve-result-findings.md).
- Exact shared-reference consumption comes from [FR-016](./FR-016-read-shared-references-strictly.md).
- The FS05 representation dependency is satisfied by the exact retained
  `agent-ix/quire-specification@8ab058beff03cc76c0f39cc06c87ef98f76e8d78`
  packet and the independently qualified reader in FR-016/IT-002.
