---
id: SR-139
title: "Rust review of VP03 semantic-gap remediation"
type: SpecReview
analysis: code-review
scope: "quire-verification 8fdfc8e..15a76f0; tests/verification.rs TC-033"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-verification/FR-017", type: reviews }
  - { target: "ix://agent-ix/quire-verification/TC-033", type: references }
---

## Summary

The exact `/home/peter/dev/agent-skills/rust-review/SKILL.md` checklist was
reapplied after QUOIN semantic gap-analysis found weak TC-033 discrimination.
The test-only remediation at `15a76f0` resolves the finding and introduces no
production, wire, dependency, or executable-language change.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: TC-033 previously checked dependence kinds and bases but did not assert exact correlated members, unknown-member/reason pairs, or an observable correlated graph in which one result satisfied two obligations. Incorrect member attribution or duplicate selected-result membership could therefore pass. The test now asserts the exact seven-member component, all three exact unknown mappings, and a two-member component despite three selecting obligations. | FR-017-AC-4..FR-017-AC-6, TC-033, `tests/verification.rs` |
| FND-002 | medium | Resolved: the schema test did not exercise every forbidden relation-field pairing, and the strict-reader test mutated only the oracle path. Shared-plus-derived, derived-plus-group, independent/unknown-plus-either-conditional-field, both reference roles, and unsupported envelope/reference versions now have discriminating refusals. | FR-017-AC-1, FR-017-AC-2, TC-033, `tests/verification.rs` |

## Review checklist

The new assertions execute the real schema compiler, strict shared-reference
reader, evidence assessor, lineage validator, and dependence graph. They do not
mock a collaborator, inspect source text, depend on a clock or hash-map order,
introduce a test-only production branch, or merely compare values set by the
test. Each case fails under a concrete violation of its traced criterion. The
four existing compiler-checked `#[trace(...)]` attributes still bind every
FR-017 criterion through TC-033.

No first-party `unsafe`, panic surface, public API, numeric conversion, blocking
operation, resource loop, lint allowance, placeholder, or external dependency
was added. Formatting and strict all-target Clippy pass. The targeted TC-033
lane passes 4/4, and the complete workspace passes 48/48 first-party plus 3/3
vendored Rust tests.

## Gate results

```text
$ cargo fmt --all --check
(no output; exit 0)

$ cargo clippy --workspace --all-targets --all-features --locked -j 2 -- -D warnings
Checking quire-verification v0.1.0
Finished `dev` profile; exit 0

$ cargo test --locked -j 2 --test verification tc_033
4 passed; 0 failed

$ cargo test --workspace --all-features --locked -j 2
48 first-party passed; 0 failed
3 vendored ix-trace-rs passed; 0 failed
```
