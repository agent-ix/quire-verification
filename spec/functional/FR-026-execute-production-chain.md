---
id: FR-026
title: "Execute the declared production chain"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-004"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-022"
    type: "depends_on"
  - target: "ix://agent-ix/quire-verification/FR-025"
    type: "depends_on"
---
# FR-026: Execute the declared production chain

## Description

When every producer interface selected by the qualification profile is accepted at
an exact revision, the integration runner SHALL execute the real declared caller,
`quire-spec` command path, portable result boundary, and assurance handoff without
substituting a test-only transport or duplicated producer implementation.

## Inputs

- Frozen profile and exact accepted producer/consumer references.
- Real caller request and retained corpus artifacts.
- Versioned portable contracts and assurance handoff.

## Outputs

- End-to-end run, observation, result, and handoff references.
- Typed blocked or failed results at the exact boundary that refused.

## Behavior

- The runner SHALL verify every interface version and artifact identity before invocation.
- If Agent A, Agent C, contract-agent-c, or another producer has not delivered an
  accepted required interface, then the runner SHALL remain blocked without creating
  a local substitute.
- Test-only adapters SHALL NOT stand in for the selected production caller in a
  production-chain qualification result.
- A boundary rejection, transport failure, stale dependency, or lost result SHALL
  identify the responsible seam and preserve upstream observations.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-026-CON-1 | `quire-verification` SHALL own only portable planning/result semantics without absorbing parser/compiler, Engineering Assurance, Quoin orchestration, or evidence-store behavior. | Ownership | Inspection (TC-043) |
| FR-026-CON-2 | Cross-repository execution SHALL use versioned structured interfaces and exact revisions. | Compatibility | Test (TC-043) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-026-AC-1 | The selected real caller reaches the exact candidate and returns through the declared result and assurance boundaries with every reference intact. | Test (TC-043) |
| FR-026-AC-2 | Test-only transport, changed interface version, changed candidate, stale dependency, missing handoff, or copied producer semantics refuses qualification. | Test (TC-043) |
| FR-026-AC-3 | Each failed or blocked seam identifies its owner and retains all observations completed before that seam. | Test (TC-043) |
| FR-026-AC-4 | The result claims only the semantic stages actually executed by the selected production chain. | Test (TC-043) |

## Dependencies

Execution requires [FR-022](./FR-022-freeze-qualification-claim.md), the completed
evidence in [FR-025](./FR-025-run-qualification-techniques.md), and accepted exact
interfaces from their existing owners.
