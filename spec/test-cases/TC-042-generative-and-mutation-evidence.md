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
   the fuzz obligations. Retain each emitted Rust target or its validated ungrounded
   finding. Attempt to replace an ungrounded finding with a hand-written weaker property
   or private fuzz target.
4. Generate cases for example, property, metamorphic, and differential configurations.
   Repeat with the same seed and ordinal, permute every set-like input, change each bound
   dimension separately, and permute an ordered projection and shrink path.
5. Submit exact candidate, oracle, harness, and comparison requests through the real
   accepted Rust Engineering Assurance producer-execution interface. Consume typed
   completed satisfaction and violation results, then force unavailable, refused,
   failed, timed-out, malformed-response, containment-failure, and unsupported-method
   states separately.
6. For a formal-clause obligation, supply the exact accepted native Quire source-profile
   identity and separate evidence for every carried-forward admission rule. Remove each
   admission record in turn. Repeat using only a first-party parser identity, and repeat
   with OCL 2.4, SysML v2, and FRETish export targets or their upstream foreign-runtime
   tools as claimed source authorities or differential oracles.
7. For each export-only mapping profile, bind an exact loss report that classifies every
   source obligation as preserved, conditional, unrepresented, or refused. Remove one
   source obligation and change each classification in turn.
8. Force a counterexample, execute a multi-step shrink, and replay both original and
   minimized cases against the exact candidate and applicable oracle. Change the
   candidate, oracle, request, observation, shrink order, and replay outcome one at a
   time.
9. In Task-029, exhaust an unfinished fuzz budget, retain a crash and hang, retain a
   surviving mutant, then remove each named qualification-critical guard and execute its
   assigned bare-`ix-trace-rs` Rust test.
10. Permute independent result arrival order and inspect the repository for a Task-028/029
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

QUOIN-generated property/fuzz source or its validated grounding finding remains visible.
Every seeded guard-removal mutant fails the assigned traced test for the intended reason;
survivors, crashes, hangs, and unfinished fuzz budgets block their exact obligations.
Formal-clause evidence contributes only when every applicable native-profile admission
record is present. A missing record remains an owned gap; first-party origin does not
close it. Export-only mappings retain a complete loss report and neither their emitted
text nor an upstream tool's acceptance becomes source-clause evidence.
Static inspection finds no local execution/evidence framework or private catalog method,
forbidden foreign runtime, or adapter shim. The output makes no sufficiency,
qualification, or human decision.
