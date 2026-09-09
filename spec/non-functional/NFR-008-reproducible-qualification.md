---
id: NFR-008
title: "Reproduce every qualification identity and result"
type: NFR
quality_attribute: reliability
relationships:
  - target: "ix://agent-ix/quire-verification/FR-027"
    type: "constrains"
---
# NFR-008: Reproduce every qualification identity and result

## Statement

The qualification report SHALL bind exact source, release binary, dependency,
profile, corpus, oracle, environment, command, seed, budget, observation, and
result identities so an independent reviewer can reproduce or explain every result.

## Scope

Reproduction applies to the profile's claimed environments. Differences outside
that set remain explicit exclusions and do not become silent portability claims.

## Rationale

An internally valid report can still refer to unavailable bytes, mutable tools, or
unreproducible generated inputs. Qualification needs retained identities and inputs
that reach beyond the report's JSON into every source artifact it claims.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Required identity dimensions with retained or retrievable exact bytes | 100% | 100% | Test (TC-044) |
| Reproduced result classes matching or carrying explained environmental differences | 100% | 100% | Demonstration (IT-004) |
| Self-consistent report reseals that bypass a changed source artifact | 0 | 0 | Mutation Test (TC-045) |

## Verification

An independent local reproduction rebuilds the exact candidate from its lockfile,
revalidates every artifact digest, replays deterministic cases and minimized
counterexamples, repeats the measurement procedure, and records any stochastic or
environmental difference without rewriting the original report.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-008-AC-1 | Every required source, binary, lockfile, corpus, oracle, environment, seed, command, and result identity resolves to exact retained or authorized retrievable bytes. | Test (TC-044) |
| NFR-008-AC-2 | Deterministic evidence replays exactly; stochastic evidence retains seed, budget, distribution, and explained differences. | Demonstration (IT-004) |
| NFR-008-AC-3 | Changing any retained source artifact invalidates the qualification core even when an attacker recomputes internal report digests. | Mutation Test (TC-045) |
| NFR-008-AC-4 | A reproduction on an unclaimed environment is recorded as exploratory and cannot expand the accepted profile. | Test (TC-044) |

## Dependencies

This requirement constrains [FR-027](../functional/FR-027-bind-qualification-disposition.md)
and reuses the existing JCS and reference boundaries in FR-008 and FR-016.

