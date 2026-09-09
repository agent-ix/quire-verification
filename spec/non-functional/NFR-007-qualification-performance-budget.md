---
id: NFR-007
title: "Measure the declared production performance budget"
type: NFR
quality_attribute: performance_efficiency
relationships:
  - target: "ix://agent-ix/quire-verification/FR-026"
    type: "constrains"
---
# NFR-007: Measure the declared production performance budget

## Statement

When a production profile declares a hot-path budget, the qualification campaign SHALL
measure every named phase against that owner-supplied threshold without
inventing a threshold from the VP04 baseline.

## Scope

The measurement covers only the exact caller, candidate, corpus population,
environment, cache policy, concurrency, and phase boundaries in the frozen profile.
Build and download time, cold setup, steady-state execution, and cleanup remain
separate measures.

## Rationale

The production chain may be latency-sensitive, but no owner has yet supplied a
universal numerical SLA. Qualification must wait for the actual profile budget and
must retain distributions and uncertainty rather than promote a sampled mean.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Declared hot-path phases with owner thresholds | 100% | 100% | Inspection (TC-044) |
| Declared phases with retained raw observations and recomputable summaries | 100% | 100% | Test (TC-044) |
| Threshold violations hidden by aggregation | 0 | 0 | Test (TC-044) |
| Unbounded concurrent qualification processes | 0 | 0 | Test (TC-044) |

## Verification

The measurement harness uses [MP-002](../assurance/MP-002-production-qualification.md)
to execute each profile phase with bounded local concurrency, retain raw observations,
recompute the declared estimator, and distinguish setup, hot-path execution, cleanup,
invalid samples, and uncertainty.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-007-AC-1 | A missing owner threshold for any claimed hot-path phase keeps performance qualification incomplete. | Test (TC-044) |
| NFR-007-AC-2 | Every raw observation, exclusion, invalid sample, estimator, uncertainty source, and phase boundary is retained and recomputable. | Test (TC-044) |
| NFR-007-AC-3 | A threshold violation remains visible even when another phase or aggregate passes. | Test (TC-044) |
| NFR-007-AC-4 | Local qualification uses at most two Cargo jobs and serializes high-load phases unless the profile explicitly owns another bounded limit. | Test (TC-044) |

## Dependencies

This requirement constrains [FR-026](../functional/FR-026-execute-production-chain.md)
and uses the exact production profile from FR-022.
