---
id: FR-022
title: "Freeze an exact qualification claim"
type: FR
relationships:
  - target: "ix://agent-ix/quire-verification/US-004"
    type: "implements"
  - target: "ix://agent-ix/quire-verification/FR-016"
    type: "depends_on"
---
# FR-022: Freeze an exact qualification claim

## Description

When qualification evidence collection is requested, the qualification planner SHALL
freeze one versioned claim profile naming the candidate, commands, production caller,
input population, semantic guarantees, environment, resource budgets, required
evidence, and explicit exclusions.

## Inputs

- Exact candidate and dependency-closure references.
- Owner-authored command, caller, population, guarantee, claim-authority, environment,
  and budget definitions.
- Accepted FS02/FS05 reference versions for every used role.
- Required evidence techniques, budget assignments, stopping rules, and decision
  procedure.

## Outputs

- One immutable qualification-profile artifact with a JCS identity.
- Actionable prerequisite records for every unresolved external interface represented
  by an otherwise structurally complete profile candidate.

## Behavior

- The planner SHALL reject an empty or open-ended command, population, guarantee,
  environment, budget, or exclusion field.
- If an external interface revision is missing, provisional, unsupported, or not
  accepted for the selected role, then the planner SHALL retain an exact blocked
  prerequisite and SHALL NOT substitute a local schema.
- Candidate, dependency-closure, caller, and environment references are intrinsic
  profile identity. Their absence refuses profile construction; the verification owner
  records that prerequisite before invoking the planner rather than constructing a
  partial identity or substituting bytes.
- Accepted portable-contract, shared-reference, Engineering Assurance, and Quoin
  interfaces SHALL name `source` artifacts.
- The accepted parser/compiler interface SHALL name an `executable-projection`.
- The accepted production-caller interface SHALL name an `invocation`.
- The planner SHALL treat syntax, import resolution, model binding, type checking,
  evaluation, backend projection, and production integration as distinct claim
  dimensions.
- Each semantic guarantee SHALL carry exactly one closed `claimAuthority` form:
  `non-formal`, `native-formal-clause`, or `export-mapping`.
- A `non-formal` authority SHALL carry only its tag and a nonempty reason why the
  guarantee makes no formal-clause claim.
- A `native-formal-clause` authority SHALL carry only its tag, the exact accepted native
  Quire `sourceProfileRef`, and exactly one closed admission result for each required
  rule. An `accepted` result SHALL carry only the rule, status, and one strict-reader-
  proved `observation` reference. A `gap` result SHALL carry only the rule, status,
  owner, nonempty reason, and nonempty unique limitations.
- The required native admission rules SHALL be `numeric-domain`,
  `presence-null-invalid`, `containment-references`, `collections`, `calls-result`,
  `source-authority`, `evaluation-instants`, `reference-design-gate`,
  `temporal-boundary`, and `runtime-dependency`.
- An `export-mapping` authority SHALL carry only its tag, an exact `mappingProfileRef`,
  and an exact `lossReportRef` whose artifact roles are respectively `source` and
  `observation`.
- The planner SHALL refuse a missing, duplicate, unknown, open, or wrong-role authority,
  admission result, source-profile, mapping-profile, or loss-report value. It SHALL
  preserve every admission gap and SHALL NOT infer acceptance from the presence of an
  evidence reference.
- The planner SHALL NOT infer native-profile admission from the candidate's first-party
  origin, parser/compiler interface, or executable reference.
- The planner SHALL NOT treat an export mapping as an editable clause authority or as
  evidence that source-clause semantics are preserved beyond its exact loss report.
- Each evidence requirement SHALL bind exactly one declared `budgetId` and one
  `guaranteeId` to its exact technique and stopping rule.
- The planner SHALL refuse an evidence requirement whose budget or guarantee is absent
  from the same profile.
- The JCS profile identity SHALL change when any identity-bearing claim dimension changes.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-022-CON-1 | The claim SHALL use the one strict shared-reference reader without duplicating FS02/FS05 definitions. | Compatibility | Test (TC-039) |
| FR-022-CON-2 | The profile SHALL NOT infer a production guarantee from the presence of a command or catalog technique. | Claims | Test (TC-039) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-022-AC-1 | A complete exact profile retains every declared claim dimension, closed claim authority, selected external revision, evidence obligation with its exact guarantee and budget assignment, and exclusion under one reproducible JCS identity. | Test (TC-039) |
| FR-022-AC-2 | Omitting or opening any command, caller, population, guarantee, claim authority, native admission, mapping loss report, environment, budget, evidence, evidence-to-budget assignment, or exclusion dimension refuses profile emission. | Test (TC-039) |
| FR-022-AC-3 | An unavailable or unaccepted external interface becomes a named blocked prerequisite without a copied local replacement. | Test (TC-039) |
| FR-022-AC-4 | Changing one identity-bearing dimension changes the profile identity while the previous profile remains immutable. | Test (TC-039) |
| FR-022-AC-5 | A non-formal guarantee records its exclusion reason; a native formal-clause guarantee binds every exact admission rule as accepted evidence or an owned gap without inferring coverage; and an export-mapping guarantee binds its exact mapping profile and complete loss report without becoming source authority. | Test (TC-039) |

## Dependencies

The profile consumes [FR-016](./FR-016-read-shared-references-strictly.md) and
constrains all later qualification evidence.
