---
id: TC-042
title: "Run generative, metamorphic, differential, and mutation evidence"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-025"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/NFR-004"
    type: "verifies"
---
# TC-042: Run generative, metamorphic, differential, and mutation evidence

## Description

Verify that one schema-derived technique manifest covers every frozen evidence
obligation, uses accepted shared technique and execution interfaces, produces
deterministic cases and shrinking decisions, and preserves every non-success state.

## Test Procedure

1. Freeze a profile, corpus, and oracle manifest containing example, property,
   metamorphic, differential, fuzz, and mutation obligations. Freeze one technique
   manifest containing a closed form for each. In a separate profile variant, replace
   one configuration with one owned gap for the same obligation.
2. Delete and wrong-type every required common and family-specific field. Duplicate or
   conflict configuration/gap identifiers and obligation coverage; name an unknown
   guarantee, catalog technique, input entry, oracle binding, budget, or stopping rule;
   add one field from another family. Change each strict reference to every wrong
   artifact role. Cross every seed, case, shrink, execution, array, and document bound at
   its exact limit and one value beyond it.
3. Run QUOIN `spec-correctness` against the property obligations and `spec-fuzz` against
   the fuzz obligations using retained exact advisor and catalog records. Begin without a
   repository fuzz harness, then establish the owner-selected catalog-declared Rust
   harness and rerun. Retain each runnable grounded Rust target, method-fit finding,
   ungrounded finding, or absent-harness finding. Attempt to replace a finding with a
   hand-written weaker property, private target, or target file absent from the harness
   manifest.
4. Generate cases for example, property, metamorphic, and differential configurations.
   Repeat with the same seed and ordinal, permute every set-like input, change each bound
   dimension separately, and permute an ordered projection and shrink path. Exercise
   zero as the first case ordinal, the last ordinal below each declared limit, the limit
   itself, a leading-zero ordinal, and an unsigned 64-bit overflow.
5. Submit exact candidate, oracle, harness, and comparison requests through the real
   accepted Rust Engineering Assurance producer-execution interface. Consume typed
   completed satisfaction and violation results, then force unavailable, refused,
   failed, timed-out, malformed-response, containment-failure, and unsupported-method
   states separately.
6. For a formal-clause obligation, supply the exact accepted native Quire source-profile
   identity and a separate accepted-or-gap result for every carried-forward admission
   rule. Turn each result into an owned gap in turn and confirm the technique obligation
   carries the same exact rule set. Remove each result in turn. Repeat using only a
   first-party parser identity, and repeat
   with OCL 2.4, SysML v2, and FRETish export targets or their upstream foreign-runtime
   tools as claimed source authorities or differential oracles.
7. For each export-only mapping profile, bind an exact loss report that classifies every
   source obligation as preserved, conditional, unrepresented, or refused. Remove one
   source obligation and change each classification in turn.
8. Force a counterexample, execute a multi-step shrink, and replay both original and
   minimized cases against the exact candidate and applicable oracle. Change the
   candidate, oracle, request, observation, shrink order, and replay outcome one at a
   time.
9. In Task-029, compile every generated target and confirm that source creation records no
   completed run. Through the accepted Engineering Assurance boundary, exhaust an
   unfinished fuzz budget, retain a crash and hang with their original inputs, minimize
   each input, and replay it against the exact candidate and oracle revisions. Change one
   candidate, seed-corpus, harness, tool, budget, stopping, or replay identity at a time.
10. Run each seeded mutant through the accepted catalog-declared Rust mutation producer.
    Retain killed, surviving, timed-out, crashed, unrelated-test-failure, and unfinished
    results separately. For every killed mutant, confirm that the exact assigned
    bare-`ix-trace-rs` Rust test fails for the declared reason; remove that test and change
    the expected failure independently.
11. Permute independent result arrival order and inspect the repository for a Task-028/029
   process runner, arbitrary stdout parser, evidence store, private technique catalog,
   aggregate score, qualification decision, human disposition, or forbidden foreign
   runtime dependency.

## Expected Results

The manifest freezes only with exact complete obligation coverage and one deterministic
JCS identity. Every malformed, missing, unknown, cross-tagged, conflicting, privately
invented, changed, or ordered-input permutation case refuses with its typed code; set-only
permutations remain identical.

The same case inputs produce the same case and shrink identities. Only typed results from
the exact accepted Rust Engineering Assurance library interface contribute. Completed
satisfaction and violation remain separate, every producer failure remains its exact
finding, and an unfinished stopping rule remains incomplete. A minimized counterexample
retains its original and ordered shrink path and contributes only after exact replay.

QUOIN-generated property/fuzz source or its validated harness, method-fit, or grounding
finding remains visible. Target creation is undischarged. Every seeded guard-removal
mutant fails the assigned traced test for the intended reason; survivors, unrelated test
failures, producer crashes or timeouts, hangs, and unfinished budgets block their exact
obligations. Original crash/hang inputs are retained, and minimized inputs contribute only
after exact replay.
Formal-clause evidence contributes only when every applicable native-profile admission
record is present. A missing record remains an owned gap; first-party origin does not
close it. The planner reads the closed claim-authority tag rather than inferring scope
from prose. Export-only mappings retain a complete loss report and neither their emitted
text nor an upstream tool's acceptance becomes source-clause evidence.
Static inspection finds no local execution/evidence framework or private catalog method,
forbidden foreign runtime, or adapter shim. The output makes no sufficiency,
qualification, or human decision.
