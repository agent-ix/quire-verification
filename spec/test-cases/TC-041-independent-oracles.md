---
id: TC-041
title: "Classify oracle independence and disagreement"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-024"
    type: "verifies"
---
# TC-041: Classify oracle independence and disagreement

## Description

Verify that one schema-derived oracle manifest covers every frozen guarantee with exact
bounded bindings or owned no-oracle gaps, preserves dependence honestly, and treats
agreement, disagreement, execution failure, and reuse drift as distinct outcomes.

## Test Procedure

1. Freeze a multi-guarantee profile and corpus, then freeze a manifest containing
   multiple independent bindings for one guarantee and one owned no-oracle gap for
   another. Exercise both input-projection modes; permute set-like binding, proof, and
   observation inputs and recompute identity without reordering an ordered projection.
2. Delete or duplicate each guarantee coverage record; bind and gap one guarantee;
   name an unknown guarantee, an unretained corpus entry, empty limitations, or a changed
   profile/corpus identity.
3. Exercise every closed independent, shared, derived, and unknown dependence form.
   Remove independence evidence; intersect exact candidate/oracle ancestry; reuse the
   candidate executable, oracle, comparison procedure, or candidate-origin expected
   value while declaring independence; remove each shared field; inject a foreign tagged
   field; and build absent, self, and cyclic derivations.
4. Compile against the exact Engineering Assurance producer-execution revision selected
   by the frozen profile. Through that Rust interface, submit the exact oracle and
   comparison invocation references, bind each canonical result and retained output to
   the retained corpus projection, and consume one agreeing and one disagreeing
   observation. This real downstream execution also supplies Engineering Assurance
   TC-126 consumer evidence. Change each binding and reuse dimension one at a time,
   including Engineering Assurance interface/request/result identity, oracle and
   comparison executable digest, projection, expected relation, limitation,
   observations, and manifest identity.
5. Make the Rust Engineering Assurance test producer return unavailable, refused,
   failed, timed-out, malformed-response, containment-failure, cancelled, and completed
   states. Assert that every non-completed state retains its exact closed category and
   supplies no observation. Supply a disagreement procedure record for a different
   procedure and then for the exact declared procedure; require its retained result
   bytes, attribution or bound, and absence of any qualification disposition.

## Expected Results

The complete manifest freezes with one deterministic JCS identity; permutations produce
that same identity. Every deletion, duplicate, conflicting coverage state, unknown
guarantee, unretained input, empty limitation, changed identity, or invalid dependence
refuses with `invalid_qualification_oracle_manifest`.

Only typed results from the exact accepted Rust Engineering Assurance library interface
whose compiled revision matches the frozen profile, canonical request and result
references, strict-reader-bound executable, and retained projection contribute.
Non-completed producer states retain their typed state and create no observation.
Agreement supports only the declared relation. Disagreement
retains both exact observations and stays unresolved; its exact disagreement procedure
may attribute or bound the difference without issuing a qualification disposition. A
no-oracle guarantee remains an owned gap, and
candidate-produced or shared-source evidence never becomes independent. Static inspection
finds no Task-027 process runner, evidence store, fixture script, or arbitrary stdout
parser in this repository.
